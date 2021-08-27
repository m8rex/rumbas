use super::{extract_multiple_choice_answer_data, MultipleChoiceAnswerData};
use crate::question::part::multiple_choice::MultipleChoiceAnswerDataInput;
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::VariableReplacementStrategyInput;
use crate::question::QuestionPartInput;
use crate::question::QuestionParts;
use crate::question::QuestionPartsInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::Value;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use crate::support::translatable::TranslatableStrings;
use crate::support::translatable::TranslatableStringsInput;
use crate::support::variable_valued::VariableValued;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

//TODO: defaults
question_part_type! {
    pub struct QuestionPartChooseOne {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: RumbasBool,
        show_cell_answer_state: RumbasBool,
        /// Whether the student has to select an option (if false: can submit without selecting)
        has_to_select_option: RumbasBool,
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

optional_overwrite_newtype! {
    pub struct MatrixRowPrimitive(Primitives)
}

type Primitives = Vec<numbas::support::primitive::Primitive>;
type PrimitivesInput = Vec<Value<numbas::support::primitive::Primitive>>;

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixRowPrimitive {
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Row(self.0.clone())
    }
}

optional_overwrite_newtype! {
    pub struct MatrixRow(TranslatableStrings)
}

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

pub type VariableValuedsPrimitivessInput =
    Vec<Value<VariableValued<Vec<Value<numbas::support::primitive::Primitive>>>>>;
pub type VariableValuedsPrimitivess =
    Vec<VariableValued<Vec<numbas::support::primitive::Primitive>>>;
optional_overwrite_newtype! {
    pub struct MatrixPrimitive(VariableValuedsPrimitivess)
}

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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum ChooseOneDisplayInput {
    #[serde(rename = "dropdown")]
    DropDown,
    #[serde(rename = "radio")]
    Radio { columns: Value<RumbasNatural> },
}

impl OptionalOverwrite<ChooseOneDisplayInput> for ChooseOneDisplayInput {
    fn overwrite(&mut self, other: &ChooseOneDisplayInput) {
        match (self, other) {
            (
                &mut Self::Radio { ref mut columns },
                Self::Radio {
                    columns: ref columns2,
                },
            ) => columns.overwrite(&columns2),
            _ => (),
        };
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match self {
            &mut Self::Radio { ref mut columns } => columns.insert_template_value(&key, &val),
            _ => (),
        };
    }
}

impl Input for ChooseOneDisplayInput {
    type Normal = ChooseOneDisplay;
    fn to_normal(&self) -> Self::Normal {
        match self {
            Self::DropDown => Self::Normal::DropDown,
            Self::Radio { columns } => Self::Normal::Radio {
                columns: columns.unwrap(),
            },
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        match normal {
            Self::Normal::DropDown => Self::DropDown,
            Self::Normal::Radio { columns } => Self::Radio {
                columns: Value::Normal(columns),
            },
        }
    }
}

impl OptionalCheck for ChooseOneDisplayInput {
    fn find_missing(&self) -> OptionalCheckResult {
        match self {
            Self::DropDown => OptionalCheckResult::empty(),
            Self::Radio { columns } => columns.find_missing(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChooseOneDisplay {
    DropDown,
    Radio { columns: usize },
}

impl RumbasCheck for ChooseOneDisplay {
    fn check(&self, _locale: &str) -> RumbasCheckResult {
        match self {
            Self::DropDown => RumbasCheckResult::empty(),
            Self::Radio { columns: _ } => RumbasCheckResult::empty(),
        }
    }
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
