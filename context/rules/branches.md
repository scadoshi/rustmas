## Branches

```
main        the tool, no solutions, no personal notes   <- default, what people clone
scadoshi    main plus solutions and working notes       <- where the work happens
```

`main` is what a stranger gets from `git clone`. `scadoshi` is `main` with the
solutions, the journal, the todo, and the owner-specific parts of
`context/README.md` added on top.

### Keeping them in step

Additive, in one direction:

```
git checkout scadoshi
git merge main
```

A change to the tool lands on `main` first, then merges down. Nothing is deleted
to produce a branch, so there is no drift to catch, no force push, and nobody who
cloned `main` gets their history rewritten.

Check what a merge did before trusting it:

```
git diff main scadoshi --name-status
```

It should list the solutions as added, plus `README.md`, `context/README.md`,
`context/progress/`, `context/todo.md`, and this file. If the solutions are
missing from that list, the merge removed them.

The cost is discipline: a tool fix noticed while writing a day belongs on `main`,
not here. Committing it here means merging it up later or living with the
divergence.

### The one merge that misbehaved

`main` was originally carved out of this branch by deleting things, so its
history contains those deletions. The first `git merge main` replayed them and
took the solutions and the owner-specific parts of `context/README.md` with it.

That was fixed by restoring both from the pre-merge commit inside the merge
itself. It cannot happen again: the merge commit is now the shared base, so a
later merge only carries what `main` has changed since. This is worth knowing
only if the branches are ever re-cut, which would reintroduce it.

### Why not the other way

`main` used to be produced by deleting solutions from `master`, which needed a
runbook of removals, a force push, and a drift check every time. It drifted
anyway: the derived branch kept a stale `Solution::new(&str)` in its design notes
for several commits after the signature changed. Deriving by subtraction makes
drift the default and correctness the thing you have to work at.

### What belongs where

Only on `scadoshi`:

- `src/lib/domain/solution/year_YYYY/` and their `pub mod year_YYYY;` lines.
  Not `year_template/`, which is tooling for writing solutions rather than a
  solution, and which a fresh clone of `main` should have.
- the arms in `solver_for`
- `context/progress/`, `context/todo.md`, and this file
- the "Who" and "How to work with him" sections of `context/README.md`

On `main`, and therefore on both:

- everything else, including `context/design/` and `context/references.md`

`design/` records why the code has its shape, including rejected options.
`references.md` holds both service contracts, which took real probing to
establish. Those are worth inheriting.
