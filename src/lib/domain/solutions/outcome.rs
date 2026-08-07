use crate::{
    domain::solutions::answer::Answer,
    outbound::client::{aoc_verdict::AocVerdict, solver_verdict::SolverVerdict},
};
use std::{fmt::Display, time::Duration};

/// One part's answer and everything learned about it afterwards.
///
/// The three fields have three different sources: [`Answer`] is computed,
/// `elapsed` is measured, and the verdicts arrive over the network. Only
/// [`Answer::Value`] can carry a verdict, which the attaching methods enforce.
#[derive(Debug)]
pub struct Outcome {
    answer: Answer,
    /// Time to compute the answer. Never includes a network round trip.
    elapsed: Duration,
    /// From the third-party solver. Repeatable, so it gates submission.
    verdict: Option<SolverVerdict>,
    /// From adventofcode.com. Says whether the star exists.
    submission: Option<AocVerdict>,
}

impl Outcome {
    pub fn new(answer: Answer, elapsed: Duration) -> Self {
        Self {
            answer,
            elapsed,
            verdict: None,
            submission: None,
        }
    }

    /// Attaches a solver verdict, ignored unless there is something to check.
    pub fn with_verdict(mut self, verdict: SolverVerdict) -> Self {
        if self.answer.value().is_some() {
            self.verdict = Some(verdict);
        }
        self
    }

    /// Attaches what AOC said, ignored unless there is something to submit.
    pub fn with_submission(mut self, submission: AocVerdict) -> Self {
        if self.answer.value().is_some() {
            self.submission = Some(submission);
        }
        self
    }

    pub fn answer(&self) -> &Answer {
        &self.answer
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// The submittable text, if there is any.
    pub fn value(&self) -> Option<&str> {
        self.answer.value()
    }

    pub fn verdict(&self) -> Option<&SolverVerdict> {
        self.verdict.as_ref()
    }

    pub fn submission(&self) -> Option<&AocVerdict> {
        self.submission.as_ref()
    }
}

impl Display for Outcome {
    /// The answer, then whatever is known about it, then how long it took, so a
    /// part is always one line.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.answer)?;

        // AOC's word supersedes the solver's, so a starred part reads as starred
        // rather than repeating that the solver agreed.
        let notes: Vec<String> = match (&self.verdict, &self.submission) {
            (_, Some(AocVerdict::Correct)) => vec!["new star".to_string()],
            (_, Some(AocVerdict::AlreadySolved)) => vec!["starred".to_string()],
            (Some(v), Some(s)) => vec![v.to_string(), s.to_string()],
            (Some(v), None) => vec![v.to_string()],
            (None, Some(s)) => vec![s.to_string()],
            (None, None) => vec![],
        };
        if !notes.is_empty() {
            write!(f, " ({})", notes.join(", "))?;
        }

        write!(f, " [{:?}]", self.elapsed)
    }
}
