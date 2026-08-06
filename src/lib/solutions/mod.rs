use crate::{day::Day, part::Part, session::Session, session::verdict::Verdict};
use std::fmt::Display;

pub mod year_2015;

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

pub trait Solution: Sized {
    fn new(input: &'static str) -> anyhow::Result<Self>;
    fn input(&self) -> &str;
    fn part_one(&self) -> Answer;
    fn part_two(&self) -> Answer;
}

/// Runs both parts of a solution, validating each answer when given a session.
///
/// `session` doubles as the validate flag: `None` means solve offline and skip
/// the network entirely. Only [`Answer::Value`] is ever validated, since a
/// visual or absent answer has nothing to check.
pub fn solve<S: Solution>(
    session: Option<&Session>,
    input: &'static str,
    day: &Day,
) -> anyhow::Result<(Answer, Answer)> {
    let solution = S::new(input)?;

    let mut one = solution.part_one();
    let mut two = solution.part_two();

    if let Some(session) = session {
        if let Some(value) = one.value() {
            let verdict = session.validate_answer(day, input, Part::One, value)?;
            one = one.with_verdict(verdict);
        }
        if let Some(value) = two.value() {
            let verdict = session.validate_answer(day, input, Part::Two, value)?;
            two = two.with_verdict(verdict);
        }
    }

    Ok((one, two))
}
