use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
pub enum Verdict {
    Correct,
    Incorrect,
    Low,
    High,
    Unsupported,
}

impl From<Ordering> for Verdict {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Equal => Self::Correct,
            Ordering::Less => Self::Low,
            Ordering::Greater => Self::High,
        }
    }
}

impl From<bool> for Verdict {
    fn from(value: bool) -> Self {
        if value {
            Self::Correct
        } else {
            Self::Incorrect
        }
    }
}

impl Display for Verdict {
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
