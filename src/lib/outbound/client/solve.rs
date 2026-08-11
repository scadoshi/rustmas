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
/// Validation runs after both parts are measured, so no timing includes a
/// network round trip, and only a submittable answer is checked at all.
///
/// A failing part goes into its own [`Outcome`] rather than being propagated,
/// so the other part still runs. Only [`Solution::new`] failing ends the day.
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
