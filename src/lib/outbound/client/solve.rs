use crate::{
    domain::{
        address::{Day, Part},
        solution::{Solution, Solved, outcome::Outcome},
    },
    outbound::client::solver_client::SolverClient,
};
use std::time::Instant;

/// Runs both parts, checking each answer against the solver when `validate`.
///
/// Timings cover parsing and solving only. Validation happens after both parts
/// are measured, so no duration includes a network round trip.
///
/// Only a submittable answer is checked, since nothing else has anything to
/// check. [`Outcome::value`] is what reports that, and it is also what
/// [`Outcome::with_verdict`] gates on, so an unsubmittable part cannot pick up
/// a verdict even if one were offered.
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
