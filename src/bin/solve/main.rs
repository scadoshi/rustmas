pub mod utils;

use crate::utils::Args;
use clap::Parser;
use rustmas::{
    day::{Day, days_in_year},
    session::Session,
    solutions::{Answer, solve, year_2015::day_01::Day01},
};

/// Generates `dispatch`, which maps a runtime `(year, day)` to the concrete
/// type that solves it. Returns `None` when no solution is registered.
macro_rules! solutions {
    ($(($y:literal, $d:literal) => $t:ty),* $(,)?) => {
        // Days are written zero-padded so `stringify!` builds the right filename.
        #[allow(clippy::zero_prefixed_literal)]
        fn dispatch(
            session: Option<&Session>,
            day: &Day,
        ) -> Option<anyhow::Result<(Answer, Answer)>> {
            match (day.year(), day.value()) {
                $(($y, $d) => Some(solve::<$t>(
                    session,
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
    // Only built when validating, so solving offline never needs a cookie.
    let session = args.validate.then(Session::from_env).transpose()?;

    for year in 2015..=2025 {
        if args.year.is_some_and(|y| y != year) {
            continue;
        }
        for day in 1..=days_in_year(year) {
            if args.day.is_some_and(|d| d != day) {
                continue;
            }
            let day = Day::new(day, year)?;
            let Some(result) = dispatch(session.as_ref(), &day) else {
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
