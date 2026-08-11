//! Solving a puzzle: the contract a day implements, what a part produced, and
//! everything learned about it afterwards.
//!
//! Both verdicts live here rather than beside the clients that parse them.
//! Neither mentions HTTP or carries a status code, [`outcome::Outcome`] matches
//! on them to render a line, and a domain type cannot depend on an adapter. The
//! clients map their replies onto these.

pub mod answer;
pub mod aoc_verdict;
pub mod common;
pub mod outcome;
pub mod solver_verdict;
pub mod year_2015;
pub mod year_2016;

use std::time::Duration;

use crate::domain::solution::{answer::Answer, outcome::Outcome};

/// One day's puzzle, parsed and ready to answer both parts.
///
/// [`Sized`], so not object safe. That is deliberate: `new` returns `Self` and
/// could never go through a vtable, and nothing needs `dyn` because dispatch is
/// a match that already knows each concrete type.
pub trait Solution: Sized {
    /// Parses `input` once, so both parts are reads over the result.
    fn new(input: impl Into<String>) -> anyhow::Result<Self>;

    /// The raw input this was built from.
    fn input(&self) -> &str;

    /// [`Answer::solved`] for a submittable value, [`Answer::Visual`] for art,
    /// [`Answer::None`] for nothing.
    ///
    /// Fallible so a part can `?` on work that only it needs, and so a bug in
    /// the day reads as an error rather than as [`Answer::None`]. The two mean
    /// different things: `None` is "there is no answer here", an error is "this
    /// day is broken". Parsing shared by both parts belongs in
    /// [`Solution::new`], which is fallible for the same reason.
    ///
    /// A failing part does not stop the other one. The error lands in that
    /// part's [`Outcome`] and the run continues.
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
