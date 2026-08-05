pub mod verdict;

use crate::{day::Day, part::Part, session::verdict::Verdict};
use anyhow::{Context, bail};
use reqwest::{Url, blocking::Client};
use std::fmt::Display;

const COOKIE_KEY: &str = "COOKIE";
const AOC_BASE_URL: &str = "https://adventofcode.com";
const REPO_GITHUB_URL: &str = "https://github.com/scadoshi/rustmas";
const AOC_SOLVER_BASE_URLS: [&str; 3] = [
    "https://advent.fly.dev",
    "https://aoc.fornwall.workers.dev",
    "https://mystifying-blackwell-9e705f.netlify.app",
];

/// An authenticated handle to adventofcode.com.
///
/// Bundles the session cookie, the site base URL, and a reusable HTTP client so
/// that input requests share one connection pool. Build one with
/// [`Session::from_env`].
pub struct Session {
    cookie: String,
    client: Client,
}

impl Session {
    /// Returns the session cookie value used to authenticate requests.
    pub fn cookie(&self) -> &str {
        &self.cookie
    }

    /// Returns the shared HTTP client backing this session.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Builds a [`Session`] from the environment.
    ///
    /// Loads a `.env` file if present (optional; the cookie may already be set
    /// in the real environment) and reads the cookie from `COOKIE`. Errors if
    /// that variable is missing or the base URL fails to parse.
    pub fn from_env() -> anyhow::Result<Self> {
        // `.env` is optional: the cookie may already live in the real environment.
        dotenvy::dotenv().ok();
        Ok(Self {
            cookie: std::env::var(COOKIE_KEY)
                .with_context(|| format!("failed to get {}", COOKIE_KEY))?,
            client: Client::new(),
        })
    }

    /// Fetches the raw puzzle input for `day` from adventofcode.com.
    ///
    /// Sends an authenticated GET to `/<year>/day/<day>/input` and returns the
    /// response body verbatim. Errors if the request fails to send, the server
    /// returns a non-success status (e.g. a bad cookie or an unreleased day),
    /// or the body cannot be read.
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

    pub fn submit_answer(&self, day: &Day, part: Part) -> anyhow::Result<()> {
        todo!()
    }

    /// Checks `answer` against the third-party Advent of Code solver.
    ///
    /// Posts the puzzle input to `/solve/<year>/<day>/<part>` and compares the
    /// solver's answer to yours. Numeric answers compare as numbers, so a
    /// mismatch reports [`Verdict::TooLow`] or [`Verdict::TooHigh`]. Anything
    /// else compares as text.
    ///
    /// The solver signals every failure with a 400 plus a message body, so the
    /// status alone can't classify a response. A body beginning with
    /// `Unsupported` means it has no implementation for that puzzle, which is
    /// [`Verdict::Unsupported`]. Any other 4xx is a fault on our side, such as
    /// an input the solver couldn't parse, and errors.
    ///
    /// Only transport failures and 5xx responses fall through to the next host,
    /// since all three run the same solver and would reject a bad request
    /// identically. Errors if every host fails.
    pub fn validate_answer(
        &self,
        input: impl Display,
        day: &Day,
        part: Part,
        answer: &str,
    ) -> anyhow::Result<Verdict> {
        let input = input.to_string();
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
