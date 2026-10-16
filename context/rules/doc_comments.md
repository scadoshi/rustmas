## Doc Comments

This is the shape of a doc comment. What a comment may say at all is in [comment_guidelines.md](comment_guidelines.md).

Write one line. Go longer only when a reader would otherwise get it wrong.

```rust
/// The submittable text, if there is any.
pub fn value(&self) -> Option<&str> {
```

Aim for that shape: what it is or what it gives you, in one sentence, with no ceremony. Most items in this repo need nothing more.

### When a second paragraph earns its place

Add one only for something the signature can't say and a reader would otherwise get wrong:

- A decision that looks like a mistake until explained. `Solution` is `Sized`, which forecloses `dyn`. Without a line saying that's deliberate, the next person tries to "fix" it.
- A rejected alternative that will otherwise get proposed again.
- A trap: panics, clamping, an argument that is ignored, an ordering that matters.
- A cross-reference that saves a search, like ``[`Cell`]`` from `Point`, since the first question about either is which one to use.

If the extra paragraph only restates the signature in prose, cut it.

### What not to write

- No `# Examples`, `# Panics`, `# Errors`, or `# Arguments` headers. This is a puzzle runner, not a published crate, so say it in the sentence.
- Don't restate the name. `/// Creates a new Point.` on `Point::new` is noise.
- Don't document the obvious accessor. `elapsed()` returning `elapsed` needs nothing.
- Don't repeat a parent's doc on every child. Say it once, on the type or the module.
- No history. Describe the code as it is, never how it got there.

The history examples all belong in the journal: "Used to take a `String`", "this used to parse in `part_one`", "the old version clobbered the file". A reader arriving cold needs to know what the code does now. What it used to do only gives them a second thing to hold.

### Where the long-form goes instead

The repo already has better homes for anything that won't fit:

- `context/design/` for why the code has its shape, including rejected options.
- `context/progress/journal.md` for what happened and when.
- `context/references.md` for external contracts.

A doc comment growing into an essay is usually a design note trying to escape. Move it, and leave a line pointing at it.

### Module docs

`//!` gets one line saying what lives in the module and why those things are together. The same rule applies: go longer only when the grouping isn't obvious.

### Tests

A test name should carry the intent, so most tests need no doc at all. Write one only when the test encodes a decision the assertions don't show, such as why `unsigned_abs` is used instead of `abs`.
