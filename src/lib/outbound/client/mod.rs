//! The two services this tool talks to, the runner that drives one of them, and
//! the environment they read their configuration from.
//!
//! The services are kept apart because they differ in auth, contract, and
//! failure semantics: AOC is authenticated and grades once, the solver needs no
//! auth and answers as often as you ask.
//!
//! [`solve::solve`] sits here rather than in the domain because it holds a
//! client, which is the dependency the domain is not allowed to have.

pub mod aoc_client;
pub mod environment;
pub mod solve;
pub mod solver_client;
