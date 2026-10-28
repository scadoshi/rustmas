use std::fmt::Display;

use anyhow::bail;
use reqwest::Url;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use crate::domain::address::{Day, Part};
use crate::domain::solution::solver_verdict::SolverVerdict;
use crate::outbound::client::environment::Environment;

/// Three deployments of the same third-party solver, tried in order. See
/// `context/references.md`.
const BASE_URLS: [&str; 3] = [
    "https://advent.fly.dev",
    "https://aoc.fornwall.workers.dev",
    "https://mystifying-blackwell-9e705f.netlify.app",
];

/// A client for the third-party Advent of Code solver.
///
/// No cookie, no stars, and no memory, which is what makes it repeatable
/// enough to use as a regression check.
#[derive(Debug)]
pub(crate) struct SolverClient {
    client: Client,
}

impl SolverClient {
    /// Builds a client carrying the `User-Agent`, loading `.env` if present.
    ///
    /// Nothing is required, since the solver needs no authentication. Fails
    /// only when the user agent cannot be a header value, which a stray
    /// newline in `.env` is enough to cause.
    pub(crate) fn from_env() -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&Environment::user_agent())?,
        );
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self { client })
    }

    /// Checks `answer` against the solver.
    ///
    /// Numeric answers compare as numbers and report a direction, anything
    /// else as text. Every failure is a 400 with the reason in the body. Only
    /// transport failures and 5xx try the next host.
    pub(crate) fn validate_answer(
        &self,
        day: &Day,
        input: impl Display,
        part: Part,
        answer: impl AsRef<str>,
    ) -> anyhow::Result<SolverVerdict> {
        let input = input.to_string();
        let answer = answer.as_ref();
        let path = format!(
            "/solve/{}/{}/{}",
            day.year(),
            day.value(),
            part.wire_value()
        );

        for base in BASE_URLS {
            let url = Url::parse(base)?.join(&path)?;

            let response = match self.client.post(url.clone()).body(input.clone()).send() {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("failed to reach solver at {url}: {e:?}; trying next url");
                    continue;
                }
            };

            // The solver puts the reason in the body, which
            // `error_for_status` would consume.
            let status = response.status();
            let body = match response.text() {
                Ok(body) => body,
                Err(e) => {
                    eprintln!("failed to read solver body from {url}: {e:?}; trying next url");
                    continue;
                }
            };
            let body = body.trim();

            if status.is_success() {
                return Ok(compare(answer, body));
            }

            if status.is_client_error() {
                if body.starts_with("Unsupported") {
                    return Ok(SolverVerdict::Unsupported);
                }
                bail!("solver at {url} rejected the request: {body}");
            }

            eprintln!("solver at {url} returned {status}: {body}; trying next url");
        }

        bail!(
            "failed to check answer for year: {} and day: {} via all urls",
            day.year(),
            day.value()
        );
    }
}

/// Grades `answer` against the solver's `solved`: as numbers when both parse,
/// so a mismatch has a direction, else as text.
fn compare(answer: &str, solved: &str) -> SolverVerdict {
    match (answer.parse::<i64>(), solved.parse::<i64>()) {
        (Ok(answer), Ok(solved)) => answer.cmp(&solved).into(),
        _ if answer == solved => SolverVerdict::Correct,
        _ => SolverVerdict::Incorrect,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Non-numeric answers can only match or not, with no direction to report.
    #[test]
    fn text_comparison_has_no_direction() {
        assert!(matches!(compare("abc", "abc"), SolverVerdict::Correct));
        assert!(matches!(compare("abc", "abd"), SolverVerdict::Incorrect));
    }
}
