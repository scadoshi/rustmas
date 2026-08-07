# Solutions and dispatch

`src/lib/solutions/`, `src/bin/solve/`

## The trait

```rust
pub trait Solution: Sized {
    fn new(input: &'static str) -> anyhow::Result<Self>;
    fn part_one(&self) -> Option<String>;
    fn part_two(&self) -> Option<String>;
}
```

`new` parses once and returns `Result<Self>`, so both parts are pure reads over
already-parsed fields rather than re-parsing raw text twice.

The trait is `Sized` and therefore deliberately not object safe. `dyn Solution`
does not work and is not wanted. A method returning `Self` cannot go through a
vtable: there is no receiver to find the vtable from, and the caller cannot know
the size of an erased return type. Since dispatch is a match that already knows
each concrete type, monomorphized generics cover everything and erasure buys
nothing.

Parts return `Answer`, which says what kind of result it is:

```rust
pub enum Answer {
    Value { value: String, verdict: Option<Verdict> },
    Visual(String),
    None,
}
```

This replaced `Option<String>`, which conflated two different absences and left
a visual part unable to hand back its grid at all, since any `Some` would have
been sent off for validation. So visual parts printed from inside the solver,
putting a side effect somewhere awkward to test. Now the solution returns the
art and the caller prints it.

The verdict sits inside the `Value` variant rather than beside the enum, so a
visual answer cannot carry a verdict at all.
Same move as `Day` wrapping `Year`. `Answer::solved(value)` is the constructor a
day uses, and `with_verdict` attaches a verdict afterwards, landing only on
`Value`.

An alternative was a separate `is_visual(part)` method on the trait. Rejected
because it is a second source of truth that can disagree with what the part
actually returned, and it leaves the grid trapped inside the solver.

## Running a solution

`solve` is a free function in `src/lib/solutions/mod.rs`, not a method on
`Session`:

```rust
pub fn solve<S: Solution>(
    session: Option<&Session>,
    input: &'static str,
    day: &Day,
) -> anyhow::Result<(Answer, Answer)>
```

It briefly lived on `Session`, which put application logic inside what is
otherwise an HTTP adapter. `Session` never needs its cookie or client to run a
solution, it only delegates.

`Option<&Session>` doubles as the validate flag, since "no session" and "do not
validate" are the same condition. That removed a `bool` parameter and made the
lazy-session question answer itself: `main` builds the session only when
`--validate` is set, so solving offline never needs a cookie.

## Dispatch

A `macro_rules!` in `src/bin/solve/main.rs` takes one line per day and generates
a single `dispatch` function whose arms map a runtime `(year, day)` to a
concrete type:

```rust
use rustmas::solutions::{year_2015, year_2016};

solutions! {
    (2015, 01) => year_2015::day_01::Puzzle,
    (2016, 01) => year_2016::day_01::Puzzle,
}
```

Every day's type is named `Puzzle`, with the module path carrying the
coordinate, so `day_01::Day01` stops repeating itself. `Solver` was the first
choice and was rejected: `SolverClient` already means the third-party service
throughout this repo, and a local `Solver` would blur that. `Solution` is the
trait and `Day` is the coordinate type, which left `Puzzle`.

Importing the year modules rather than the types is what avoids aliasing. Two
years both exporting `Puzzle` would collide as imports, but the path
disambiguates them, and the import list grows by one per year rather than one
per day.

The input path is built at expansion with `concat!` and `stringify!`, which is
why days are written zero-padded. `stringify!(01)` gives `"01"`, and `01` is a
valid Rust literal equal to `1`, so the match arm still compares correctly. The
`zero_prefixed_literal` lint is allowed on the generated function, not on the
invocation, because attributes on a macro invocation are silently ignored.

Rust has no runtime reflection, so the year-and-day to type mapping has to exist
in source somewhere. The macro does not remove it, it just makes each entry one
line.

## Rejected: linkme

A distributed slice would let each day register itself with no central list.
Rejected because a day still needs its `mod` declaration or it never compiles
and never registers, so the central list does not actually go away, it just gets
shorter and less informative. A missing `mod` fails silently as a day that
quietly does not exist, where a missing match arm is at least visible where you
would look for it.

## Argument filtering

`solve` takes optional `-y`/`--year` and `-d`/`--day`. Both are filters rather
than a lookup, so omitting one means "all". Day alone runs that day across every
year. That collapses the four flag combinations into two `continue` guards
inside nested loops, with no matching on `Option` pairs.

## Open

Asking for a day with no registered solution prints nothing, exactly like
skipping it during a run-all. `dispatch` returns `None` in both cases and `main`
does not distinguish them.

Timing is planned but not built. It belongs in `solve()` in
`src/lib/solutions/mod.rs`, where `new` and the two parts are actually called,
so parse time can be reported separately from solve time.

`Solution::input()` exists but nothing calls it now that `solve` already holds
the input it was given. Worth deleting unless something needs it.

No day returns `Answer::Visual` yet, so that path is written but unexercised.
