use crate::{
    domain::{address::{Day, Part}, solutions::answer::Answer},
    outbound::client::{AocClient, verdict::Verdict},
};
use std::io::{self, Write};

/// Submits `answer` if the solver backed it, returning it with AOC's reply
/// attached.
///
/// A wrong answer costs an escalating cooldown, so the solver verdict gates the
/// send. [`Verdict::Unsupported`] goes through anyway, since an unimplemented
/// puzzle is one the solver cannot judge either way.
///
/// Rejected answers come back untouched, carrying only the solver's objection.
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
/// The question goes to stderr so redirecting output doesn't swallow it. Closed
/// stdin counts as no.
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
