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

Download every published puzzle input into `inputs/<year>/<NN>.txt`:

```
cargo run --bin init
```

Re-running is safe. Existing inputs count as cached and are left untouched, so
`init` only fetches what's missing. Advent of Code asks that you not re-download.

## Layout

```
src/
  lib/            # shared library
    part.rs       # Year -> Day -> Part validated coordinates
    session.rs    # authenticated adventofcode.com client
  bin/
    init/         # the input downloader
    run.rs        # solution runner (WIP)
inputs/           # downloaded puzzle inputs (gitignored)
```

## License

MIT. See [LICENSE](LICENSE).
