use crate::data::optional_overwrite::*;
use crate::data::question_part::question_part::JMENotes;
use crate::data::question_part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::ToNumbas;
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartExtension {}
}

impl ToNumbas<numbas::exam::ExamQuestionPartExtension> for QuestionPartExtension {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartExtension {
        numbas::exam::ExamQuestionPartExtension {
            part_data: self.to_numbas_shared_data(locale),
        }
    }
}

impl ToRumbas<QuestionPartExtension> for numbas::exam::ExamQuestionPartExtension {
    fn to_rumbas(&self) -> QuestionPartExtension {
        QuestionPartExtension {
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
        }
    }
}
