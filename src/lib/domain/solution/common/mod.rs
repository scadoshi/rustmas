//! Helpers more than one day needs: positions, directions, and turns.
//!
//! Ships empty on `main`. Put a type here the second day wants it, not the
//! first: a helper written for one puzzle is a helper shaped by one puzzle.
//!
//! These are the shared types the testing rule is about. A break here corrupts
//! every day at once, so anything in this module earns tests, where a single
//! day's logic does not.

pub mod cell;
pub mod direction;
pub mod point;
pub mod turn;
