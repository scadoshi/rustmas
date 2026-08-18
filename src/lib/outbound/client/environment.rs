//! Every environment variable this tool reads, in one place.
//!
//! Each client asks for what it needs, so a caller never spells a key itself
//! and the set of things `.env` must hold is readable from one file.

use anyhow::Context;

/// Optional env var: an address AOC can reach you at, for the `User-Agent`.
///
/// Kept out of the source so a fork identifies its own owner, not this repo's.
const CONTACT_KEY: &str = "CONTACT";
/// Optional env var: the repo reported in the `User-Agent`.
///
/// No default, so an unset value never attributes traffic to another repo.
const REPO_URL_KEY: &str = "REPO_URL";
/// Env var holding the adventofcode.com session cookie.
const COOKIE_KEY: &str = "COOKIE";
/// Names the tool when neither `REPO_URL` nor `CONTACT` is set.
///
/// Points at nobody on purpose, so a stranger's traffic names no real contact.
const UNCONFIGURED_USER_AGENT: &str = "rustmas (unconfigured; set CONTACT in .env)";

/// The environment, read on demand. Holds nothing.
pub struct Environment;

impl Environment {
    /// One variable, `None` when it is unset or blank.
    ///
    /// Blank counts as unset, so a key left empty in `.env` means what it looks
    /// like. Errors only when a value exists and cannot be read, which keeps
    /// "not configured" separate from "configured wrongly".
    fn get(key: &str) -> anyhow::Result<Option<String>> {
        // `.env` is optional: values may already live in the real environment.
        dotenvy::dotenv().ok();
        match std::env::var(key) {
            Ok(value) => Ok(Some(value.trim().to_string()).filter(|s| !s.is_empty())),
            Err(std::env::VarError::NotPresent) => Ok(None),
            Err(e) => Err(e).with_context(|| format!("{key} is set but unreadable")),
        }
    }

    /// How this tool identifies itself, from `REPO_URL` and `CONTACT`.
    ///
    /// AOC asks automated clients to be reachable. Both are optional, so an
    /// unreadable one is treated as unset rather than failing a request that
    /// would otherwise work.
    pub fn user_agent() -> String {
        let get = |key| Self::get(key).ok().flatten();
        match (get(REPO_URL_KEY), get(CONTACT_KEY)) {
            (Some(repo), Some(contact)) => format!("{repo} by {contact}"),
            (Some(repo), None) => repo,
            (None, Some(contact)) => format!("rustmas by {contact}"),
            (None, None) => UNCONFIGURED_USER_AGENT.to_string(),
        }
    }

    /// The session cookie, or `None` when it is unset or blank.
    ///
    /// For callers that can work offline, where no cookie means skip the
    /// network rather than fail.
    pub fn cookie_if_set() -> anyhow::Result<Option<String>> {
        Self::get(COOKIE_KEY)
    }

    /// The session cookie, required.
    ///
    /// For callers that cannot proceed without one, such as building a client.
    /// The pair exists so the requirement is named here rather than at every
    /// call site.
    pub fn cookie() -> anyhow::Result<String> {
        Self::cookie_if_set()?.with_context(|| format!("{COOKIE_KEY} is not set"))
    }
}
