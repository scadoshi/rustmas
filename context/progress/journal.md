# Journal

Newest first.

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
`src/lib/session/answer.rs`. A duplicate definition is still sitting in
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
so panicking would have blocked `init`, which is the binary you need to fix the
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

Scoped `init` to inputs only. Scaffolding solution modules has a different
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
