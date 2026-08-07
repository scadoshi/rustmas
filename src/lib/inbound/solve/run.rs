use crate::{
    domain::{
        address::{Day, FIRST_YEAR, Part, days_in_year, latest_year},
        solutions::{solution::solve, year_2015, year_2016},
    },
    inbound::solve::{
        args::SolveArgs,
        utils::{confirm, submit},
    },
    outbound::{
        client::{AocClient, SolverClient},
        store,
    },
};

pub fn run(args: &SolveArgs) -> anyhow::Result<()> {
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
            let Some(input) = store::read_input(&day)? else {
                continue;
            };
            let client = solver.as_ref();
            // Days with no arm here are not written yet, so a run over every
            // year skips them.
            let result = match (day.year(), day.value()) {
                (2015, 1) => solve::<year_2015::day_01::Puzzle>(client, &input, &day),
                (2016, 1) => solve::<year_2016::day_01::Puzzle>(client, &input, &day),
                _ => continue,
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
