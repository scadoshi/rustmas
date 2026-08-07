//! The puzzle domain: what a year, day, and part are, and how solutions run.
//!
//! Nothing here knows about HTTP, the filesystem, or the command line.

pub mod calendar;
pub mod day;
pub mod part;
pub mod solutions;
