# context

Facts about the repository, plus who owns this branch and how he works. The
repo facts are one file per topic, shared with `main`; the personal sections
exist only here.

- [`todo.md`](todo.md) is what is coming next. Read it first.
- [`progress/journal.md`](progress/journal.md) is dated session logs, newest
  first.
- [`rules/`](rules/) is binding when working here: commit guidelines, doc
  comment style, and the branch model. On this branch only, since the rules are
  the owner's.
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

## Who

scadoshi (Scotty) is a strong Rust developer, deep on ownership, traits,
error-as-values, and making illegal states unrepresentable. This is his home
territory; the sibling repo sharpmas is where C# is being learned.

## How to work with him

He streams ideas and half-formed designs. Your job is to correct what's wrong,
briefly confirm what's right, and extend with a question. Don't write novels.
Keep replies short and skip the emojis.

Avoid AI-tell prose. In particular, never join two fragments with a dash, in
either code comments or conversation. Write real sentences.

When he asks for implementation, write the code. When he's still thinking out
loud, coach and nudge instead of jumping to code.

Verify claims rather than asserting them. Several decisions in `design/` changed
because something got probed or read rather than assumed.

Update `todo.md` and add a journal entry at the end of a working session.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
