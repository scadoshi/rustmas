use std::time::Instant;

use crate::domain::address::{Day, Part};
use crate::domain::solution::outcome::Outcome;
use crate::domain::solution::{Solution, Solved};
use crate::outbound::client::solver_client::SolverClient;

/// Whether a run checks its answers against the solver, and with which client.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Validation<'client> {
    Off,
    Against(&'client SolverClient),
}

/// Runs both parts of one day, checking each answer as `validation` says.
///
/// Validation runs after both parts are measured, so no timing includes a
/// network round trip. A failing part goes into its own [`Outcome`], so the
/// other still runs; only [`Solution::new`] failing ends the day.
pub(crate) fn run_day<S: Solution>(
    validation: Validation<'_>,
    input: &str,
    day: &Day,
) -> anyhow::Result<Solved> {
    let start = Instant::now();
    let solution = S::new(input)?;
    let parsed_in = start.elapsed();

    let start = Instant::now();
    let part_one = solution.part_one();
    let mut part_one = Outcome::new(part_one, start.elapsed());

    let start = Instant::now();
    let part_two = solution.part_two();
    let mut part_two = Outcome::new(part_two, start.elapsed());

    if let Validation::Against(client) = validation {
        if let Some(value) = part_one.value() {
            let verdict = client.validate_answer(day, input, Part::One, value)?;
            part_one = part_one.with_solver_verdict(verdict);
        }
        if let Some(value) = part_two.value() {
            let verdict = client.validate_answer(day, input, Part::Two, value)?;
            part_two = part_two.with_solver_verdict(verdict);
        }
    }

    Ok(Solved {
        parsed_in,
        part_one,
        part_two,
    })
}
