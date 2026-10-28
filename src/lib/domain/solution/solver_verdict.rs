//! What the third-party solver made of an answer.

use std::cmp::Ordering;
use std::fmt::{self, Display};

/// What the third-party solver made of an answer.
///
/// Repeatable, having no accounts and no memory, which is what makes it usable
/// as a gate before submitting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverVerdict {
    /// Matches the solver's answer.
    Correct,
    /// Differs from the solver's answer, where the two are not both numbers.
    Incorrect,
    /// A number below the solver's.
    Low,
    /// A number above the solver's.
    High,
    /// No implementation for that puzzle.
    Unsupported,
    /// The check itself failed, so nothing was graded. Holds why.
    Unchecked(String),
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

impl Display for SolverVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Correct => write!(f, "correct"),
            Self::Incorrect => write!(f, "incorrect"),
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::Unchecked(reason) => write!(f, "unchecked: {reason}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Comparing our answer against the solver's, so `Less` means ours was low.
    #[test]
    fn ordering_reads_from_our_side() {
        assert!(matches!(
            SolverVerdict::from(Ordering::Less),
            SolverVerdict::Low
        ));
        assert!(matches!(
            SolverVerdict::from(Ordering::Greater),
            SolverVerdict::High
        ));
        assert!(matches!(
            SolverVerdict::from(Ordering::Equal),
            SolverVerdict::Correct
        ));
    }
}
