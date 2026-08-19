# Puzzle inputs and instructions

`src/lib/outbound/store/`, `src/lib/inbound/input.rs`, `cache/`

One directory per day under `cache/<year>/<NN>/`, zero padded:

```text
input.txt     the puzzle input, verbatim
session       hash of the cookie that fetched it
part_one.md   puzzle text
part_two.md   puzzle text, absent until part one is solved
```

`cache/` is gitignored, since inputs are tied to a personal account and AOC asks
that puzzle text not be republished.

Every file opens in an editor on its own. An earlier version put all of it in
one JSON file per day, which read badly: a 7000 character input and a page of
markdown both collapse onto a single escaped line. Plain files cost the
guarantee that the input and its hash are written together, but the failure mode
is mild, since a missing or unreadable `session` reads as "refetch", which is
what a mismatch does anyway.

`part_two.md` existing is what says part two is available, so nothing can
disagree with the text beside it.

## Session fingerprinting

`Input` carries a SHA-256 of the cookie that fetched it. `Input::new` generates
it, `Input::from_parts` rebuilds one already on disk, and `is_from(cookie)`
answers whether an input belongs to the current session.

This exists because swapping `COOKIE` silently invalidates every input, and
nothing caught it: `2015/01` answered `280` one day and `138` the next, and only
the changed answers gave it away.

A mismatch refetches the input and keeps the instructions, since puzzle text is
identical for everyone. `read_entry` returns whatever is on disk and leaves that
judgement to the caller, so the two lifecycles stay separate inside one file.

With no cookie configured at all, a cached entry is used as-is. Being unable to
verify should not make an input unusable.

## Fetch on demand

`ensure_entry` reads from disk and downloads what is missing, so both
subcommands share one path. `fetch` calls it and discards the result, `solve`
calls it and solves. Nothing cached means both the input and the page are
fetched, so a day always arrives complete.

The `AocClient` is built on first download rather than up front, which keeps a
run over cached inputs entirely offline and cookie-free. `--submit` is the one
exception: it builds the client immediately so a bad cookie fails before any
solving happens rather than partway through.

Solving matches the day against the registry before calling `ensure_entry`, so
a run over every year never downloads anything for a day with no solution.

`Environment::cookie` sits apart from `AocClient::from_env` so a hash check never
forces a client into existence.

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

## Instructions

The day page holds one `<article class="day-desc">` per unlocked part, so two
articles means part two is available and one means it is still behind the first
star. That makes completeness structural rather than a flag that could disagree
with the text beside it.

`html2text` renders each article at width 80. The client does the split and the
rendering, so the store never sees a tag and stays unaware of where its data
came from, the same way `verdict_from` keeps AOC's reply parsing inside the
client.

## Open

One failed download aborts the whole run. The alternative is log-and-continue,
which is more resilient but would let a bad cookie fail quietly.

Part two's text is only cached if the day was fetched after part one was
starred. Refetching the page on a `Correct` submission would fill that in, and
is not built yet.

Nothing displays the instructions. They are stored and unread.
