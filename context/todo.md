# Todo

## Planned rework (2026-08-06)

Six changes, decided but not started. Doing them in this order matters: the
first two reshape everything the rest sits on.

### 1. One binary, two modes (done, 2026-08-07)

`fetch` and `solve` are subcommands of one binary. `src/bin/main.rs` parses a
`Cli` and calls `Command::run`, which dispatches to `inbound::fetch::run::run`
or `inbound::solve::run::run`.

The library moved to ports and adapters at the same time: `domain` for puzzle
types that know nothing about IO, `inbound` for the CLI, `outbound` for the two
HTTP clients.

Subcommand variants hold types deriving `clap::Args` rather than `Parser`, and
`#[command(version)]` moved up to `Cli`, since a subcommand does not carry its
own version.

### 2. Inputs at runtime, and solve fetches what it needs (done, 2026-08-07)

Drop `include_str!`. Read inputs from disk when a solution runs, and if the file
is missing, fetch it first as long as a session exists. Solving stops requiring
a prior `fetch`.

This is the big one. Consequences:

- The `include_str!` build-ordering problem disappears completely. A fresh clone
  compiles. That whole caveat leaves the README.
- `Solution::new(&'static str)` loses its `'static`. Every `Puzzle` currently
  holds `input: &'static str` and will need an owned `String` or a lifetime
  parameter. Owned is simpler and the cost is nothing at this size.
- The `solutions!` macro gets simpler. It exists partly to build input paths
  from zero-padded literals at expansion time. At runtime the path is just a
  `format!`, so days no longer need to be written `01`, and the
  `zero_prefixed_literal` allow can go.
- Solving now needs a cookie in the case where the input is missing, which was
  previously never. Offline solving still works when the file is already there.

### 3. Session fingerprinting on inputs

Inputs are tied to an account, so swapping `COOKIE` silently invalidates them.
We hit exactly this: `inputs/2015/01.txt` answered `280` one day and `138` the
next, and only the changed answers gave it away.

Store a hash of the cookie alongside each input. On both fetch and solve,
compare the current cookie's hash against the stored one, and refetch when they
differ. Overwrite rather than keeping both, since there is no reason to hold
another account's input.

Notes:

- Never persist the cookie itself, only the hash. Gitignore the store either
  way.
- Use `sha2`. Decided. `std::hash::DefaultHasher` would mostly work, and a
  mismatch only costs a refetch, but that refetch is a few hundred requests at
  AOC after any toolchain bump, and they ask you not to re-download.
- This changes the no-clobber rule that has held since day one. Existing inputs
  are still cached, but a session mismatch now overwrites. Worth updating
  `design/inputs.md`, which currently says inputs are never overwritten.
- One file per input or a single manifest keyed by coordinate. A manifest is
  fewer files and easier to inspect.

### 4. Fetch instructions alongside inputs

Fetch pulls the puzzle text as well as the input, always, with no flag. There is
no reason to want one without the other.

- The page is per day, not per part. Part two's text is absent from the HTML
  until part one is solved, so a complete cache needs a second pass after
  starring part one.
- Needs HTML parsing. `scraper` or similar, or substring extraction of the
  `<article>` blocks the way `verdict_from` does it today.
- Gitignore it. AOC asks that puzzle text not be republished. No session hash
  needed, since the text is the same for everyone.

### 5. Refetch instructions after starring part one

Part two's text is not in the HTML until part one is solved, so the moment a
submission comes back `Correct` for part one is exactly when that day's
instructions should be pulled again.

That makes the flow within a single `solve --submit` run: solve part one,
validate, submit, and if it earned a star, refetch the day page so part two's
text lands before you go looking for it.

Worth deciding whether a day's cached instructions record which parts they
contain, so a later run knows the file is half complete rather than assuming a
present file is a finished one. Same shape of problem as the session hash: a
file existing does not mean it is the file you want.

### 6. Timing

Report how long a solution took. Already listed below, kept here because it
lands with this work.

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
  cargo run solve -y 2016 -d 1 --submit
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
- `solve` as a free function taking `Option<&SolverClient>`.
- 2015 day 1, verified end to end. `280` and `1797`, both matching the solver.
- `validate_answer` against the third-party solver, all branches driven live.
- Solver contract recorded and verified against its source.
- `fetch` year and day filtering, same shape as `solve`.
- `fetch` downloading inputs with no-clobber caching.
- Solution trait, dispatch macro, clap arguments.
- Validated `Year`, `Day`, and `Part` types.
