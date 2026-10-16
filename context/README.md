# context

Facts about the repository, for a person or an AI getting familiar with it. It explains the repo and assumes nothing about who you are.

- [`architecture.md`](architecture.md) is the layout, how a solve runs, and the cache on disk.
- [`design/`](design/) is why the code looks the way it does, including the options that were rejected and the reasons.
- [`references.md`](references.md) covers external things this depends on, where to clone them, when they were last checked, and the contracts relied on.

Add to `design/` whenever a decision gets made, even if no code changed. Record what was rejected and why. Nobody can recover that part from reading the code.

## Branches

`main` is the tool with no solutions. Clone it to start.

- Solutions and their registry arms live on a personal branch layered on top of `main`.
- Changes flow one way: merge `main` down into the personal branch.
- The `scadoshi` branch is Scotty's, with his solutions and working notes, if you want worked examples.

Whoever's branch you're on, verify claims rather than asserting them. Several decisions in `design/` changed because someone probed or read something instead of assuming it:

- The solver's error contract was guessed wrong twice before anyone ran a curl against it.
- The day page structure was confirmed across three pages before anything was built on it.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
