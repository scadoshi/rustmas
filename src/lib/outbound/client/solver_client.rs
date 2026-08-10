use crate::domain::{
    address::{Day, Part},
    solution::solver_verdict::SolverVerdict,
};
use anyhow::bail;
use reqwest::{Url, blocking::Client};
use std::fmt::Display;

/// Three deployments of the same third-party solver, tried in order. See
/// `context/references.md`.
const BASE_URLS: [&str; 3] = [
    "https://advent.fly.dev",
    "https://aoc.fornwall.workers.dev",
    "https://mystifying-blackwell-9e705f.netlify.app",
];

/// A client for the third-party Advent of Code solver.
///
/// Needs no cookie, since it knows nothing about your account and cannot award
/// stars. In exchange it answers the same question as many times as you ask,
/// which is what makes it usable as a regression check.
pub struct SolverClient {
    user_agent: String,
    client: Client,
}

impl Default for SolverClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SolverClient {
    pub fn new() -> Self {
        Self {
            user_agent: super::user_agent_from_env(),
            client: Client::new(),
        }
    }

    /// Builds one sharing an existing HTTP client, whose connection pool is
    /// reference counted internally, so cloning shares it.
    pub fn with_client(client: Client) -> Self {
        Self {
            user_agent: super::user_agent_from_env(),
            client,
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Checks `answer` against the solver.
    ///
    /// Numeric answers compare as numbers, so a mismatch reports a direction.
    /// Anything else compares as text.
    ///
    /// The solver returns 400 for every failure with the reason in the body, so
    /// classification reads the body rather than the status. Only transport
    /// failures and 5xx retry the next host, since all three run the same code.
    pub fn validate_answer(
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

            let response = match self
                .client
                .post(url.clone())
                .header("User-Agent", self.user_agent())
                .body(input.clone())
                .send()
            {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("failed to reach solver at {url}: {e:?}; trying next url");
                    continue;
                }
            };

            // Read the body before classifying. The solver puts the reason
            // there, and `error_for_status` would consume it.
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
                return Ok(match (answer.parse::<i64>(), body.parse::<i64>()) {
                    (Ok(answer), Ok(solved)) => answer.cmp(&solved).into(),
                    _ => (answer == body).into(),
                });
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
