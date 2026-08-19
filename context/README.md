# context

Facts about the repository, for a person or an AI getting familiar with it. It
explains the repo and assumes nothing about who you are.

- [`architecture.md`](architecture.md) is the layout, how a solve runs, and the
  cache on disk.
- [`design/`](design/) is why the code looks the way it does, including the
  options that were rejected and the reasons.
- [`references.md`](references.md) covers external things this depends on, where
  to clone them, when they were last checked, and the contracts relied on.

Add to `design/` whenever a decision gets made, even if no code changed. The
notes are most useful when they record what was rejected and why, since that is
the part nobody can recover from reading the code.

## Branches

`main` is the tool with no solutions: what you clone to start. Solutions and
their registry arms live on a personal branch layered on top, with changes
flowing one way by merging `main` down. The `scadoshi` branch is Scotty's, with
his solutions and working notes, if you want worked examples.

One habit worth keeping regardless of whose branch you are on: verify claims
rather than asserting them.
Several decisions recorded in `design/` changed because something got probed or
read rather than assumed. The solver's error contract was guessed wrong twice
before anyone ran a curl against it, and the day page structure was confirmed
across three pages before anything was built on it.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
