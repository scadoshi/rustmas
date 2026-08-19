# rustmas

Rust tooling for Advent of Code. Downloads your puzzle inputs, runs your
solutions, checks the answers against an independent solver, and submits them
for stars.

No solutions ship with it. Clone it, add your `.env`, and write your first day.
Scotty's own solutions live on the `scadoshi` branch if you want worked
examples.

## Quick start

**1. Add your session cookie.** Log in at
[adventofcode.com](https://adventofcode.com), copy the value of the cookie named
`session` from your browser's dev tools, then:

```sh
cp .env.template .env    # paste the cookie into COOKIE=
```

Only `COOKIE` is required. `CONTACT` and `REPO_URL` shape the `User-Agent`,
since the site asks automated clients to be reachable.

**2. Run something.** One binary, one subcommand per mode. Cargo stops parsing
at the subcommand, so no `--` is needed:

```sh
cargo run fetch -y 2015 -d 1              # download one puzzle into cache/
cargo run solve -y 2015 -d 1              # run your solution offline
cargo run solve -y 2015 -d 1 --validate   # check answers against the solver
cargo run solve -y 2015 -d 1 --submit     # check, then send for stars
```

`-y` and `-d` are filters: omit either for all of them. `--submit` validates
first and only sends what the solver agrees with, because wrong answers cost an
escalating cooldown. Unfiltered submits ask before posting everything; `--yes`
skips that. Debug builds drag on brute-force days, so `cargo run --release`
when it does.

**3. Write a day.** Say 2015 day 1. Copy the template, which is compiled on
every build and so cannot drift from the trait:

```sh
cp -r src/lib/domain/solution/year_template src/lib/domain/solution/year_2015
```

That gives you `year_2015/day_01/` with both parts stubbed and a `mod.rs` that
already declares the day. Write the parts; each returns
`Answer::solved(value)`, `Answer::Visual(art)`, or `Answer::None`:

```rust
fn part_one(&self) -> anyhow::Result<Answer> {
    Ok(Answer::solved(self.input.len().to_string()))
}
```

Register it: `solution/mod.rs` needs `pub mod year_2015;`, and `solver_for` in
`src/lib/inbound/solve/run.rs` needs an arm:

```rust
(2015, 1) => solve::<year_2015::day_01::Puzzle>,
```

That match is the only list of what has been solved. Then:

```sh
cargo run solve -y 2015 -d 1 --validate
```

## Reading the output

```
year 2015 day 1 in 12.707µs (3.291µs parsing)
  part one: 138 (correct) [7.125µs]
  part two: 1771 (correct) [2.291µs]
```

One line per part: the answer, what is known about it, and how long it took.
Timings never include the network.

| Note | Meaning |
| --- | --- |
| nothing | Solved offline, unchecked |
| `correct` | The solver agrees |
| `high`, `low`, `incorrect` | The solver disagrees, so nothing was submitted |
| `new star` | Advent of Code just accepted it |
| `starred` | Advent of Code says the part was already solved |
| `unsupported` | The solver has no implementation for this puzzle |
| `rate limited, 1m 0s left to wait` | Advent of Code refused to grade |
| `(none)` | The part has no answer, such as day 25 part two |
| `(unwritten)` | Nobody has written this part yet |
| `error: ...` | The part failed. The other part still ran |

## Worth knowing

- Re-running `fetch` is safe: inputs are never re-downloaded, and a day still
  waiting on part two is rechecked until it unlocks.
- With no cookie set, `solve` works entirely offline from the cache, and
  `--validate` still works, since the solver needs no account.
- Shared helpers more than one day needs go in `solution/common/`.

## Going deeper

`context/` holds the facts about the repo, one file per topic:

- [`context/architecture.md`](context/architecture.md), the layout, how a solve
  runs, and the cache on disk.
- [`context/design/`](context/design/), why the code looks the way it does,
  including the options that were rejected and the reasons.
- [`context/references.md`](context/references.md), the two service contracts,
  verified live and against the solver's source.

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
