# Comment Guidelines

A comment says what the code does, in the present tense, and only when the code doesn't say it itself.

The code explains itself. A comment covers the part it can't: the surprise, the invariant, the unit.

A function with a comment on every line has the wrong names. Fix the names.

## Do not write

- History. No "previously", "used to", "now", "no longer", or "was changed to", and no dates, commit hashes, or issue numbers. The code has no past tense, and git has the history.
- The situation that led to the code. No "this is needed because we ran into" and no "this prevents the bug where". If the code only makes sense with the story attached, the code is wrong, not the comment.
- Sermons, like "without this, X would happen, and then Y". Say what the line does. If the consequence is the non-obvious part, give it one short clause.
- Restatements. `/// Returns the name.` above `fn name()` is noise, so delete it. When every item in a container explains itself, put `#[allow(missing_docs)]` on the container.
- Advice, unless it's an instruction the reader must follow, like "loopback only, do not widen".

## Do write

- What a caller can rely on: invariants, guarantees, ordering.
- The surprise. A reply that looks wrong but is right, an argument that is ignored, a close that looks early.
- Units, ranges, and formats the type doesn't carry.

## Form

- Use one line when you can. A doc comment may run longer, with an example when the signature can't say it, and then it stops.
- Open every module with a one-line `//!` header.
- Give every public item a doc comment, unless it explains itself.
- Keep section banners to one line: `// == errors ==`.
- Never use em dashes, in code or in prose. Don't join two fragments with a spaced hyphen either. Use a sentence, a colon, or a comma.
- Use American spelling.

## Before and after

```rust
// Bad: a past the reader does not need, and a sermon.
// Previously we kept the connection open after an error reply, but this caused
// a desync bug (see commit 3f2a9c1) where the next command read a stale error.
// Without this fix the cache would return wrong values.

// Good: what it does, and the one fact that is not obvious.
/// Puts `connection` back for the next command, unless the reply is an error.
///
/// A server that rejects a command partway through reading it answers the rest
/// as new commands, and those answers would be taken for the next replies.
```

```rust
// Bad: says nothing the field does not.
/// The additions.
pub additions: Option<u64>,

// Good: says what the number is and when it is absent.
/// Lines added across all commits, every rewrite counted again. `None` while
/// GitHub is still computing contributor statistics.
pub additions: Option<u64>,
```

## Checking

Search for the banned phrasings:

```bash
grep -rnE 'previously|used to|no longer|without this|in order to|—' src
```

A hit isn't always wrong, since "used to" also means "used for". Read each one.

Then reread every comment you wrote and delete the ones a competent reader wouldn't miss.
