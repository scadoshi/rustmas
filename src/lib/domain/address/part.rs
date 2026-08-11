//! Which of a day's two puzzles.

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
