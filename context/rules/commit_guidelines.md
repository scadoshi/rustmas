## Commit Guidelines

- Concise, one-line messages (multi-line only when many changes)
- Group related files logically, one concern per commit
- No emojis
- Use `git diff` to understand changes before committing
- **Never** include AI-agent signatures in your commits.
    - Example: "Written with the help of Claude Opus 4.5"
    - Never commit with something like this in your message.

### Check before committing, not after

`cargo test`, then `cargo build` for warnings, then `cargo doc --no-deps`. The
build never checks intra-doc links, so a rename compiles fine while `[`Type`]`
links point at nothing.

If a change touches instructions, follow them rather than reading them. The
README's "Adding a solution" steps have gone stale three times, twice while
someone was looking straight at them.

### Committing across both branches

Tool changes land on `main` first, then merge down. See
[`branches.md`](branches.md) for what belongs where.

Work in a worktree rather than switching branches, so uncommitted work stays
where it is:

```sh
git worktree add /tmp/wt main
# copy the changed files in, commit there
git worktree remove /tmp/wt
```

Only copy files that are identical on both branches. `git diff main scadoshi
--name-only` lists the ones that are not, and those get the same edit by hand on
each side. Today that is `README.md`, `context/README.md`,
`solution/mod.rs`, and `inbound/solve/run.rs`.

### Never leave a commit that does not build

A change on `main` cannot know the days exist, so one that breaks them would
leave the merge commit broken. Fold their fixes into the merge instead of
committing after it:

```sh
git merge --no-commit --no-ff main
# fix the days, then commit once
```
