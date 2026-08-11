use std::fmt::Display;

/// What one part of a puzzle produced. Nothing else.
///
/// Verdicts and timings come from elsewhere and live in
/// [`super::outcome::Outcome`].
#[derive(Debug)]
pub enum Answer {
    /// A submittable answer.
    Value(String),
    /// Art you read rather than submit. Returned, not printed, so solving
    /// does no IO.
    Visual(String),
    /// Nothing to produce. Day 25 part two is the usual case.
    None,
}

impl Answer {
    /// A submittable answer. What a day returns.
    pub fn solved(value: impl Into<String>) -> Self {
        Self::Value(value.into())
    }

    /// The submittable text, if there is any.
    pub fn value(&self) -> Option<&str> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value}"),
            Self::Visual(art) => write!(f, "\n{art}"),
            Self::None => write!(f, "(none)"),
        }
    }
}
