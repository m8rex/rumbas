use crate::data::question_part::question_part::JMENotes;
use crate::data::question_part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::translatable::ContentAreaTranslatableString;
use crate::data::translatable::TranslatableString;
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
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
            part_data: self.to_numbas_shared_data(locale),
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
            custom_marking_algorithm_notes: Value::Normal(
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
            ),
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
            pattern: Value::Normal(self.answer.to_string().into()),
            display_answer: Value::Normal(
                self.display_answer
                    .clone()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| self.answer.to_string())
                    .into(),
            ), // TDDO: check default
            match_mode: Value::Normal(self.match_mode),
        }
    }
}
