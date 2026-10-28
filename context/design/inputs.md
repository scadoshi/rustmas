# Puzzle inputs and instructions

`src/lib/outbound/store/`, `src/lib/inbound/input.rs`, `cache/`

Each day gets one directory under `cache/<year>/<NN>/`, zero padded:

```text
input.txt     the puzzle input, verbatim
session       hash of the cookie that fetched it
part_one.md   puzzle text
part_two.md   puzzle text, absent until part one is solved
```

`cache/` is gitignored. Inputs are tied to a personal account, and AOC asks that puzzle text not be republished.

Every file opens in an editor on its own. An earlier version put everything in one JSON file per day, which read badly: a 7000 character input and a page of markdown both collapsed onto a single escaped line.

Plain files give up one guarantee: the input and its hash are no longer written together. The failure is mild. A missing or unreadable `session` reads as "refetch", which is what a mismatch does anyway.

The existence of `part_two.md` is what says part two is available, so no flag can disagree with the text beside it.

Each file is written to a `.tmp` beside it and renamed into place, so a crash leaves the old file or the new one, never a truncated one. A file that already holds the same text isn't written at all, so a part-two chase writes `part_two.md` and leaves `input.txt` alone.

## Session fingerprinting

`Input` carries a SHA-256 of the cookie that fetched it.

- `Input::new` generates the hash.
- `Input::from_parts` rebuilds an input already on disk.
- `is_from(cookie)` answers whether an input belongs to the current session.

This exists because swapping `COOKIE` silently invalidates every input, and nothing caught it. `2015/01` answered `280` one day and `138` the next, and only the changed answers gave it away.

A mismatch refetches the input and keeps the instructions, since puzzle text is identical for everyone. `read_entry` returns whatever is on disk and leaves that judgment to the caller, so the two lifecycles stay separate inside one entry.

With no cookie configured at all, a cached entry is used as-is. Being unable to verify an input shouldn't make it unusable.

## Fetch on demand

`ensure_entry` reads from disk and downloads what is missing, so both subcommands share one path.

- `fetch` calls it and discards the result.
- `solve` calls it and solves.
- With nothing cached, it fetches both the input and the page, so a day always arrives complete.

`LazyAocClient` builds the `AocClient` on first download rather than up front. A run over cached inputs stays entirely offline and needs no cookie.

`--submit` is the one exception. It connects the client immediately, so a bad cookie fails before any solving happens rather than partway through.

Solving matches the day against the registry before calling `ensure_entry`. A run over every year never downloads anything for a day with no solution.

`ensure_entry` reads the cookie through `Environment::cookie_if_set`, which sits apart from `AocClient::from_env`. A hash check never forces a client into existence.

## Rejected: compile-time embedding

For the first week, inputs were embedded with `include_str!`. It worked. The cost was that `cargo build` needed `inputs/` populated, so a fresh clone couldn't compile until `fetch` had run.

It went because `solve` should fetch a missing input itself, and that can't happen at compile time. Reading at runtime also retired the dispatch macro, which existed mostly to build those paths from literals.

A `build.rs` that warned about a missing `inputs/` was tried and removed while embedding was still in place. `include_str!` already named the missing path, so it earned little. The attempt taught two things:

- A build script gates the whole package. Panicking would have blocked `fetch`, the binary you need to fix the problem.
- It could only check that the directory was non-empty. The days actually embedded lived in a macro invocation, which a build script can't read without parsing source.

A `build.rs` that *downloaded* missing inputs was rejected outright. Build scripts run for `cargo check` and for rust-analyzer, so it would:

- put the network and the session cookie in the path of every keystroke
- fail offline
- turn an expired cookie into a compile error

## Caching and no-clobber

An existing input counts as a cached download and is never refetched, since AOC asks that inputs not be re-downloaded. `ensure_dir` errors rather than overwriting when the wrong kind of thing sits in the way.

`fetch_input` passes the response through `accepted`, which calls `error_for_status()`. An error page can't get written to disk and cached as though it were real input.

## Instructions

The day page holds one `<article class="day-desc">` per unlocked part. Two articles means part two is available; one means it's still behind the first star. Completeness is structural, not a flag that could disagree with the text beside it.

`fetch_instructions` renders each article with `html2text` at width 80. The client does the split and the rendering, so the store never sees a tag and doesn't know where its data came from. `verdict_from` keeps AOC's reply parsing inside the client the same way.

## Open

One failed download aborts the whole run. The alternative is log-and-continue, which is more resilient but would let a bad cookie fail quietly.

Nothing displays the instructions. They are stored and never read.

## Resolved

Part two's text used to be cached only if the day was fetched after part one was starred. Two things fill it in now:

- `ensure_entry` refetches the page on every run with a cookie while `part_two.md` is missing, except on day 25.
- `solve` calls `ensure_entry` again when a submission earns part one a new star.
