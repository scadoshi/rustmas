# Todo

## Next

Wire `validate_answer` into `solve`. Nothing calls it today, so `solve` prints
answers without ever finding out whether they are right. Decided design, not yet
written:

**Mid-migration as of 2026-08-05. The `solve` binary does not compile.** The
library does. Pick this up before anything else.

What moved: `solve` went from a free function in `src/bin/solve/utils.rs` to
`Session::solve<S: Solution>`, because validating needs the session and the
input together. `Solution` gained an `input()` method so the session can reach
the input while checking. `Answer` (a value plus an optional `Verdict`) is new,
in `src/lib/session/answer.rs`.

To make it build again:

- `main.rs` still imports `solve` and `Answer` from `crate::utils`. Both are
  gone from there. `Answer` now lives at `rustmas::session::answer::Answer`.
- `utils.rs` still holds a duplicate `Answer` definition. Delete it, the real
  one is in the library.
- The macro still calls `solve::<$t>(input)` with the old one-argument shape.
  `Session::solve` now wants `(input, day, validate)` and is a method, so
  `dispatch` needs a `&Session` and the validate flag threaded in from `main`.
- Building the `Session` in `main` has to stay lazy, or `solve` starts requiring
  a cookie even when not validating.
- `Answer` has no accessors and `main` prints with `{:?}`. Fine for now, worth
  a `Display` later.

**Known bug, not yet fixed.** `Session::solve` takes one `part` and passes it to
both validation calls, so part two gets validated against whatever part was
handed in. The parameter should not exist: `solve` runs both parts, so it can
use `Part::One` and `Part::Two` internally and drop `part` from the signature.

The `-v`/`--validate` flag is declared on `Args` but does nothing yet. What is
left:

- `validate_answer` needs the puzzle input, but `dispatch` returns only the
  answers. Either dispatch hands the input back too, or the check happens inside
  the generated match arm where `include_str!` is already in scope. The second
  is less plumbing.
- Build the `Session` lazily, only when the flag is set, or `solve` starts
  failing without a `.env` even when nothing is being validated. Worth noting
  that `validate_answer` never touches the cookie, only `get_input` and
  `submit_answer` do, so pulling the client off `Session` is another way out.
- Skip the call when a part returns `None`. Nothing to validate.

Then:

- `submit_answer` is `todo!()`. Needs the POST to `/answer` with `level`, plus
  parsing AOC's HTML reply into a `Verdict`. Substring matching is enough, no
  HTML parser needed. Capture the cooldown message rather than discarding it.
- Cache verdicts to disk. Required rather than optional, because AOC confirms a
  correct answer exactly once and the cache becomes the only durable record.
  Also keeps a run-all from firing hundreds of requests at a hobby project.
- Route between the two clients on solved state: AOC when unsolved, solver when
  already solved.

## Soon

- Timing per part, split from parse time. Belongs in `solve()` in
  `src/bin/solve/utils.rs`.
- Distinguish an explicitly requested day with no solution from one skipped
  during a run-all. `dispatch` already returns `None` for both and `main`
  ignores the difference.
- Decide whether a failed download should abort `init` or log and continue.
- Give `init` the same `-y`/`--year` and `-d`/`--day` filters `solve` has, so a
  single puzzle can be downloaded without walking every year. Same treatment:
  clap derive with `Option<u32>` on both, filters rather than a lookup, and the
  two `is_some_and` guards inside the existing loops. Worth doing when a live
  event starts and only the newest day is missing, and it would have made
  recovering the deleted `inputs/` a one-day fetch instead of all 262.

## Later

- Cache puzzle instructions for full offline use. Two passes, since part two's
  text is hidden until part one is solved. Gitignore it.
- More solutions. Each is one line in the `solutions!` invocation plus a module.
- Revisit the visual-answer case. A part returning `None` while printing from
  inside the solver puts a side effect somewhere awkward to test.
- Note in the README that a fresh clone must run `init` before `solve` compiles,
  since `include_str!` needs `inputs/` and that directory is gitignored.

## Done

- 2015 day 1, verified end to end. `280` and `1797`, both matching the solver.
- `validate_answer` against the third-party solver, all branches driven live.
- Solver contract recorded and verified against its source.
- `init` downloading inputs with no-clobber caching.
- Solution trait, dispatch macro, clap arguments.
- Validated `Year`, `Day`, and `Part` types.
