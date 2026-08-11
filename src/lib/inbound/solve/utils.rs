use crate::{
    domain::{
        address::{Day, Part},
        solution::{outcome::Outcome, solver_verdict::SolverVerdict},
    },
    outbound::client::aoc_client::AocClient,
};
use std::io::{self, Write};

/// Submits the answer if the solver backed it, with AOC's reply attached.
///
/// A wrong answer costs an escalating cooldown, so the solver verdict gates the
/// send. [`SolverVerdict::Unsupported`] goes through anyway, since the solver
/// cannot judge it either way. Rejected answers come back untouched.
pub fn submit(aoc: &AocClient, day: &Day, part: Part, outcome: Outcome) -> anyhow::Result<Outcome> {
    let Some(value) = outcome.value() else {
        return Ok(outcome);
    };
    if !matches!(
        outcome.verdict(),
        Some(SolverVerdict::Correct) | Some(SolverVerdict::Unsupported)
    ) {
        return Ok(outcome);
    }

    let verdict = aoc.submit_answer(day, part, value)?;
    Ok(outcome.with_submission(verdict))
}

/// Asks before an unfiltered submit run, which would post every solved day.
///
/// Goes to stderr so redirecting output cannot swallow it. Closed stdin is no.
pub fn confirm(count: usize) -> anyhow::Result<bool> {
    eprintln!(
        "About to submit up to {count} answers to adventofcode.com, across every \
         year and day. Wrong answers are rate limited. Narrow it with --year or \
         --day, or pass --yes to skip this."
    );
    eprint!("Continue? [y/N] ");
    io::stderr().flush()?;

    let mut reply = String::new();
    if io::stdin().read_line(&mut reply)? == 0 {
        return Ok(false);
    }
    Ok(matches!(reply.trim(), "y" | "Y" | "yes" | "Yes"))
}
