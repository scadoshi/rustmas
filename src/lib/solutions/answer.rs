use crate::client::verdict::Verdict;
use std::fmt::Display;

/// What one part of a puzzle produced.
///
/// The verdicts sit on the submittable variant rather than beside the enum, so
/// a visual answer carrying one is unrepresentable.
#[derive(Debug)]
pub enum Answer {
    /// A submittable answer, plus what each checker made of it if asked.
    Value {
        value: String,
        /// From the third-party solver. Repeatable, so it gates submission.
        verdict: Option<Verdict>,
        /// From adventofcode.com. Says whether the star exists.
        submission: Option<Verdict>,
    },
    /// Art you read rather than submit. Handed back rather than printed, so
    /// solving stays free of IO.
    Visual(String),
    /// Nothing to produce. Day 25 part two is the usual case.
    None,
}

impl Answer {
    /// A submittable answer, not yet checked. What a day returns.
    pub fn solved(value: impl Into<String>) -> Self {
        Self::Value {
            value: value.into(),
            verdict: None,
            submission: None,
        }
    }

    /// Attaches a solver verdict. Only lands on [`Answer::Value`].
    pub fn with_verdict(self, verdict: Verdict) -> Self {
        match self {
            Self::Value {
                value, submission, ..
            } => Self::Value {
                value,
                verdict: Some(verdict),
                submission,
            },
            other => other,
        }
    }

    /// Attaches what AOC said about a submission. Only lands on
    /// [`Answer::Value`].
    pub fn with_submission(self, submission: Verdict) -> Self {
        match self {
            Self::Value {
                value, verdict, ..
            } => Self::Value {
                value,
                verdict,
                submission: Some(submission),
            },
            other => other,
        }
    }

    /// The submittable text, if there is any.
    pub fn value(&self) -> Option<&str> {
        match self {
            Self::Value { value, .. } => Some(value),
            _ => None,
        }
    }

    /// What the solver made of this answer, if it was asked.
    pub fn verdict(&self) -> Option<&Verdict> {
        match self {
            Self::Value { verdict, .. } => verdict.as_ref(),
            _ => None,
        }
    }

    /// What AOC said when this answer was submitted, if it was.
    pub fn submission(&self) -> Option<&Verdict> {
        match self {
            Self::Value { submission, .. } => submission.as_ref(),
            _ => None,
        }
    }
}

impl Display for Answer {
    /// Renders the value with whatever is known about it in one set of
    /// parentheses, so a part is always one line.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value {
                value,
                verdict,
                submission,
            } => {
                write!(f, "{value}")?;
                // AOC's word supersedes the solver's, so a starred part reads as
                // starred rather than repeating that the solver agreed.
                let notes: Vec<String> = match (verdict, submission) {
                    (_, Some(Verdict::Correct)) => vec!["new star".to_string()],
                    (_, Some(Verdict::AlreadySolved)) => vec!["starred".to_string()],
                    (Some(v), Some(s)) => vec![v.to_string(), s.to_string()],
                    (Some(v), None) => vec![v.to_string()],
                    (None, Some(s)) => vec![s.to_string()],
                    (None, None) => vec![],
                };
                if !notes.is_empty() {
                    write!(f, " ({})", notes.join(", "))?;
                }
                Ok(())
            }
            Self::Visual(art) => write!(f, "\n{art}"),
            Self::None => write!(f, "(none)"),
        }
    }
}
