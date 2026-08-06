use crate::session::verdict::Verdict;
use std::fmt::Display;

/// What one part of a puzzle produced.
///
/// The verdict sits on the submittable variant rather than beside the enum, so
/// a visual answer carrying one is unrepresentable.
#[derive(Debug)]
pub enum Answer {
    /// A submittable answer, plus what a checker made of it if asked.
    Value {
        value: String,
        verdict: Option<Verdict>,
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
        }
    }

    /// Attaches a verdict. Only lands on [`Answer::Value`].
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
