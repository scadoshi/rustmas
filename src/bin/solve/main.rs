pub mod args;
pub mod utils;

use crate::{
    args::Args,
    utils::{confirm, submit},
};
use clap::Parser;
use rustmas::{
    calendar::{FIRST_YEAR, latest_year},
    client::{AocClient, SolverClient},
    day::{Day, days_in_year},
    part::Part,
    solutions::{answer::Answer, solution::solve, year_2015, year_2016},
};

/// Generates `dispatch`, which maps a runtime `(year, day)` to the concrete
/// type that solves it. Returns `None` when no solution is registered.
macro_rules! solutions {
    ($(($y:literal, $d:literal) => $t:ty),* $(,)?) => {
        // Days are written zero-padded so `stringify!` builds the right filename.
        #[allow(clippy::zero_prefixed_literal)]
        fn dispatch(
            client: Option<&SolverClient>,
            day: &Day,
        ) -> Option<anyhow::Result<(Answer, Answer)>> {
            match (day.year(), day.value()) {
                $(($y, $d) => Some(solve::<$t>(
                    client,
                    include_str!(
                        concat!(
                            "../../../inputs/",
                            stringify!($y),
                            "/",
                            stringify!($d),
                            ".txt"
                        )
                    ),
                    day,
                )),)*
                _ => None,
            }
        }
    };
}

solutions! {
    (2015, 01) => year_2015::day_01::Puzzle,
    (2016, 01) => year_2016::day_01::Puzzle,
}

fn run(args: &Args) -> anyhow::Result<()> {
    // Submitting gates on a solver verdict, so it validates too.
    let validate = args.validate || args.submit;
    let solver = validate.then(SolverClient::new);

    if args.submitting_everything() && !args.yes {
        let count: usize = (FIRST_YEAR..=latest_year())
            .map(|year| days_in_year(year) as usize * 2)
            .sum();
        if !confirm(count)? {
            eprintln!("Nothing submitted.");
            return Ok(());
        }
    }

    // Only built when submitting, so validating alone never needs a cookie.
    let aoc = args.submit.then(AocClient::from_env).transpose()?;

    for year in FIRST_YEAR..=latest_year() {
        if args.year.is_some_and(|y| y != year) {
            continue;
        }
        for day in 1..=days_in_year(year) {
            if args.day.is_some_and(|d| d != day) {
                continue;
            }
            let day = Day::new(day, year)?;
            let Some(result) = dispatch(solver.as_ref(), &day) else {
                continue;
            };
            match result {
                Ok((mut one, mut two)) => {
                    // Submit before printing, so each part reports what both
                    // checkers said on one line.
                    if let Some(aoc) = aoc.as_ref() {
                        one = submit(aoc, &day, Part::One, one)?;
                        two = submit(aoc, &day, Part::Two, two)?;
                    }
                    println!("year {year} day {}", day.value());
                    println!("  part one: {one}");
                    println!("  part two: {two}");
                }
                Err(e) => eprintln!("year {year} day {} failed: {e:?}", day.value()),
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("Error: {e:?}");
    }
}
