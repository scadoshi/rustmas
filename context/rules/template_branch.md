## The template branch

`template` is `master` with the solutions removed, for anyone who wants the tool
without someone else's answers. It carries the full history, which nobody
cloning a starting point is likely to read.

Do not maintain it in parallel. Every fix applied to both branches by hand is a
chance for them to drift, and they have: `template` once kept a stale
`Solution::new(&str)` in its design notes for several commits after `master`
changed the signature. Delete it and recut instead.

### Recutting

```
git branch -D template
git checkout -b template master
```

Then strip the solutions:

- `rm -rf src/lib/domain/solutions/year_*`
- Drop the `pub mod year_*;` lines from `src/lib/domain/solutions/mod.rs`
- In `src/lib/inbound/solve/run.rs`, empty the registry and drop the imports it
  used:

```rust
// Empty until the first day is written, so the match has one arm.
#[allow(clippy::match_single_binding)]
fn solver_for(year: i32, day: i32) -> Option<Solver> {
    match (year, day) {
        // One arm per day. Import the year module and `solve` at the top of this
        // file, then:
        //
        //     (2015, 1) => Some(solve::<year_2015::day_01::Puzzle>),
        _ => None,
    }
}
```

The import becomes `solutions::solution::Solved` alone, since nothing calls
`solve` any more. The `allow` is needed because a match with only a catch-all
is a lint.

Then reapply the three differences that are not deletions:

- `README.md` title becomes `# rustmas (template)`, followed by a short note
  saying the branch has no solutions and pointing at `master` for examples.
- `README.md` layout block lists `year_<year>/` rather than the real years.
- `context/README.md` gains a note saying which branch this is and that the
  journal belongs to someone else.

### Checking it

```
cargo build && cargo clippy --all-targets && cargo test
cargo run solve -y 2015 -d 1        # "year 2015 day 1 has no solution yet"
cargo run solve --submit            # no prompt, since the count is zero
```

Then confirm the delta is only what you expect:

```
git diff master template --stat
```

Solutions, `solutions/mod.rs`, `solve/run.rs`, and the two README files. Anything
else means something drifted or the strip was incomplete.

### Publishing

The branch is rewritten rather than advanced, so it needs a force push. Scotty
pushes, not the assistant.
