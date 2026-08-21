# Todo

## Where this is

Feature complete and pushed. `fetch`, `solve`, `--validate`, and `--submit` all
work, 71 tests pass, and both service contracts are in `references.md` from
live probing. Day one of every year except 2019 is solved, with 2019 saved to
be done in one run.

The 2026-08-20 session added the eager `Filter` (`-y 2030` errors instead of
matching nothing in silence), split the address errors per producer, made day
expansion infallible, and renamed `Solved`'s fields. The journal has the detail.

## Next

The Filter revision pass from 2026-08-20 is done: the day 25 gate is wired so
nothing rechecks a part two that cannot exist, the pair filter is pinned to the
day it yields, and both error messages are asserted in full. Nothing queued
beyond what follows.

## Still open from before

- Go back through every finished day and test the parsing: trailing blank line,
  a line the parser cannot split, and whether the message names the line and
  the day. The 2026-08-12 journal note has the reasoning.
- 2019, all days in one run, when the mood strikes.
- The day twos, now that every day one is done.
