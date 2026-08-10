# Solutions and dispatch

`src/lib/domain/solution/`, `src/lib/inbound/solve/`,
`src/lib/outbound/client/solve.rs`

## The trait

```rust
pub trait Solution: Sized {
    fn new(input: impl Into<String>) -> anyhow::Result<Self>;
    fn input(&self) -> &str;
    fn part_one(&self) -> Answer;
    fn part_two(&self) -> Answer;
}
```

`new` parses once and returns `Result<Self>`, so both parts are reads over
already-parsed fields rather than two passes over raw text.

The trait is `Sized` and therefore not object safe, deliberately. A method
returning `Self` cannot go through a vtable: there is no receiver to find the
vtable from, and the caller cannot know the size of an erased return type.
Dispatch is a match that already knows each concrete type, so monomorphized
generics cover everything and erasure buys nothing.

Each day's type is named `Puzzle` and holds its input as an owned `String`. A
lifetime parameter would work too, but it would infect the trait and every call
site for no gain at this size.

## Answer and Outcome

```rust
pub enum Answer {
    Value(String),
    Visual(String),
    None,
}

pub struct Outcome {
    answer: Answer,
    elapsed: Duration,
    verdict: Option<Verdict>,     // the solver
    submission: Option<Verdict>,  // adventofcode.com
}
```

Split by provenance. `Answer` is what the part computed and nothing else,
`elapsed` is measured, and the verdicts arrive over the network.

They were one type for a while, with the verdicts folded into the `Value`
variant so a visual answer could not carry one. Timing broke that: a duration
applies to every variant, so it could not go inside `Value`, and once it sat
beside the enum the type was measuring one field while holding two others from
completely different sources. `Timed { answer, elapsed }` read as "this answer
took this long" when it meant "computing this value took this long", with the
verdicts unmeasured and unmentioned.

`Outcome` keeps the old invariant without the enum trick: `with_verdict` and
`with_submission` check for a submittable answer and ignore anything else, so a
visual answer still ends up with no verdict.

`Answer` replaced `Option<String>`, which conflated two different absences and
left a visual part unable to hand back its grid at all, since any `Some` would
have been sent off for validation. Visual parts printed from inside the solver
as a result. Now the solution returns the art and the caller prints it.

An alternative was a separate `is_visual(part)` method on the trait. Rejected
because it is a second source of truth that can disagree with what the part
actually returned, and it leaves the grid trapped inside the solver.

## Running a solution

```rust
pub fn solve<S: Solution>(
    client: &SolverClient,
    validate: bool,
    input: &str,
    day: &Day,
) -> anyhow::Result<Solved>
```

It lives in `outbound/client/solve.rs`, not in the domain. Holding a
`SolverClient` is a dependency the domain is not allowed to have, and it was in
`domain/solutions/solution.rs` for a long time without anyone minding. Moving it
was what made the domain import nothing outside itself.

Whether `outbound/client/` is the right home is still open. It is not a client,
it is the orchestration that drives one, and `inbound/solve/` already holds its
only callers. The alternative to moving it is a port trait the domain owns and
`SolverClient` implements, which is the textbook answer but buys nothing here,
since the domain does not need to call out once the runner is elsewhere.

A free function rather than a method on a client. It briefly lived on the HTTP
client, which put application logic inside an adapter that never needed its own
cookie to run a solution.

Intent is an explicit flag. An earlier version passed `Option<&SolverClient>`
and read `None` as "do not validate", which removed a parameter but hid the
decision in a type. The client is always built now, since `SolverClient::new()`
needs no cookie and cannot fail, so there is no invalid combination to guard
against and the call site says what it is doing.

## Dispatch

One match arm per day, in `solver_for`:

```rust
fn solver_for(year: i32, day: i32) -> Option<Solver> {
    Some(match (year, day) {
        (2015, 1) => solve::<year_2015::day_01::Puzzle>,
        (2016, 1) => solve::<year_2016::day_01::Puzzle>,
        _ => return None,
    })
}
```

Returning a function pointer rather than calling means the registry can be asked
whether a day exists without holding its input. Three things fall out of that.
A run skips unwritten days before `ensure_entry` downloads anything for them,
`--submit` can count what it is about to send, and asking for a specific
unwritten day can say so instead of printing nothing.

It briefly lived inline in the solve loop, which read fine while nothing else
needed to ask the question.

Rust has no runtime reflection, so this mapping has to exist in source
somewhere. A `macro_rules!` generated these arms while inputs were embedded with
`include_str!`, since building the path needed the year and day as literals at
expansion time. Reading at runtime removed that reason, and the longhand version
formats, jumps to definition, and reports errors on real lines.

Every day's type is named `Puzzle`, with the module path carrying the
coordinate. `Solver` was the first choice and was rejected, since `SolverClient`
already means the third-party service throughout this repo. `Solution` is the
trait and `Day` is a coordinate, which left `Puzzle`.

Importing the year modules rather than the types avoids aliasing. Two years both
exporting `Puzzle` would collide as imports, but the path disambiguates them.

## Rejected: linkme

A distributed slice would let each day register itself with no central list.
Rejected because a day still needs its `mod` declaration or it never compiles
and never registers, so the central list does not go away, it just gets shorter
and less informative. A missing `mod` fails silently as a day that quietly does
not exist, where a missing match arm is at least visible where you would look.

## Walking the days

`Day::each(year, day)` yields every published day, narrowed by the filters. Both
subcommands use it, so the year and day loops exist once rather than being
duplicated with slightly different guards. `None` means all of them, so the four
flag combinations need no matching.

It sits on `Day` because a `Day` already carries its `Year`, so enumerating days
is enumerating the pairs. It iterates validated `Year`s rather than raw integers,
which is why `days_in` can stay a method rather than a loose function taking an
unchecked number.

## Timing

`solve` returns `Solved { parse, one: Outcome, two: Outcome }`. Parsing is
measured separately, since a slow day is often slow in one place or the other.

Both parts are computed and measured before any validation runs, so no duration
includes a network round trip. Debug and release differ by roughly twenty times
on 2015 day 1, which is the easiest sanity check that the numbers mean anything.

Parse time belongs to neither part, which is why `Solved` exists rather than the
durations hanging off the parts alone.

## Open

`Solution::input()` exists but nothing calls it, since `solve` already holds the
input it was given. Worth deleting unless something needs it.

No day returns `Answer::Visual` yet. Its rendering is covered by tests, but
nothing produces one in a real run.

## Tests

Mostly pure functions, so they live beside what they test. The `Outcome` display
matrix gets the most attention, since it decides everything a user reads and had
no coverage at all until it had six branches.

The store threads its root through private `_in` variants, so tests use a
temporary directory rather than the real cache. An env var would have been
simpler but tests run in parallel and would fight over it.
