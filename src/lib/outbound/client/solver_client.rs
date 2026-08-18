use crate::{
    domain::{
        address::{Day, Part},
        solution::solver_verdict::SolverVerdict,
    },
    outbound::client::environment::Environment,
};
use anyhow::bail;
use reqwest::{
    Url,
    blocking::Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
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
/// No cookie, no stars, and no memory, which is what makes it repeatable
/// enough to use as a regression check.
pub struct SolverClient {
    client: Client,
}

impl SolverClient {
    /// Builds a client carrying the `User-Agent`, loading `.env` if present.
    ///
    /// Nothing here is required, since the solver needs no authentication. It
    /// fails only when the user agent cannot be a header value, which a stray
    /// newline in `.env` is enough to cause.
    pub fn from_env() -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&Environment::user_agent())?);
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Checks `answer` against the solver.
    ///
    /// Numeric answers compare as numbers and report a direction; anything else
    /// compares as text. Every failure is a 400 with the reason in the body, so
    /// classification reads the body. Only transport failures and 5xx try the
    /// next host, since all three run the same code.
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

            let response = match self.client.post(url.clone()).body(input.clone()).send() {
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
