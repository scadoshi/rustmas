pub mod utils;

use crate::utils::{Args, solve};
use clap::Parser;
use rustmas::{day::days_in_year, solutions::year_2015::day_01::Day01};

/// Generates `dispatch`, which maps a runtime `(year, day)` to the concrete
/// type that solves it. Returns `None` when no solution is registered.
macro_rules! solutions {
    ($(($y:literal, $d:literal) => $t:ty),* $(,)?) => {
        // Days are written zero-padded so `stringify!` builds the right filename.
        #[allow(clippy::zero_prefixed_literal)]
        fn dispatch(year: u32, day: u32)
            -> Option<anyhow::Result<(Option<String>, Option<String>)>>
        {
            match (year, day) {
                $(($y, $d) => Some(
                    solve::<$t>(include_str!(
                        concat!(
                            "../../../inputs/",
                            stringify!($y),
                            "/",
                            stringify!($d),
                            ".txt"
                        )
                    )
                )
            ),)*
                _ => None,
            }
        }
    };
}

solutions! {
    (2015, 01) => Day01,
}

fn main() {
    let args = Args::parse();
    for year in 2015..=2025 {
        if args.year.is_some_and(|y| y != year) {
            continue;
        }
        for day in 1..=days_in_year(year) {
            if args.day.is_some_and(|d| d != day) {
                continue;
            }
            let Some(result) = dispatch(year, day) else {
                continue;
            };
            match result {
                Ok((one, two)) => println!("{year} day {day}: {one:?} {two:?}"),
                Err(e) => eprintln!("{year} day {day} failed: {e:?}"),
            }
        }
    }
}
