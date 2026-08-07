# rustmas

Rust tooling for Advent of Code. Downloads your puzzle inputs, runs your
solutions, checks the answers against an independent solver, and submits them
for stars.

## Setup

You need your Advent of Code session cookie. Log in at
[adventofcode.com](https://adventofcode.com), open your browser dev tools, and
copy the value of the cookie named `session`.

Copy [`.env.template`](.env.template) to `.env` and fill it in:

```
COOKIE=<your session cookie>
CONTACT=<an address AOC can reach you at>
REPO_URL=<your fork, if you forked>
```

Only `COOKIE` is required, and it belongs to your account, so `.env` is
gitignored. `CONTACT` and `REPO_URL` shape the `User-Agent`, because the site
asks automated clients to be reachable. Neither has a default that names anyone,
so leaving them blank identifies the tool and nobody else.

## Running it

One binary, one subcommand per mode.

```
cargo run fetch -y 2015 -d 1
cargo run solve -y 2015 -d 1
```

Cargo stops parsing at the subcommand and hands the rest to the program, so the
usual `--` separator is optional. Cargo's own flags go before it:

```
cargo run --release solve -y 2015 -d 1
```

Debug builds are slow enough to notice on anything that brute forces, so
`--release` is worth reaching for once a day takes longer than you want to sit
through.

## fetch

Downloads puzzle inputs and instructions into `cache/<year>/<NN>/`.

```
cargo run fetch                 # everything
cargo run fetch -y 2015         # one year
cargo run fetch -d 1            # day 1 of every year
cargo run fetch -y 2015 -d 1    # one puzzle
```

| Flag | Meaning |
| --- | --- |
| `-y`, `--year` | Only this year. Omit for all. |
| `-d`, `--day` | Only this day. Omit for all. |

Both flags are filters rather than a lookup, so omitting one means all of them.
Re-running is safe: an existing input counts as a cached download and is never
refetched, so `fetch` only fills gaps. Advent of Code asks that you not
re-download.

## solve

Runs your solutions, with the same filters.

```
cargo run solve -y 2015 -d 1              # offline
cargo run solve -y 2015 -d 1 --validate   # check the answers
cargo run solve -y 2015 -d 1 --submit     # check, then send for stars
```

| Flag | Meaning |
| --- | --- |
| `-y`, `--year` | Only this year. Omit for all. |
| `-d`, `--day` | Only this day. Omit for all. |
| `-v`, `--validate` | Check each answer against a third-party solver, one request per part. |
| `-s`, `--submit` | Submit to Advent of Code. Implies `--validate`. |
| `--yes` | Skip the confirmation prompt on an unfiltered `--submit`. |

Solving reads inputs from disk and downloads any that are missing, so a run
over cached days is entirely offline and needs no cookie. `--validate` needs
none either, since the third-party solver has no accounts.

`--submit` always validates first and only sends what the solver agrees with,
because a wrong answer to Advent of Code earns a cooldown that escalates with
repeats. If the solver has no implementation for that puzzle, which happens
during a live event, the answer is submitted anyway and flagged as unchecked.

Run `--submit` with no year or day and it would post every solved part, so it
prints the count and asks first. `--yes` skips that. There is no short flag for
it on purpose, since `-y` is `--year` and this one is worth typing out.

### Reading the output

```
year 2015 day 1 in 12.707µs (3.291µs parsing)
  part one: 138 (starred) [7.125µs]
  part two: 1771 (correct) [2.291µs]
```

Each part is one line: the answer, then whatever is known about it, then how
long it took. Timings cover parsing and solving only, never the network, and
they are worth reading in `--release` since debug builds run roughly twenty
times slower.

| Note | Meaning |
| --- | --- |
| nothing | Solved offline, unchecked |
| `correct` | The solver agrees |
| `high`, `low`, `incorrect` | The solver disagrees, so nothing was submitted |
| `new star` | Advent of Code just accepted it |
| `starred` | Advent of Code says the part was already solved |
| `unsupported` | The solver has no implementation for this puzzle |
| `(none)` | The part has no answer, such as day 25 part two |

Advent of Code grades each part exactly once, so a part solved earlier reports
`starred` rather than confirming the answer again.

## Adding a solution

Create `src/lib/solutions/year_<year>/day_<NN>/mod.rs` with a `Puzzle` type
implementing `Solution`, declare it in the year's `mod.rs`, then register it:

```rust
solutions! {
    (2015, 01) => year_2015::day_01::Puzzle,
}
```

Days are written zero padded, because the macro builds the input path from those
literals. Every day's type is named `Puzzle`, with the module path carrying the
coordinate.

Parts return an `Answer`: `Answer::solved(value)` for something submittable,
`Answer::Visual(art)` for a grid you read yourself, or `Answer::None` when
there is nothing to produce. Returning the art rather than printing it keeps
solving free of IO.

## Layout

```
src/
  bin/main.rs       # entry point, nothing else
  lib/
    domain/         # puzzles, with no idea HTTP or files exist
      address/      # which puzzle: Year, Day, Part
      solutions/
        answer.rs   # what a part produced
        solution.rs # the Solution trait and the runner
        year_2015/  # one dir per day, each with a Puzzle
        year_2016/
    inbound/        # ways in
      cli.rs        # the command line
      command.rs    # which subcommand was asked for
      fetch/        # the input downloader
      solve/        # the solution runner
    outbound/       # ways out
      client/
        aoc.rs      # adventofcode.com: inputs and submissions
        solver.rs   # third-party solver: repeatable answer checks
        verdict.rs  # what a checker made of an answer
cache/              # downloaded inputs and puzzle text (gitignored)
context/            # design notes, journal, and todo
```

The library is arranged as ports and adapters. `domain` holds the puzzle types
and knows nothing about the network, the filesystem, or the command line.
`inbound` is how a request arrives, `outbound` is how it leaves.

Inputs are read at runtime, so a fresh clone compiles with `cache/` empty.
`solve` downloads what it needs, which means `fetch` is for pulling things ahead
of time rather than a prerequisite.

## License

MIT. See [LICENSE](LICENSE).
