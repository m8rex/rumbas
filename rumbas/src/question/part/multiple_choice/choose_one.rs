use super::{extract_multiple_choice_answer_data, MultipleChoiceAnswerData};
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionPart;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::variable_valued::VariableValued;
use comparable::Comparable;
use numbas::defaults::DEFAULTS;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

//TODO: defaults
question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartChooseOneInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartChooseOne {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        display: ChooseOneDisplay
        //TODO wrong_nb_choices_warning:
    }
}

impl ToNumbas<numbas::question::part::choose_one::QuestionPartChooseOne> for QuestionPartChooseOne {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::choose_one::QuestionPartChooseOne {
        let (choices, marking_matrix, distractors) = match &self.answer_data {
            MultipleChoiceAnswerData::ItemBased(answers) => (
                VariableValued::Value(
                    answers
                        .iter()
                        .map(|a| a.statement.clone())
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                VariableValued::Value(answers.iter().map(|a| a.marks.clone()).collect::<Vec<_>>())
                    .to_numbas(locale),
                answers
                    .iter()
                    .map(|a| {
                        a.feedback.clone() //TODO
                    })
                    .collect::<Vec<_>>()
                    .to_numbas(locale),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                data.marks.to_numbas(locale),
                data.feedback.to_numbas(locale).unwrap_or_default(),
            ),
        };
        numbas::question::part::choose_one::QuestionPartChooseOne {
            part_data: self.to_numbas(locale),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            choices,
            display_type: self.display.to_numbas(locale),
            columns: self.display.get_nb_columns().into(),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale),
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
                        self.show_cell_answer_state.to_rumbas()
            }
        }
    }
}

impl ToRumbas<MultipleChoiceAnswerData>
    for numbas::question::part::choose_one::QuestionPartChooseOne
{
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.choices, &self.marking_matrix, &self.distractors)
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "ChooseOneDisplayInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
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
