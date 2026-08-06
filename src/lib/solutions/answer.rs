use crate::session::verdict::Verdict;
use std::fmt::Display;

/// What one part of a puzzle produced.
///
/// A verdict only makes sense for a value that could be submitted, so it lives
/// on that variant rather than alongside the enum. A visual answer carrying a
/// verdict is unrepresentable.
#[derive(Debug)]
pub enum Answer {
    /// A submittable answer, plus what a checker made of it if asked.
    Value {
        value: String,
        verdict: Option<Verdict>,
    },
    /// Something you read rather than submit, such as ASCII art. The solution
    /// hands it back instead of printing it, so solving stays free of IO.
    Visual(String),
    /// Nothing to produce. Day 25 part two is the usual case.
    None,
}

impl Answer {
    /// A submittable answer, not yet checked. This is what a day returns.
    pub fn solved(value: impl Into<String>) -> Self {
        Self::Value {
            value: value.into(),
            verdict: None,
        }
    }

    /// Attaches a verdict, which only lands on [`Answer::Value`].
    pub fn with_verdict(self, verdict: Verdict) -> Self {
        match self {
            Self::Value { value, .. } => Self::Value {
                value,
                verdict: Some(verdict),
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
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value {
                value,
                verdict: Some(verdict),
            } => write!(f, "{value} ({verdict:?})"),
            Self::Value {
                value,
                verdict: None,
            } => write!(f, "{value}"),
            Self::Visual(art) => write!(f, "\n{art}"),
            Self::None => write!(f, "(none)"),
        }
    }
}
