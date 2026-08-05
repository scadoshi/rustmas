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

`src/lib/session/verdict.rs`. `Correct`, `Incorrect`, `TooLow`, `TooHigh`,
`Unsupported`. Built via `From<Ordering>` for numeric comparisons and
`From<bool>` for text. The same type should cover the AOC client later, which
adds a cooldown case.

Too high and too low are real signals from both sources and worth keeping rather
than flattening into a bool.

## check_answer

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

Checking is opt-in, behind `-c`/`--check`, defaulting to off. Two reasons.
Solving offline is the common case and should stay fast, and a run-all with
checking on is roughly 500 requests against a hobby project's free tier. Making
it opt-out would turn that into the accident you have to remember to prevent.

Once verdicts are cached, defaulting to on becomes reasonable, because a
confirmed part would cost nothing to re-check.

## Open

The verifier does not exist yet, only `check_answer`, and nothing calls it.
`solve` prints answers without finding out whether they are right.
`submit_answer` is `todo!()`.

Caching verdicts is not built. It matters for two reasons: a full run-all is
roughly 500 requests against a hobby project's free tier, and AOC's one-shot
answer means the cache is the only durable record of a correct submission.

Parsing AOC's HTML response needs no HTML parser. The verdict strings are stable
enough for substring matching.
