# rustmas

Rust tooling for Advent of Code. Downloads your puzzle inputs, runs your
solutions, checks the answers against an independent solver, and submits them
for stars.

This is scadoshi's working branch, with his solutions attached. The `main`
branch is the same tool with no solutions, which is the one to clone if you want
a starting point.

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

Re-running is safe. Inputs never change, so a cached one is never fetched again;
Advent of Code asks that you not re-download. Instructions are different, since
part two stays locked until part one is solved. A day cached without
`part_two.md` is incomplete rather than finished, so `fetch` asks for it again
on every run until it arrives.

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

Solving reads inputs from disk and downloads what is missing. With no cookie set
it stays entirely offline. With one set, a day still waiting on part two costs a
request to see whether it has unlocked. `--validate` needs no cookie either,
since the third-party solver has no accounts.

`--submit` always validates first and only sends what the solver agrees with,
because a wrong answer to Advent of Code earns a cooldown that escalates with
repeats. If the solver has no implementation for that puzzle, which happens
during a live event, the answer is submitted anyway and flagged as unchecked.

A new star on part one unlocks part two, so `--submit` fetches its text before
the run finishes rather than leaving it for next time.

Run `--submit` with no year or day and it would post every solved part, so it
prints the count and asks first. `--yes` skips that. There is no short flag for
it on purpose, since `-y` is `--year` and this one is worth typing out.

### Reading the output

```
year 2015 day 1 in 12.707µs (3.291µs parsing)
  part one: 138 (starred) [7.125µs]
  part two: 1771 (correct) [2.291µs]
```

Until a day has a solution, `solve` skips it. Asking for one by name says so
rather than printing nothing.

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
| `rate limited, 1m 0s left to wait` | Advent of Code refused to grade, wait it out |
| `(none)` | The part has no answer, such as day 25 part two |
| `(unwritten)` | Nobody has written this part yet |
| `error: ...` | The part failed. The other part still ran |

Advent of Code grades each part exactly once, so a part solved earlier reports
`starred` rather than confirming the answer again.

## Adding a solution

Three steps. Say you are writing 2015 day 1.

Copy the template, which is compiled on every build and so cannot drift from
the trait:

```sh
cp -r src/lib/domain/solution/year_template \
      src/lib/domain/solution/year_2015
```

That gives you `year_2015/day_01/` with both parts stubbed, and a `mod.rs` that
already declares the day. Write the parts:

```rust
fn part_one(&self) -> anyhow::Result<Answer> {
    Ok(Answer::solved(self.input.len().to_string()))
}
```

Register it. `solution/mod.rs` needs `pub mod year_2015;`, and `solver_for` in
`src/lib/inbound/solve/run.rs` needs an arm, importing the year module and
`solve` at the top of that file:

```rust
use crate::{
    domain::solution::year_2015,
    outbound::client::solve::solve,
};

// ...

(2015, 1) => solve::<year_2015::day_01::Puzzle>,
```

That match is the only list of what has been solved. A day missing from it is
skipped rather than failing.

Anything more than one day needs goes in `src/lib/domain/solution/common/`,
which ships empty on `main` and holds `Point`, `Cell`, `Direction`, and `Turn`
here. Add a `pub mod` line there and write the type. Grid and geometry work is
what usually ends up there, since Advent of Code returns to it every year.

Write those the second day that wants them rather than the first, and give them
tests: a break in a shared type corrupts every day at once, where a single day's
logic is already checked by `--validate`.

Every day's type is named `Puzzle`, with the module path carrying the
coordinate, so importing the year modules keeps two years from colliding.

Parts return `anyhow::Result<Answer>`: `Answer::solved(value)` for something
submittable, `Answer::Visual(art)` for a grid you read yourself, `Answer::None`
when there is genuinely no answer, and `Answer::Unwritten` while a part is still
a stub. An `Err` means the day is broken, and stops that part without stopping
the other.

Returning art rather than printing it keeps solving free of IO. `new` parses
once so both parts read the result, and takes `impl AsRef<str>` so a day that
parses into its own types keeps no copy of the raw text.

## Layout

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
        common/              # helpers days share: Point, Cell, Direction, Turn
        year_template/       # copy this to start a year
        year_2015/           # one dir per day, each with a Puzzle
        year_2016/           # 2017, 2018, 2020, 2021, 2022 too
    inbound/                 # ways in
      cli.rs                 # the command line and its subcommands
      input.rs               # read the cache, fetch what is missing
      fetch/                 # the input downloader
      solve/                 # the solution runner
    outbound/                # ways out
      client/
        aoc_client.rs        # adventofcode.com: inputs and submissions
        solver_client.rs     # third-party solver: repeatable answer checks
        solve.rs             # runs a day, validating against the solver
      store/                 # the cache on disk
cache/                       # downloaded inputs and puzzle text (gitignored)
context/                     # design notes, journal, and todo
```

The library is arranged as ports and adapters. `domain` holds the puzzle types
and knows nothing about the network, the filesystem, or the command line.
`inbound` is how a request arrives, `outbound` is how it leaves.

Inputs are read at runtime, so a fresh clone compiles with `cache/` empty.
`solve` downloads what it needs, which means `fetch` is for pulling things ahead
of time rather than a prerequisite.

## Credits

`--validate` and `--submit` both lean on
[fornwall/advent-of-code](https://github.com/fornwall/advent-of-code), an
independent solver by [Fredrik Fornwall](https://fornwall.net/) covering the
published puzzles. It answers the same question as many times as you ask,
which is what makes it usable as a regression check and as a guard before
spending a submission. Its API is documented at
[aoc.fornwall.net/api](https://aoc.fornwall.net/api/).

Advent of Code itself is by [Eric Wastl](https://was.tl/). Please read the
[about page](https://adventofcode.com/about) on how to treat the site kindly:
this tool caches inputs rather than re-downloading them, keeps puzzle text out
of git, and identifies itself in its `User-Agent`.

## License

MIT. See [LICENSE](LICENSE).
