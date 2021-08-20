use super::{extract_multiple_choice_answer_data, MultipleChoiceAnswerData};
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

//TODO: defaults
question_part_type! {
    pub struct QuestionPartChooseOne {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        /// Whether the student has to select an option (if false: can submit without selecting)
        has_to_select_option: bool,
        /// !FLATTENED: all its attributes should be added to [QuestionPartChooseOne]
        #[serde(flatten)]
        display: ChooseOneDisplay
        //TODO wrong_nb_choices_warning:
    }
}

impl ToNumbas<numbas::exam::ExamQuestionPartChooseOne> for QuestionPartChooseOne {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartChooseOne {
        let (answers, marking_matrix, distractors) = match self.answer_data.unwrap() {
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
        numbas::exam::ExamQuestionPartChooseOne {
            part_data: self.to_numbas(locale),
            min_answers: Some(if self.has_to_select_option.to_numbas(locale) {
                1
            } else {
                0
            }),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            answers,
            display_type: self.display.unwrap().to_numbas(locale),
            columns: self.display.unwrap().get_nb_columns().into(),
            wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
            show_cell_answer_state: Some(self.show_cell_answer_state.to_numbas(locale)),
            marking_matrix,
            distractors,
        }
    }
}

impl ToRumbas<QuestionPartChooseOne> for numbas::exam::ExamQuestionPartChooseOne {
    fn to_rumbas(&self) -> QuestionPartChooseOne {
        create_question_part! {
            QuestionPartChooseOne with &self.part_data => {
                answer_data: Value::Normal(self.to_rumbas()),
                display: Value::Normal(self.to_rumbas()),
                shuffle_answers: Value::Normal(self.shuffle_answers),
                show_cell_answer_state: Value::Normal(
                    self.show_cell_answer_state
                        .unwrap_or(DEFAULTS.choose_one_show_cell_answer_state),
                ),
                has_to_select_option: Value::Normal(
                    self.min_answers
                        .map(|v| v == 1)
                        .unwrap_or(DEFAULTS.choose_one_has_to_select_option),
                )
            }
        }
    }
}

impl ToRumbas<MultipleChoiceAnswerData> for numbas::exam::ExamQuestionPartChooseOne {
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.answers, &self.marking_matrix, &self.distractors)
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixRowPrimitive(Vec<numbas::exam::Primitive>);
impl_optional_overwrite!(MatrixRowPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixRowPrimitive {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Row(self.0.clone())
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixRow(Vec<TranslatableString>);
impl_optional_overwrite!(MatrixRow); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixRow {
    fn to_numbas(&self, locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Row(
            self.0
                .to_numbas(locale)
                .into_iter()
                .map(|a| a.into())
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixPrimitive(Vec<VariableValued<Vec<numbas::exam::Primitive>>>);
impl_optional_overwrite!(MatrixPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixPrimitive {
    fn to_numbas(&self, locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Matrix(self.0.to_numbas(locale))
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum ChooseOneDisplay {
    #[serde(rename = "dropdown")]
    DropDown,
    #[serde(rename = "radio")]
    Radio { columns: usize },
}
impl_optional_overwrite!(ChooseOneDisplay);

impl ToNumbas<numbas::exam::ChooseOneDisplayType> for ChooseOneDisplay {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ChooseOneDisplayType {
        match self {
            ChooseOneDisplay::DropDown => numbas::exam::ChooseOneDisplayType::DropDown,
            ChooseOneDisplay::Radio { columns: _ } => numbas::exam::ChooseOneDisplayType::Radio,
        }
    }
}

impl ToRumbas<ChooseOneDisplay> for numbas::exam::ExamQuestionPartChooseOne {
    fn to_rumbas(&self) -> ChooseOneDisplay {
        match self.display_type {
            numbas::exam::ChooseOneDisplayType::Radio => ChooseOneDisplay::Radio {
                columns: self.columns.0,
            },
            numbas::exam::ChooseOneDisplayType::DropDown => ChooseOneDisplay::DropDown,
        }
    }
}

impl ChooseOneDisplay {
    pub fn get_nb_columns(&self) -> usize {
        match self {
            ChooseOneDisplay::DropDown => 0,
            ChooseOneDisplay::Radio { columns } => *columns,
        }
    }
}
