use rustmas::{
    client::{AocClient, verdict::Verdict},
    day::Day,
    part::Part,
    solutions::answer::Answer,
};
use std::io::{self, Write};

/// Submits `answer` if the solver backed it, returning it with whatever AOC
/// said attached.
///
/// A wrong answer costs an escalating cooldown, so a solver verdict is the gate.
/// [`Verdict::Unsupported`] is let through: that means the solver has no
/// implementation, which happens during a live event when a day is solved before
/// the solver catches up, and is exactly when submitting matters most.
///
/// Answers the solver rejected come back untouched, so printing shows the
/// solver's objection and nothing about a submission that never happened.
pub fn submit(aoc: &AocClient, day: &Day, part: Part, answer: Answer) -> anyhow::Result<Answer> {
    let Some(value) = answer.value() else {
        return Ok(answer);
    };
    if !matches!(
        answer.verdict(),
        Some(Verdict::Correct) | Some(Verdict::Unsupported)
    ) {
        return Ok(answer);
    }

    let verdict = aoc.submit_answer(day, part, value)?;
    Ok(answer.with_submission(verdict))
}

/// Asks before an unfiltered submit run, which would post every solved day.
///
/// Reads stdin and writes the question to stderr, so redirecting output doesn't
/// swallow it. A closed stdin counts as no, since the point is to stop an
/// accident rather than to block a script.
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
