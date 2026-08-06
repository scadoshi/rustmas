use crate::{day::Day, part::Part, session::Session, solutions::answer::Answer};

/// One day's puzzle, parsed and ready to answer both parts.
///
/// Implementors hold whatever parsed form the puzzle needs. [`Solution::new`]
/// does the parsing once, so `part_one` and `part_two` are reads over that
/// rather than two passes over the raw text.
///
/// The trait is [`Sized`] and therefore not object safe, which is deliberate.
/// `new` returns `Self`, so it could never go through a vtable: there is no
/// receiver to find the vtable from, and an erased return type has no known
/// size. Nothing needs `dyn` here, because dispatch is a match that already
/// knows each concrete type (see the `solutions!` macro in
/// `src/bin/solve/main.rs`), and monomorphized generics cover it.
///
/// A day looks like this:
///
/// ```ignore
/// impl Solution for Day01 {
///     fn new(input: &'static str) -> anyhow::Result<Self> {
///         Ok(Self { input })
///     }
///     fn input(&self) -> &str { self.input }
///     fn part_one(&self) -> Answer { Answer::solved(count.to_string()) }
///     fn part_two(&self) -> Answer { Answer::None }
/// }
/// ```
pub trait Solution: Sized {
    /// Parses `input` into whatever form both parts need.
    ///
    /// Called once per run. Errors if the input doesn't parse, which surfaces
    /// as a failure for that day rather than a panic.
    fn new(input: &'static str) -> anyhow::Result<Self>;

    /// Returns the raw puzzle input this was built from.
    fn input(&self) -> &str;

    /// Solves the first part.
    ///
    /// Return [`Answer::solved`] for a submittable value, [`Answer::Visual`]
    /// for art the reader interprets, or [`Answer::None`] when there is
    /// nothing to produce.
    fn part_one(&self) -> Answer;

    /// Solves the second part.
    ///
    /// Same contract as [`Solution::part_one`]. Day 25 has no second puzzle, so
    /// it returns [`Answer::None`].
    fn part_two(&self) -> Answer;
}

/// Runs both parts of a solution, validating each answer when given a session.
///
/// `session` doubles as the validate flag: `None` means solve offline and skip
/// the network entirely. Only [`Answer::Value`] is ever validated, since a
/// visual or absent answer has nothing to check.
pub fn solve<S: Solution>(
    session: Option<&Session>,
    input: &'static str,
    day: &Day,
) -> anyhow::Result<(Answer, Answer)> {
    let solution = S::new(input)?;

    let mut one = solution.part_one();
    let mut two = solution.part_two();

    if let Some(session) = session {
        if let Some(value) = one.value() {
            let verdict = session.validate_answer(day, input, Part::One, value)?;
            one = one.with_verdict(verdict);
        }
        if let Some(value) = two.value() {
            let verdict = session.validate_answer(day, input, Part::Two, value)?;
            two = two.with_verdict(verdict);
        }
    }

    Ok((one, two))
}
