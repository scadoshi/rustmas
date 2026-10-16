# Todo

## Where this is

Feature complete and pushed. `fetch`, `solve`, `--validate`, and `--submit` all work, and 82 tests pass.

- Both service contracts are in `references.md`, from live probing.
- Day one of every year except 2019 is solved. 2019 is saved to be done in one run.

The 2026-08-20 session did four things (the journal has the detail):

- Added the eager `Filter`, so `-y 2030` errors instead of silently matching nothing.
- Split the address errors per producer.
- Made day expansion infallible.
- Renamed `Solved`'s fields.

The 2026-09-13 session added a run summary: totals, both means, and the slowest part, aggregated by `Totals` in `domain/solution/totals.rs`. It also cut 48 comment lines and deleted two files that had stopped compiling.

## Next

Nothing queued. The summary and the comment pass both landed on `main` and merged down. `src/lib/solve.rs` and `src/lib/cli.rs` are gone from both branches.

## Still open from before

- Go back through every finished day and test the parsing: a trailing blank line, a line the parser can't split, and whether the message names the line and the day. The 2026-08-12 journal note has the reasoning.
- 2019, all days in one run, when the mood strikes.
- The day twos, now that every day one is done.
