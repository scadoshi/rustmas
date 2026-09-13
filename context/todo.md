# Todo

## Where this is

Feature complete and pushed. `fetch`, `solve`, `--validate`, and `--submit` all
work, 81 tests pass, and both service contracts are in `references.md` from
live probing. Day one of every year except 2019 is solved, with 2019 saved to
be done in one run.

The 2026-08-20 session added the eager `Filter` (`-y 2030` errors instead of
matching nothing in silence), split the address errors per producer, made day
expansion infallible, and renamed `Solved`'s fields. The journal has the detail.

The 2026-09-13 session added a run summary: totals, both means, and the slowest
part, aggregated by `Totals` in `domain/solution/totals.rs`.

## Next

Get the summary onto `main`, then merge down. It is tool code, so it belongs
there, but it is sitting uncommitted on `scadoshi`.

Identical on both branches, so they copy into a `main` worktree as they are:

- `src/lib/domain/solution/totals.rs`, new
- `src/lib/domain/solution/outcome.rs`, for `solve_time`
- `src/lib/domain/address/part.rs`, for `Display`

Divergent, so each gets the same edit by hand on both sides:

- `src/lib/domain/solution/mod.rs`, one `pub mod totals;` line
- `src/lib/inbound/solve/mod.rs`, the `Totals` import, the accumulator, the
  `add` call, and the tail block

`context/` stays here.

Then delete `src/lib/solve.rs` and `src/lib/cli.rs`, which are not in the
module tree and do not compile. A `main` job, since both exist there too.

## Still open from before

- Go back through every finished day and test the parsing: trailing blank line,
  a line the parser cannot split, and whether the message names the line and
  the day. The 2026-08-12 journal note has the reasoning.
- 2019, all days in one run, when the mood strikes.
- The day twos, now that every day one is done.
