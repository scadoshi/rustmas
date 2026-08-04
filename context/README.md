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
`init` downloads every published puzzle input to `inputs/<year>/<NN>.txt`.
`solve` runs the solutions, filtered by optional `-y`/`--year` and `-d`/`--day`
flags. Omitting a flag means "all", so both flags are filters rather than a
lookup, and no flags at all runs everything.

## Design decisions made so far

`Year -> Day` are validated newtypes in `src/lib/day.rs`. `Day` wraps `Year`,
constructors are the only way in, and the fields are private. That makes the
illegal states unrepresentable: a bad year, a day that doesn't exist for its
year, or a day with no year at all. `days_in_year()` is the single source of
truth for how many days a year has. 2025 was a 12-day event; the rest are 25.
A `Part` type lived here once and was removed as dead code; it belongs with the
answer verifier whenever that gets built.

Solutions implement `Solution` (`src/lib/solutions/mod.rs`), which is `Sized`
and so deliberately not object safe. `new` parses the input once and returns
`Result<Self>`; `part_one` and `part_two` are pure reads returning
`Option<String>`. `None` means there is nothing submittable, either because the
answer is a grid you read in the terminal or because it is Day 25 part two.

Dispatch is a `macro_rules!` in `src/bin/solve/main.rs`. It takes one line per
day and generates a single `dispatch` function whose match arms map a runtime
`(year, day)` to a concrete type. The input path is built at expansion with
`concat!` and `stringify!`, which is why days are written zero-padded (`01`) and
why the `zero_prefixed_literal` lint is allowed on the generated function. An
unregistered day returns `None`, distinct from a part returning `None`.

Inputs are `&'static str` via `include_str!`, so they are baked in at compile
time. The tradeoff is that `cargo build` needs `inputs/` populated, and that dir
is gitignored, so a fresh clone must run `init` before `solve` will compile. A
`build.rs` that fetched inputs was considered and rejected, since it would drag
the network and the session cookie into every `cargo check`.

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

Targeting: should `init` take year and day flags the way `solve` does, or stay
sync-everything?

Failure mode: one failed download currently aborts the whole run. Keep it loud,
or log-and-continue? The resilient version has a downside, since a bad cookie
would then fail quietly.

Explicit misses: asking `solve` for a day with no registered solution prints
nothing, same as skipping it during a run-all. The `None` from `dispatch` knows
the difference, but `main` does not act on it.

Timing: planned but not built. It belongs in `solve()` in `src/bin/solve/utils.rs`,
where `new` and the two parts are actually called, so parse time can be reported
separately from solve time.

The answer verifier does not exist yet. Open question is whether it checks
against stored known-good answers offline, or submits to AOC and reads the
graded reply, in which case the interesting type is a `Verdict` enum rather than
a bool.

`run.rs` is unimplemented, and solution-module scaffolding is deferred.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
