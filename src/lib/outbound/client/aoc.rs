use crate::{
    domain::address::{Day, Part},
    outbound::client::verdict::Verdict,
};
use anyhow::Context;
use reqwest::{Url, blocking::Client};

/// Env var holding the adventofcode.com session cookie.
const COOKIE_KEY: &str = "COOKIE";

const AOC_BASE_URL: &str = "https://adventofcode.com";

/// An authenticated handle to adventofcode.com, with a pooled client shared
/// across requests. Build one with [`Session::from_env`].
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

    /// How this tool identifies itself to AOC, which asks automated clients to
    /// be reachable. Built from `REPO_URL` and `CONTACT`.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Reads configuration from the environment, loading `.env` if present.
    ///
    /// `COOKIE` is required. `REPO_URL` and `CONTACT` are optional and only
    /// shape the `User-Agent`, so a fresh clone still runs without them.
    pub fn from_env() -> anyhow::Result<Self> {
        // `.env` is optional: values may already live in the real environment.
        dotenvy::dotenv().ok();

        let user_agent = super::user_agent_from_env();

        Ok(Self {
            cookie: std::env::var(COOKIE_KEY)
                .with_context(|| format!("failed to get {COOKIE_KEY}"))?,
            user_agent,
            client: Client::new(),
        })
    }

    /// Fetches the raw puzzle input for `day`, verbatim.
    ///
    /// Errors on a non-success status, which usually means a bad cookie or an
    /// unreleased day.
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
    /// AOC answers 200 for everything, wrong answers included, so the verdict
    /// comes entirely from the body. It also grades a part only once: after that
    /// it returns [`Verdict::AlreadySolved`] rather than confirming again, which
    /// is why a correct answer is worth caching.
    ///
    /// A direction hint is optional. A wrong answer may come back as
    /// [`Verdict::High`] or [`Verdict::Low`], or just [`Verdict::Incorrect`].
    pub fn submit_answer(
        &self,
        day: &Day,
        part: Part,
        answer: impl AsRef<str>,
    ) -> anyhow::Result<Verdict> {
        let path = format!("/{}/day/{}/answer", day.year(), day.value());
        let url = Url::parse(AOC_BASE_URL)?.join(&path)?;
        let form = [("level", part.to_wire_value()), ("answer", answer.as_ref())];

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
/// Every reply is a 200, so the body is the only signal. Direction is checked
/// before the generic wrong-answer phrase, since "too high" replies contain
/// that phrase too. Strings verified live against 2015 day 1 on a scratch
/// account; see `context/references.md`.
fn verdict_from(body: &str) -> Verdict {
    if body.contains("That's the right answer") {
        return Verdict::Correct;
    }
    if body.contains("your answer is too high") {
        return Verdict::High;
    }
    if body.contains("your answer is too low") {
        return Verdict::Low;
    }
    if body.contains("You don't seem to be solving the right level") {
        return Verdict::AlreadySolved;
    }
    if body.contains("You gave an answer too recently") {
        return Verdict::Cooldown(wait_from(body));
    }
    Verdict::Incorrect
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
    use super::{verdict_from, wait_from};
    use crate::outbound::client::verdict::Verdict;

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
        assert!(matches!(verdict_from(CORRECT), Verdict::Correct));
        assert!(matches!(verdict_from(HIGH), Verdict::High));
        assert!(matches!(verdict_from(LOW), Verdict::Low));
        assert!(matches!(verdict_from(WRONG), Verdict::Incorrect));
        assert!(matches!(verdict_from(SOLVED), Verdict::AlreadySolved));
        assert!(matches!(verdict_from(COOLDOWN), Verdict::Cooldown(_)));
    }

    /// A directional reply also contains the generic phrase, so order matters.
    #[test]
    fn direction_beats_generic() {
        assert!(HIGH.contains("That\'s not the right answer"));
        assert!(matches!(verdict_from(HIGH), Verdict::High));
    }

    #[test]
    fn extracts_wait() {
        assert_eq!(wait_from(COOLDOWN), "1m 0s");
        assert_eq!(wait_from("nothing here"), "unknown");
    }
}
