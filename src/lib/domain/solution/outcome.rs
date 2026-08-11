use crate::domain::solution::{
    answer::Answer, aoc_verdict::AocVerdict, solver_verdict::SolverVerdict,
};
use std::{fmt::Display, time::Duration};

/// One part's answer and everything learned about it afterwards.
///
/// The three fields have three different sources: [`Answer`] is computed,
/// `elapsed` is measured, and the verdicts arrive over the network. Only
/// [`Answer::Value`] can carry a verdict, which the attaching methods enforce.
///
/// A part that failed is held here rather than propagated, so one broken part
/// does not hide the other one's answer. An error has no value, so it can
/// never be validated or submitted, which falls out of the same
/// [`Outcome::value`] gate the other unsubmittable answers use.
#[derive(Debug)]
pub struct Outcome {
    answer: anyhow::Result<Answer>,
    /// Time to compute the answer. Never includes a network round trip.
    elapsed: Duration,
    /// From the third-party solver. Repeatable, so it gates submission.
    verdict: Option<SolverVerdict>,
    /// From adventofcode.com. Says whether the star exists.
    submission: Option<AocVerdict>,
}

impl Outcome {
    /// Takes whatever the part returned, error included.
    pub fn new(answer: anyhow::Result<Answer>, elapsed: Duration) -> Self {
        Self {
            answer,
            elapsed,
            verdict: None,
            submission: None,
        }
    }

    /// Attaches a solver verdict, ignored unless there is something to check.
    pub fn with_verdict(mut self, verdict: SolverVerdict) -> Self {
        if self.value().is_some() {
            self.verdict = Some(verdict);
        }
        self
    }

    /// Attaches what AOC said, ignored unless there is something to submit.
    pub fn with_submission(mut self, submission: AocVerdict) -> Self {
        if self.value().is_some() {
            self.submission = Some(submission);
        }
        self
    }

    /// What the part produced, or why it could not.
    pub fn answer(&self) -> Result<&Answer, &anyhow::Error> {
        self.answer.as_ref()
    }

    /// Why the part failed, if it did.
    pub fn error(&self) -> Option<&anyhow::Error> {
        self.answer.as_ref().err()
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// The submittable text, if there is any. A failed part has none.
    pub fn value(&self) -> Option<&str> {
        self.answer.as_ref().ok()?.value()
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
    /// part is always one line. A failed part reads as its error chain, still
    /// on one line, still timed.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.answer {
            Ok(answer) => write!(f, "{answer}")?,
            // `{:#}` puts the whole chain on the line rather than just the
            // outermost message.
            Err(e) => write!(f, "error: {e:#}")?,
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn value() -> Outcome {
        Outcome::new(Ok(Answer::solved("138")), Duration::from_micros(7))
    }

    /// The timing suffix is asserted separately, so these compare the part
    /// before it.
    fn notes(outcome: &Outcome) -> String {
        let rendered = outcome.to_string();
        rendered
            .split_once(" [")
            .map(|(before, _)| before.to_string())
            .unwrap_or(rendered)
    }

    #[test]
    fn bare_answer_has_no_notes() {
        assert_eq!(notes(&value()), "138");
    }

    #[test]
    fn solver_verdict_alone_shows() {
        assert_eq!(
            notes(&value().with_verdict(SolverVerdict::High)),
            "138 (high)"
        );
    }

    /// AOC's word supersedes the solver's, so a starred part reads as starred
    /// rather than repeating that the solver agreed.
    #[test]
    fn aoc_supersedes_the_solver() {
        let starred = value()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::AlreadySolved);
        assert_eq!(notes(&starred), "138 (starred)");

        let fresh = value()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::Correct);
        assert_eq!(notes(&fresh), "138 (new star)");
    }

    /// Any other AOC reply is worth seeing next to what the solver thought.
    #[test]
    fn both_verdicts_show_when_aoc_did_not_grade() {
        let cooled = value()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::Cooldown("1m 0s".to_string()));
        assert_eq!(
            notes(&cooled),
            "138 (correct, rate limited, 1m 0s left to wait)"
        );
    }

    #[test]
    fn timing_always_renders() {
        assert!(value().to_string().ends_with("[7µs]"));
    }

    /// Nothing but a submittable answer can carry a verdict, which is the
    /// invariant that survived splitting `Answer` from `Outcome`.
    #[test]
    fn unsubmittable_answers_never_take_a_verdict() {
        for answer in [Answer::Visual("art".to_string()), Answer::None] {
            let outcome = Outcome::new(Ok(answer), Duration::ZERO)
                .with_verdict(SolverVerdict::Correct)
                .with_submission(AocVerdict::Correct);
            assert!(outcome.verdict().is_none());
            assert!(outcome.submission().is_none());
        }
    }

    #[test]
    fn visual_answers_render_their_art() {
        let outcome = Outcome::new(Ok(Answer::Visual("###".to_string())), Duration::ZERO);
        assert!(outcome.to_string().contains("###"));
        assert_eq!(notes(&outcome), "\n###");
    }

    #[test]
    fn absent_answers_say_so() {
        let outcome = Outcome::new(Ok(Answer::None), Duration::ZERO);
        assert_eq!(notes(&outcome), "(none)");
    }

    fn failed() -> Outcome {
        let cause = anyhow::anyhow!("given string was too short");
        Outcome::new(
            Err(cause.context("could not parse the input")),
            Duration::ZERO,
        )
    }

    /// A failed part still prints, and prints the whole chain, since the
    /// outermost message alone rarely says which day is broken.
    #[test]
    fn failed_parts_render_their_error() {
        assert_eq!(
            notes(&failed()),
            "error: could not parse the input: given string was too short"
        );
    }

    /// Nothing to submit, so nothing to check. Same gate the unsubmittable
    /// answers go through.
    #[test]
    fn failed_parts_never_take_a_verdict() {
        let outcome = failed()
            .with_verdict(SolverVerdict::Correct)
            .with_submission(AocVerdict::Correct);
        assert!(outcome.value().is_none());
        assert!(outcome.verdict().is_none());
        assert!(outcome.submission().is_none());
    }
}
