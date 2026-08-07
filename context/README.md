# context (read me first)

Hand this dir to any AI assistant to resume work on `rustmas` with full context.

## Where things are

- [`todo.md`](todo.md) is what is coming next and what is already done.
- [`progress/journal.md`](progress/journal.md) is dated session logs, newest
  first.
- [`design/`](design/) is why the code looks the way it does, including the
  options that were rejected and the reasons.
- [`references.md`](references.md) covers external things we depend on, where to
  clone them, when they were last checked, and the contracts we rely on.
- [`rules/commit_guidelines.md`](rules/commit_guidelines.md) is binding for any
  commit.

Update `todo.md` and add a journal entry at the end of a working session. Add to
`design/` whenever a decision gets made, even if no code changed.

## Who

scadoshi (Scotty) is a strong Rust developer. He's deep on ownership, traits,
error-as-values, and making illegal states unrepresentable, so skip beginner
Rust explanations.

## How to work with him

He streams ideas and half-formed designs. Your job is to correct what's wrong,
briefly confirm what's right, and extend with a question. Don't write novels.
Keep replies short and skip the emojis.

Avoid AI-tell prose. In particular, never join two fragments with a dash, in
either code comments or conversation. Write real sentences.

When he asks for implementation, write the code. When he's still thinking out
loud, coach and nudge instead of jumping to code.

Verify claims rather than asserting them. Several decisions here changed because
something got probed or read rather than assumed. The solver's error contract
was guessed wrong twice before anyone ran a curl against it.

## What rustmas is

Advent of Code tooling in Rust. Two binaries over one shared library.

`fetch` downloads puzzle inputs to `inputs/<year>/<NN>.txt`.

`solve` runs the solutions, and with `--validate` checks each answer against a
third-party solver. `--submit` posts them to adventofcode.com for stars, sending
only what the solver agreed with.

Both binaries take `-y`/`--year` and `-d`/`--day`. Omitting a flag means all of
them, so they filter rather than look up.

The library holds the validated `Year`, `Day`, and `Part` types, the two HTTP
clients, and the solutions. `AocClient` and `SolverClient` are separate on
purpose: the first is authenticated and grades once, the second is anonymous and
repeatable.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
