use crate::domain::{
    address::{Day, Part},
    solution::aoc_verdict::AocVerdict,
};
use anyhow::Context;
use reqwest::{Url, blocking::Client};

/// Env var holding the adventofcode.com session cookie.
const COOKIE_KEY: &str = "COOKIE";

/// Marks a puzzle part on a day page. Two means part two is unlocked.
const ARTICLE: &str = r#"<article class="day-desc">"#;

const AOC_BASE_URL: &str = "https://adventofcode.com";

/// Reads the session cookie without building a client.
///
/// Checking which session an input came from needs the cookie but no requests.
pub fn cookie_from_env() -> anyhow::Result<String> {
    cookie_if_set()?.with_context(|| format!("{COOKIE_KEY} is not set"))
}

/// The session cookie, or `None` when it is unset or blank.
///
/// Errors only when it is set to something unusable, so a caller that can work
/// offline is not told to go offline by a typo.
pub fn cookie_if_set() -> anyhow::Result<Option<String>> {
    // `.env` is optional: values may already live in the real environment.
    dotenvy::dotenv().ok();
    match std::env::var(COOKIE_KEY) {
        Ok(cookie) if cookie.trim().is_empty() => Ok(None),
        Ok(cookie) => Ok(Some(cookie)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(e) => Err(e).with_context(|| format!("{COOKIE_KEY} is set but unreadable")),
    }
}

/// An authenticated handle to adventofcode.com, pooling one client.
pub struct AocClient {
    cookie: String,
    user_agent: String,
    client: Client,
}

impl AocClient {
    pub fn cookie(&self) -> &str {
        &self.cookie
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// How this tool identifies itself, from `REPO_URL` and `CONTACT`.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Reads configuration from the environment, loading `.env` if present.
    ///
    /// Only `COOKIE` is required, so a fresh clone runs without the rest.
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            cookie: cookie_from_env()?,
            user_agent: super::user_agent_from_env(),
            client: Client::new(),
        })
    }

    /// Fetches `day`'s puzzle text, rendered from HTML.
    ///
    /// Part two comes back `None` until part one is solved.
    pub fn get_instructions(&self, day: &Day) -> anyhow::Result<(String, Option<String>)> {
        let html = self
            .client
            .get(Url::parse(AOC_BASE_URL)?.join(&format!("{}/day/{}", day.year(), day.value()))?)
            .header("User-Agent", self.user_agent())
            .header("Cookie", format!("session={}", self.cookie()))
            .send()
            .with_context(|| format!("failed to reach AOC for {day:?}"))?
            .error_for_status()
            .with_context(|| format!("bad response status for {day:?}"))?
            .text()
            .with_context(|| format!("failed to read page body for {day:?}"))?;

        let mut parts = html.split(ARTICLE).skip(1).map(|part| {
            let body = part.split("</article>").next().unwrap_or_default();
            html2text::from_read(body.as_bytes(), 80)
                .with_context(|| format!("failed to render instructions for {day:?}"))
        });

        let one = parts
            .next()
            .with_context(|| format!("no puzzle text found for {day:?}"))??;
        let two = parts.next().transpose()?;
        Ok((one, two))
    }

    /// Fetches the raw puzzle input for `day`, verbatim.
    ///
    /// A non-success status usually means a bad cookie or an unreleased day.
    pub fn get_input(&self, day: &Day) -> anyhow::Result<String> {
        self.client
            .get(Url::parse(AOC_BASE_URL)?.join(&format!(
                "{}/day/{}/input",
                day.year(),
                day.value()
            ))?)
            .header("User-Agent", self.user_agent())
            .header("Cookie", format!("session={}", self.cookie()))
            .send()
            .with_context(|| {
                format!(
                    "failed to reach AOC for year: {} and day: {}",
                    day.year(),
                    day.value()
                )
            })?
            .error_for_status()
            .with_context(|| {
                format!(
                    "bad response status for year: {} and day: {}",
                    day.year(),
                    day.value()
                )
            })?
            .text()
            .with_context(|| {
                format!(
                    "failed to read input body for year: {} and day: {}",
                    day.year(),
                    day.value()
                )
            })
    }

    /// Submits `answer` to adventofcode.com and reads the graded reply.
    ///
    /// Everything is a 200, so the verdict comes entirely from the body. A part
    /// grades once, which is why a correct answer is worth caching. A wrong one
    /// may or may not come with a direction hint.
    pub fn submit_answer(
        &self,
        day: &Day,
        part: Part,
        answer: impl AsRef<str>,
    ) -> anyhow::Result<AocVerdict> {
        let path = format!("/{}/day/{}/answer", day.year(), day.value());
        let url = Url::parse(AOC_BASE_URL)?.join(&path)?;
        let form = [("level", part.wire_value()), ("answer", answer.as_ref())];

        let body = self
            .client
            .post(url)
            .header("User-Agent", self.user_agent())
            .header("Cookie", format!("session={}", self.cookie()))
            .form(&form)
            .send()
            .with_context(|| format!("failed to reach AOC for {day:?}"))?
            .error_for_status()
            .with_context(|| format!("bad response status for {day:?}"))?
            .text()
            .with_context(|| format!("failed to read submit body for {day:?}"))?;

        Ok(verdict_from(&body))
    }
}

/// Classifies AOC's HTML reply to a submission.
///
/// Direction is checked before the generic wrong-answer phrase, since "too
/// high" replies contain that phrase too. Strings verified live; see
/// `context/references.md`.
fn verdict_from(body: &str) -> AocVerdict {
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
    AocVerdict::Incorrect
}

/// Pulls the remaining wait out of a cooldown reply, e.g. `1m 0s`.
fn wait_from(body: &str) -> String {
    body.split_once("You have ")
        .and_then(|(_, rest)| rest.split_once(" left to wait"))
        .map(|(wait, _)| wait.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use crate::domain::solution::aoc_verdict::AocVerdict;

    use super::{verdict_from, wait_from};

    // Fixtures are the real replies AOC gave for 2015 day 1 on a scratch
    // account, trimmed to the sentence that carries the verdict.
    const CORRECT: &str = "That's the right answer!  You are <span class=\"day-success\">one gold star</span> closer to powering the weather machine.";
    const HIGH: &str = "That's not the right answer; your answer is too high.  If you're stuck, make sure you're using the full input data";
    const LOW: &str = "That's not the right answer; your answer is too low.  If you're stuck, make sure you're using the full input data";
    const WRONG: &str = "That's not the right answer.  If you're stuck, make sure you're using the full input data; there are also some general tips";
    const COOLDOWN: &str = "You gave an answer too recently; you have to wait after submitting an answer before trying again.  You have 1m 0s left to wait.";
    const SOLVED: &str =
        "You don't seem to be solving the right level.  Did you already complete it?";

    #[test]
    fn classifies_replies() {
        assert!(matches!(verdict_from(CORRECT), AocVerdict::Correct));
        assert!(matches!(verdict_from(HIGH), AocVerdict::High));
        assert!(matches!(verdict_from(LOW), AocVerdict::Low));
        assert!(matches!(verdict_from(WRONG), AocVerdict::Incorrect));
        assert!(matches!(verdict_from(SOLVED), AocVerdict::AlreadySolved));
        assert!(matches!(verdict_from(COOLDOWN), AocVerdict::Cooldown(_)));
    }

    /// A directional reply also contains the generic phrase, so order matters.
    #[test]
    fn direction_beats_generic() {
        assert!(HIGH.contains("That\'s not the right answer"));
        assert!(matches!(verdict_from(HIGH), AocVerdict::High));
    }

    #[test]
    fn extracts_wait() {
        assert_eq!(wait_from(COOLDOWN), "1m 0s");
        assert_eq!(wait_from("nothing here"), "unknown");
    }
}
