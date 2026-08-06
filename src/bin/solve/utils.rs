use rustmas::{
    client::{AocClient, verdict::Verdict},
    day::Day,
    part::Part,
    solutions::answer::Answer,
};
use std::io::{self, Write};

/// Submits `answer` if the solver backed it, and reports either way.
///
/// A wrong answer costs an escalating cooldown, so a solver verdict is the gate.
/// [`Verdict::Unsupported`] is let through: that means the solver has no
/// implementation, which happens during a live event when a day is solved before
/// the solver catches up, and is exactly when submitting matters most.
pub fn submit(aoc: &AocClient, day: &Day, part: Part, answer: &Answer) -> anyhow::Result<()> {
    let Some(value) = answer.value() else {
        return Ok(());
    };
    let label = format!("  part {} submit", part.to_wire_value());

    match answer.verdict() {
        Some(Verdict::Correct) => {}
        Some(Verdict::Unsupported) => {
            println!("{label}: solver has no answer to check against, submitting anyway");
        }
        Some(other) => {
            println!("{label}: skipped, solver says {other}");
            return Ok(());
        }
        None => {
            println!("{label}: skipped, no solver verdict");
            return Ok(());
        }
    }

    println!("{label}: {}", aoc.submit_answer(day, part, value)?);
    Ok(())
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
