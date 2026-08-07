# context (read me first)

Hand this dir to any AI assistant to resume work on `rustmas` with full context.

This branch is the tool itself, with no solutions and none of the owner's
working notes. What survives is the reasoning behind the design and the two
service contracts, both of which are yours to build on. scadoshi's solutions,
journal, and todo live on the `scadoshi` branch.

## Where things are

- [`design/`](design/) is why the code looks the way it does, including the
  options that were rejected and the reasons.
- [`references.md`](references.md) covers external things we depend on, where to
  clone them, when they were last checked, and the contracts we rely on.
- [`rules/commit_guidelines.md`](rules/commit_guidelines.md) is scadoshi's, and
  worth keeping or replacing with your own.

Add to `design/` whenever a decision gets made, even if no code changed. The
notes here are most useful when they record what was rejected and why, since
that is the part nobody can recover from reading the code.

## Working in this repo

Sections describing who the owner is and how they like to be worked with live on
the `scadoshi` branch, since they describe someone else. Write your own if you
want an assistant to follow them.

One habit worth keeping regardless: verify claims rather than asserting them.
Several decisions recorded in `design/` changed because something got probed or
read rather than assumed. The solver's error contract was guessed wrong twice
before anyone ran a curl against it, and the day page structure was confirmed
across three pages before anything was built on it.

## What rustmas is

Advent of Code tooling in Rust. One binary with a subcommand per mode, over a
library arranged as ports and adapters: `domain` knows nothing about the network
or the filesystem, `inbound` is how a request arrives, `outbound` is how it
leaves.

`fetch` downloads puzzle inputs and instructions into `cache/<year>/<NN>/`, one
directory of plain files per day.

`solve` runs the solutions, fetching any input it does not have. `--validate`
checks each answer against a third-party solver, and `--submit` posts them to
adventofcode.com for stars, sending only what the solver agreed with.

Both subcommands take `-y`/`--year` and `-d`/`--day`. Omitting a flag means all
of them, so they filter rather than look up.

The library holds the validated `Year`, `Day`, and `Part` types, the two HTTP
clients, and the solutions. `AocClient` and `SolverClient` are separate on
purpose: the first is authenticated and grades once, the second is anonymous and
repeatable.

## Do not

Read, cat, or print `.env`. It holds the personal session cookie.
