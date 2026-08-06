use crate::{client::SolverClient, day::Day, part::Part, solutions::answer::Answer};

/// One day's puzzle, parsed and ready to answer both parts.
///
/// [`Sized`], so not object safe. That is deliberate: `new` returns `Self` and
/// could never go through a vtable, and nothing needs `dyn` because dispatch is
/// a match that already knows each concrete type.
pub trait Solution: Sized {
    /// Parses `input` once, so both parts are reads over the result.
    fn new(input: &'static str) -> anyhow::Result<Self>;

    /// The raw input this was built from.
    fn input(&self) -> &str;

    /// [`Answer::solved`] for a submittable value, [`Answer::Visual`] for art,
    /// [`Answer::None`] for nothing.
    fn part_one(&self) -> Answer;

    /// Same contract as [`Solution::part_one`]. Day 25 has no second puzzle.
    fn part_two(&self) -> Answer;
}

/// Runs both parts, validating each answer when given a client.
///
/// `session` doubles as the validate flag: `None` solves offline. Only
/// [`Answer::Value`] is validated, since nothing else has anything to check.
pub fn solve<S: Solution>(
    client: Option<&SolverClient>,
    input: &'static str,
    day: &Day,
) -> anyhow::Result<(Answer, Answer)> {
    let solution = S::new(input)?;

    let mut one = solution.part_one();
    let mut two = solution.part_two();

    if let Some(client) = client {
        if let Some(value) = one.value() {
            let verdict = client.validate_answer(day, input, Part::One, value)?;
            one = one.with_verdict(verdict);
        }
        if let Some(value) = two.value() {
            let verdict = client.validate_answer(day, input, Part::Two, value)?;
            two = two.with_verdict(verdict);
        }
    }

    Ok((one, two))
}
