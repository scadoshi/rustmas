## Commit Guidelines

- Keep messages to one concise line. Use multiple lines only when there are many changes.
- Group related files logically, one concern per commit.
- No emojis.
- Read `git diff` to understand the changes before committing.
- Never include AI-agent signatures in a commit. Something like "Written with the help of Claude Opus 4.5" never goes in the message.

### Check before committing, not after

Run these in order:

1. `cargo test`
2. `cargo build`, for warnings
3. `cargo clippy --all-targets`
4. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`, the same flags CI uses, so a dangling link fails here first

Each one sees what the others miss. Clippy reads past a clean build. The build never checks intra-doc links, so a rename compiles fine while ``[`Type`]`` links point at nothing.

Format with `cargo +nightly fmt`. `rustfmt.toml` sets four unstable options, `imports_granularity = "Module"` (one `use` line per module), `group_imports = "StdExternalCrate"` (std, then external crates, then this crate, blank line between groups), `hex_literal_case = "Lower"` and `format_macro_matchers = true`, and stable rustfmt ignores all of them.

CI in `.github/workflows/ci.yml` runs the same checks on every push to `main` or `scadoshi`, with warnings as errors. A red run means one of these got skipped, so run them locally first instead of letting CI find it.

If a change touches instructions, follow them instead of just reading them. The README's "Writing a day" steps have gone stale three times, twice while someone was looking straight at them.

### Committing across both branches

Tool changes land on `main` first, then merge down. See [`branches.md`](branches.md) for what belongs where.

Work in a worktree instead of switching branches, so uncommitted work stays where it is:

```sh
git worktree add /tmp/wt main
# copy the changed files in, commit there
git worktree remove /tmp/wt
```

Only copy files that are identical on both branches. `git diff main scadoshi --name-only` lists the ones that aren't, and those get the same edit by hand on each side. Today that's `README.md`, `context/README.md`, `solution/mod.rs`, `solution/common/mod.rs`, and `inbound/solve/mod.rs`.

### Never leave a commit that does not build

A change on `main` can't know the days exist. If it breaks them, the merge commit is left broken.

Fold their fixes into the merge instead of committing after it:

```sh
git merge --no-commit --no-ff main
# fix the days, then commit once
```
