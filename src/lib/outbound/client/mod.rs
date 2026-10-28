//! The two services this tool talks to, plus the environment they read.
//!
//! AOC is authenticated and grades once, the solver needs no auth and answers
//! as often as you ask, so they stay apart. [`solve::solve`] is here rather
//! than in the domain because it holds a client.

pub(crate) mod aoc_client;
pub(crate) mod environment;
pub(crate) mod session_cookie;
pub(crate) mod solve;
pub(crate) mod solver_client;
