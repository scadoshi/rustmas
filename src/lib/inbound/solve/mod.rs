//! The `solve` subcommand: run each matching day, then optionally check and
//! submit its answers.

pub mod args;
pub(crate) mod submission;

use crate::domain::address::{Day, Filter, PARTS_PER_DAY};
use crate::domain::solution::aoc_verdict::AocVerdict;
use crate::domain::solution::totals::Totals;
use crate::domain::solution::{Solved, year_template};
use crate::inbound::input;
use crate::inbound::solve::args::SolveArgs;
use crate::outbound::client::aoc_client::LazyAocClient;
use crate::outbound::client::solve;
use crate::outbound::client::solve::Validation;
use crate::outbound::client::solver_client::SolverClient;

/// A day's solver, once its concrete type is known.
type Solver = fn(Validation<'_>, &str, &Day) -> anyhow::Result<Solved>;

/// The template day as a [`Solver`]. With no arms below, nothing else reaches
/// the runner, and this keeps it compiled and checked until the first day lands.
const _: Solver = solve::run_day::<year_template::day_01::Puzzle>;

/// The solver for a day, or `None` when nobody has written one.
///
/// Returns a pointer rather than calling, so a run skips unwritten days before
/// downloading and `--submit` counts first. One line per day, and the only
/// list of what has been solved.
fn solver_for(year: i32, day: i32) -> Option<Solver> {
    match (year, day) {
        // One arm per day. Import the year module and `outbound::client::solve`
        // at the top of this file, then:
        //
        //     (2015, 1) => Some(solve::run_day::<year_2015::day_01::Puzzle>),
        _ => None,
    }
}

/// How many parts a run over these filters would touch.
fn part_count(filter: Filter) -> usize {
    Day::matching(filter)
        .filter(|day| solver_for(day.year(), day.value()).is_some())
        .count()
        .saturating_mul(PARTS_PER_DAY)
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a CLI: the report and skipped days are its output"
)]
pub(crate) fn run(args: &SolveArgs) -> anyhow::Result<()> {
    let solver = SolverClient::from_env()?;
    // Submitting gates on a solver verdict, so it validates too.
    let validation = if args.validate || args.submit {
        Validation::Against(&solver)
    } else {
        Validation::Off
    };
    let filter = Filter::new(args.year, args.day)?;

    let count = part_count(filter);
    if args.submitting_everything() && !args.yes && count > 0 && !submission::confirm(count)? {
        eprintln!("Nothing submitted.");
        return Ok(());
    }

    // Built up front when submitting, so a bad cookie fails before any solving.
    let mut aoc = LazyAocClient::default();
    if args.submit {
        aoc.connected()?;
    }

    let mut totals = Totals::default();
    for day in Day::matching(filter) {
        // Asked before fetching, so unsolvable days download nothing.
        let Some(solver_fn) = solver_for(day.year(), day.value()) else {
            if args.day.is_some() {
                eprintln!("{day} has no solution yet");
            }
            continue;
        };

        let entry = input::ensure_entry(&mut aoc, &day)?;
        match solver_fn(validation, entry.input.data(), &day) {
            Ok(mut solved) => {
                // Before printing, so each part reports both checkers on one
                // line. Gated on the flag, not on the client existing.
                if args.submit {
                    let client = aoc.connected()?;
                    solved = submission::submit_day(
                        |part, value| client.submit_answer(&day, part, value),
                        solved,
                    )?;
                }

                // A new star on part one unlocks part two's text, so fetch it.
                if matches!(solved.part_one.aoc_verdict(), Some(AocVerdict::Correct)) {
                    input::ensure_entry(&mut aoc, &day)?;
                }

                println!(
                    "{day} in {:?} ({:?} parsing)",
                    solved.total_elapsed(),
                    solved.parsed_in
                );
                println!("  part one: {}", solved.part_one);
                println!("  part two: {}", solved.part_two);

                totals.add(&day, &solved);
            }
            Err(e) => eprintln!("{day} failed: {e:?}"),
        }
    }
    // Below two days the summary would only restate the lines above it.
    if totals.days() > 1 {
        println!();
        print!("{totals}");
    }
    Ok(())
}
