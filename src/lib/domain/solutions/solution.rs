use crate::{
    domain::{
        address::{Day, Part},
        solutions::{answer::Answer, outcome::Outcome},
    },
    outbound::client::SolverClient,
};
use std::time::{Duration, Instant};

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
    fn part_one(&self) -> Answer;

    /// Same contract as [`Solution::part_one`]. Day 25 has no second puzzle.
    fn part_two(&self) -> Answer;
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

/// Runs both parts, checking each answer against the solver when `validate`.
///
/// Timings cover parsing and solving only. Validation happens after both parts
/// are measured, so no duration includes a network round trip.
///
/// Only [`Answer::Value`] is checked, since nothing else has anything to check.
pub fn solve<S: Solution>(
    client: &SolverClient,
    validate: bool,
    input: &str,
    day: &Day,
) -> anyhow::Result<Solved> {
    let start = Instant::now();
    let solution = S::new(input)?;
    let parse = start.elapsed();

    let start = Instant::now();
    let one = solution.part_one();
    let mut one = Outcome::new(one, start.elapsed());

    let start = Instant::now();
    let two = solution.part_two();
    let mut two = Outcome::new(two, start.elapsed());

    if validate {
        if let Some(value) = one.value() {
            let verdict = client.validate_answer(day, input, Part::One, value)?;
            one = one.with_verdict(verdict);
        }
        if let Some(value) = two.value() {
            let verdict = client.validate_answer(day, input, Part::Two, value)?;
            two = two.with_verdict(verdict);
        }
    }

    Ok(Solved { parse, one, two })
}
