# Answer verification

`src/lib/client/`

Two different things, deliberately kept apart, and since 2026-08-06 that
separation is structural: `AocClient` (`aoc.rs`) holds `get_input` and
`submit_answer`, `SolverClient` (`solver.rs`) holds `validate_answer`. They
share only the `User-Agent` builder in `mod.rs`.

Naming them for who they talk to beat naming them for a property. `official.rs`
would have described a judgment rather than a fact, and needed a counterpart
called `unofficial.rs` that said even less.

A single `AocClient` covering both was defensible, reading "AoC" as the puzzle
domain rather than the hostname, and was nearly kept. Splitting won because the
two differ in auth, contract, and failure semantics, and the split makes the
cookie's scope obvious: `--validate` needs no cookie at all, which was not true
while one struct owned both.

## Why two clients

Submitting to adventofcode.com is how you earn stars, and it is authoritative.
It is also not idempotent. Once a part is solved, resubmitting does not return
"correct" again, it returns "You don't seem to be solving the right level". The
site answers each part exactly once, so it cannot serve as a repeatable
regression check. Wrong answers also trigger a cooldown that escalates from one
minute to five.

The third-party solver answers the same question as many times as you ask, which
is what a regression check needs. It knows nothing about your account and cannot
award stars.

The plan is to route on solved state: submit to AOC when a part is unsolved, and
fall back to the solver once it is. AOC itself holds that state, and reports it
as `AlreadySolved`, so nothing needs to be tracked locally.

## Verdict

`src/lib/client/verdict.rs`. `Correct`, `Incorrect`, `Low`, `High`,
`Unsupported`, `Cooldown(String)`, `AlreadySolved`. Built via `From<Ordering>`
for numeric comparisons and `From<bool>` for text.

Direction is worth keeping rather than flattening into a bool, and both sources
produce it.

One type serves both clients even though neither can produce every variant.
`Unsupported` only ever comes from the solver; `Cooldown` and `AlreadySolved`
only from AOC. The cost is that exhaustive matches carry arms that cannot happen
for the call that was made. Accepted at this size. If it starts to hurt, the fix
is two enums sharing a core, or one enum plus a source tag.

Tagging the source inside `Verdict`, as in nested `Official` and `Unofficial`
variants, was considered and rejected. The caller already knows which client it
called, so the tag restates known information while pushing every match two
levels deep. It would matter for a stored verdict, which outlives the call that
produced it, but nothing is stored (see the rejected cache below).

`Cooldown` holds a string because AOC phrases the remaining wait as prose
(`1m 0s`). It reports and moves on rather than sleeping: the wait escalates well
past a minute, a CLI that silently blocks is indistinguishable from one that
hung, and retrying an answer already known to be wrong just earns a longer wait.

## validate_answer

Posts the input to `/solve/<year>/<day>/<part>` and compares. Numeric answers
compare as numbers so a mismatch can report a direction, anything else compares
as text.

Classification has to read the body, not the status. See
[`../references.md`](../references.md) for the contract and how it was verified.
`error_for_status()` is not used here because it consumes the body that carries
the meaning.

Only transport failures and 5xx fall through to the next host. Every 4xx is
deterministic and all three hosts run the same solver, so retrying a rejected
request just wastes round trips. 5xx comes from the hosting platforms rather
than the solver itself.

`Unsupported` is currently unreachable, because the solver's coverage ends
exactly where `days_in_year` does. It becomes reachable during a live event,
when a day is published and solved locally before the solver catches up.

## Wiring it into solve

Validation is opt-in, behind `-v`/`--validate`, defaulting to off. Two reasons.
Solving offline is the common case and should stay fast, and a run-all with
validation on is roughly 500 requests against a hobby project's free tier.
Making it opt-out would turn that into the accident you have to remember to
prevent.

The word is `validate` rather than `check` because `cargo check` already means
"compile without producing a binary", so `--check` could plausibly read as a
build-only flag. `-V` is taken by `--version`, which leaves `-v` free for this,
one shift key apart and doing something else entirely.

Every run pays for what it validates, since nothing is cached. That is the
point: the solver is idempotent, so re-running is a regression check rather
than waste.

## submit_answer

Form-encodes `level` and `answer` to `/<year>/day/<day>/answer`. AOC returns 200
for everything including wrong answers, so the verdict comes entirely from the
body. Strings and match ordering are in [`../references.md`](../references.md),
with fixtures kept as unit tests at the bottom of `src/lib/client/aoc.rs`.

## What counts as solved

Two facts come apart, and conflating them is the trap.

Whether the **star exists** is something only AOC knows. Both `Correct` and
`AlreadySolved` prove it. `AlreadySolved` reads like a rejection but it is AOC
confirming the part is already done.

Whether **this answer is right** is a separate question. `AlreadySolved` does not
answer it, because AOC never looks at the submitted value once a part is
complete. Submit nonsense to a finished part and you still get `AlreadySolved`.

That matters because AOC grades each part exactly once. Unless you were the
first to submit, you can never obtain its sign-off on a specific string again, so
a rule like "re-check until AOC confirms this answer" would loop forever.

## Rejected: a local answer cache

A JSON cache of confirmed answers was designed in some detail and then dropped
without being built.

The argument for it was that AOC grades each part exactly once, so a cache
looked like the only durable record of a correct submission. That was wrong.
AOC is itself stateful, and `AlreadySolved` is the durable record. The site
remembers your stars whether or not anything is written down, so the fact called
irreplaceable was always one request away.

What remained was an optimisation worth very little. Validation is opt-in and
usually aimed at the single day being worked on, so the saving is one request
per part. Against that sat a file format, read and write paths, key parsing,
staleness rules, and an invalidation problem: answers are tied to one account's
input, so changing `COOKIE` silently invalidates every entry.

One detail is worth keeping if it ever comes back. An entry would have to store
the answer, not just the coordinate. The solver's value is that it is
repeatable, which makes it a regression check for refactors. A cache keyed only
on year, day, and part would report a refactored solution as still validated,
turning a permanent check into a one-time one, exactly when it matters most.
Storing the answer makes the entry conditional: the cached verdict applies only
while the computed answer still matches.

## Submitting from solve

`--submit` on the `solve` binary. It forces validation on regardless of
`--validate`, because the solver verdict is the gate: a wrong answer to AOC costs
an escalating cooldown, and the solver check is free protection against firing
one.

- solver says `Correct`: submit
- solver says `Incorrect`, `High`, or `Low`: skip, report the direction
- solver says `Unsupported`: submit anyway, noting it went unchecked

That last case matters. `Unsupported` means the solver has no implementation,
which happens during a live event when a day is solved before the solver catches
up, and that is exactly when submitting is worth doing. Gating on it would block
the one case the feature exists for.

An unfiltered `--submit` walks every year and day, so it prompts first with the
count, and `--yes` skips the prompt. No short flag for `--yes`: `-y` is taken by
`--year`, and a guard against hundreds of writes is worth typing out. The prompt
writes to stderr and treats closed stdin as no, so redirecting output does not
swallow the question and a script does not hang or accidentally proceed.

The two clients are built independently. Validating alone never needs a cookie,
since only `AocClient` has one.

Submission happens before printing, so each part is one line carrying what both
checkers said. `Answer::Value` holds two optional verdicts, `verdict` from the
solver and `submission` from AOC, and `Display` merges them into one set of
parentheses.

AOC's word supersedes the solver's when both exist, since repeating that the
solver agreed adds nothing once the star is confirmed. `Correct` from a
submission reads as `new star` and `AlreadySolved` as `starred`, which keeps the
distinction that matters in the moment while collapsing both to the same fact:
you have it.

An answer the solver rejected comes back from `submit` untouched, so it prints
the solver's objection and says nothing about a submission that never
happened.

## Open

Nothing tracks which parts are already solved, so a submit run re-asks AOC and
gets `AlreadySolved` back. Harmless, and the rejected cache above is why.


