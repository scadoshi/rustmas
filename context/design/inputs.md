# Puzzle inputs

`src/lib/outbound/store/`, `src/lib/inbound/input.rs`, `inputs/`

Inputs live at `inputs/<year>/<NN>.txt`, zero padded. The directory is
gitignored because inputs are tied to a personal AOC account.

`outbound/store` owns the path and the reading and writing. `read_input` returns
`Option<String>`, so a missing file is an ordinary answer rather than an error.

## Fetch on demand

`ensure_input` reads from disk and downloads only when the file is absent, so
both subcommands share one path to an input. `fetch` calls it and discards the
text, `solve` calls it and solves.

The `AocClient` is built on first download rather than up front, which keeps a
run over cached inputs entirely offline and cookie-free. `--submit` is the one
exception: it builds the client immediately so a bad cookie fails before any
solving happens rather than partway through.

Solving matches the day against the registry before calling `ensure_input`, so
a run over every year never downloads an input for a day with no solution.

## Rejected: compile-time embedding

Inputs were embedded with `include_str!` for the first week. It worked, and the
cost was that `cargo build` needed `inputs/` populated, so a fresh clone could
not compile until `fetch` had run.

It went because `solve` should fetch a missing input itself, and that cannot
happen at compile time. Runtime reading also retired the dispatch macro, which
existed mostly to build those paths from literals.

A `build.rs` that warned about a missing `inputs/` was tried and removed while
embedding was still in place. `include_str!` already named the missing path, so
it earned little. Two things came out of the attempt. A build script gates the
whole package, so panicking would have blocked `fetch`, the binary you need to
fix the problem. And it could only check that the directory was non-empty, since
the days actually embedded lived in a macro invocation a build script cannot
read without parsing source.

A `build.rs` that *downloaded* missing inputs was rejected outright. Build
scripts run for `cargo check` and for rust-analyzer, so it would put the network
and the session cookie in the path of every keystroke, fail offline, and turn an
expired cookie into a compile error.

## Caching and no-clobber

An existing input counts as a cached download and is never refetched. AOC asks
that inputs not be re-downloaded. `ensure_dir` errors rather than overwriting
when the wrong kind of thing sits in the way.

`get_input` calls `error_for_status()` so an error page cannot be written to
disk and cached as though it were real input.

## Planned change

No-clobber gets an exception. Inputs will carry a hash of the session cookie
that fetched them, and a mismatch will overwrite rather than skip, because an
input from another account is wrong rather than cached. See `../todo.md`.

## Open

One failed download aborts the whole run. The alternative is log-and-continue,
which is more resilient but would let a bad cookie fail quietly.

Caching puzzle instructions alongside inputs would make the repo fully offline.
The day page is a plain GET, but part two's text is absent until part one is
solved, so a complete cache needs two passes. Gitignore it like inputs, since
AOC asks people not to republish puzzle text.
