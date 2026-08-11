//! The two services this tool talks to, and the runner that drives one of them.
//!
//! Kept apart because they differ in auth, contract, and failure semantics:
//! AOC is authenticated and grades once, the solver needs no auth and answers
//! as often as you ask.
//!
//! [`solve::solve`] sits here rather than in the domain because it holds a
//! client, which is the dependency the domain is not allowed to have.

pub mod aoc_client;
pub mod solve;
pub mod solver_client;

/// Optional env var: an address AOC can reach you at, for the `User-Agent`.
///
/// Kept out of the source so a fork identifies its own owner, not this repo's.
const CONTACT_KEY: &str = "CONTACT";
/// Optional env var: the repo reported in the `User-Agent`.
///
/// No default, so an unset value never attributes traffic to another repo.
const REPO_URL_KEY: &str = "REPO_URL";
/// Names the tool when neither `REPO_URL` nor `CONTACT` is set.
///
/// Points at nobody on purpose, so a stranger's traffic names no real contact.
const UNCONFIGURED_USER_AGENT: &str = "rustmas (unconfigured; set CONTACT in .env)";

/// Builds the `User-Agent` from `REPO_URL` and `CONTACT`, both optional.
///
/// AOC asks automated clients to be reachable. Loads `.env` if present.
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
