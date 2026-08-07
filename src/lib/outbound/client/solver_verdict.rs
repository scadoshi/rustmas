use std::{cmp::Ordering, fmt::Display};

/// What the third-party solver made of an answer.
///
/// Repeatable, since the solver has no accounts and no memory, which is what
/// makes it usable as a gate before submitting.
#[derive(Debug)]
pub enum SolverVerdict {
    Correct,
    Incorrect,
    Low,
    High,
    /// No implementation for that puzzle.
    Unsupported,
}

impl From<Ordering> for SolverVerdict {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Equal => Self::Correct,
            Ordering::Less => Self::Low,
            Ordering::Greater => Self::High,
        }
    }
}

impl From<bool> for SolverVerdict {
    fn from(value: bool) -> Self {
        if value {
            Self::Correct
        } else {
            Self::Incorrect
        }
    }
}

impl Display for SolverVerdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Correct => write!(f, "correct"),
            Self::Incorrect => write!(f, "incorrect"),
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
            Self::Unsupported => write!(f, "unsupported"),
        }
    }
}
