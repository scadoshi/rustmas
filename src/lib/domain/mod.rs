//! The puzzle domain: which puzzle we mean, and what solving one produces.
//!
//! No HTTP, no filesystem, no command line. Running a solution needs a client,
//! so the runner lives in `outbound`.

pub mod address;
pub mod solution;
