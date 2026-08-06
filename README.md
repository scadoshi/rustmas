# rustmas

Rust tooling for Advent of Code. Right now it fetches puzzle inputs. Solutions come later.

## Setup

You need your Advent of Code session cookie. Log in at
[adventofcode.com](https://adventofcode.com), open your browser dev tools, and
copy the value of the `session` cookie. Put it in a `.env` file at the repo
root:

```
COOKIE=<your session cookie>
```

The cookie belongs to your account, so `.env` is gitignored.

## Usage

Download published puzzle inputs into `inputs/<year>/<NN>.txt`:

```
cargo run --bin fetch                 # everything
cargo run --bin fetch -- -y 2015      # one year
cargo run --bin fetch -- -y 2015 -d 1 # one puzzle
```

Re-running is safe. Existing inputs count as cached and are left untouched, so
`fetch` only fetches what's missing. Advent of Code asks that you not re-download.

Then run the solutions, with the same filters:

```
cargo run --bin solve -- -y 2015 -d 1
cargo run --bin solve -- -y 2015 -d 1 --validate
```

`--validate` checks each answer against a third-party solver, one request per
part. Without it, solving is entirely offline.

Solutions embed their input with `include_str!`, so `solve` will not compile
until `fetch` has downloaded the days it covers. `inputs/` is gitignored, which
means a fresh clone has to run `fetch` first.

## Layout

```
src/
  lib/            # shared library
    calendar.rs   # which events exist
    day.rs        # Year -> Day validated coordinates
    part.rs       # which of a day's two puzzles
    session/      # adventofcode.com and solver clients
    solutions/    # one module per puzzle, implementing Solution
  bin/
    fetch/        # the input downloader
    solve/        # the solution runner
inputs/           # downloaded puzzle inputs (gitignored)
```

## License

MIT. See [LICENSE](LICENSE).
