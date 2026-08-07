# Solutions and dispatch

`src/lib/domain/solutions/`, `src/lib/inbound/solve/`

## The trait

```rust
pub trait Solution: Sized {
    fn new(input: &str) -> anyhow::Result<Self>;
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

## Answer

```rust
pub enum Answer {
    Value { value: String, verdict: Option<Verdict>, submission: Option<Verdict> },
    Visual(String),
    None,
}
```

This replaced `Option<String>`, which conflated two different absences and left
a visual part unable to hand back its grid at all, since any `Some` would have
been sent off for validation. Visual parts printed from inside the solver as a
result, putting a side effect somewhere awkward to test. Now the solution
returns the art and the caller prints it.

Both verdicts sit inside `Value` rather than beside the enum, so a visual answer
cannot carry one. `Answer::solved(value)` is the constructor a day uses;
`with_verdict` and `with_submission` attach results afterwards and land only on
`Value`.

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

A free function rather than a method on a client. It briefly lived on the HTTP
client, which put application logic inside an adapter that never needed its own
cookie to run a solution.

Intent is an explicit flag. An earlier version passed `Option<&SolverClient>`
and read `None` as "do not validate", which removed a parameter but hid the
decision in a type. The client is always built now, since `SolverClient::new()`
needs no cookie and cannot fail, so there is no invalid combination to guard
against and the call site says what it is doing.

## Dispatch

One match arm per day, inside the solve loop:

```rust
let solver_fn: Solver = match (day.year(), day.value()) {
    (2015, 1) => solve::<year_2015::day_01::Puzzle>,
    (2016, 1) => solve::<year_2016::day_01::Puzzle>,
    _ => continue,
};
```

The match yields a function pointer rather than calling directly, so a day with
no solution is skipped before `ensure_entry` would download anything nothing can
use.

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

`address::each(year, day)` yields every published day, narrowed by the filters.
Both subcommands use it, so the year and day loops exist once rather than being
duplicated with slightly different guards. `None` means all of them, so the four
flag combinations need no matching.

## Timing

`solve` returns `Solved { parse, one: Timed, two: Timed }`, where `Timed` pairs
an `Answer` with how long it took. Parsing is measured separately, since a slow
day is often slow in one place or the other.

Both parts are computed and measured before any validation runs, so no duration
includes a network round trip. Debug and release differ by roughly twenty times
on 2015 day 1, which is the easiest sanity check that the numbers mean anything.

Duration stays out of `Answer` deliberately. It applies to every variant, so it
could not live inside `Value` the way verdicts do, and putting it beside the enum
would turn `Answer` into a struct wrapping one, undoing the reason a visual
answer cannot carry a verdict. Parse time also belongs to neither part, so
something like `Solved` has to exist regardless.

## Open

Asking for a day with no registered solution prints nothing, the same as
skipping it during a run over everything. The match `continue`s in both cases.

`Solution::input()` exists but nothing calls it, since `solve` already holds the
input it was given. Worth deleting unless something needs it.

No day returns `Answer::Visual` yet, so that path is written but unexercised.
