use std::cmp::Ordering;

#[derive(Debug)]
pub enum Verdict {
    Correct,
    Incorrect,
    TooLow,
    TooHigh,
    Unsupported,
}

impl From<Ordering> for Verdict {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Equal => Self::Correct,
            Ordering::Less => Self::TooLow,
            Ordering::Greater => Self::TooHigh,
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
