# Journal

Newest first.

## 2026-08-04

Wrote 2015 day 1. Part one folds over the characters, part two returns early at
the first index where the floor hits -1.

Decided how answer checking works, and why it needs two clients rather than one.
AOC gives stars but answers each part only once, so it cannot be a repeatable
check. The third-party solver is idempotent but cannot award anything. See
[`../design/verification.md`](../design/verification.md).

Brought `Part` back, as a plain enum this time rather than the struct that was
deleted the day before, since submitting needs to name a part.

Built `check_answer` against the solver. First attempt keyed `Unsupported` off a
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
