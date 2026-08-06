# Todo

## Next

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
- Decide whether a failed download should abort `fetch` or log and continue.
- Give `fetch` the same `-y`/`--year` and `-d`/`--day` filters `solve` has, so a
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
- Note in the README that a fresh clone must run `fetch` before `solve` compiles,
  since `include_str!` needs `inputs/` and that directory is gitignored.

## Done

- `--validate` wired end to end. `-y 2015 -d 1 --validate` reports
  `280 (Correct)` and `1797 (Correct)`.
- `Answer` enum with the verdict folded into the submittable variant.
- `solve` as a free function taking `Option<&Session>`.
- 2015 day 1, verified end to end. `280` and `1797`, both matching the solver.
- `validate_answer` against the third-party solver, all branches driven live.
- Solver contract recorded and verified against its source.
- `fetch` downloading inputs with no-clobber caching.
- Solution trait, dispatch macro, clap arguments.
- Validated `Year`, `Day`, and `Part` types.
