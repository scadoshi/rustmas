# Answer verification

`src/lib/outbound/client/`

Two different things, kept apart on purpose. Since 2026-08-06 the separation is structural:

- `AocClient` (`aoc_client.rs`) holds `get_input` and `submit_answer`.
- `SolverClient` (`solver_client.rs`) holds `validate_answer`.
- They share only the `User-Agent`, which both build from `Environment::user_agent` in `environment.rs`.

They're named for who they talk to, not for a property. `official.rs` would have described a judgment rather than a fact, and it needed a counterpart called `unofficial.rs` that said even less.

A single `AocClient` covering both was defensible, reading "AoC" as the puzzle domain rather than the hostname, and it was nearly kept. Splitting won because the two differ in auth, contract, and failure semantics.

The split also makes the cookie's scope obvious. `--validate` needs no cookie at all, which wasn't true while one struct owned both.

## Why two clients

Submitting to adventofcode.com is how you earn stars, and it's authoritative. It's also not idempotent:

- Once a part is solved, resubmitting doesn't return "correct" again. It returns "You don't seem to be solving the right level".
- The site answers each part exactly once, so it can't serve as a repeatable regression check.
- Wrong answers trigger a cooldown that escalates from one minute to five.

The third-party solver answers the same question as many times as you ask, which is what a regression check needs. It knows nothing about your account and can't award stars.

The plan is to route on solved state: submit to AOC while a part is unsolved, and fall back to the solver once it is. AOC itself holds that state and reports it as `AlreadySolved`, so nothing needs tracking locally.

## Verdicts

`src/lib/domain/solution/solver_verdict.rs` and `aoc_verdict.rs` hold two types, one per source.

- `SolverVerdict` is `Correct`, `Incorrect`, `Low`, `High`, or `Unsupported`. It's built with `From<Ordering>` for numeric comparisons and `From<bool>` for text.
- `AocVerdict` is `Correct`, `Incorrect`, `Low`, `High`, `Cooldown(String)`, `AlreadySolved`, or `NotLoggedIn`.

Direction is worth keeping rather than flattening into a bool, and both sources produce it.

One shared type came first, and it was wrong. Neither client could produce every variant: `Unsupported` only ever comes from the solver, and `Cooldown` and `AlreadySolved` only from AOC.

Every exhaustive match carried arms that couldn't happen for the call that was made. Splitting the type deleted those arms instead of documenting them.

Before the split, tagging the source inside one enum (nested `Official` and `Unofficial` variants) was considered and rejected. The caller already knows which client it called, so the tag restates known information and pushes every match two levels deep. Two types get the same result with no tag.

The verdicts live in the domain, not beside the clients that parse them.

- Neither mentions HTTP or carries a status code.
- `Outcome` matches on both to render a line, so leaving them in the adapter would make a domain type depend on it.
- They sat in `outbound/client/` for a long time only because that's where they were first written.

The clients map their replies onto them.

`Cooldown` holds a string because AOC phrases the remaining wait as prose (`1m 0s`). The tool reports it and moves on instead of sleeping:

- The wait escalates well past a minute.
- A CLI that silently blocks looks exactly like one that hung.
- Retrying an answer already known to be wrong just earns a longer wait.

## validate_answer

`validate_answer` posts the input to `/solve/<year>/<day>/<part>` and compares. Numeric answers compare as numbers so a mismatch can report a direction. Anything else compares as text.

Classification has to read the body, not the status. [`../references.md`](../references.md) has the contract and how it was verified. `error_for_status()` isn't used here because it consumes the body that carries the meaning.

Only transport failures and 5xx fall through to the next host. Every 4xx is deterministic and all three hosts run the same solver, so retrying a rejected request wastes round trips. A 5xx comes from the hosting platform, not the solver.

`Unsupported` is currently unreachable, because the solver's coverage ends exactly where `Year::days_in` does. It becomes reachable during a live event, when a day is published and solved locally before the solver catches up.

## Wiring it into solve

Validation is opt-in, behind `-v`/`--validate`, and off by default. Two reasons:

- Solving offline is the common case and should stay fast.
- A run-all with validation on is roughly 500 requests against a hobby project's free tier. Opt-out would make that the accident you have to remember to prevent.

The word is `validate`, not `check`, because `cargo check` already means "compile without producing a binary". `--check` could plausibly read as a build-only flag. `-V` is taken by `--version`, which leaves `-v` free for this, one shift key apart and doing something else entirely.

Every run pays for what it validates, since nothing is cached. That's the point: the solver is idempotent, so re-running is a regression check, not waste.

## submit_answer

`submit_answer` form-encodes `level` and `answer` to `/<year>/day/<day>/answer`. AOC returns 200 for everything, wrong answers included, so the verdict comes entirely from the body.

The strings and match order are in [`../references.md`](../references.md). Fixtures are unit tests at the bottom of `src/lib/outbound/client/aoc_client.rs`.

## What counts as solved

Two facts come apart here, and conflating them is the trap.

Whether the star exists is something only AOC knows. Both `Correct` and `AlreadySolved` prove it. `AlreadySolved` reads like a rejection, but it's AOC confirming the part is already done.

Whether this answer is right is a separate question, and `AlreadySolved` doesn't answer it. AOC never looks at the submitted value once a part is complete. Submit nonsense to a finished part and you still get `AlreadySolved`.

That matters because AOC grades each part exactly once. Unless you were the first to submit, you can never get its sign-off on a specific string again. A rule like "re-check until AOC confirms this answer" would loop forever.

## Rejected: a local answer cache

A JSON cache of confirmed answers was designed in some detail, then dropped without being built.

The argument for it: AOC grades each part exactly once, so a cache looked like the only durable record of a correct submission. That was wrong.

AOC is itself stateful, and `AlreadySolved` is the durable record. The site remembers your stars whether or not anything is written down, so the fact called irreplaceable was always one request away.

What remained was an optimization worth very little. Validation is opt-in and usually aimed at the one day being worked on, so the saving is one request per part. Against that sat:

- a file format
- read and write paths
- key parsing
- staleness rules
- an invalidation problem, since answers are tied to one account's input and changing `COOKIE` silently invalidates every entry

One detail is worth keeping if the cache ever comes back: an entry has to store the answer, not just the coordinate.

1. The solver's value is that it's repeatable, which makes it a regression check for refactors.
2. A cache keyed only on year, day, and part would report a refactored solution as still validated. That turns a permanent check into a one-time one, exactly when it matters most.
3. Storing the answer makes the entry conditional: the cached verdict applies only while the computed answer still matches.

## Submitting from solve

`--submit` on the `solve` subcommand forces validation on, whatever `--validate` says. The solver verdict is the gate. A wrong answer to AOC costs an escalating cooldown, and the solver check is free protection against firing one.

- Solver says `Correct`: submit.
- Solver says `Incorrect`, `High`, or `Low`: skip, and report the direction.
- Solver says `Unsupported`: submit anyway, noting it went unchecked.

That last case matters. `Unsupported` means the solver has no implementation.

That happens during a live event, when a day is solved before the solver catches up, which is exactly when submitting is worth doing. Gating on it would block the one case the feature exists for.

An unfiltered `--submit` walks every year and day, so it prompts first with the count. `--yes` skips the prompt.

- There's no short flag for `--yes`. `-y` is taken by `--year`, and a guard against hundreds of writes is worth typing out.
- The prompt writes to stderr, so redirecting output doesn't swallow the question.
- Closed stdin counts as no, so a script doesn't hang or accidentally proceed.

The two clients are built independently. Validating alone never needs a cookie, since only `AocClient` has one.

Intent travels as flags, not as the presence of a client.

- `solve` takes `validate: bool` alongside a `&SolverClient` that's always built. That client needs no cookie and fails to build only when the `User-Agent` can't be a header value.
- `AocClient` sits behind a `LazyAocClient`, because it genuinely can be absent. Nothing reads intent from it.
- Submission is gated on `--submit`.
- `ensure_entry` connects the `LazyAocClient` only when a download is needed, so a connected client doesn't imply anything was asked for.

Submission happens before printing, so each part is one line carrying what both checkers said. `Outcome` holds two optional verdicts, `solver_verdict` and `aoc_verdict`, and its `Display` merges them into one set of parentheses.

AOC's word supersedes the solver's when both exist. Repeating that the solver agreed adds nothing once the star is confirmed.

- `Correct` from a submission reads as `new star`.
- `AlreadySolved` reads as `starred`.

That keeps the distinction that matters in the moment while collapsing both to the same fact: you have it.

An answer the solver rejected comes back from `submit` untouched. It prints the solver's objection and says nothing about a submission that never happened.

## Open

Nothing tracks which parts are already solved, so a submit run re-asks AOC and gets `AlreadySolved` back. Harmless, and the rejected cache above is why.
