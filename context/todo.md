# Todo

## Next

- **Write 2016 day 1.** `src/lib/solutions/year_2016/day_01/mod.rs` is a stub
  returning `Answer::None` from both parts. Its input is already fetched and it
  is already registered in the `solutions!` macro, so it only needs the two
  parts filled in.

  It exists to drive the one output branch never seen live: `new star`, the
  `Display` for a submission that AOC graded `Correct` rather than
  `AlreadySolved`. Both parts of 2015 day 1 are already solved on the scratch
  account, so nothing there can produce it. Once written:

  ```
  cargo run --bin solve -- -y 2016 -d 1 --submit
  ```

  should print `(new star)` for each part. The puzzle is "No Time for a Taxicab":
  follow `R2, L3` style turns on a grid, part one is the Manhattan distance to
  the end, part two is the first location visited twice.

- More solutions. The pipeline is complete now: fetch, solve, validate, submit.
  Each new day is one line in the `solutions!` invocation plus a module.
- Timing per part, split from parse time. Belongs in `solve()` in
  `src/lib/solutions/solution.rs`, where `new` and the two parts are called.

## Soon
- Distinguish an explicitly requested day with no solution from one skipped
  during a run-all. `dispatch` already returns `None` for both and `main`
  ignores the difference.
- Decide whether a failed download should abort `fetch` or log and continue.

## Later

- Cache puzzle instructions for full offline use. Two passes, since part two's
  text is hidden until part one is solved. Gitignore it.
- Revisit the visual-answer case. A part returning `None` while printing from
  inside the solver puts a side effect somewhere awkward to test.

## Done

- `--submit` on `solve`, gated on a solver verdict, with a confirmation prompt
  for unfiltered runs and `--yes` to skip it.
- Client split into `AocClient` and `SolverClient`.
- `submit_answer` with the AOC reply parser, all verdicts driven live against a
  scratch account, fixtures kept as unit tests.
- `--validate` wired end to end. `-y 2015 -d 1 --validate` reports
  `280 (Correct)` and `1797 (Correct)`.
- `Answer` enum with the verdict folded into the submittable variant.
- `solve` as a free function taking `Option<&Session>`.
- 2015 day 1, verified end to end. `280` and `1797`, both matching the solver.
- `validate_answer` against the third-party solver, all branches driven live.
- Solver contract recorded and verified against its source.
- `fetch` year and day filtering, same shape as `solve`.
- `fetch` downloading inputs with no-clobber caching.
- Solution trait, dispatch macro, clap arguments.
- Validated `Year`, `Day`, and `Part` types.
