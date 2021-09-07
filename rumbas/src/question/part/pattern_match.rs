use crate::question::part::question_part::{JMENotes, JMENotesInput};
use crate::question::part::question_part::{
    QuestionPartInput, VariableReplacementStrategy, VariableReplacementStrategyInput,
};
use crate::question::part::question_part::{QuestionParts, QuestionPartsInput};
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartPatternMatch {
        case_sensitive: RumbasBool,
        partial_credit: RumbasFloat,
        pattern: TranslatableString, //TODO: type
        display_answer: TranslatableString,
        match_mode: PatternMatchMode
    }
}
type PatternMatchMode = numbas::question::part::pattern_match::PatternMatchMode;
impl_optional_overwrite!(PatternMatchMode);

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
            answer: numbas::support::primitive::Primitive::String(self.pattern.to_numbas(locale)),
            display_answer: Some(numbas::support::primitive::Primitive::String(
                self.display_answer.to_numbas(locale),
            )),
            match_mode: self.match_mode.to_numbas(locale),
        }
    }
}
impl_to_numbas!(numbas::question::part::pattern_match::PatternMatchMode);

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
                pattern: self.answer.to_string().to_rumbas(),
                display_answer:
                    self.display_answer
                        .clone()
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| self.answer.to_string())
                        .to_rumbas(),
                match_mode: self.match_mode
            }
        }
    }
}
