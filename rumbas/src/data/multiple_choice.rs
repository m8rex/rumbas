use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

//TODO: defaults
question_part_type! {
    pub struct QuestionPartChooseOne {
        answers: Vec<MultipleChoiceAnswer>,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize, // TODO 0 or 1?
        /// !FLATTENED: all its attributes should be added to [QuestionPartChooseOne]
        #[serde(flatten)]
        display: ChooseOneDisplay
        //TODO wrong_nb_choices_warning:
    }
}

impl ToNumbas for QuestionPartChooseOne {
    type NumbasType = numbas::exam::ExamQuestionPartChooseOne;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let answers = self.answers.unwrap();
            Ok(numbas::exam::ExamQuestionPartChooseOne {
                part_data: self.to_numbas_shared_data(&locale),
                min_answers: Some(self.should_select_at_least.clone().unwrap()),
                shuffle_answers: self.shuffle_answers.unwrap(),
                answers: answers
                    .iter()
                    .map(|a| a.statement.clone().unwrap().to_string(&locale).unwrap())
                    .collect(),
                display_type: self.display.unwrap().to_numbas_type(),
                columns: self.display.unwrap().to_nb_columns(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix: Some(numbas::exam::MultipleChoiceMatrix::Row(
                    answers.iter().map(|a| a.marks.clone().unwrap()).collect(),
                )),
                distractors: Some(numbas::exam::MultipleChoiceMatrix::Row(
                    answers
                        .iter()
                        .map(|a| {
                            numbas::exam::Primitive::String(
                                a.feedback.clone().unwrap().to_string(&locale).unwrap(), //TODO
                            )
                        })
                        .collect(),
                )),
            })
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum ChooseOneDisplay {
    #[serde(rename = "dropdown")]
    DropDown,
    #[serde(rename = "radio")]
    Radio { columns: u8 },
}
impl_optional_overwrite!(ChooseOneDisplay);

impl ChooseOneDisplay {
    pub fn to_numbas_type(&self) -> numbas::exam::ChooseOneDisplayType {
        match self {
            ChooseOneDisplay::DropDown => numbas::exam::ChooseOneDisplayType::DropDown,
            ChooseOneDisplay::Radio { columns: _ } => numbas::exam::ChooseOneDisplayType::Radio,
        }
    }

    pub fn to_nb_columns(&self) -> u8 {
        match self {
            ChooseOneDisplay::DropDown => 0,
            ChooseOneDisplay::Radio { columns } => *columns,
        }
    }
}

impl_optional_overwrite!(numbas::exam::Primitive);
optional_overwrite! {
    pub struct MultipleChoiceAnswer {
        statement: TranslatableString,
        feedback: TranslatableString,
        marks: numbas::exam::Primitive
    }
}

question_part_type! {
    pub struct QuestionPartChooseMultiple {
        answers: Vec<MultipleChoiceAnswer>,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: usize,
        columns: usize
        //min_marks & max_marks?
        //TODO wrong_nb_choices_warning:
        //TODO other?
    }
}

impl ToNumbas for QuestionPartChooseMultiple {
    type NumbasType = numbas::exam::ExamQuestionPartChooseMultiple;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let answers = self.answers.unwrap();
            Ok(numbas::exam::ExamQuestionPartChooseMultiple {
                part_data: self.to_numbas_shared_data(&locale),
                min_answers: Some(self.should_select_at_least.clone().unwrap()),
                max_answers: Some(self.should_select_at_most.clone().unwrap()),
                min_marks: Some(0),
                max_marks: Some(0),
                shuffle_answers: self.shuffle_answers.unwrap(),
                choices: answers
                    .iter()
                    .map(|a| a.statement.clone().unwrap().to_string(&locale).unwrap())
                    .collect(),
                display_columns: self.columns.unwrap(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix: Some(numbas::exam::MultipleChoiceMatrix::Row(
                    answers.iter().map(|a| a.marks.clone().unwrap()).collect(),
                )),
                distractors: Some(numbas::exam::MultipleChoiceMatrix::Row(
                    answers
                        .iter()
                        .map(|a| {
                            numbas::exam::Primitive::String(
                                a.feedback.clone().unwrap().to_string(&locale).unwrap(), //TODO
                            )
                        })
                        .collect(),
                )),
            })
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    pub struct MatchAnswersItemMarks {
        marks: numbas::exam::Primitive,
        answer: TranslatableString
    }
}

optional_overwrite! {
    pub struct MatchAnswersItem {
        statement: TranslatableString,
        /// Map points to strings of answers ! use anchors in yaml
        answer_marks: Vec<MatchAnswersItemMarks>
    }
}

question_part_type! {
    pub struct QuestionPartMatchAnswersWithItems {
        answers: Vec<Value<TranslatableString>>,  // Values of the answers
        items: Vec<Value<MatchAnswersItem>>, // Items for which the answer can be selected
        shuffle_answers: bool,
        shuffle_items: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: usize,
        /// !FLATTENED
        #[serde(flatten)]
        display: MatchAnswerWithItemsDisplay,
        layout: numbas::exam::MatchAnswersWithChoicesLayout
        //min_marks & max_marks?
        //TODO wrong_nb_choices_warning:
        //TODO other?
    }
}
impl_optional_overwrite!(
    numbas::exam::MatchAnswersWithChoicesLayout,
    numbas::exam::MatchAnswersWithChoicesDisplayType
);

impl ToNumbas for QuestionPartMatchAnswersWithItems {
    type NumbasType = numbas::exam::ExamQuestionPartMatchAnswersWithChoices;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let answers = self.answers.unwrap();
            let items = self.items.unwrap();
            Ok(numbas::exam::ExamQuestionPartMatchAnswersWithChoices {
                part_data: self.to_numbas_shared_data(&locale),
                min_answers: Some(self.should_select_at_least.clone().unwrap()),
                max_answers: Some(self.should_select_at_most.clone().unwrap()),
                min_marks: Some(0),
                max_marks: Some(0),
                shuffle_answers: self.shuffle_answers.unwrap(),
                shuffle_choices: self.shuffle_items.unwrap(),
                answers: answers
                    .iter()
                    .map(|a| a.clone().unwrap().to_string(&locale).unwrap())
                    .collect(),
                choices: items
                    .iter()
                    .map(|a| {
                        a.clone()
                            .unwrap()
                            .statement
                            .clone()
                            .unwrap()
                            .to_string(&locale)
                            .unwrap()
                    })
                    .collect(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                layout: self.layout.clone().unwrap(),
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix: Some(numbas::exam::MultipleChoiceMatrix::Matrix(
                    items
                        .iter()
                        .map(|i| {
                            answers
                                .iter()
                                .map(|a| {
                                    i.unwrap()
                                        .answer_marks
                                        .unwrap()
                                        .iter()
                                        .find(|am| am.answer.unwrap() == a.unwrap())
                                        .map_or_else(
                                            || numbas::exam::Primitive::Integer(0),
                                            |v| v.marks.unwrap(),
                                        )
                                })
                                .collect()
                        })
                        .collect(),
                )),
                display_type: self.display.unwrap().to_numbas_type(),
            })
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum MatchAnswerWithItemsDisplay {
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "check")]
    Check,
}
impl_optional_overwrite!(MatchAnswerWithItemsDisplay);

impl MatchAnswerWithItemsDisplay {
    pub fn to_numbas_type(&self) -> numbas::exam::MatchAnswersWithChoicesDisplayType {
        match self {
            MatchAnswerWithItemsDisplay::Check => {
                numbas::exam::MatchAnswersWithChoicesDisplayType::Check
            }
            MatchAnswerWithItemsDisplay::Radio => {
                numbas::exam::MatchAnswersWithChoicesDisplayType::Radio
            }
        }
    }
}
