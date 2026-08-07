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

#[cfg(test)]
mod tests {
    use super::*;

    fn value() -> Outcome {
        Outcome::new(Answer::solved("138"), Duration::from_micros(7))
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
            let outcome = Outcome::new(answer, Duration::ZERO)
                .with_verdict(SolverVerdict::Correct)
                .with_submission(AocVerdict::Correct);
            assert!(outcome.verdict().is_none());
            assert!(outcome.submission().is_none());
        }
    }

    #[test]
    fn visual_answers_render_their_art() {
        let outcome = Outcome::new(Answer::Visual("###".to_string()), Duration::ZERO);
        assert!(outcome.to_string().contains("###"));
        assert_eq!(notes(&outcome), "\n###");
    }

    #[test]
    fn absent_answers_say_so() {
        let outcome = Outcome::new(Answer::None, Duration::ZERO);
        assert_eq!(notes(&outcome), "(none)");
    }
}
