use std::cmp::Ordering;
use std::fmt::{self, Display};

/// What the third-party solver made of an answer.
///
/// Repeatable, having no accounts and no memory, which is what makes it usable
/// as a gate before submitting.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Display for SolverVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Correct => write!(f, "correct"),
            Self::Incorrect => write!(f, "incorrect"),
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
            Self::Unsupported => write!(f, "unsupported"),
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
