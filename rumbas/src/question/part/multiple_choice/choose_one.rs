use super::{extract_multiple_choice_answer_data, MultipleChoiceAnswerData};
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionParts;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableStrings;
use crate::support::variable_valued::VariableValued;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

//TODO: defaults
question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck)]
    #[input(name = "QuestionPartChooseOneInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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

impl ToNumbas<numbas::question::part::choose_one::QuestionPartChooseOne> for QuestionPartChooseOne {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::choose_one::QuestionPartChooseOne {
        let (answers, marking_matrix, distractors) = match &self.answer_data {
            MultipleChoiceAnswerData::ItemBased(answers) => (
                VariableValued::Value(
                    answers
                        .iter()
                        .map(|a| a.statement.clone())
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                Some(
                    VariableValued::Value(
                        answers.iter().map(|a| a.marks.clone()).collect::<Vec<_>>(),
                    )
                    .to_numbas(locale),
                ),
                Some(
                    answers
                        .iter()
                        .map(|a| {
                            a.feedback.clone() //TODO
                        })
                        .collect::<Vec<_>>()
                        .to_numbas(locale),
                ),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                Some(data.marks.to_numbas(locale)),
                data.feedback.clone().map(|f| f.to_numbas(locale)).into(),
            ),
        };
        numbas::question::part::choose_one::QuestionPartChooseOne {
            part_data: self.to_numbas(locale),
            min_answers: Some(if self.has_to_select_option.to_numbas(locale) {
                1
            } else {
                0
            }),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            answers,
            display_type: self.display.to_numbas(locale),
            columns: self.display.get_nb_columns().into(),
            wrong_nb_choices_warning: Some(
                numbas::question::part::match_answers::MultipleChoiceWarningType::None,
            ), //TODO
            show_cell_answer_state: Some(self.show_cell_answer_state.to_numbas(locale)),
            marking_matrix,
            distractors,
        }
    }
}

impl ToRumbas<QuestionPartChooseOne> for numbas::question::part::choose_one::QuestionPartChooseOne {
    fn to_rumbas(&self) -> QuestionPartChooseOne {
        create_question_part! {
            QuestionPartChooseOne with &self.part_data => {
                answer_data: self.to_rumbas(),
                display: self.to_rumbas(),
                shuffle_answers: self.shuffle_answers.to_rumbas(),
                show_cell_answer_state:
                    self.show_cell_answer_state
                        .unwrap_or(DEFAULTS.choose_one_show_cell_answer_state).to_rumbas(),
                has_to_select_option:
                    self.min_answers
                        .map(|v| v == 1)
                        .unwrap_or(DEFAULTS.choose_one_has_to_select_option).to_rumbas()
            }
        }
    }
}

impl ToRumbas<MultipleChoiceAnswerData>
    for numbas::question::part::choose_one::QuestionPartChooseOne
{
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.answers, &self.marking_matrix, &self.distractors)
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatrixRowPrimitiveInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatrixRowPrimitive(Vec<numbas::support::primitive::Primitive>);

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixRowPrimitive {
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Row(self.0.clone())
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatrixRowInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatrixRow(TranslatableStrings);

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixRow {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Row(
            self.0
                .to_numbas(locale)
                .into_iter()
                .map(|a| a.into())
                .collect(),
        )
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatrixPrimitiveInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatrixPrimitive(Vec<VariableValued<Vec<numbas::support::primitive::Primitive>>>);

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixPrimitive {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Matrix(
            self.0.to_numbas(locale),
        )
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "ChooseOneDisplayInput")]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum ChooseOneDisplay {
    #[serde(rename = "dropdown")]
    DropDown,
    #[serde(rename = "radio")]
    Radio { columns: usize },
}

impl ToNumbas<numbas::question::part::choose_one::ChooseOneDisplayType> for ChooseOneDisplay {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::choose_one::ChooseOneDisplayType {
        match self {
            ChooseOneDisplay::DropDown => {
                numbas::question::part::choose_one::ChooseOneDisplayType::DropDown
            }
            ChooseOneDisplay::Radio { columns: _ } => {
                numbas::question::part::choose_one::ChooseOneDisplayType::Radio
            }
        }
    }
}

impl ToRumbas<ChooseOneDisplay> for numbas::question::part::choose_one::QuestionPartChooseOne {
    fn to_rumbas(&self) -> ChooseOneDisplay {
        match self.display_type {
            numbas::question::part::choose_one::ChooseOneDisplayType::Radio => {
                ChooseOneDisplay::Radio {
                    columns: self.columns.0,
                }
            }
            numbas::question::part::choose_one::ChooseOneDisplayType::DropDown => {
                ChooseOneDisplay::DropDown
            }
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
