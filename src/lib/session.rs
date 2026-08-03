use crate::part::Day;
use anyhow::Context;
use reqwest::{Url, blocking::Client};

const COOKIE_KEY: &str = "COOKIE";
const AOC_BASE_URL: &str = "https://adventofcode.com";
const REPO_GITHUB_URL: &str = "https://github.com/scadoshi/rustmas";

/// An authenticated handle to adventofcode.com.
///
/// Bundles the session cookie, the site base URL, and a reusable HTTP client so
/// that input requests share one connection pool. Build one with
/// [`Session::from_env`].
pub struct Session {
    cookie: String,
    url: Url,
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
            url: Url::parse(AOC_BASE_URL)?,
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
            .get(
                self.url
                    .join(&format!("{}/day/{}/input", day.year(), day.value()))?,
            )
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
}
