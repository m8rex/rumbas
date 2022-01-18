use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::QuestionPart;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use numbas::defaults::DEFAULTS;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use comparable::Comparable;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartPatternMatchInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartPatternMatch {
        case_sensitive: bool,
        partial_credit: f64,
        pattern: EmbracedJMETranslatableString,
        display_answer: EmbracedJMETranslatableString,
        match_mode: PatternMatchMode
    }
}

impl ToNumbas<numbas::question::part::pattern_match::QuestionPartPatternMatch>
    for QuestionPartPatternMatch
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::pattern_match::QuestionPartPatternMatch {
        numbas::question::part::pattern_match::QuestionPartPatternMatch {
            part_data: self.to_numbas(locale),
            case_sensitive: Some(self.case_sensitive.to_numbas(locale)),
            partial_credit: Some(self.partial_credit.to_numbas(locale)),
            answer: self.pattern.to_numbas(locale),
            display_answer: Some(self.display_answer.to_numbas(locale)),
            match_mode: self.match_mode.to_numbas(locale),
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
                        .unwrap_or(DEFAULTS.pattern_match_case_sensitive).to_rumbas(),
                partial_credit:
                    self.partial_credit
                        .unwrap_or(DEFAULTS.pattern_match_partial_credit)
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "PatternMatchModeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PatternMatchMode {
    Regex,
    Exact,
}

impl ToNumbas<numbas::question::part::pattern_match::PatternMatchMode> for PatternMatchMode {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::pattern_match::PatternMatchMode {
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
