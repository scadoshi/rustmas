# context

Facts about the repository, plus who owns this branch and how he works. The repo facts are one file per topic and are shared with `main`. The personal sections exist only here.

- [`todo.md`](todo.md) is what's coming next. Read it first.
- [`progress/journal.md`](progress/journal.md) holds dated session logs, newest first.
- [`rules/`](rules/) is binding when you work here: commit guidelines, comment guidelines, doc comment style, and the branch model. It lives on this branch only, since the rules are the owner's.
- [`architecture.md`](architecture.md) covers the layout, how a solve runs, and the cache on disk.
- [`design/`](design/) explains why the code looks the way it does, including the rejected options and the reasons.
- [`references.md`](references.md) covers the external things this depends on: where to clone them, when they were last checked, and the contracts relied on.

Add to `design/` whenever a decision gets made, even if no code changed. Record what was rejected and why. Nobody can recover that part from reading the code.

## Branches

`main` is the tool with no solutions, the thing you clone to start. Solutions and their registry arms live on a personal branch layered on top.

Changes flow one way, by merging `main` down. The `scadoshi` branch is Scotty's, with his solutions and working notes, if you want worked examples.

Whoever's branch you're on, keep one habit: verify claims instead of asserting them. Several decisions in `design/` changed because someone probed or read something instead of assuming.

- The solver's error contract was guessed wrong twice before anyone ran a curl against it.
- The day page structure was confirmed across three pages before anything was built on it.

## Who

scadoshi (Scotty) is a strong Rust developer. He's deep on ownership, traits, error-as-values, and making illegal states unrepresentable.

Rust is his home territory. The sibling repo sharpmas is where he's learning C#.

## How to work with him

He streams ideas and half-formed designs. Your job:

- Correct what's wrong.
- Briefly confirm what's right.
- Extend with a question.

Don't write novels. Keep replies short and skip the emojis.

Avoid AI-tell prose. In particular, never join two fragments with a dash, in code comments or in conversation. Write real sentences.

When he asks for implementation, write the code. When he's still thinking out loud, coach and nudge instead of jumping to code.

Verify claims instead of asserting them. Several decisions in `design/` changed because something got probed or read rather than assumed.

At the end of a working session, update `todo.md` and add a journal entry.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
