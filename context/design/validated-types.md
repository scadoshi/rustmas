# Validated types

`src/lib/day.rs`, `src/lib/part.rs`

`Year` and `Day` are newtypes over `u32`. `Day` wraps a `Year`, constructors are
the only way in, and fields are private. That makes several states
unrepresentable: a year before 2015 or after the current one, a day outside the
range its year actually published, and a day with no year attached.

`days_in_year()` is the single source of truth for how long an event ran. 2025
was a 12-day event, everything else is 25. Both `Day::new` and the `solve` loop
read it, so they cannot drift apart.

`Part` is a fieldless enum with `to_wire_value()` returning 1 or 2. It exists so
call sites read `submit(&day, Part::One, answer)` rather than passing a bare `1`
that is indistinguishable from a day number. Both the AOC submit form (`level=`)
and the solver API path want the number, so one type covers both renderings.

## History

`Part` was originally a struct wrapping `Day` plus a kind, was deleted in
`c60d2d2` as dead code, and came back as a plain enum once the answer checker
needed it. The wrapper did not come back, because `submit(&Day, Part, ...)` reads
as well as `submit(&Part, ...)` without the accessors that had to reach through
two layers to get a year.
