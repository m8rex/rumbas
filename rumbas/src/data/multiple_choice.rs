use crate::data::optional_overwrite::VariableValued;
use crate::data::optional_overwrite::{EmptyFields, Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};
use std::convert::Into;

//TODO: defaults
question_part_type! {
    pub struct QuestionPartChooseOne {
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

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum MultipleChoiceAnswerData {
        ItemBased(Vec<MultipleChoiceAnswer>),
        NumbasLike(MultipleChoiceAnswerDataNumbasLike)
    }
}

optional_overwrite! {
    pub struct MultipleChoiceAnswerDataNumbasLike {
        answers: VariableValued<Vec<VariableValued<TranslatableString>>>,
        marks: VariableValued<Vec<VariableValued<numbas::exam::Primitive>>>,
        feedback: Noneable<Vec<TranslatableString>>
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MatrixRowPrimitive(Vec<numbas::exam::Primitive>);
impl_optional_overwrite!(MatrixRowPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas for MatrixRowPrimitive {
    type NumbasType = numbas::exam::MultipleChoiceMatrix;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::MultipleChoiceMatrix::Row(self.0.clone()))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MatrixRow(Vec<TranslatableString>);
impl_optional_overwrite!(MatrixRow); // TODO: Does this do what it needs to do?

impl ToNumbas for MatrixRow {
    type NumbasType = numbas::exam::MultipleChoiceMatrix;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::MultipleChoiceMatrix::Row(
                self.0
                    .to_numbas(locale)
                    .unwrap()
                    .into_iter()
                    .map(|a| a.into())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MatrixPrimitive(Vec<VariableValued<Vec<numbas::exam::Primitive>>>);
impl_optional_overwrite!(MatrixPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas for MatrixPrimitive {
    type NumbasType = numbas::exam::MultipleChoiceMatrix;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::MultipleChoiceMatrix::Matrix(
                self.0
                    .clone()
                    .into_iter()
                    .map(|r| r.to_numbas(&locale).unwrap())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
impl ToNumbas for QuestionPartChooseOne {
    type NumbasType = numbas::exam::ExamQuestionPartChooseOne;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let (answers, marking_matrix, distractors) = match self.answer_data.unwrap() {
                MultipleChoiceAnswerData::ItemBased(answers) => (
                    VariableValued::Value(
                        answers
                            .iter()
                            .map(|a| VariableValued::Value(a.statement.clone().unwrap()))
                            .collect::<Vec<_>>(),
                    )
                    .to_numbas(&locale)
                    .unwrap(),
                    Some(
                        VariableValued::Value(
                            answers
                                .iter()
                                .map(|a| VariableValued::Value(a.marks.clone().unwrap()))
                                .collect::<Vec<_>>(),
                        )
                        .to_numbas(&locale)
                        .unwrap(),
                    ),
                    Some(
                        answers
                            .iter()
                            .map(|a| {
                                a.feedback.clone().unwrap() //TODO
                            })
                            .collect::<Vec<_>>()
                            .to_numbas(&locale)
                            .unwrap(),
                    ),
                ),
                MultipleChoiceAnswerData::NumbasLike(data) => (
                    data.answers.to_numbas(&locale).unwrap(),
                    Some(data.marks.to_numbas(&locale).unwrap()),
                    data.feedback
                        .map(|f| f.to_numbas(&locale).unwrap())
                        .flatten(),
                ),
            };
            Ok(numbas::exam::ExamQuestionPartChooseOne {
                part_data: self.to_numbas_shared_data(&locale),
                min_answers: Some(if self.has_to_select_option.unwrap() {
                    1
                } else {
                    0
                }),
                shuffle_answers: self.shuffle_answers.unwrap(),
                answers,
                display_type: self.display.unwrap().to_numbas_type(),
                columns: self.display.unwrap().to_nb_columns().into(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix,
                distractors,
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
    Radio { columns: usize },
}
impl_optional_overwrite!(ChooseOneDisplay);

impl ChooseOneDisplay {
    pub fn to_numbas_type(&self) -> numbas::exam::ChooseOneDisplayType {
        match self {
            ChooseOneDisplay::DropDown => numbas::exam::ChooseOneDisplayType::DropDown,
            ChooseOneDisplay::Radio { columns: _ } => numbas::exam::ChooseOneDisplayType::Radio,
        }
    }

    pub fn to_nb_columns(&self) -> usize {
        match self {
            ChooseOneDisplay::DropDown => 0,
            ChooseOneDisplay::Radio { columns } => *columns,
        }
    }
}

impl ToNumbas for numbas::exam::MultipleChoiceMatrix {
    type NumbasType = Self;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(self.clone())
    }
}
impl_optional_overwrite!(numbas::exam::MultipleChoiceMatrix);
impl ToNumbas for numbas::exam::Primitive {
    type NumbasType = Self;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(self.clone())
    }
}
impl_optional_overwrite!(numbas::exam::Primitive);
optional_overwrite! {
    pub struct MultipleChoiceAnswer {
        statement: TranslatableString,
        feedback: TranslatableString,
        marks: numbas::exam::Primitive // TODO: variable valued?
    }
}

question_part_type! {
    pub struct QuestionPartChooseMultiple {
        answers: VariableValued<Vec<MultipleChoiceAnswer>>,
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
                min_marks: Some(0usize.into()),
                max_marks: Some(0usize.into()),
                shuffle_answers: self.shuffle_answers.unwrap(),
                choices: answers
                    .clone()
                    .map(|aa| {
                        aa.iter()
                            .map(|a| a.statement.clone().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .to_numbas(&locale)
                    .unwrap(),
                display_columns: self.columns.unwrap().into(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix: Some(
                    answers
                        .clone()
                        .map(|aa| {
                            MatrixRowPrimitive(
                                aa.iter().map(|a| a.marks.clone().unwrap()).collect(),
                            )
                        })
                        .to_numbas(&locale)
                        .unwrap(),
                ),
                distractors: Some(
                    answers
                        .clone()
                        .map(|aa| {
                            MatrixRow(
                                aa.iter()
                                    .map(|a| {
                                        a.feedback.clone().unwrap() //TODO
                                    })
                                    .collect(),
                            )
                        })
                        .to_numbas(&locale)
                        .unwrap(),
                ),
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
        answers: VariableValued<Vec<Value<TranslatableString>>>,  // Values of the answers
        items: VariableValued<Vec<Value<MatchAnswersItem>>>, // Items for which the answer can be selected
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

// TODO move
impl ToNumbas for TranslatableString {
    type NumbasType = String;
    fn to_numbas(&self, locale: &String) -> NumbasResult<String> {
        // TODO: check if translation exists?
        Ok(self.clone().to_string(&locale).unwrap())
        /*self
        .iter()
        .map(|a| a.clone().unwrap().to_string(&locale).unwrap())
        .collect()
            */
    }
}

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
                answers: answers.clone().to_numbas(&locale).unwrap(),
                choices: items
                    .clone()
                    .map(|v| {
                        v.iter()
                            .map(|a| a.clone().unwrap().statement.clone())
                            .collect::<Vec<_>>()
                    })
                    .to_numbas(&locale)
                    .unwrap(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                layout: self.layout.clone().unwrap(),
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix: Some(
                    items // TODO: better handling
                        .clone()
                        .map(|v| {
                            {
                                MatrixPrimitive(
                                    v.iter()
                                        .map(|i| {
                                            answers.clone().map(|aa| {
                                                aa.iter()
                                                    .map(|a| {
                                                        i.unwrap()
                                                            .answer_marks
                                                            .unwrap()
                                                            .iter()
                                                            .find(|am| {
                                                                am.answer.unwrap() == a.unwrap()
                                                            })
                                                            .map_or_else(
                                                                || 0usize.into(),
                                                                |v| v.marks.unwrap(),
                                                            )
                                                    })
                                                    .collect()
                                            })
                                        })
                                        .collect(),
                                )
                            }
                            .to_numbas(&locale)
                            .unwrap()
                        })
                        .to_numbas(&locale)
                        .unwrap(),
                ),
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
