use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::QuestionPart;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::{AdaptiveMarking, CustomMarking};
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
    #[input(name = "QuestionPartPatternMatchInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartPatternMatch {
        /// If this is ticked, the capitalisation of the student’s answer must match that of the answer pattern. If it doesn’t, partial credit will be awarded.
        case_sensitive: bool,
        #[serde(alias="partial_credit")]
        /// The partial credits awarded if the students capitalisation is wrong
        wrong_case_partial_credit: f64,
        /// The text or pattern the student must match.
        pattern: EmbracedJMETranslatableString,
        /// A representative correct answer string to display to the student, in case they press
        /// the Reveal answers button.
        display_answer: EmbracedJMETranslatableString,
        /// The test to use to decide if the student’s answer is correct.
        /// Some examples
        /// https://numbas-editor.readthedocs.io/en/latest/question/parts/match-text-pattern.html#regular-expressions
        match_mode: PatternMatchMode
    }
}

impl ToNumbas<numbas::question::part::pattern_match::QuestionPartPatternMatch>
    for QuestionPartPatternMatch
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        locale: &str,
        _data: &Self::ToNumbasHelper
    ) -> numbas::question::part::pattern_match::QuestionPartPatternMatch {
        numbas::question::part::pattern_match::QuestionPartPatternMatch {
            part_data: self.to_numbas(locale, &()),
            case_sensitive: self.case_sensitive.to_numbas(locale, &()),
            partial_credit: self.wrong_case_partial_credit.to_numbas(locale, &()),
            answer: self.pattern.to_numbas(locale, &()),
            display_answer: Some(self.display_answer.to_numbas(locale, &())),
            match_mode: self.match_mode.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<QuestionPartPatternMatch>
    for numbas::question::part::pattern_match::QuestionPartPatternMatch
{
    fn to_rumbas(&self) -> QuestionPartPatternMatch {
        create_question_part! {
            QuestionPartPatternMatch with &self.part_data => {
                case_sensitive:
                    self.case_sensitive
                        .to_rumbas(),
                wrong_case_partial_credit:
                    self.partial_credit
                        .0.to_rumbas(),
                pattern: self.answer.to_rumbas(),
                display_answer:
                    self.display_answer
                        .clone()
                        .unwrap_or_else(|| self.answer.clone())
                        .to_rumbas(),
                match_mode: self.match_mode.to_rumbas()
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "PatternMatchModeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatternMatchMode {
    /// The pattern is interpreted as a regular expression
    /// (https://developer.mozilla.org/en-US/docs/JavaScript/Guide/Regular_Expressions)
    Regex,
    /// Marks the student’s answer as correct only if it is exactly the same as the text given in Answer pattern. Space characters are removed from the start and end of the student’s answer as well as the answer pattern before comparison.
    Exact,
}

impl ToNumbas<numbas::question::part::pattern_match::PatternMatchMode> for PatternMatchMode {
    type ToNumbasHelper = ();
    fn to_numbas(&self, _locale: &str, _data: &Self::ToNumbasHelper) -> numbas::question::part::pattern_match::PatternMatchMode {
        match self {
            Self::Exact => numbas::question::part::pattern_match::PatternMatchMode::Exact,
            Self::Regex => numbas::question::part::pattern_match::PatternMatchMode::Regex,
        }
    }
}

impl ToRumbas<PatternMatchMode> for numbas::question::part::pattern_match::PatternMatchMode {
    fn to_rumbas(&self) -> PatternMatchMode {
        match self {
            Self::Exact => PatternMatchMode::Exact,
            Self::Regex => PatternMatchMode::Regex,
        }
    }
}
