# Journal

Newest first. Names in older entries were updated when things got renamed, so
they read consistently rather than historically.

## 2026-08-10

A layering fix, prompted by translating `Outcome` into C# and noticing the Rust
had a dependency pointing the wrong way. Same pattern as 2026-08-08: the
translation keeps finding things the original got away with.

The domain imported from `outbound` in exactly two places, which was smaller
than it felt. `domain/solutions/outcome.rs` pulled both verdicts from
`outbound::client`, and `domain/solutions/solution.rs` pulled `SolverClient`.

Both verdicts moved into the domain. They were always domain vocabulary: neither
mentions HTTP, neither carries a status code, and `Outcome` matches on them to
render a line. They lived beside the clients purely because that is where they
were first written. Now the clients map their replies onto them, which is the
direction ports and adapters wants.

`solve()` moved out to `outbound/client/solve.rs`. The alternative was a port
trait the domain owns and `SolverClient` implements, which is the textbook
answer and buys nothing here: `solve()` is orchestration rather than domain
logic, its only callers are in `inbound/solve/`, and moving it removes the need
for the domain to call out at all. Whether `outbound/client/` is the right
destination is still open, since it is not a client. `inbound/solve/` sits next
to the callers and would let the module docs stay as written.

`solutions/` became `solution/`, matching the `Solution` trait now in its
`mod.rs`. `aoc.rs` and `solver.rs` became `aoc_client.rs` and
`solver_client.rs`.

Worth remembering from the cleanup:

- `cargo build` never checks intra-doc links. Both renames compiled fine while
  three `[`Type`]` links pointed at nothing. Only `cargo doc` catches it, so run
  it after any rename.
- A module doc that has gone false is the signal the code moved somewhere it
  should not have. `outbound/client/mod.rs` said "HTTP clients for the two
  services" and could not honestly describe `solve.rs`.
- The README's "Adding a solution" instructions were stale in four places. They
  are checked now by actually following them against the tree and building, the
  same way they were checked when they were wrong once before.

The merge into this branch conflicted the way a directory rename always does.
Git followed the files both branches touched, but `year_2015/` and `year_2016/`
exist only here, so nothing told it they belonged under the new name and they
stayed behind under `solutions/`. Rebase would not have helped: five of the ten
commits touch that path, so it would ask the same question five times and
rewrite pushed history. Point at the destination once and move on.

Solutions were never at risk. `git diff` against the pre-merge tag showed the
day files changed by exactly one line each, the `use`.

Also fixed: the first merge commit went in without the import edits, because
`git mv` stages a rename but not later edits to the moved file. The build passed
anyway since the working tree had them. `git status` showing modified files
straight after committing is the tell. Verified the fix by cloning the committed
branch and building that, rather than trusting the working copy.

Started `domain/solution/common/`, the pieces more than one puzzle needs.
`Direction` is the four axis moves, parsing from a letter or a word. `Point` is
cartesian, signed, `y` up. `Cell` is a grid index, unsigned, rows down from the
top. Both get `checked_moved` and `saturating_moved`, so a solution picks
whether leaving the grid is an error or a clamp.

The two conventions are the whole reason they are separate types, and the tests
proved the point by getting it wrong: the row assertions in `cell.rs` were
copy-pasted from `point.rs`, so they expected `Up` to increase the row. Three of
the four tests were also marked `#[ignore]` with no `#[test]`, which is not a
skipped test but a plain function the harness never sees. That hid the one real
failure. Both fixed, all four run.

That work surfaced the missing piece: parsing can fail and there was nowhere
for the failure to go. `Instruction::try_from` returns `InvalidInstruction`, but
the parts returned a bare `Answer`, so the only honest option was `.unwrap()`.
Which is what day 1 had.

Parts are now `anyhow::Result<Answer>`, and `Outcome` holds the whole
`Result` rather than just an `Answer`.

Rejected putting an `Error` variant on `Answer`. Its doc comment says "what one
part produced, nothing else", and an error is not something the part produced.
Every match on `Answer` would have grown a case, `value()` would have to
remember to exclude it, and nothing would stop a day from constructing one by
hand as if it were an answer. `Result` says the same thing in the type and
brings `?` with it.

Rejected `Box<dyn Error>` for `anyhow::Error`: `Solution::new` and `solve`
already return `anyhow::Result`, `anyhow` is `Send + Sync` where the box is not
by default, and `.context()` is the whole reason to want the error in the first
place.

The part worth having thought about is where the error stops. Propagating it out
of `solve` would have been less code, but a puzzle runner should never let a
broken part two hide a working part one. So a failing part is caught into its
own `Outcome`, prints as `error: <chain>` with its timing, and the other part
runs regardless. Only a failure in `Solution::new` ends the day, since then
neither part has anything to read. Errors carry no value, so they cannot be
validated or submitted, and that falls out of the `value()` gate the visual and
absent answers already went through rather than needing a new rule.

2016 day 1 part one is done, and validates at 278. It parses in `new`, holds the
`Instructions`, and walks them with a new `Pose`.

`Pose` is a `Point` plus a `Direction`, because these instructions are relative:
`R2` turns from wherever the last one left you pointing, so a position on its
own is not enough state to walk. It went through three names before landing.
`Position { facing, located_at }` was circular, since the inner field wanted the
same word as the type. `Position { facing, location }` fixed that. `Pose` is the
term for position plus orientation, and once the type carried that meaning the
fields could take the plain nouns: `heading` and `position`. Worth the two
renames. The first version read as a struct that had run out of names.

The four-variant parameter was the day's real find. `Pose::saturating_turned`
took a `Direction`, and matched `Up` and `Down` to no-ops, which felt silly
enough to ask about. It was worse than silly. `Instruction` parsed its letter
with `Direction::try_from`, which accepts `u` and `d`, so `U3` parsed happily,
turned nowhere, and walked three blocks along whatever heading it already had. A
malformed input became a confidently wrong answer, and `InvalidInstruction`
never fired.

So `Turn { Left, Right }` now lives in `common/`, with its own `TryFrom` that
takes only `l` and `r`. `Instruction` holds a `Turn`, `Direction::turned(Turn)`
joins the two turn methods, and `saturating_turned` became `turned`, since
nothing saturates in a turn. The ignored arms are gone because there is nothing
left to ignore, and `U3` now fails at parse time with the error that was already
written for it. The lesson to keep: an argument type with dead arms is usually a
parser accepting more than it should, one layer up.

`Turn` shipped with two methods nobody called, `applied_to` and `reversed`, and
both are gone again. `applied_to(direction)` was the worse of the two: it was
`direction.turned(turn)` with the arguments swapped, so the same operation had
two spellings. Speculative API on a type that exists to be parsed into.

### Dead code to look at next

Audited the rest while that was fresh. Nothing below is urgent and nothing is
broken; this is a list to work through rather than a bug report. Verified by
grepping for callers outside each item's own definition and test module.

Probably keep, since each is the unused half of a deliberate pair:

- `Cell`, the whole type. Never used by a day. It is the grid counterpart to
  `Point` and no puzzle has needed a grid yet. The first maze or occupancy day
  will want it.
- `Point::checked_moved` and `Cell::checked_moved`. Tests only. Days so far use
  `saturating_moved`; a day that must not leave the grid will want the other.
- `Outcome::answer`. No callers, but it is the accessor for a private field.
- `Outcome::error`. No callers. Written when parts became fallible, on the guess
  that something would want to inspect the failure. Nothing does, since
  `Display` reads the field directly. Delete unless a caller shows up.

Deleted, being leftovers from earlier shapes rather than halves of anything:

- `store::day_path`, which only wrapped `day_path_in` with the real cache root.
  `day_path_in` took over its doc line.
- `SolverClient::with_client`. `new` is the only way one gets built.
- `fetch::utils::download_input`, which also took `Day` and `AocClient` out of
  that module's imports. Downloading goes through `inbound/input.rs` now, and
  this was the earlier path that nothing had removed.

None of the three had a caller or a test, so nothing else moved.

Also worth deciding rather than leaving: `Solution::input` is only ever called
by a day on itself. If it goes, `new` stops having to retain the raw input, days
store only what they parsed, and the `impl Into<String>` bound can relax to
`&str`. See the `AsRef<str>` note that prompted it.

Also swept every doc comment in the repo, 422 lines down to 290, and wrote
`../rules/doc_comments.md` so it stays that way. The rule is one line by
default, longer only for a decision that reads as a mistake, a rejected
alternative, a trap, or a cross-reference that saves a search. Most of what came
out was accessors restating their own names and rationale that had grown a
paragraph past its point. Two blocks stayed long on purpose: the cache layout in
`store/mod.rs`, which is a diagram rather than prose, and the module doc in
`outbound/client/`, which answers why there are two clients instead of one.

## 2026-08-08

Tail end of the rework, mostly prompted by translating the same types into C#
and noticing what the Rust was doing awkwardly.

`Day::each` moved off the module and onto `Day`, since a `Day` already carries
its `Year` and enumerating days is enumerating the pairs. It iterates validated
`Year`s now rather than raw integers.

`Day::new` takes a built `Year` rather than a number, so `each` stops
constructing a year only for `new` to throw it away and rebuild one. `Year` is
`Copy`, being one integer.

The day bounds check dropped `(1..=n).contains(&day)` for two comparisons. That
came straight from C# having no range type to lean on, which made the
indirection obvious in both languages.

`Part::to_wire_value` became `wire_value` taking `self`. Rust's `to_` signals
expense and this allocates nothing, where C#'s `To*` only signals conversion.
Same method, different right answer per language.

Two stale doc links fixed, `latest_year` and `Session::from_env`, both left over
from earlier moves. `cargo doc` is clean.

## 2026-08-07

Four of the six planned changes landed. The repo looks quite different.

**One binary.** `fetch` and `solve` are subcommands now, so `--bin` is gone from
every invocation. Cargo also stops parsing at the subcommand, which means the
`--` separator is optional and cargo's own flags simply go first:
`cargo run --release solve -y 2015 -d 1`.

**Ports and adapters.** The library split into `domain`, `inbound`, and
`outbound`. Fixing the wiring turned up empty `mod.rs` files, `run.rs` modules
still declaring `args` from when they were `main.rs`, and every internal path
needing a rewrite. Grouped `use crate::{a, b, c}` imports made that fiddly, since
only the first element matches a naive replace.

`calendar` became `address`, and `Part` moved inside it. Year and day being
calendar-shaped was incidental to their real job, which is naming one puzzle, so
`calendar` left `Part` sitting outside a module about dates. `coordinates` and
`path` were the other candidates. `path` lost because `std::path` is imported in
the same files.

**Inputs at runtime.** `include_str!` is gone, so the project compiles with an
empty cache and `solve` fetches what it needs. That also killed the dispatch
macro, which existed mostly to build input paths from literals at expansion
time. The registry is a plain match now, one arm per day, and it yields a
function pointer rather than calling directly so an unregistered day is skipped
before anything downloads.

Two bugs of mine on the way. The first version fetched before checking the
registry, which would have pulled all 262 inputs on an unfiltered run. The
second gated submission on `if let Some(aoc)`, and since `ensure_entry` builds a
client lazily, solving a day with a missing input and no `--submit` would have
submitted anyway.

Also replaced `Option<&SolverClient>` as an implicit validate flag with an
explicit `validate: bool`. The client is always built now, since it needs no
cookie and cannot fail, so there is no invalid combination to guard.

**Session fingerprinting and instructions.** Each day is a directory of plain
files: `input.txt`, `session` holding a SHA-256 of the cookie, and
`part_one.md` / `part_two.md`. A session mismatch refetches the input and keeps
the instructions, since puzzle text is the same for everyone.

That landed as one JSON file per day first, which read badly: a 7000 character
input and a page of markdown both collapse onto one escaped line. Splitting into
plain files cost the guarantee that an input and its hash are written together,
but a missing `session` reads as "refetch" anyway, so the failure mode is the
same either way. `serde` and `serde_json` were added and removed within the
hour.

The day page splits on `<article class="day-desc">`, one per unlocked part,
verified across three pages before building on it. `part_two.md` existing is
what says part two is available, so nothing can disagree with the text beside
it. `html2text` does the rendering, inside the client, so the store never sees a
tag.

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

Verified day 1 end to end. `cargo run solve -y 2015 -d 1` gives `280`
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
