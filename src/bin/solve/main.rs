pub mod args;

use crate::args::Args;
use clap::Parser;
use rustmas::{
    calendar::{FIRST_YEAR, latest_year},
    client::SolverClient,
    day::{Day, days_in_year},
    solutions::{answer::Answer, solution::solve, year_2015::day_01::Day01},
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
    (2015, 01) => Day01,
}

fn run(args: &Args) -> anyhow::Result<()> {
    // Submitting gates on a solver verdict, so it validates too.
    let validate = args.validate || args.submit;
    let solver = validate.then(SolverClient::new);

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
                Ok((one, two)) => {
                    println!("{year} day {}", day.value());
                    println!("  part one: {one}");
                    println!("  part two: {two}");
                }
                Err(e) => eprintln!("{year} day {} failed: {e:?}", day.value()),
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
