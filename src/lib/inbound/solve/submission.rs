use std::io::{self, Write};

use crate::domain::address::Part;
use crate::domain::solution::Solved;
use crate::domain::solution::aoc_verdict::AocVerdict;
use crate::domain::solution::outcome::Outcome;
use crate::domain::solution::solver_verdict::SolverVerdict;

/// Submits both parts of a day through `send`, with AOC's replies attached.
///
/// `send` posts one part's answer and returns AOC's verdict, which keeps the
/// network out of the decision about what to send. Part two goes only when
/// AOC said [`AocVerdict::Correct`] or [`AocVerdict::AlreadySolved`] to part
/// one this run, since AOC answers an early part two as though it were solved.
/// A part one that was not sent leaves part two unsent too.
pub(crate) fn submit_day(
    mut send: impl FnMut(Part, &str) -> anyhow::Result<AocVerdict>,
    solved: Solved,
) -> anyhow::Result<Solved> {
    let part_one = submit(|value| send(Part::One, value), solved.part_one)?;
    let starred = matches!(
        part_one.aoc_verdict(),
        Some(AocVerdict::Correct | AocVerdict::AlreadySolved)
    );
    let part_two = if starred {
        submit(|value| send(Part::Two, value), solved.part_two)?
    } else {
        solved.part_two
    };
    Ok(Solved {
        parsed_in: solved.parsed_in,
        part_one,
        part_two,
    })
}

/// Submits the answer if the solver backed it, with AOC's reply attached.
///
/// A wrong answer costs an escalating cooldown, so the verdict gates the send.
/// [`SolverVerdict::Unsupported`] goes through anyway. Rejected answers, and
/// [`SolverVerdict::Unchecked`] ones, come back untouched.
fn submit(
    send: impl FnOnce(&str) -> anyhow::Result<AocVerdict>,
    outcome: Outcome,
) -> anyhow::Result<Outcome> {
    let Some(value) = outcome.value() else {
        return Ok(outcome);
    };
    if !matches!(
        outcome.solver_verdict(),
        Some(SolverVerdict::Correct | SolverVerdict::Unsupported)
    ) {
        return Ok(outcome);
    }

    let verdict = send(value)?;
    Ok(outcome.with_aoc_verdict(verdict))
}

/// Asks before an unfiltered submit run, which would post every solved day.
///
/// Goes to stderr so redirecting output cannot swallow it. Closed stdin is no.
#[expect(
    clippy::print_stderr,
    reason = "a CLI prompt, on stderr so redirected output cannot swallow it"
)]
pub(crate) fn confirm(count: usize) -> anyhow::Result<bool> {
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::domain::solution::answer::Answer;

    fn backed(value: &str) -> Outcome {
        Outcome::new(Ok(Answer::solved(value)), Duration::ZERO)
            .with_solver_verdict(SolverVerdict::Correct)
    }

    fn day(part_one: Outcome) -> Solved {
        Solved {
            parsed_in: Duration::ZERO,
            part_one,
            part_two: backed("two"),
        }
    }

    /// Submits `solved` against a fake AOC that grades part one as
    /// `part_one_verdict`, returning what came back and which parts went out.
    fn run(solved: Solved, part_one_verdict: &AocVerdict) -> (Solved, Vec<Part>) {
        let mut sent = Vec::new();
        let solved = submit_day(
            |part, _| {
                sent.push(part);
                Ok(match part {
                    Part::One => part_one_verdict.clone(),
                    Part::Two => AocVerdict::Correct,
                })
            },
            solved,
        )
        .unwrap();
        (solved, sent)
    }

    #[test]
    fn part_two_follows_a_starred_part_one() {
        for verdict in [AocVerdict::Correct, AocVerdict::AlreadySolved] {
            let (solved, sent) = run(day(backed("one")), &verdict);
            assert_eq!(sent, [Part::One, Part::Two]);
            assert_eq!(solved.part_two.aoc_verdict(), Some(&AocVerdict::Correct));
        }
    }

    /// AOC answers a part two sent too early with the same reply as one
    /// already solved, which would print as a star that does not exist.
    #[test]
    fn part_two_waits_while_part_one_is_unstarred() {
        let unstarred = [
            AocVerdict::Incorrect,
            AocVerdict::High,
            AocVerdict::Cooldown(None),
            AocVerdict::NotLoggedIn,
        ];
        for verdict in unstarred {
            let (solved, sent) = run(day(backed("one")), &verdict);
            assert_eq!(sent, [Part::One], "part one {verdict:?}");
            assert!(solved.part_two.aoc_verdict().is_none());
        }
    }

    /// A part one that never went out has no star to show for this run, so
    /// part two stays home too.
    #[test]
    fn part_two_waits_when_part_one_was_not_sent() {
        let rejected = Outcome::new(Ok(Answer::solved("one")), Duration::ZERO)
            .with_solver_verdict(SolverVerdict::High);
        let failed = Outcome::new(Err(anyhow::anyhow!("broke")), Duration::ZERO);
        for part_one in [rejected, failed] {
            let (_, sent) = run(day(part_one), &AocVerdict::Correct);
            assert_eq!(sent, []);
        }
    }
}
