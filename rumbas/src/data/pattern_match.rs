use crate::data::optional_overwrite::*;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
use crate::data::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

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

impl ToNumbas for QuestionPartPatternMatch {
    type NumbasType = numbas::exam::ExamQuestionPartPatternMatch;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(locale),
                case_sensitive: Some(self.case_sensitive.unwrap()),
                partial_credit: Some(self.partial_credit.unwrap().into()),
                answer: numbas::exam::Primitive::String(
                    self.pattern.clone().unwrap().to_string(locale).unwrap(),
                ),
                display_answer: Some(numbas::exam::Primitive::String(
                    self.display_answer
                        .clone()
                        .unwrap()
                        .to_string(locale)
                        .unwrap(),
                )),
                match_mode: self.match_mode.unwrap(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionPartPatternMatch> for numbas::exam::ExamQuestionPartPatternMatch {
    fn to_rumbas(&self) -> QuestionPartPatternMatch {
        QuestionPartPatternMatch {
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(extract_part_common_prompt(&self.part_data)),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&self.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&self.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&self.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &self.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&self.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &self.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &self.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                self.part_data.variable_replacement_strategy.to_rumbas(),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &self.part_data,
            )),
            custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
                &self.part_data,
            )),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),

            case_sensitive: Value::Normal(
                self.case_sensitive
                    .unwrap_or(DEFAULTS.pattern_match_case_sensitive),
            ),
            partial_credit: Value::Normal(
                self.partial_credit
                    .unwrap_or(DEFAULTS.pattern_match_partial_credit)
                    .0,
            ),
            pattern: Value::Normal(TranslatableString::s(&self.answer.to_string())),
            display_answer: Value::Normal(TranslatableString::s(
                &self
                    .display_answer
                    .clone()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| self.answer.to_string()),
            )), // TDDO: check default
            match_mode: Value::Normal(self.match_mode),
        }
    }
}
