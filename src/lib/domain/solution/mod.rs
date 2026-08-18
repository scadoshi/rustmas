//! Solving a puzzle: the contract a day implements, and what came of it.
//!
//! Both verdicts live here rather than beside the clients that parse them,
//! since neither mentions HTTP and a domain type cannot depend on an adapter.

pub mod answer;
pub mod aoc_verdict;
pub mod common;
pub mod outcome;
pub mod solver_verdict;
// One `pub mod year_YYYY;` per year you write solutions for.
// Compiled but never dispatched, so copying it starts from something that
// builds against the current `Solution`.
pub mod year_template;

use crate::domain::solution::{answer::Answer, outcome::Outcome};
use std::time::Duration;

/// One day's puzzle, parsed and ready to answer both parts.
///
/// [`Sized`] on purpose: `new` returns `Self` and could never go through a
/// vtable, and dispatch is a match that already knows each concrete type.
pub trait Solution: Sized {
    /// Parses `input` once, so both parts are reads over the result.
    ///
    /// Takes a borrow, so a day that parses into its own types keeps no copy
    /// of the text. A day that reads the raw input in its parts owns one.
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self>;

    /// [`Answer::solved`] for a submittable value, [`Answer::Visual`] for art,
    /// [`Answer::None`] when there is no answer, [`Answer::Unwritten`] for a
    /// part not yet written.
    ///
    /// An error means the day is broken, which is not the same as having no
    /// answer. It stops this part only; the other still runs.
    fn part_one(&self) -> anyhow::Result<Answer>;

    /// Same contract as [`Solution::part_one`]. Day 25 has no second puzzle.
    fn part_two(&self) -> anyhow::Result<Answer>;
}

/// One run of a day: both parts, and where the time went.
#[derive(Debug)]
pub struct Solved {
    pub parse: Duration,
    pub one: Outcome,
    pub two: Outcome,
}

impl Solved {
    /// Parsing plus both parts. Excludes any network time.
    pub fn total(&self) -> Duration {
        self.parse + self.one.elapsed() + self.two.elapsed()
    }
}
