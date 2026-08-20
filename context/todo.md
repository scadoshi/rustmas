# Todo

## Where this is

Feature complete and pushed. `fetch`, `solve`, `--validate`, and `--submit` all
work, 71 tests pass, and both service contracts are in `references.md` from
live probing. Day one of every year except 2019 is solved, with 2019 saved to
be done in one run.

The 2026-08-20 session added the eager `Filter` (`-y 2030` errors instead of
matching nothing in silence), split the address errors per producer, made day
expansion infallible, and renamed `Solved`'s fields. The journal has the detail.

## Next: a revision pass on the Filter work

Come back and keep revising until it is solid. Known items, in order:

- **Wire `has_second_puzzle` into `ensure_entry`.** It is dead code today: the
  `chase_part_two` gate still asks only whether `part_two.md` is missing, so a
  cached day 25 costs a network request every run, forever, chasing text that
  cannot exist. One clause: `&& day.has_second_puzzle()`.
- **Check the first-fetch path for day 25** while in there: a fresh fetch should
  already write nothing for part two, but confirm rather than assume.
- **Pin `matching` with both sides.** The test asserts `count() == 1` but never
  that it is the right day. One `assert_eq!` on year and value.
- Optional, marginal: `Year`'s own tests assert `is_err()` only; the message
  content is covered through the filter test and each error has one producer.

## Still open from before

- Go back through every finished day and test the parsing: trailing blank line,
  a line the parser cannot split, and whether the message names the line and
  the day. The 2026-08-12 journal note has the reasoning.
- 2019, all days in one run, when the mood strikes.
- The day twos, now that every day one is done.
