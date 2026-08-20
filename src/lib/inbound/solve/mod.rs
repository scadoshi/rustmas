pub mod args;
pub mod utils;

use crate::{
    domain::{
        address::{Day, Part},
        solution::{Solved, aoc_verdict::AocVerdict},
    },
    inbound::{
        input::ensure_entry,
        solve::{
            args::SolveArgs,
            utils::{confirm, submit},
        },
    },
    outbound::client::{aoc_client::AocClient, solver_client::SolverClient},
};

/// A day's solver, once its concrete type is known.
type Solver = fn(&SolverClient, bool, &str, &Day) -> anyhow::Result<Solved>;

/// The solver for a day, or `None` when nobody has written one.
///
/// Returns a pointer rather than calling, so the registry can be asked whether
/// a day exists without holding its input. That is what lets a run skip
/// unwritten days before downloading and lets `--submit` count first.
///
/// One line per day. This is the only list of what has been solved.
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
fn part_count(year: Option<i32>, day: Option<i32>) -> usize {
    Day::each(year, day)
        .filter_map(Result::ok)
        .filter(|day| solver_for(day.year(), day.value()).is_some())
        .count()
        * 2
}

pub fn run(args: &SolveArgs) -> anyhow::Result<()> {
    // Submitting gates on a solver verdict, so it validates too.
    let validate = args.validate || args.submit;
    let solver = SolverClient::from_env()?;

    let count = part_count(args.year, args.day);
    if args.submitting_everything() && !args.yes && count > 0 && !confirm(count)? {
        eprintln!("Nothing submitted.");
        return Ok(());
    }

    // Built up front when submitting, so a bad cookie fails before any solving.
    // Otherwise built on first download, leaving cached runs offline.
    let mut aoc = args.submit.then(AocClient::from_env).transpose()?;

    for day in Day::each(args.year, args.day) {
        let day = day?;

        // Asked before fetching, so a run over every year downloads nothing for
        // days it cannot solve.
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
                // Submit before printing, so each part reports what both
                // checkers said on one line.
                // Gated on the flag, not on the client existing: fetching a
                // missing input builds one too.
                if args.submit {
                    let client = aoc.as_ref().expect("built up front when submitting");
                    solved.one = submit(client, &day, Part::One, solved.one)?;
                    solved.two = submit(client, &day, Part::Two, solved.two)?;
                }

                // A new star on part one unlocks part two, which was still
                // locked when this run read the cache.
                if matches!(solved.one.submission(), Some(AocVerdict::Correct)) {
                    ensure_entry(&mut aoc, &day)?;
                }

                println!(
                    "year {} day {} in {:?} ({:?} parsing)",
                    day.year(),
                    day.value(),
                    solved.total(),
                    solved.parsed_in
                );
                println!("  part one: {}", solved.one);
                println!("  part two: {}", solved.two);
            }
            Err(e) => eprintln!("year {} day {} failed: {e:?}", day.year(), day.value()),
        }
    }
    Ok(())
}
