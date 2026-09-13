pub mod args;
pub mod utils;

use crate::{
    domain::{
        address::{Day, Filter, Part},
        solution::{Solved, aoc_verdict::AocVerdict, totals::Totals},
    },
    inbound::{
        input::ensure_entry,
        solve::{
            args::SolveArgs,
            utils::{confirm, submit},
        },
    },
    outbound::client::{aoc_client::LazyAocClient, solver_client::SolverClient},
};

/// A day's solver, once its concrete type is known.
type Solver = fn(&SolverClient, bool, &str, &Day) -> anyhow::Result<Solved>;

/// The solver for a day, or `None` when nobody has written one.
///
/// Returns a pointer rather than calling, so a run skips unwritten days before
/// downloading and `--submit` counts first. One line per day, and the only
/// list of what has been solved.
fn solver_for(year: i32, day: i32) -> Option<Solver> {
    match (year, day) {
        // One arm per day. Import the year module and `solve` at the top of this
        // file, then:
        //
        //     (2015, 1) => Some(solve::<year_2015::day_01::Puzzle>),
        _ => None,
    }
}

/// How many parts a run over these filters would touch.
fn part_count(filter: Filter) -> usize {
    Day::matching(filter)
        .filter(|day| solver_for(day.year(), day.value()).is_some())
        .count()
        * 2
}

pub fn run(args: &SolveArgs) -> anyhow::Result<()> {
    // Submitting gates on a solver verdict, so it validates too.
    let validate = args.validate || args.submit;
    let solver = SolverClient::from_env()?;
    let filter = Filter::new(args.year, args.day)?;

    let count = part_count(filter);
    if args.submitting_everything() && !args.yes && count > 0 && !confirm(count)? {
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
                eprintln!(
                    "year {} day {} has no solution yet",
                    day.year(),
                    day.value()
                );
            }
            continue;
        };

        let entry = ensure_entry(&mut aoc, &day)?;
        let input = entry.input.data();
        match solver_fn(&solver, validate, input, &day) {
            Ok(mut solved) => {
                // Before printing, so each part reports both checkers on one
                // line. Gated on the flag, not on the client existing.
                if args.submit {
                    let client = aoc.connected()?;
                    solved.part_one = submit(client, &day, Part::One, solved.part_one)?;
                    solved.part_two = submit(client, &day, Part::Two, solved.part_two)?;
                }

                // A new star on part one unlocks part two, locked until now.
                if matches!(solved.part_one.aoc_verdict(), Some(AocVerdict::Correct)) {
                    ensure_entry(&mut aoc, &day)?;
                }

                println!(
                    "year {} day {} in {:?} ({:?} parsing)",
                    day.year(),
                    day.value(),
                    solved.total_elapsed(),
                    solved.parsed_in
                );
                println!("  part one: {}", solved.part_one);
                println!("  part two: {}", solved.part_two);

                totals.add(&day, &solved);
            }
            Err(e) => eprintln!("year {} day {} failed: {e:?}", day.year(), day.value()),
        }
    }
    // Below two days the summary would only restate the lines above it.
    if totals.days() > 1 {
        println!();
        print!("{totals}");
    }
    Ok(())
}
