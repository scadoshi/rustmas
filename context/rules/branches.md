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

A change to the tool lands on `main` first, then merges down. Nothing is ever
deleted to produce a branch, so there is no drift to catch, no force push, and
nobody who cloned `main` gets their history rewritten.

The cost is discipline: a tool fix noticed while writing a day belongs on `main`,
not here. Committing it here means merging it up later or living with the
divergence.

### Why not the other way

`main` used to be produced by deleting solutions from `master`, which needed a
runbook of removals, a force push, and a drift check every time. It drifted
anyway: the derived branch kept a stale `Solution::new(&str)` in its design notes
for several commits after the signature changed. Deriving by subtraction makes
drift the default and correctness the thing you have to work at.

### What belongs where

Only on `scadoshi`:

- `src/lib/domain/solutions/year_*/` and the `pub mod year_*;` lines
- the arms in `solver_for`
- `context/progress/`, `context/todo.md`, and this file
- the "Who" and "How to work with him" sections of `context/README.md`

On `main`, and therefore on both:

- everything else, including `context/design/` and `context/references.md`

`design/` records why the code has its shape, including rejected options.
`references.md` holds both service contracts, which took real probing to
establish. Those are worth inheriting.
