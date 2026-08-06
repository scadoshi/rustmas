# Todo

## Next

- Nothing calls `submit_answer` yet. It needs a flag on `solve`, or its own
  binary.
- Route between the two clients on solved state: AOC when unsolved, solver when
  already solved.

## Soon

- Timing per part, split from parse time. Belongs in `solve()` in
  `src/bin/solve/utils.rs`.
- Distinguish an explicitly requested day with no solution from one skipped
  during a run-all. `dispatch` already returns `None` for both and `main`
  ignores the difference.
- Decide whether a failed download should abort `fetch` or log and continue.

## Later

- Cache puzzle instructions for full offline use. Two passes, since part two's
  text is hidden until part one is solved. Gitignore it.
- More solutions. Each is one line in the `solutions!` invocation plus a module.
- Revisit the visual-answer case. A part returning `None` while printing from
  inside the solver puts a side effect somewhere awkward to test.

## Done

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
