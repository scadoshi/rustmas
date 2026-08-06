# Answer verification

`src/lib/session/`

Two different things, deliberately kept apart.

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
fall back to the solver once it is. The local cache is the first source for that
state, with an AOC response able to correct it when they disagree, such as when
a puzzle was solved on the website or from another machine.

## Verdict

`src/lib/session/verdict.rs`. `Correct`, `Incorrect`, `Low`, `High`,
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
levels deep. Where the source genuinely matters is the cache, because a stored
verdict outlives the call that produced it, so that is where it gets recorded.

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

Once verdicts are cached, defaulting to on becomes reasonable, because a
confirmed part would cost nothing to re-check.

## submit_answer

Form-encodes `level` and `answer` to `/<year>/day/<day>/answer`. AOC returns 200
for everything including wrong answers, so the verdict comes entirely from the
body. Strings and match ordering are in [`../references.md`](../references.md),
with fixtures kept as unit tests at the bottom of `src/lib/session/mod.rs`.

## What counts as solved

Two facts come apart, and conflating them is the trap.

Whether the **star exists** is something only AOC knows. Both `Correct` and
`AlreadySolved` prove it. `AlreadySolved` is not a failure, it is AOC confirming
the part is already done.

Whether **this answer is right** is a separate question. `AlreadySolved` does not
answer it, because AOC never looks at the submitted value once a part is
complete. Submit nonsense to a finished part and you still get `AlreadySolved`.

That matters because AOC grades each part exactly once. Unless you were the
first to submit, you can never obtain its sign-off on a specific string again, so
a rule like "re-check until AOC confirms this answer" would loop forever.

The cache therefore records two booleans rather than one notion of official:

- `solved`, from `Correct` or `AlreadySolved`
- `solver_agrees`, from the third-party solver matching the stored answer

Both true is treated as officially solved. That is an inference, not AOC's word:
it means the site says the part is done and an independent implementation
produced the same answer. For it to be wrong, the solver and our code would have
to share a bug producing identical output while some different answer earned the
star. Not worth designing around.

The re-check rule falls out of it:

- `solved && solver_agrees`: done, never touch the network again
- `solved && !solver_agrees`: the star exists but the stored answer is unproven,
  ask the solver once
- `!solved`: submit

## Open

Nothing calls `submit_answer` yet. It needs a flag on `solve` or its own
binary.

The cache is not built. It matters for two reasons: a full run-all is roughly
500 requests against a hobby project's free tier, and AOC's one-shot answer means
the cache is the only durable record of a correct submission. Planned as JSON at the project root, no TTL,
trusted on read. A flat map keyed `"2015/1/1"`, since lookups are always by exact
coordinate. JSON over a binary format because at a few hundred entries the speed
difference is irrelevant, while being able to read, grep, and hand-fix the file
matters: a corrupted cache cannot be re-earned once AOC has spent its one
grading.
