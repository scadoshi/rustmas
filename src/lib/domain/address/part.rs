//! Which of a day's two puzzles.

use std::fmt::Display;

/// Named so call sites read `Part::One` rather than a `1` that could be a day.
#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Part {
    /// `1` or `2`, for AOC's `level` field and the solver's path.
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::One => "1",
            Self::Two => "2",
        }
    }
}

impl Display for Part {
    /// The word, as the output reads it. [`Part::wire_value`] is the one to
    /// send to a service.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "one"),
            Self::Two => write!(f, "two"),
        }
    }
}
