use std::cell::Cell;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Context;
use reqwest::blocking::{Client, Response};
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{StatusCode, Url};

use crate::domain::address::{Day, Part};
use crate::domain::solution::aoc_verdict::AocVerdict;
use crate::outbound::client::environment::Environment;
use crate::outbound::client::session_cookie::SessionCookie;

/// Marks a puzzle part on a day page. Two means part two is unlocked.
const ARTICLE_OPEN_TAG: &str = r#"<article class="day-desc">"#;

/// Column the rendered puzzle text wraps at, a terminal's default width.
const INSTRUCTIONS_WIDTH: usize = 80;

const AOC_BASE_URL: &str = "https://adventofcode.com";

/// Least time between two requests. adventofcode.com throttles a client that
/// fetches back to back (a 405, then a 403 for a while), and asks for one
/// request per puzzle at a human pace.
const REQUEST_GAP: Duration = Duration::from_millis(1500);

/// Most characters of an unrecognized submit reply worth printing.
const SNIPPET_CHARS: usize = 60;

/// An authenticated handle to adventofcode.com, pooling one client.
#[derive(Debug)]
pub(crate) struct AocClient {
    client: Client,
    /// When the last request went out, for [`REQUEST_GAP`].
    last_request: Cell<Option<Instant>>,
}

impl AocClient {
    /// Builds a client carrying the cookie and `User-Agent` on every request.
    ///
    /// Only `COOKIE` is required, so a fresh clone runs without the rest. Both
    /// headers go on the client, so nothing can send a request without them.
    /// Sends nothing, but a missing or malformed cookie fails here, before any
    /// work is done.
    pub(crate) fn from_env() -> anyhow::Result<Self> {
        Self::new(&Environment::cookie()?, &Environment::user_agent())
    }

    /// Builds a client sending `cookie` and `user_agent` on every request.
    fn new(cookie: &SessionCookie, user_agent: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
        headers.insert(COOKIE, cookie_header(cookie)?);
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self {
            client,
            last_request: Cell::new(None),
        })
    }

    /// Waits out the rest of [`REQUEST_GAP`] since the last request.
    fn pace(&self) {
        if let Some(last) = self.last_request.get() {
            let since = last.elapsed();
            if since < REQUEST_GAP {
                thread::sleep(REQUEST_GAP.saturating_sub(since));
            }
        }
        self.last_request.set(Some(Instant::now()));
    }

    /// Fetches `day`'s puzzle text, rendered from HTML.
    ///
    /// Part two comes back `None` until part one is solved.
    pub(crate) fn fetch_instructions(&self, day: &Day) -> anyhow::Result<(String, Option<String>)> {
        self.pace();
        let html = self
            .client
            .get(Url::parse(AOC_BASE_URL)?.join(&format!("{}/day/{}", day.year(), day.value()))?)
            .send()
            .with_context(|| format!("failed to reach AOC for {day}"))
            .and_then(|response| accepted(response, day))?
            .text()
            .with_context(|| format!("failed to read page body for {day}"))?;

        let mut parts = html.split(ARTICLE_OPEN_TAG).skip(1).map(|part| {
            let body = part.split("</article>").next().unwrap_or_default();
            html2text::from_read(body.as_bytes(), INSTRUCTIONS_WIDTH)
                .with_context(|| format!("failed to render instructions for {day}"))
        });

        let one = parts
            .next()
            .with_context(|| format!("no puzzle text found for {day}"))??;
        let two = parts.next().transpose()?;
        Ok((one, two))
    }

    /// Fetches the raw puzzle input for `day`, verbatim.
    ///
    /// A non-success status usually means a bad cookie or an unreleased day.
    pub(crate) fn fetch_input(&self, day: &Day) -> anyhow::Result<String> {
        self.pace();
        self.client
            .get(Url::parse(AOC_BASE_URL)?.join(&format!(
                "{}/day/{}/input",
                day.year(),
                day.value()
            ))?)
            .send()
            .with_context(|| format!("failed to reach AOC for {day}"))
            .and_then(|response| accepted(response, day))?
            .text()
            .with_context(|| format!("failed to read input body for {day}"))
    }

    /// Submits `answer` to adventofcode.com and reads the graded reply.
    ///
    /// Everything is a 200, so the verdict comes entirely from the body. A
    /// part grades once, which is why a correct answer is worth caching.
    pub(crate) fn submit_answer(
        &self,
        day: &Day,
        part: Part,
        answer: impl AsRef<str>,
    ) -> anyhow::Result<AocVerdict> {
        let path = format!("/{}/day/{}/answer", day.year(), day.value());
        let url = Url::parse(AOC_BASE_URL)?.join(&path)?;
        let form = [("level", part.wire_value()), ("answer", answer.as_ref())];

        self.pace();
        let body = self
            .client
            .post(url)
            .form(&form)
            .send()
            .with_context(|| format!("failed to reach AOC for {day}"))
            .and_then(|response| accepted(response, day))?
            .text()
            .with_context(|| format!("failed to read submit body for {day}"))?;

        Ok(verdict_from(&body))
    }
}

/// The `Cookie` header carrying `cookie` as AOC's `session`, marked sensitive
/// so its `Debug` prints `Sensitive` rather than the secret. The bytes sent
/// are unchanged.
fn cookie_header(cookie: &SessionCookie) -> anyhow::Result<HeaderValue> {
    let mut header = HeaderValue::from_str(&format!("session={cookie}"))?;
    header.set_sensitive(true);
    Ok(header)
}

/// The response if its status is a success, else an error that says what the
/// status means. 403 and 405 are adventofcode.com refusing a client it has
/// taken for a scraper; the fix is to wait, not to retry.
fn accepted(response: Response, day: &Day) -> anyhow::Result<Response> {
    let status = response.status();
    response.error_for_status().with_context(|| match status {
        StatusCode::FORBIDDEN | StatusCode::METHOD_NOT_ALLOWED => format!(
            "adventofcode.com refused {day} with HTTP {status}: it throttles bulk fetching. \
             Wait a while, then fetch one year at a time (`fetch -y <year>`)"
        ),
        _ => format!("bad response status for {day}"),
    })
}

/// Classifies AOC's HTML reply to a submission.
///
/// Direction is checked before the generic wrong-answer phrase, since "too
/// high" replies contain that phrase too. A logged-out submission redirects to
/// the puzzle page, which grades nothing, so that is checked first. A reply
/// matching none of them is [`AocVerdict::Unrecognized`], never a guess.
/// Strings verified live; see `context/references.md`.
fn verdict_from(body: &str) -> AocVerdict {
    if body.contains("auth/login") {
        return AocVerdict::NotLoggedIn;
    }
    if body.contains("That's the right answer") {
        return AocVerdict::Correct;
    }
    if body.contains("your answer is too high") {
        return AocVerdict::High;
    }
    if body.contains("your answer is too low") {
        return AocVerdict::Low;
    }
    if body.contains("You don't seem to be solving the right level") {
        return AocVerdict::AlreadySolved;
    }
    if body.contains("You gave an answer too recently") {
        return AocVerdict::Cooldown(wait_from(body));
    }
    if body.contains("That's not the right answer") {
        return AocVerdict::Incorrect;
    }
    AocVerdict::Unrecognized(snippet_from(body))
}

/// The start of a reply's message as plain text: the `<article>` if there is
/// one, tags dropped, whitespace collapsed, cut at [`SNIPPET_CHARS`].
fn snippet_from(body: &str) -> String {
    let message = body
        .split_once("<article>")
        .and_then(|(_, rest)| rest.split_once("</article>"))
        .map_or(body, |(inner, _)| inner);

    let mut text = String::new();
    let mut in_tag = false;
    for c in message.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => text.push(c),
            _ => {}
        }
    }

    let words: Vec<&str> = text.split_whitespace().collect();
    let text = words.join(" ");
    if text.chars().count() <= SNIPPET_CHARS {
        return text;
    }
    let cut: String = text.chars().take(SNIPPET_CHARS).collect();
    format!("{}...", cut.trim_end())
}

/// Pulls the remaining wait out of a cooldown reply, e.g. `1m 0s`.
fn wait_from(body: &str) -> Option<String> {
    body.split_once("You have ")
        .and_then(|(_, rest)| rest.split_once(" left to wait"))
        .map(|(wait, _)| wait.trim().to_string())
}

/// An [`AocClient`] built on first use, so nothing offline pays for one.
///
/// Holding one does not mean a cookie exists: that is checked the first time
/// [`LazyAocClient::connected`] runs, which for a fully cached run is never.
#[derive(Debug, Default)]
pub(crate) struct LazyAocClient(Option<AocClient>);

impl LazyAocClient {
    /// The client, built on the first call. Idempotent, so calling it eagerly
    /// and again later costs one construction.
    pub(crate) fn connected(&mut self) -> anyhow::Result<&AocClient> {
        let client = match self.0.take() {
            Some(client) => client,
            None => AocClient::from_env()?,
        };
        Ok(self.0.insert(client))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real replies AOC gave for 2015 day 1, trimmed to the verdict sentence.
    const CORRECT: &str = "That's the right answer!  You are <span class=\"day-success\">one gold star</span> closer to powering the weather machine.";
    const HIGH: &str = "That's not the right answer; your answer is too high.  If you're stuck, make sure you're using the full input data";
    const LOW: &str = "That's not the right answer; your answer is too low.  If you're stuck, make sure you're using the full input data";
    const WRONG: &str = "That's not the right answer.  If you're stuck, make sure you're using the full input data; there are also some general tips";
    const COOLDOWN: &str = "You gave an answer too recently; you have to wait after submitting an answer before trying again.  You have 1m 0s left to wait.";
    const SOLVED: &str =
        "You don't seem to be solving the right level.  Did you already complete it?";
    // The nav of the puzzle page a logged-out submission redirects to.
    const LOGGED_OUT: &str = "<li><a href=\"/2016/auth/login\">[Log In]</a></li></ul></nav></div><div><h1 class=\"title-event\">";

    #[test]
    fn classifies_replies() {
        assert!(matches!(verdict_from(CORRECT), AocVerdict::Correct));
        assert!(matches!(verdict_from(HIGH), AocVerdict::High));
        assert!(matches!(verdict_from(LOW), AocVerdict::Low));
        assert!(matches!(verdict_from(WRONG), AocVerdict::Incorrect));
        assert!(matches!(verdict_from(SOLVED), AocVerdict::AlreadySolved));
        assert!(matches!(verdict_from(COOLDOWN), AocVerdict::Cooldown(_)));
        assert!(matches!(verdict_from(LOGGED_OUT), AocVerdict::NotLoggedIn));
    }

    /// Only the generic wrong-answer phrase means incorrect. Anything else is
    /// shown rather than guessed at, with the message trimmed out of the page.
    #[test]
    fn an_unknown_reply_is_unrecognized_not_incorrect() {
        let body = "<html><main><article><p>Something   new\n happened.  <a href=\"/2015\">[Return]</a></p></article></main></html>";
        assert_eq!(
            verdict_from(body),
            AocVerdict::Unrecognized("Something new happened. [Return]".to_string())
        );
    }

    /// A long reply is cut short, so it fits on the part's line.
    #[test]
    fn an_unrecognized_snippet_is_short() {
        let body = "word ".repeat(100);
        let AocVerdict::Unrecognized(snippet) = verdict_from(&body) else {
            panic!("an unknown reply is unrecognized");
        };
        assert!(snippet.chars().count() <= SNIPPET_CHARS + 3, "{snippet}");
        assert!(snippet.ends_with("..."));
    }

    fn cookie() -> (String, SessionCookie) {
        let value = "a1b2c3d4".repeat(16);
        let cookie = SessionCookie::try_from(value.as_str()).unwrap();
        (value, cookie)
    }

    /// Anything that prints the client's default headers with `{:?}` shows
    /// the cookie's as `Sensitive`, never the secret.
    #[test]
    fn the_cookie_header_is_sensitive() {
        let (value, cookie) = cookie();
        let header = cookie_header(&cookie).unwrap();

        assert!(header.is_sensitive());
        assert!(!format!("{header:?}").contains(&value));
        assert_eq!(header.to_str().unwrap(), format!("session={value}"));
    }

    #[test]
    fn debug_never_prints_the_cookie() {
        let (value, cookie) = cookie();
        let client = AocClient::new(&cookie, "rustmas test").unwrap();
        assert!(!format!("{client:?}").contains(&value));
    }

    /// A graded reply never carries the login link, so no verdict is stolen.
    #[test]
    fn logged_out_does_not_shadow_a_real_verdict() {
        for body in [CORRECT, HIGH, LOW, WRONG, COOLDOWN, SOLVED] {
            assert!(!matches!(verdict_from(body), AocVerdict::NotLoggedIn));
        }
    }

    /// A directional reply also contains the generic phrase, so order matters.
    #[test]
    fn direction_beats_generic() {
        assert!(HIGH.contains("That\'s not the right answer"));
        assert!(matches!(verdict_from(HIGH), AocVerdict::High));
    }

    #[test]
    fn extracts_wait() {
        assert_eq!(wait_from(COOLDOWN).as_deref(), Some("1m 0s"));
        assert_eq!(wait_from("nothing here"), None);
    }
}
