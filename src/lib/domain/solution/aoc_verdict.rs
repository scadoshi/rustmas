use std::fmt::Display;

/// What adventofcode.com said about a submission.
///
/// Each part is graded exactly once, so a second correct answer comes back as
/// [`AocVerdict::AlreadySolved`] rather than another confirmation.
#[derive(Debug)]
pub enum AocVerdict {
    Correct,
    Incorrect,
    Low,
    High,
    /// Refused to grade because an answer was submitted too recently. Holds the
    /// remaining wait as AOC phrased it, such as `1m 0s`.
    Cooldown(String),
    /// The part is already solved, so nothing was graded.
    AlreadySolved,
}

impl Display for AocVerdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Correct => write!(f, "correct"),
            Self::Incorrect => write!(f, "incorrect"),
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
            Self::Cooldown(wait) => write!(f, "rate limited, {wait} left to wait"),
            Self::AlreadySolved => write!(f, "already solved"),
        }
    }
}
