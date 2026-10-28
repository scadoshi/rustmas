# Solutions and dispatch

`src/lib/domain/solution/`, `src/lib/inbound/solve/`, `src/lib/outbound/client/solve.rs`

## The trait

```rust
pub trait Solution: Sized {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self>;
    fn part_one(&self) -> anyhow::Result<Answer>;
    fn part_two(&self) -> anyhow::Result<Answer>;
}
```

`new` parses once and returns `Result<Self>`. Both parts then read already-parsed fields instead of making two passes over raw text.

It takes `impl AsRef<str>` rather than `impl Into<String>` because nothing obliges a day to keep the raw text.

- A day that parses into its own types keeps no copy.
- A day whose parts read the input owns a `String` as a private field of its own choosing.

Parts return `Result` for two reasons: a day can use `?`, and a broken day reads as an error rather than as `Answer::None`. A failing part doesn't stop the other. Its error lands in that part's `Outcome`.

The trait is `Sized`, and so deliberately not object safe. A method returning `Self` can't go through a vtable: there's no receiver to find the vtable from, and the caller can't know the size of an erased return type. Dispatch is a match that already knows each concrete type, so monomorphized generics cover everything and erasure buys nothing.

Each day's type is named `Puzzle`. The template's `Puzzle` holds its input as an owned `String`. A lifetime parameter would work too, but it would infect the trait and every call site for no gain at this size.

## Answer and Outcome

```rust
pub enum Answer {
    Value(String),
    Visual(String),
    None,
    Unwritten,
}

pub struct Outcome {
    answer: anyhow::Result<Answer>,
    elapsed: Duration,
    solver_verdict: Option<SolverVerdict>,  // the solver
    aoc_verdict: Option<AocVerdict>,        // adventofcode.com
}
```

The split follows provenance:

- `Answer` is what the part computed and nothing else.
- `elapsed` is measured.
- The verdicts arrive over the network.

`Unwritten` is separate from `None` so a stub can't pass for a finished part that has nothing to say.

For a while these were one type, with the verdicts folded into the `Value` variant so a visual answer couldn't carry one. Timing broke that. A duration applies to every variant, so it couldn't go inside `Value`.

Once it sat beside the enum, the type was measuring one field while holding two others from completely different sources. `Timed { answer, elapsed }` read as "this answer took this long" when it meant "computing this value took this long", with the verdicts unmeasured and unmentioned.

`Outcome` keeps the old invariant without the enum trick. `with_solver_verdict` and `with_aoc_verdict` check for a submittable answer and ignore anything else, so a visual answer still ends up with no verdict.

`Answer` replaced `Option<String>`, which had two problems:

- It conflated two different absences.
- A visual part couldn't hand back its grid at all, since any `Some` would have been sent off for validation. Visual parts printed from inside the solver as a result.

Now the solution returns the art and the caller prints it.

A separate `is_visual(part)` method on the trait was the alternative. It was rejected because it's a second source of truth that can disagree with what the part actually returned, and it leaves the grid trapped inside the solver.

## Running a solution

```rust
pub(crate) fn run_day<S: Solution>(
    validation: Validation<'_>,
    input: &str,
    day: &Day,
) -> anyhow::Result<Solved>
```

It lives in `outbound/client/solve.rs`, not in the domain. Holding a `SolverClient` is a dependency the domain isn't allowed to have. It sat in `domain/solutions/solution.rs` for a long time without anyone minding, and moving it is what made the domain import nothing outside itself.

Whether `outbound/client/` is the right home is still open.

- It isn't a client. It's the orchestration that drives one.
- `inbound/solve/` already holds its only callers.
- The alternative to moving it is a port trait the domain owns and `SolverClient` implements. That's the textbook answer, but it buys nothing here: once the runner lives elsewhere, the domain doesn't need to call out.

It's a free function rather than a method on a client. It briefly lived on the HTTP client, which put application logic inside an adapter that never needed its own cookie to run a solution.

Intent is an explicit enum, `Validation::Off` or `Validation::Against(&solver)`. A bare `bool` beside the client says nothing at the call site, and `Option<&SolverClient>` would read `None` as "do not validate", hiding the decision in a type. The enum names the decision and carries the client only when it is used.

The module is `solve` and the function `run_day`, so the registry reads `solve::run_day::<Puzzle>` rather than `solve::solve`.

The client is always built now. `SolverClient::from_env()` needs no cookie and fails only when the `User-Agent` can't be a header value, so there's no invalid combination to guard against and the call site says what it is doing.

## Dispatch

`solver_for` has one match arm per day:

```rust
fn solver_for(year: i32, day: i32) -> Option<Solver> {
    match (year, day) {
        (2015, 1) => Some(solve::run_day::<year_2015::day_01::Puzzle>),
        (2016, 1) => Some(solve::run_day::<year_2016::day_01::Puzzle>),
        _ => None,
    }
}
```

It returns a function pointer rather than calling, so you can ask the registry whether a day exists without holding its input. Three things fall out of that:

1. A run skips unwritten days before `ensure_entry` downloads anything for them.
2. `--submit` can count what it's about to send.
3. Asking for a specific unwritten day says so instead of printing nothing.

It briefly lived inline in the solve loop, which read fine while nothing else needed to ask the question.

Rust has no runtime reflection, so this mapping has to exist in source somewhere. A `macro_rules!` generated these arms while inputs were embedded with `include_str!`, since building the path needed the year and day as literals at expansion time. Reading at runtime removed that reason.

The longhand version formats, jumps to definition, and reports errors on real lines.

Every day's type is named `Puzzle`, and the module path carries the coordinate. Other names were taken:

- `Solver` was the first choice, rejected because `SolverClient` already means the third-party service throughout this repo.
- `Solution` is the trait.
- `Day` is a coordinate.

That left `Puzzle`.

Import the year modules, not the types, to avoid aliasing. Two years both exporting `Puzzle` would collide as imports, but the path tells them apart.

## Rejected: linkme

A distributed slice would let each day register itself with no central list. Rejected, because a day still needs its `mod` declaration or it never compiles and never registers. The central list doesn't go away; it just gets shorter and says less.

A missing `mod` also fails silently, as a day that quietly doesn't exist. A missing match arm is at least visible where you'd look.

## Walking the days

`Day::matching(filter)` yields every unlocked day that a `Filter` allows. Both subcommands use it, so the year and day loops exist once instead of twice with slightly different guards. `None` on either side of the filter means all of them.

It sits on `Day` because a `Day` already carries its `Year`, so enumerating days is enumerating the pairs. `Day::all` iterates validated `Year`s rather than raw integers, which is why `days_in` can stay a method rather than a loose function taking an unchecked number. Each year and day is built straight from the range that bounds it, so the walk has no constructor to fail and nothing to `expect`.

## Timing

`solve` returns `Solved { parsed_in, part_one: Outcome, part_two: Outcome }`. Parsing is measured separately, since a slow day is often slow in one place or the other.

Both parts are computed and measured before any validation runs, so no duration includes a network round trip. Debug and release differ by roughly twenty times on 2015 day 1, which is the easiest sanity check that the numbers mean anything.

Parse time belongs to neither part. That's why `Solved` exists, rather than the durations hanging off the parts alone.

## Open

No day returns `Answer::Visual` yet. Tests cover its rendering, but nothing produces one in a real run.

## Tests

The code is mostly pure functions, so tests live beside what they test. The `Outcome` display matrix gets the most attention. It decides everything a user reads, and it had no coverage at all until it had six branches.

The store threads its root through private `_in` variants, so tests use a temporary directory instead of the real cache. An env var would have been simpler, but tests run in parallel and would fight over it.
