use super::{extract_multiple_choice_answer_data, MultipleChoiceAnswerData};
use crate::data::optional_overwrite::VariableValued;
use crate::data::optional_overwrite::*;
use crate::data::question_part::question_part::JMENotes;
use crate::data::question_part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::ToNumbas;
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

question_part_type! {
    pub struct QuestionPartChooseMultiple {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
        columns: usize,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type:  numbas::exam::MultipleChoiceWarningType
        //min_marks & max_marks?
        //TODO other?
    }
}
impl_optional_overwrite!(numbas::exam::MultipleChoiceWarningType);
impl_to_numbas!(numbas::exam::MultipleChoiceWarningType);

impl ToNumbas<numbas::exam::ExamQuestionPartChooseMultiple> for QuestionPartChooseMultiple {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartChooseMultiple {
        // TODO: below is duplicated in CHooseOne
        let (choices, marking_matrix, distractors) = match self.answer_data.unwrap() {
            MultipleChoiceAnswerData::ItemBased(answers) => (
                VariableValued::Value(
                    answers
                        .iter()
                        .map(|a| a.statement.clone().unwrap())
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                Some(
                    VariableValued::Value(
                        answers
                            .iter()
                            .map(|a| a.marks.clone().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .to_numbas(locale),
                ),
                Some(
                    answers
                        .iter()
                        .map(|a| {
                            a.feedback.clone().unwrap() //TODO
                        })
                        .collect::<Vec<_>>()
                        .to_numbas(locale),
                ),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                Some(data.marks.to_numbas(locale)),
                data.feedback.map(|f| f.to_numbas(locale)).flatten(),
            ),
        };
        numbas::exam::ExamQuestionPartChooseMultiple {
            part_data: self.to_numbas_shared_data(locale),
            min_answers: Some(self.should_select_at_least.to_numbas(locale)),
            max_answers: self.should_select_at_most.to_numbas(locale),
            min_marks: Some(0usize), // todo?
            max_marks: Some(0usize.into()),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            choices,
            display_columns: self.columns.to_numbas(locale),
            wrong_nb_choices_warning: self.wrong_nb_answers_warning_type.to_numbas(locale),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale),
            marking_matrix,
            distractors,
        }
    }
}

impl ToRumbas<MultipleChoiceAnswerData> for numbas::exam::ExamQuestionPartChooseMultiple {
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.choices, &self.marking_matrix, &self.distractors)
    }
}

impl ToRumbas<QuestionPartChooseMultiple> for numbas::exam::ExamQuestionPartChooseMultiple {
    fn to_rumbas(&self) -> QuestionPartChooseMultiple {
        QuestionPartChooseMultiple {
            // Default section
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

            answer_data: Value::Normal(self.to_rumbas()),
            shuffle_answers: Value::Normal(self.shuffle_answers),
            show_cell_answer_state: Value::Normal(self.show_cell_answer_state),
            should_select_at_least: Value::Normal(
                self.min_answers
                    .unwrap_or(DEFAULTS.choose_multiple_min_answers)
                    .0,
            ),
            should_select_at_most: Value::Normal(
                self.max_answers
                    .map(|v| v.0)
                    .map(Noneable::NotNone)
                    .unwrap_or_else(Noneable::nn),
            ),
            columns: Value::Normal(self.display_columns.0),
            wrong_nb_answers_warning_type: Value::Normal(self.wrong_nb_choices_warning),
        }
    }
}
