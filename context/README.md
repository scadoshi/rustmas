# context (read me first)

Hand this dir to any AI assistant to resume work on `rustmas` with full context.

## Who

scadoshi (Scotty) is a strong Rust developer. He's deep on ownership, traits,
error-as-values, and making illegal states unrepresentable, so skip beginner
Rust explanations.

## How to work with him

He streams ideas and half-formed designs. Your job is to correct what's wrong,
briefly confirm what's right, and extend with a question. Don't write novels.
Keep replies short and skip the emojis.

When he asks for implementation, write the code. When he's still thinking out
loud, coach and nudge instead of jumping to code.

Follow [`rules/commit_guidelines.md`](rules/commit_guidelines.md) for any commit.

## What rustmas is

Advent of Code tooling in Rust. Two binaries sit over one shared library.
`init` downloads every published puzzle input to `inputs/<year>/<NN>.txt`. `run`
is the solution runner, still `todo!()`.

## Design decisions made so far

`Year -> Day -> Part` are validated newtypes in `src/lib/part.rs`. Each wraps
the one before it, constructors are the only way in, and the fields are private.
That makes the illegal states unrepresentable: a bad year, a day that doesn't
exist for its year, or a day with no year at all. `days_in_year()` is the single
source of truth for how many days a year has. 2025 was a 12-day event; the rest
are 25.

`init` only touches inputs. Scaffolding solution modules stayed out on purpose,
since it has a different trigger and a different owner. Empty `mod.rs` stubs
don't wire the module graph anyway, so scaffolding becomes its own step later.

Caching reuses the no-clobber rule. `ensure_dir` and `download_input` no-op when
the target already exists, and they error rather than overwrite when the wrong
kind of thing is in the way. An existing input is a cached download, and AOC
asks you not to re-fetch.

`Session` (`src/lib/session.rs`) reads `COOKIE` from the env or `.env`, reuses a
pooled client, sends a `User-Agent` for AOC etiquette, and calls
`error_for_status()` so an error page can't get cached as fake input.

## Open forks (undecided)

Targeting: should `init` take optional `year day` args to fetch one puzzle, or
stay sync-everything with targeting built into `run`?

Failure mode: one failed download currently aborts the whole run. Keep it loud,
or log-and-continue? The resilient version has a downside, since a bad cookie
would then fail quietly.

`run.rs` is unimplemented, and solution-module scaffolding is deferred.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
