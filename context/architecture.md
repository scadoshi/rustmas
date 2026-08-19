# Architecture

Ports and adapters. `domain` holds the puzzle types and imports nothing outside
itself: no network, no filesystem, no CLI. `inbound` is how a request arrives,
`outbound` is how it leaves, and each depends on the domain rather than the
reverse.

```
src/
  bin/main.rs                # entry point, nothing else
  lib/
    domain/                  # puzzles, with no idea HTTP or files exist
      address/               # which puzzle: Year, Day, Part
      solution/
        mod.rs               # the Solution trait, and one run's timings
        answer.rs            # what a part produced
        outcome.rs           # that answer, plus timing and verdicts
        aoc_verdict.rs       # what AOC said about a submission
        solver_verdict.rs    # what the solver made of an answer
        common/              # helpers more than one day needs, empty to start
        year_template/       # copy this to start a year
        year_<year>/         # your days go here, one dir each
    inbound/                 # ways in
      cli.rs                 # the command line and its subcommands
      input.rs               # read the cache, fetch what is missing
      fetch/                 # the input downloader
      solve/                 # the solution runner
    outbound/                # ways out
      client/
        environment.rs       # every env var the tool reads
        aoc_client.rs        # adventofcode.com: inputs and submissions
        solver_client.rs     # third-party solver: repeatable answer checks
        solve.rs             # runs a day, validating against the solver
      store/                 # the cache on disk
cache/                       # downloaded inputs and puzzle text (gitignored)
context/                     # these notes
```

## How a solve runs

```
cli               parses solve -y 2015 -d 1 --validate
  solve/run       looks up the day in solver_for, gets solve::<Puzzle>
    input         reads the cache, downloads whatever is missing
      store       one directory per day of plain files
    solve         S::new(input), times each part, holds a failure per part
      Puzzle      the day's own logic
      solver_client  checks each submittable answer when validating
    Outcome       renders answer, verdicts, and timing on one line
```

A day is a `Puzzle` type implementing `Solution`: `new` parses once and errors
on bad input, and `part_one`/`part_two` read the parsed result. The `solver_for`
match is the only list of what has been solved; a day missing from it is skipped
rather than failing.

## The cache

```
cache/2015/01/input.txt     the puzzle input, verbatim
cache/2015/01/session       hash of the cookie that fetched it
cache/2015/01/part_one.md   puzzle text
cache/2015/01/part_two.md   puzzle text, absent until part one is solved
```

Every file is readable on its own. The session hash is what catches a swapped
account, since inputs are account specific. Part two's absence means still
locked, which is rechecked each run until it arrives.

Inputs are read at runtime, so a fresh clone compiles with `cache/` empty.
`solve` downloads what it needs, which makes `fetch` a way to pull things ahead
of time rather than a prerequisite.
