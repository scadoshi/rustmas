use crate::{
    domain::{
        address::{self, Part},
        solutions::{
            solution::{Solved, solve},
            year_2015, year_2016,
        },
    },
    inbound::{
        input::ensure_entry,
        solve::{
            args::SolveArgs,
            utils::{confirm, submit},
        },
    },
    outbound::client::{AocClient, SolverClient},
};

/// A day's solver, once its concrete type is known.
type Solver = fn(&SolverClient, bool, &str, &address::Day) -> anyhow::Result<Solved>;

pub fn run(args: &SolveArgs) -> anyhow::Result<()> {
    // Submitting gates on a solver verdict, so it validates too.
    let validate = args.validate || args.submit;
    let solver = SolverClient::new();

    if args.submitting_everything() && !args.yes {
        let count = address::each(None, None).count() * 2;
        if !confirm(count)? {
            eprintln!("Nothing submitted.");
            return Ok(());
        }
    }

    // Built up front when submitting, so a bad cookie fails before any solving.
    // Otherwise built on first download, leaving cached runs offline.
    let mut aoc = args.submit.then(AocClient::from_env).transpose()?;

    for day in address::each(args.year, args.day) {
        let day = day?;

        // Days with no arm here are not written yet. Matching before fetching
        // keeps a run over every year from downloading inputs it cannot use.
        let solver_fn: Solver = match (day.year(), day.value()) {
            (2015, 1) => solve::<year_2015::day_01::Puzzle>,
            (2016, 1) => solve::<year_2016::day_01::Puzzle>,
            _ => continue,
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
                    let aoc = aoc.as_ref().expect("built up front when submitting");
                    solved.one = submit(aoc, &day, Part::One, solved.one)?;
                    solved.two = submit(aoc, &day, Part::Two, solved.two)?;
                }
                println!(
                    "year {} day {} in {:?} ({:?} parsing)",
                    day.year(),
                    day.value(),
                    solved.total(),
                    solved.parse
                );
                println!("  part one: {}", solved.one);
                println!("  part two: {}", solved.two);
            }
            Err(e) => eprintln!("year {} day {} failed: {e:?}", day.year(), day.value()),
        }
    }
    Ok(())
}
