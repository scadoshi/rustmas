# Todo

## Where this is

Feature complete and pushed. `fetch`, `solve`, `--validate`, and `--submit` all
work, 28 tests pass, and both service contracts are recorded in
`references.md` from live probing rather than guesswork.

The six-item rework planned on 2026-08-06 is done except for one piece, below.
What each change was and why is in `design/` and `progress/journal.md`.

The domain now imports nothing outside itself, as of 2026-08-10. Both verdicts
live in `domain/solution/`, and `solve()` moved to `outbound/client/solve.rs`.

## Next

- **Decide where `solve()` belongs.** It is in `outbound/client/solve.rs`, which
  works and is documented, but `outbound` means "how a request leaves this
  program" and `solve()` does not leave anywhere. It is orchestration that
  drives a client, and `inbound/solve/` already holds its only callers plus
  `args.rs` and `utils.rs`.

  Cost of leaving it: `outbound/mod.rs` and `outbound/client/mod.rs` both had to
  be widened to describe a runner sitting among the clients. Moving it lets both
  read as they did.

  Small change, no behaviour, just the module docs and one import in `run.rs`.

- **Refetch instructions after starring part one.** The last piece of the
  rework. Part two's text is absent from the day page until part one is solved,
  so a day fetched early keeps a half-complete cache forever. The moment a
  submission comes back `Correct` for part one is exactly when to re-pull the
  page and write `part_two.md`.

  Worth deciding whether an entry should record which parts it holds. Right now
  `part_two.md` existing is the answer, which is structural and hard to get
  wrong, so probably leave it.

- **Write 2016 day 1.** `src/lib/domain/solutions/year_2016/day_01/mod.rs` is a
  stub returning `Answer::None` from both parts.

  Its input and instructions are already cached and it is already in the
  dispatch match, so it only needs the two parts filled in.

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

- More solutions. Each is one match arm in `inbound/solve/run.rs` plus a module.

## Soon
- Decide whether a failed download should abort `fetch` or log and continue.

## Later

- No day returns `Answer::Visual` yet. Its rendering is tested, but nothing
  produces one in a real run.
- Nothing displays cached instructions. They are stored and unread.
- `Solution::input()` has no callers, since `solve` holds the input already.

## Done

- Tests: 28 covering the domain bounds, `Day::each` filters, the `Outcome`
  display matrix, its no-verdict-on-unsubmittable invariant, the store round
  trip, and session hashing. Two were mutation checked to confirm they fail when
  the thing they describe breaks.
- `solver_for` as the one registry, so unwritten days are skipped before
  downloading, `--submit` counts what it will send, and an explicitly requested
  day with no solution says so.
- Timing per part and for parsing, measured before validation so no network
  time leaks in.
- Single binary with `fetch` and `solve` subcommands, library split into
  domain, inbound, and outbound.
- Runtime input reading, so a fresh clone compiles with `cache/` empty and
  `solve` fetches what it needs.
- Session fingerprinting: inputs carry a SHA-256 of the cookie that fetched
  them, and a mismatch refetches the input while keeping the instructions.
- Instructions cached as markdown per part, split on `<article class="day-desc">`
  and rendered with `html2text`.
- `--submit` on `solve`, gated on a solver verdict, with a confirmation prompt
  for unfiltered runs and `--yes` to skip it.
- Client split into `AocClient` and `SolverClient`.
- `submit_answer` with the AOC reply parser, all verdicts driven live against a
  scratch account, fixtures kept as unit tests.
- `--validate` wired end to end, passing an explicit flag rather than reading
  intent from an optional client.
- `Answer` split from `Outcome`, one type per provenance, and `Verdict` split
  into `SolverVerdict` and `AocVerdict` so no match carries impossible arms.
- 2015 day 1, verified end to end against the solver.
- `validate_answer` against the third-party solver, all branches driven live.
- Solver contract recorded and verified against its source.
- `fetch` year and day filtering, same shape as `solve`.
- `fetch` downloading inputs with no-clobber caching.
- Solution trait, dispatch macro, clap arguments.
- Validated `Year`, `Day`, and `Part` types.
