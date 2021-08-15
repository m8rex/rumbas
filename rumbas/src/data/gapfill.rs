use crate::data::optional_overwrite::*;
use crate::data::question_part::JMENotes;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
use numbas::defaults::DEFAULTS;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

question_part_type! {
    // The Gap fill question part type
    pub struct QuestionPartGapFill {
        /// Whether the answers should be sorted
        sort_answers: bool,
        /// The gaps
        gaps: Vec<QuestionPart>
    }
}

impl ToNumbas for QuestionPartGapFill {
    type NumbasType = numbas::exam::ExamQuestionPartGapFill;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamQuestionPartGapFill> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamQuestionPartGapFill {
                part_data: self.to_numbas_shared_data(locale),
                sort_answers: Some(self.sort_answers.clone().unwrap()),
                gaps: self
                    .gaps
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|g| g.to_numbas(locale).unwrap())
                    .collect(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionPartGapFill> for numbas::exam::ExamQuestionPartGapFill {
    fn to_rumbas(&self) -> QuestionPartGapFill {
        QuestionPartGapFill {
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

            sort_answers: Value::Normal(self.sort_answers.unwrap_or(DEFAULTS.gapfill_sort_answers)),

            gaps: Value::Normal(self.gaps.to_rumbas()),
        }
    }
}
