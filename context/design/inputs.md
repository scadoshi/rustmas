# Puzzle inputs

`src/bin/init/`, `inputs/`

Inputs live at `inputs/<year>/<NN>.txt`, zero-padded, downloaded by the `init`
binary. The directory is gitignored because inputs are tied to a personal AOC
account.

## Compile-time embedding

Solutions take `&'static str` from `include_str!`, so inputs are baked into the
binary at build time. The consequence is that `cargo build` needs `inputs/`
populated, and since that directory is gitignored, a fresh clone must run `init`
before `solve` will compile.

This was chosen deliberately over reading files at runtime. The tradeoff is
known and accepted for a personal repo.

## Rejected: a build script that fetches

A `build.rs` that downloaded missing inputs before compiling would remove the
ordering requirement, and it does work, since build scripts run before the crate
compiles. It was rejected because build scripts also run for `cargo check` and
for rust-analyzer, which would put the network and the session cookie in the
path of every keystroke. Compiling would fail offline, and an expired cookie
would surface as a compile error rather than a run error.

If a build script is ever added, generating the dispatch table is the use worth
having. That is hermetic and needs no network.

## Caching and no-clobber

`ensure_dir` and `download_input` no-op when the target already exists, and
error rather than overwrite when the wrong kind of thing sits in the way. An
existing input file counts as a cached download, so `init` only fills gaps.
AOC asks that inputs not be re-downloaded.

`get_input` calls `error_for_status()` so an error page cannot be written to
disk and cached as though it were real input.

## Open

One failed download aborts the whole run. The alternative is log-and-continue,
which is more resilient but would let a bad cookie fail quietly.

`init` has no year or day filtering, unlike `solve`.

Caching puzzle instructions alongside inputs would make the repo fully offline.
The day page is a plain GET, but part two's text is absent until part one is
solved, so a complete cache needs two passes. Gitignore it like inputs, since
AOC asks people not to republish puzzle text.
