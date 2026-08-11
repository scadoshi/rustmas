## Doc Comments

One line. Go longer only when a reader would otherwise get it wrong.

```rust
/// The submittable text, if there is any.
pub fn value(&self) -> Option<&str> {
```

That is the shape to aim for: what it is or what it gives you, in a sentence,
no ceremony. Most items in this repo need nothing more.

### When a second paragraph earns its place

Add one only for something the signature cannot say and a reader would
otherwise get wrong:

- A decision that looks like a mistake until explained. `Solution` is `Sized`,
  which forecloses `dyn`; without a line saying that is deliberate, the next
  person tries to "fix" it.
- A rejected alternative that will otherwise be proposed again.
- A trap. Panics, clamping, an argument that is ignored, an ordering that
  matters.
- A cross-reference that saves a search. `[`Cell`]` from `Point`, since the
  first question about either is which one to use.

If the extra paragraph only restates the signature in prose, cut it.

### What not to write

- No `# Examples`, `# Panics`, `# Errors`, `# Arguments` headers. This is a
  puzzle runner, not a published crate. Say it in the sentence.
- No restating the name. `/// Creates a new Point.` on `Point::new` is noise.
- No documenting the obvious accessor. `elapsed()` returning `elapsed` needs
  nothing.
- No repeating a parent's doc on every child. Say it once on the type or the
  module.
- No history. "Used to take a `String`" belongs in the journal, not the code.

### Where the long-form goes instead

The repo already has better homes for anything that will not fit:

- `context/design/` for why the code has its shape, including rejected options.
- `context/progress/journal.md` for what happened and when.
- `context/references.md` for external contracts.

A doc comment that is growing into an essay is usually a design note trying to
escape. Move it and leave a line pointing at it.

### Module docs

`//!` gets one line saying what lives in the module and why those things are
together. The same rule applies: longer only when the grouping is not obvious.

### Tests

A test name should carry the intent, so most tests need no doc at all. Write
one only when the test encodes a decision the assertions do not show, such as
why `unsigned_abs` is used instead of `abs`.
