## The template branch

`template` is `master` with the solutions and personal notes removed, for anyone
who wants the tool without someone else's answers. It carries the full history,
which nobody cloning a starting point is likely to read.

Do not maintain it in parallel. Every fix applied to both branches by hand is a
chance for them to drift, and they have: `template` once kept a stale
`Solution::new(&str)` in its design notes for several commits after `master`
changed the signature. Delete it and recut instead.

### Recut

```
git branch -D template
git checkout -b template master
```

### Remove

Solutions:

- `src/lib/domain/solutions/year_*/`

Notes that belong to the owner rather than the tool:

- `context/progress/`
- `context/todo.md`
- `context/rules/template_branch.md`, since a cloner is not maintaining this
  branch

Everything else in `context/` stays. `design/` records why the code has its
shape, including rejected options, and `references.md` holds both service
contracts, which took real probing to establish. Those are the parts worth
inheriting.

### Update

`src/lib/domain/solutions/mod.rs`: drop the `pub mod year_*;` lines.

`src/lib/inbound/solve/run.rs`: empty the registry, and cut the import down to
`solutions::solution::Solved` since nothing calls `solve` any more.

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

The `allow` is needed because a match with only a catch-all is a lint.

`README.md`: title becomes `# rustmas (template)`, add a short note after the
opening paragraph saying the branch has no solutions and pointing at `master`
for worked examples, and change the layout block to list `year_<year>/` rather
than the real years.

`context/README.md`: replace the opening line about resuming work with a note
saying which branch this is and what was left behind. Drop the `todo.md` and
`progress/journal.md` entries from the index, and the line about updating them.
Replace the "Who" and "How to work with him" sections, which describe the owner
rather than the reader, with a short note saying they were dropped and that the
reader should write their own. Keep the point about verifying claims rather than
asserting them, since that habit shaped several of the decisions in `design/`.

### Check

```
cargo build && cargo clippy --all-targets && cargo test
cargo run solve -y 2015 -d 1        # "year 2015 day 1 has no solution yet"
cargo run solve --submit            # no prompt, since the count is zero
```

Then confirm the delta is only what you expect:

```
git diff master template --name-status
```

Four deleted solution files, three deleted note files, and four modified:
`README.md`, `context/README.md`, `solutions/mod.rs`, `solve/run.rs`. Anything
else means something drifted or the strip was incomplete.

Also grep the survivors for references to what you removed:

```
grep -rn "todo\|journal" context/
```

### Publish

The branch is rewritten rather than advanced, so it needs a force push. Scotty
pushes, not the assistant.
