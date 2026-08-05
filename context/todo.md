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
- Decide whether a failed download should abort `init` or log and continue.
- Give `init` the same year and day filters `solve` has.

## Later

- Cache puzzle instructions for full offline use. Two passes, since part two's
  text is hidden until part one is solved. Gitignore it.
- More solutions. Each is one line in the `solutions!` invocation plus a module.
- Revisit the visual-answer case. A part returning `None` while printing from
  inside the solver puts a side effect somewhere awkward to test.
- Note in the README that a fresh clone must run `init` before `solve` compiles,
  since `include_str!` needs `inputs/` and that directory is gitignored.

## Done

- 2015 day 1.
- `check_answer` against the third-party solver, all branches driven live.
- Solver contract recorded and verified against its source.
- `init` downloading inputs with no-clobber caching.
- Solution trait, dispatch macro, clap arguments.
- Validated `Year`, `Day`, and `Part` types.
