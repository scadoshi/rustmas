//! Solving a puzzle: the contract a day implements, and what came of it.
//!
//! Both verdicts live here rather than beside the clients that parse them,
//! since neither mentions HTTP.

pub mod answer;
pub mod aoc_verdict;
pub mod common;
pub mod outcome;
pub mod solver_verdict;
pub mod totals;
/// Solutions to the 2015 event, one module per day.
pub mod year_2015;
/// Solutions to the 2016 event, one module per day.
pub mod year_2016;
/// Solutions to the 2017 event, one module per day.
pub mod year_2017;
/// Solutions to the 2018 event, one module per day.
pub mod year_2018;
/// Solutions to the 2019 event, one module per day.
pub mod year_2019;
/// Solutions to the 2020 event, one module per day.
pub mod year_2020;
/// Solutions to the 2021 event, one module per day.
pub mod year_2021;
/// Solutions to the 2022 event, one module per day.
pub mod year_2022;
/// Solutions to the 2023 event, one module per day.
pub mod year_2023;
/// Solutions to the 2024 event, one module per day.
pub mod year_2024;
/// Solutions to the 2025 event, one module per day.
pub mod year_2025;
/// A year to copy when starting a new one. Compiled but never dispatched, so a
/// copy starts from something that builds.
pub mod year_template;

use std::time::Duration;

use crate::domain::solution::answer::Answer;
use crate::domain::solution::outcome::Outcome;

/// One day's puzzle, parsed and ready to answer both parts.
///
/// [`Sized`] on purpose: `new` returns `Self` and could never go through a
/// vtable, and dispatch is a match that already knows each concrete type.
pub trait Solution: Sized {
    /// Parses `input` once, so both parts are reads over the result.
    ///
    /// Takes a borrow, so a day that parses into its own types keeps no copy
    /// of the text.
    ///
    /// # Errors
    ///
    /// Whatever the day rejects in its input. The runner reports it and both
    /// parts are skipped.
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self>;

    /// [`Answer::solved`] for a submittable value, [`Answer::Visual`] for art,
    /// [`Answer::None`] when there is no answer, [`Answer::Unwritten`] for a
    /// part not yet written.
    ///
    /// # Errors
    ///
    /// Whatever the day finds wrong while solving, such as input the parser
    /// let through but the part cannot use. An error stops this part only; the
    /// other still runs.
    fn part_one(&self) -> anyhow::Result<Answer>;

    /// Same contract as [`Solution::part_one`]. An event's final day has no
    /// second puzzle.
    ///
    /// # Errors
    ///
    /// As for [`Solution::part_one`].
    fn part_two(&self) -> anyhow::Result<Answer>;
}

/// One run of a day: both parts, and where the time went.
#[derive(Debug)]
pub struct Solved {
    /// Time spent in [`Solution::new`].
    pub parsed_in: Duration,
    /// Part one's answer, with any verdicts attached later.
    pub part_one: Outcome,
    /// Part two's answer, with any verdicts attached later.
    pub part_two: Outcome,
}

impl Solved {
    /// Parsing plus both parts. Excludes any network time.
    pub fn total_elapsed(&self) -> Duration {
        self.parsed_in + self.part_one.elapsed() + self.part_two.elapsed()
    }
}
