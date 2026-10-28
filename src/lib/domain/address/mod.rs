//! Which puzzle we mean: [`year::Year`], [`day::Day`], [`part::Part`].
//!
//! `Day` wraps `Year`, both constructor-only, so holding one means it names a
//! real puzzle without re-checking.

pub mod day;
pub mod filter;
pub mod part;
pub mod year;

pub use day::Day;
pub use filter::{Filter, InvalidFilter};
pub use part::{PARTS_PER_DAY, Part};
pub use year::{FIRST_YEAR, Year};
