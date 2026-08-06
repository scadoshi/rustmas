//! Which of a day's two puzzles.

/// Names a part so call sites read `Part::One` rather than a bare `1` that
/// could be mistaken for a day number.
#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Part {
    /// `1` or `2`, wanted by both AOC's `level` field and the solver's path.
    pub fn to_wire_value(&self) -> u32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }
}
