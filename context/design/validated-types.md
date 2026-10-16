# Validated types

`src/lib/domain/address/`

`Year` and `Day` are newtypes over `i32`. Constructors are the only way in, and fields are private.

- `Day` wraps a `Year`.
- `Day::new` takes a built `Year`, not a raw number, so no day exists without a validated year first. `Day::all` also never rebuilds the year it's already holding.
- `Year` is `Copy`, being one integer, so passing it around costs nothing.

That makes several states unrepresentable:

- a year outside the published events
- a day outside the range its year actually published
- a day with no year attached

`Year::days_in()` is the single source of truth for how long an event ran. 2025 was a 12-day event; everything else is 25. `Day::new` and `Day::all` both read it, so they can't drift apart.

`Filter` narrows the published days for a run, with `None` meaning all of them. It validates eagerly, so an impossible filter errors up front instead of matching nothing in silence. Its day is an `i32` rather than a `Day`, since day 13 with no year isn't an address: valid for 2015, not for 2025.

## Naming

`Year`, `Day`, and `Part` live together in `address/`, named for the job they do rather than what they resemble. Together they name one puzzle. The same address becomes a URL path (`/2015/day/1`) and a cache path (`cache/2015/01/`).

- `calendar` was the earlier name. It grouped year and day nicely but left `Part` sitting outside a module about dates.
- `path` was considered because of the URL and cache paths, and dropped since `std::path` is already imported in the same files.

Everything a year knows sits in `year.rs`: `FIRST_YEAR`, `Year::latest()`, and `Year::days_in()`. `address/mod.rs` re-exports the types, so callers write `domain::address::Day` instead of reaching a level deeper.

## Year::latest

`Year::latest()` is the latest *published* event, not the current calendar year. AOC drops a new one each December, so before December the current year has nothing in it.

`Year::new` bounds on that rather than on `Utc::now().year()`. The old bound let a 2026 day validate against an event that didn't exist.

## Part

`Part` is a fieldless enum, and `wire_value()` returns 1 or 2. It exists so call sites read `submit(&day, Part::One, answer)` instead of passing a bare `1` that looks just like a day number.

Both the AOC submit form (`level=`) and the solver API path want the number, so one type covers both renderings.

## History

`Part` started as a struct wrapping `Day` plus a kind. `c60d2d2` deleted it as dead code, and it came back as a plain enum once the answer checker needed it.

The wrapper didn't come back. `submit(&Day, Part, ...)` reads as well as `submit(&Part, ...)` and drops the accessors that had to reach through two layers to get a year.
