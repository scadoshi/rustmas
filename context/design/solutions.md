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

`Option<String>` as the return covers the cases where there is nothing to
submit: a part whose answer is ASCII art you read in the terminal, and day 25
part two, which is free. Note that if a visual part prints from inside the
solver, the side effect lives in the solver, which makes it harder to test.

## Dispatch

A `macro_rules!` in `src/bin/solve/main.rs` takes one line per day and generates
a single `dispatch` function whose arms map a runtime `(year, day)` to a
concrete type:

```rust
solutions! {
    (2015, 01) => Day01,
}
```

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
`src/bin/solve/utils.rs`, where `new` and the two parts are actually called, so
parse time can be reported separately from solve time.
