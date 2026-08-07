# Journal

Newest first. Names in older entries were updated when things got renamed, so
they read consistently rather than historically.

## 2026-08-06 (end of day)

Planned a rework, wrote none of it. Five changes in
[`../todo.md`](../todo.md), and two of them undo decisions made earlier this
week for reasons that no longer hold.

Splitting `fetch` and `solve` into separate binaries was organisational, and
nothing here is deployed separately, so the split only bought `--bin` on every
command. They collapse into subcommands.

Dropping `include_str!` is the larger reversal. Compile-time embedding was
chosen deliberately and the build-ordering cost was accepted, but the goal now
is for `solve` to fetch a missing input itself, and that cannot happen at
compile time. Runtime reading removes the caveat entirely and simplifies the
dispatch macro on the way out.

Session fingerprinting comes from a real incident rather than a hypothetical:
`inputs/2015/01.txt` gave `280` one day and `138` the next after the cookie
changed, and nothing caught it except the answers moving. Hashing the cookie
next to each input makes that detectable, and it changes the no-clobber rule
that has held since the first session, since a session mismatch should
overwrite.

## 2026-08-06 (later)

Wired `--submit` into `solve`, which completes the pipeline: fetch, solve,
validate, submit. Submitting forces validation on and gates on the solver
verdict, since a wrong answer to AOC costs an escalating cooldown and the solver
check is free. `Unsupported` is deliberately let through rather than blocked,
because that is the live-event case where a day is solved before the solver
catches up, and it is exactly when submitting matters.

Drove all of it. An already-solved day validates `Correct` then reports
`already solved` from AOC. Temporarily breaking day 1 to return `999999999`
produced `High` from the solver and the submission was skipped, so a wrong
answer never reaches AOC. The unfiltered prompt declines on `n` and on closed
stdin, and neither path made a request.

`--yes` has no short flag, since `-y` is `--year` and a guard against 524 writes
is worth typing out. The prompt states the count, writes to stderr, and treats
EOF as no.

Moved `submit` and `confirm` into `src/bin/solve/utils.rs`, leaving `main.rs` as
the macro, `run`, and `main`.

Then tidied the output, which had drifted. It printed two lines per part, one
for solving and one for submitting, and rendered verdicts with `{:?}` rather
than the `Display` impl written for them. `Answer::Value` now carries both
verdicts and merges them, `submit` returns the answer rather than printing, and
`run` submits before printing so a part is always one line. Also `year 2015 day
1` rather than `2015 day 1`.

    year 2015 day 1
      part one: 138 (starred)
      part two: 1771 (starred)

A rejected answer shows the solver's objection and nothing else, since no
submission happened: `999999999 (high)`.

Added a 2016 day 1 stub and hit the first two-year collision. Both days would
have been `Day01`, so every day's type is now `Puzzle` and the module path
carries the coordinate. `Solver` was considered first and dropped, since
`SolverClient` already means the third-party service here. Importing the year
modules rather than the types is what avoids aliases.

The stub returns `Answer::None` from both parts and is left for Scotty to
write. It exists to drive `new star`, the one output branch never seen live,
since both parts of 2015 day 1 are already solved on the scratch account. See
[`../todo.md`](../todo.md).


Split `Session` into `AocClient` and `SolverClient` under `src/lib/client/`,
files named for who they talk to. `official.rs` was considered and rejected: it
names a judgment rather than a fact, and would need an `unofficial.rs`
counterpart saying even less.

Keeping one struct was defensible, since "AoC" reads as the puzzle domain rather
than the hostname, and it nearly stayed. What settled it was scope: the two
differ in auth, contract, and failure semantics, and splitting made the cookie's
reach obvious. `--validate` now needs no cookie at all, which was not true while
one struct owned both.

They share only the `User-Agent` builder, which moved to `client/mod.rs`. If
they ever need to share a connection pool, `reqwest::Client` is reference
counted internally, so cloning is enough and no wrapper struct is needed.

`solve` in the library now takes `Option<&SolverClient>`, which reads better
than before: the thing it optionally needs is a checker, not a session.


Designed a local answer cache in detail, then dropped it before writing it.
The case for it rested on AOC grading each part exactly once, which made a cache
look like the only durable record of a correct answer. That was wrong: AOC is
stateful and `AlreadySolved` is the record. The supposedly irreplaceable fact
was one request away.

What was left was worth very little against a file format, key parsing,
staleness rules, and an invalidation problem, since answers are tied to one
account's input and changing `COOKIE` invalidates everything. Reasoning kept in
[`../design/verification.md`](../design/verification.md), including the one
detail worth remembering: an entry would need to store the answer, not just the
coordinate, or a refactor would still read as validated and the regression check
would quietly become a one-time check.

Removed `src/lib/cache/`, and `serde` and `serde_json` with it, since nothing
else used them.

Moved the `User-Agent` into configuration. `CONTACT` and `REPO_URL` are optional
env vars, and there is deliberately no default naming this repo's author: a fork
that left them unset would otherwise report Scotty as the contact for a
stranger's traffic. Unset falls back to naming the tool alone. Added
`.env.template` so the shape is visible without a `.env`.


Built `submit_answer` and probed AOC's real replies using a scratch account
Scotty set up, submitting to 2015 day 1 deliberately wrong before deliberately
right, since AOC grades each part only once.

Every reply is HTTP 200, wrong answers included, so the verdict is entirely in
the body. Same shape as the solver client but for the opposite reason: that one
returns 400 for everything. Full table in
[`../references.md`](../references.md), with fixtures as unit tests.

Two things the probing settled that guessing would not have. A directional reply
contains the generic wrong-answer phrase as a prefix, so direction has to be
matched first or every miss reads as generic. And the direction hint is
optional: guessing 1 against 138 gave no hint at all, while 999999999 gave "too
high". `too low` was never triggered and stays inferred.

Added `Verdict::Cooldown(String)` and `Verdict::AlreadySolved`. Cooldown reports
and moves on rather than sleeping, since the wait escalates past a minute and a
CLI that silently blocks looks hung. It holds a string because AOC phrases the
remaining time as prose.

`AlreadySolved` is the cache-correction signal: it means the site knows a part is
done when local state did not.


Gave `fetch` the same `-y`/`--year` and `-d`/`--day` filters `solve` has, so a
single puzzle can be pulled without walking every year. Verified live: `-y 2015
-d 1` made one request, a re-run skipped it as cached, and `-d 25` fetched ten
files rather than eleven, correctly passing over 2025 because that event only
ran twelve days.

Renamed the `init` binary to `fetch`. `init` implied one-time setup, but with
year and day filters coming it becomes something you run repeatedly for a single
puzzle. `sync` was the other candidate, since it matches the gap-filling
caching, but `fetch` says what it does.

Renamed `src/lib/utils.rs` to `calendar.rs`, which holds `FIRST_YEAR` and
`latest_year()`. Both binaries keep their own local `utils.rs` for CLI bits.

Trimmed doc comments across the repo. The `Solution` trait and `validate_answer`
were the worst, both roughly a third of their old length now. Accessors that
restated their own signature lost their docs entirely. Documented the consts
that carry non-obvious meaning, and `Part` and `Verdict`, which had none.

## 2026-08-06

Finished the migration left half-done yesterday. `--validate` works end to end:
`-y 2015 -d 1 --validate` gives `280 (Correct)` and `1797 (Correct)`, and
without the flag it solves offline and never builds a `Session`.

`Output` became `Answer`, with the verdict folded into the submittable variant
so a visual answer cannot carry one. A day writes `Answer::solved(value)` and
the runner attaches a verdict afterwards. That also let visual answers be
returned rather than printed from inside the solver, which closes the
side-effect problem that had been open since the trait was designed.

Moved `solve` off `Session` and made it a free function taking
`Option<&Session>`. The session was doing two jobs, HTTP adapter and
orchestration, and it never needed its own cookie or client to run a solution.
Passing `Option<&Session>` also deleted the `validate` bool, since "no session"
and "do not validate" are the same thing, and it made the lazy-construction
question answer itself.

Deleted the duplicate `Answer` in `src/bin/solve/utils.rs` and the stale
`src/lib/solutions/answer.rs`. Gave `Answer` a `Display` impl so `main` prints
readable output instead of `{:?}`.

## 2026-08-05

Short session, stopped mid-change. The library compiles, the `solve` binary does
not. See [`../todo.md`](../todo.md) for exactly what to fix.

Started wiring validation into solving and hit the real decision straight away:
does a `Session` know how to run a `Solution`, or does a `Solution` know how to
talk to a `Session`? Went with the former. `Session::solve<S: Solution>` builds
the solution, runs both parts, and validates each answer when asked. `Solution`
gained an `input()` method so the session can reach the input it needs to post.

Added `Answer`, a value plus an optional `Verdict`, so a part can carry its
result and what the solver thought of it. It sits in
`src/lib/solutions/answer.rs`. A duplicate definition is still sitting in
`src/bin/solve/utils.rs` and should go.

Left one known bug in place rather than fixing it blind, since it needs a
signature change: `Session::solve` uses a single `part` argument for both
validation calls, so part two is validated against the wrong part.

One mechanical fix went in to get the library compiling: `validate_answer` takes
`answer: impl AsRef<str>` now, so it needed an `as_ref()` before parsing and
comparing.

## 2026-08-04

Wrote 2015 day 1. Part one folds over the characters, part two returns early at
the first index where the floor hits -1.

Decided how answer checking works, and why it needs two clients rather than one.
AOC gives stars but answers each part only once, so it cannot be a repeatable
check. The third-party solver is idempotent but cannot award anything. See
[`../design/verification.md`](../design/verification.md).

Brought `Part` back, as a plain enum this time rather than the struct that was
deleted the day before, since submitting needs to name a part.

Built `validate_answer` against the solver. First attempt keyed `Unsupported` off a
404, which was wrong: probing showed the API returns 400 for every failure and
puts the reason in the body. Rewrote it to read the body before classifying,
which also meant dropping `error_for_status()`, since that consumes the body.
Retries now only happen for transport failures and 5xx.

Drove every branch against the live API. `Correct`, `TooLow`, `TooHigh`,
`Incorrect`, and the 4xx rejection path all behave. `Unsupported` could not be
reached, which led to cloning the solver's source and confirming why: its
coverage stops at exactly the same day our `days_in_year` does.

Recorded the solver contract in [`../references.md`](../references.md), verified
against its source rather than guessed from responses.

Tried a `build.rs` that warned when `inputs/` was missing, then removed it.
`include_str!` already names the missing path, so it added little. Two things
came out of the attempt worth keeping: a build script gates the whole package,
so panicking would have blocked `fetch`, which is the binary you need to fix the
problem; and it could only check that the directory was non-empty, since the
days actually embedded live in the `solutions!` invocation, which a build script
cannot read without parsing `main.rs`.

Lost `inputs/` during that testing and re-downloaded it. Recovery was
uneventful, which is the no-clobber caching working as intended.

Verified day 1 end to end. `cargo run --bin solve -- -y 2015 -d 1` gives `280`
and `1797`, and the solver returns the same for both parts.

Decided how validation gets wired into `solve`, though the wiring itself is not
written. See [`../design/verification.md`](../design/verification.md) and
[`../todo.md`](../todo.md). Short version: opt-in `-v`/`--validate` flag, the
call happens where the input is already in scope, and the `Session` gets built
lazily so `solve` does not start demanding a cookie it never uses.

Settled the name at the end of the session. `check_answer` became
`validate_answer` and the flag is `--validate`, because `cargo check` already
means "compile without producing a binary" and `--check` reads like a build-only
flag. The flag is declared but inert; nothing calls `validate_answer` yet.

## 2026-08-03

Reviewed the initial downloader. Found and fixed an existence-check inversion in
the directory helpers: `!path.is_dir()` is true for a path that does not exist
yet, so the first run tried to `remove_file` something absent and bailed before
creating anything.

Reframed the force-overwrite helpers as no-clobber `ensure_*` helpers, since
neither inputs (immutable, remote-owned) nor source files (ours, accumulating
work) should ever be truncated by a re-run.

Scoped `fetch` to inputs only. Scaffolding solution modules has a different
trigger and would have dragged `mod.rs` generation and parent-module wiring into
a tool that just downloads files.

Wired the download loop, which had been built but never called. Fixed
`get_input` to reuse the pooled client, added `error_for_status()` so an error
page cannot be cached as fake input, and added a `User-Agent`.

Settled the solution model: a `Sized` trait, parse once in `new`, parts return
`Option<String>`. Settled dispatch as a macro over one line per day, after
working through why `dyn` does not apply and why `linkme` does not remove the
central list.

Chose compile-time `include_str!` for inputs and rejected a fetching `build.rs`.

Set up the repo: README, MIT license, this context directory, and the remote.
Three commits: `944a12a`, `faf0ed0`, `c60d2d2`.
