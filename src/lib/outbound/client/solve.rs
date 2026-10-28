use std::time::Instant;

use crate::domain::address::{Day, Part};
use crate::domain::solution::outcome::Outcome;
use crate::domain::solution::solver_verdict::SolverVerdict;
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
/// network round trip. A failing part, or a failing check, goes into its own
/// [`Outcome`], so the other still runs; only [`Solution::new`] failing ends
/// the day.
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
    let part_one = Outcome::new(part_one, start.elapsed());

    let start = Instant::now();
    let part_two = solution.part_two();
    let part_two = Outcome::new(part_two, start.elapsed());

    let solved = Solved {
        parsed_in,
        part_one,
        part_two,
    };
    Ok(match validation {
        Validation::Off => solved,
        Validation::Against(client) => check_day(solved, |part, value| {
            client.validate_answer(day, input, part, value)
        }),
    })
}

/// Attaches `check`'s verdict to each part that has a submittable answer.
///
/// `check` asks the solver about one part, which keeps the network out of
/// what happens to its reply. A failed check lands on that part as
/// [`SolverVerdict::Unchecked`], so an outage costs the verdict and never the
/// answers.
fn check_day(
    solved: Solved,
    mut check: impl FnMut(Part, &str) -> anyhow::Result<SolverVerdict>,
) -> Solved {
    Solved {
        parsed_in: solved.parsed_in,
        part_one: checked(solved.part_one, |value| check(Part::One, value)),
        part_two: checked(solved.part_two, |value| check(Part::Two, value)),
    }
}

/// `outcome` with `check`'s verdict, or with why the check failed.
fn checked(outcome: Outcome, check: impl FnOnce(&str) -> anyhow::Result<SolverVerdict>) -> Outcome {
    let Some(value) = outcome.value() else {
        return outcome;
    };
    // `{:#}` keeps the whole chain on the one line the part prints on.
    let verdict = check(value).unwrap_or_else(|e| SolverVerdict::Unchecked(format!("{e:#}")));
    outcome.with_solver_verdict(verdict)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::domain::solution::answer::Answer;

    fn solved() -> Solved {
        Solved {
            parsed_in: Duration::ZERO,
            part_one: Outcome::new(Ok(Answer::solved("one")), Duration::ZERO),
            part_two: Outcome::new(Ok(Answer::solved("two")), Duration::ZERO),
        }
    }

    /// An outage on the solver's side says nothing about the answers, so
    /// they stay and each part carries the failed check instead.
    #[test]
    fn an_unreachable_solver_leaves_the_answers_unchecked() {
        let solved = check_day(solved(), |_, _| Err(anyhow::anyhow!("all hosts down")));

        for outcome in [&solved.part_one, &solved.part_two] {
            assert!(outcome.value().is_some());
            assert_eq!(
                outcome.solver_verdict(),
                Some(&SolverVerdict::Unchecked("all hosts down".to_string()))
            );
        }
        assert_eq!(
            solved.part_one.to_string(),
            "one (unchecked: all hosts down) [0ns]"
        );
    }

    #[test]
    fn a_reachable_solver_grades_each_part() {
        let solved = check_day(solved(), |part, _| {
            Ok(match part {
                Part::One => SolverVerdict::Correct,
                Part::Two => SolverVerdict::Low,
            })
        });
        assert_eq!(
            solved.part_one.solver_verdict(),
            Some(&SolverVerdict::Correct)
        );
        assert_eq!(solved.part_two.solver_verdict(), Some(&SolverVerdict::Low));
    }
}
