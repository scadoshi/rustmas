pub mod verdict;

use crate::{day::Day, part::Part, session::verdict::Verdict};
use anyhow::{Context, bail};
use reqwest::{Url, blocking::Client};
use std::{collections::HashMap, fmt::Display};

/// Env var holding the adventofcode.com session cookie.
const COOKIE_KEY: &str = "COOKIE";
const AOC_BASE_URL: &str = "https://adventofcode.com";
/// Sent as `User-Agent`. AOC asks automated clients to identify themselves.
const REPO_GITHUB_URL: &str = "https://github.com/scadoshi/rustmas";
/// Three deployments of the same third-party solver, tried in order. See
/// `context/references.md`.
const AOC_SOLVER_BASE_URLS: [&str; 3] = [
    "https://advent.fly.dev",
    "https://aoc.fornwall.workers.dev",
    "https://mystifying-blackwell-9e705f.netlify.app",
];

/// An authenticated handle to adventofcode.com, with a pooled client shared
/// across requests. Build one with [`Session::from_env`].
pub struct Session {
    cookie: String,
    client: Client,
}

impl Session {
    pub fn cookie(&self) -> &str {
        &self.cookie
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Reads the cookie from the `COOKIE` env var, loading `.env` if present.
    /// Errors if it is missing.
    pub fn from_env() -> anyhow::Result<Self> {
        // `.env` is optional: the cookie may already live in the real environment.
        dotenvy::dotenv().ok();
        Ok(Self {
            cookie: std::env::var(COOKIE_KEY)
                .with_context(|| format!("failed to get {}", COOKIE_KEY))?,
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
            .header("User-Agent", REPO_GITHUB_URL)
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

    pub fn submit_answer(
        &self,
        day: &Day,
        part: Part,
        answer: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        let path = format!("/{}/day/{}/answer", day.year(), day.value());
        let url = Url::parse(AOC_BASE_URL)?.join(&path)?;
        let form_params =
            HashMap::from([("level", part.to_wire_value()), ("answer", answer.as_ref())]);
        // let response = self.client.post(url).form(&form_params).send()?;
        todo!()
    }

    /// Checks `answer` against the third-party solver.
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
    ) -> anyhow::Result<Verdict> {
        let input = input.to_string();
        let answer = answer.as_ref();
        let path = format!(
            "/solve/{}/{}/{}",
            day.year(),
            day.value(),
            part.to_wire_value()
        );

        for base in AOC_SOLVER_BASE_URLS {
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
                    return Ok(Verdict::Unsupported);
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
