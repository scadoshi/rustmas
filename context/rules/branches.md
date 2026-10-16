## Branches

```
main        the tool, no solutions, no personal notes   <- default, what people clone
scadoshi    main plus solutions and working notes       <- where the work happens
```

`main` is what a stranger gets from `git clone`. `scadoshi` is `main` plus the solutions, the journal, the todo, the rules, and the owner-specific parts of `context/README.md`.

### Keeping them in step

Changes are additive and go one direction. A change to the tool lands on `main` first, then merges down:

```
git checkout scadoshi
git merge main
```

Nothing gets deleted to produce a branch. So there is no drift to catch, no force push, and nobody who cloned `main` has their history rewritten.

Check what a merge did before you trust it:

```
git diff main scadoshi --name-status
```

The list should show the solutions as added, plus `README.md`, `context/README.md`, `context/progress/`, `context/todo.md`, and `context/rules/`. If the solutions are missing from it, the merge removed them.

The cost is discipline. A tool fix you notice while writing a day belongs on `main`, not here. Commit it here and you either merge it up later or live with the divergence.

### The one merge that misbehaved

`main` was first carved out of this branch by deleting things, so its history contains those deletions. The first `git merge main` replayed them, and took the solutions and the owner-specific parts of `context/README.md` with it.

The fix restored both from the pre-merge commit, inside the merge itself.

It can't happen again. That merge commit is now the shared base, so a later merge only carries what `main` changed since. You only need to know this if the branches ever get re-cut, which would bring the problem back.

### Why not the other way

`main` used to be produced by deleting solutions from `master`. Every time, that took a runbook of removals, a force push, and a drift check.

It drifted anyway. The derived branch kept a stale `Solution::new(&str)` in its design notes for several commits after the signature changed. Deriving by subtraction makes drift the default, and correctness becomes the thing you have to work at.

### What belongs where

Only on `scadoshi`:

- `src/lib/domain/solution/year_YYYY/` and their `pub mod year_YYYY;` lines. This excludes `year_template/`, which is tooling for writing solutions rather than a solution, and which a fresh clone of `main` should have.
- The contents of `src/lib/domain/solution/common/`, but not the module itself. `main` ships the directory and an empty `mod.rs`, so a fresh clone has a place for shared helpers and a note saying what earns a spot there. The types themselves are solution code, so they live here.

  So `common/mod.rs` differs between branches and gets the same edit by hand on each side, like `solution/mod.rs` and `inbound/solve/mod.rs`. A merge conflicts on it every time `main` touches it. That's the price of the directory being scaffolding while its contents are not.
- The arms in `solver_for`, which lives in `src/lib/inbound/solve/mod.rs`.
- `context/progress/`, `context/todo.md`, and `context/rules/`, this file included.
- The "Who" and "How to work with him" sections of `context/README.md`.

On `main`, and therefore on both: everything else, including `context/design/` and `context/references.md`.

Those two are worth inheriting. `design/` records why the code has its shape, including rejected options. `references.md` holds both service contracts, which took real probing to establish.
