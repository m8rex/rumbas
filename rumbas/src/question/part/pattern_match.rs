use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartPatternMatch {
        case_sensitive: bool,
        partial_credit: f64,
        pattern: TranslatableString, //TODO: type
        display_answer: TranslatableString,
        match_mode: numbas::exam::PatternMatchMode
    }
}
impl_optional_overwrite!(numbas::exam::PatternMatchMode);

impl ToNumbas<numbas::exam::ExamQuestionPartPatternMatch> for QuestionPartPatternMatch {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartPatternMatch {
        numbas::exam::ExamQuestionPartPatternMatch {
            part_data: self.to_numbas(locale),
            case_sensitive: Some(self.case_sensitive.to_numbas(locale)),
            partial_credit: Some(self.partial_credit.to_numbas(locale)),
            answer: numbas::exam::Primitive::String(self.pattern.to_numbas(locale)),
            display_answer: Some(numbas::exam::Primitive::String(
                self.display_answer.to_numbas(locale),
            )),
            match_mode: self.match_mode.to_numbas(locale),
        }
    }
}
impl_to_numbas!(numbas::exam::PatternMatchMode);

impl ToRumbas<QuestionPartPatternMatch> for numbas::exam::ExamQuestionPartPatternMatch {
    fn to_rumbas(&self) -> QuestionPartPatternMatch {
        create_question_part! {
            QuestionPartPatternMatch with &self.part_data => {
                case_sensitive: Value::Normal(
                    self.case_sensitive
                        .unwrap_or(DEFAULTS.pattern_match_case_sensitive),
                ),
                partial_credit: Value::Normal(
                    self.partial_credit
                        .unwrap_or(DEFAULTS.pattern_match_partial_credit)
                        .0,
                ),
                pattern: Value::Normal(self.answer.to_string().into()),
                display_answer: Value::Normal(
                    self.display_answer
                        .clone()
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| self.answer.to_string())
                        .into(),
                ), // TDDO: check default
                match_mode: Value::Normal(self.match_mode)
            }
        }
    }
}
