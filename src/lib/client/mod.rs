//! HTTP clients for the two services this tool talks to.
//!
//! [`AocClient`] is adventofcode.com: authenticated, and it grades each part
//! exactly once. [`SolverClient`] is a third-party solver: no auth, and it
//! answers the same question as many times as you ask. Kept apart because they
//! differ in auth, contract, and failure semantics.

pub mod aoc;
pub mod solver;
pub mod verdict;

pub use aoc::AocClient;
pub use solver::SolverClient;

/// Env var holding an address AOC can reach you at, folded into the
/// `User-Agent`. Optional. Kept out of the source so a fork identifies its own
/// owner rather than this repo's.
const CONTACT_KEY: &str = "CONTACT";
/// Env var naming the repo reported in the `User-Agent`. Optional, and with no
/// default, so an unset value never attributes traffic to someone else's repo.
const REPO_URL_KEY: &str = "REPO_URL";
/// Names the tool when neither `REPO_URL` nor `CONTACT` is set. Deliberately
/// points at nobody: a default naming this repo's author would make them the
/// contact for a stranger's traffic.
const UNCONFIGURED_USER_AGENT: &str = "rustmas (unconfigured; set CONTACT in .env)";

/// Builds the `User-Agent` from `REPO_URL` and `CONTACT`, both optional.
///
/// AOC asks automated clients to be reachable. Loads `.env` if present, since
/// the values may already live in the real environment.
pub(crate) fn user_agent_from_env() -> String {
    dotenvy::dotenv().ok();
    let set = |key| match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => None,
    };
    match (set(REPO_URL_KEY), set(CONTACT_KEY)) {
        (Some(repo), Some(contact)) => format!("{repo} by {contact}"),
        (Some(repo), None) => repo,
        (None, Some(contact)) => format!("rustmas by {contact}"),
        (None, None) => UNCONFIGURED_USER_AGENT.to_string(),
    }
}
