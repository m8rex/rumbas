use crate::data::optional_overwrite::VariableValued;
use crate::data::optional_overwrite::*;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
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
        NumbasLike(Box<MultipleChoiceAnswerDataNumbasLike>)
    }
}

optional_overwrite! {
    pub struct MultipleChoiceAnswerDataNumbasLike {
        answers: VariableValued<Vec<TranslatableString>>,
        marks: VariableValued<Vec<numbas::exam::Primitive>>,
        feedback: Noneable<Vec<TranslatableString>>
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MatrixRowPrimitive(Vec<numbas::exam::Primitive>);
impl_optional_overwrite!(MatrixRowPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas for MatrixRowPrimitive {
    type NumbasType = numbas::exam::MultipleChoiceMatrix;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::MultipleChoiceMatrix::Row(self.0.clone()))
        } else {
            Err(check)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MatrixRow(Vec<TranslatableString>);
impl_optional_overwrite!(MatrixRow); // TODO: Does this do what it needs to do?

impl ToNumbas for MatrixRow {
    type NumbasType = numbas::exam::MultipleChoiceMatrix;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::MultipleChoiceMatrix::Row(
                self.0
                    .to_numbas(locale)
                    .unwrap()
                    .into_iter()
                    .map(|a| a.into())
                    .collect(),
            ))
        } else {
            Err(check)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MatrixPrimitive(Vec<VariableValued<Vec<numbas::exam::Primitive>>>);
impl_optional_overwrite!(MatrixPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas for MatrixPrimitive {
    type NumbasType = numbas::exam::MultipleChoiceMatrix;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::MultipleChoiceMatrix::Matrix(
                self.0
                    .clone()
                    .into_iter()
                    .map(|r| r.to_numbas(locale).unwrap())
                    .collect(),
            ))
        } else {
            Err(check)
        }
    }
}
impl ToNumbas for QuestionPartChooseOne {
    type NumbasType = numbas::exam::ExamQuestionPartChooseOne;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            let (answers, marking_matrix, distractors) = match self.answer_data.unwrap() {
                MultipleChoiceAnswerData::ItemBased(answers) => (
                    VariableValued::Value(
                        answers
                            .iter()
                            .map(|a| a.statement.clone().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .to_numbas(locale)
                    .unwrap(),
                    Some(
                        VariableValued::Value(
                            answers
                                .iter()
                                .map(|a| a.marks.clone().unwrap())
                                .collect::<Vec<_>>(),
                        )
                        .to_numbas(locale)
                        .unwrap(),
                    ),
                    Some(
                        answers
                            .iter()
                            .map(|a| {
                                a.feedback.clone().unwrap() //TODO
                            })
                            .collect::<Vec<_>>()
                            .to_numbas(locale)
                            .unwrap(),
                    ),
                ),
                MultipleChoiceAnswerData::NumbasLike(data) => (
                    data.answers.to_numbas(locale).unwrap(),
                    Some(data.marks.to_numbas(locale).unwrap()),
                    data.feedback
                        .map(|f| f.to_numbas(locale).unwrap())
                        .flatten(),
                ),
            };
            Ok(numbas::exam::ExamQuestionPartChooseOne {
                part_data: self.to_numbas_shared_data(locale),
                min_answers: Some(if self.has_to_select_option.unwrap() {
                    1
                } else {
                    0
                }),
                shuffle_answers: self.shuffle_answers.unwrap(),
                answers,
                display_type: self.display.unwrap().get_numbas_type(),
                columns: self.display.unwrap().get_nb_columns().into(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                show_cell_answer_state: Some(self.show_cell_answer_state.unwrap()),
                marking_matrix,
                distractors,
            })
        } else {
            Err(check)
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
    pub fn get_numbas_type(&self) -> numbas::exam::ChooseOneDisplayType {
        match self {
            ChooseOneDisplay::DropDown => numbas::exam::ChooseOneDisplayType::DropDown,
            ChooseOneDisplay::Radio { columns: _ } => numbas::exam::ChooseOneDisplayType::Radio,
        }
    }

    pub fn get_nb_columns(&self) -> usize {
        match self {
            ChooseOneDisplay::DropDown => 0,
            ChooseOneDisplay::Radio { columns } => *columns,
        }
    }
}

impl ToNumbas for numbas::exam::MultipleChoiceMatrix {
    type NumbasType = Self;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
        Ok(self.clone())
    }
}
impl_optional_overwrite!(numbas::exam::MultipleChoiceMatrix);
impl ToNumbas for numbas::exam::Primitive {
    type NumbasType = Self;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
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
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
        columns: usize
        //min_marks & max_marks?
        //TODO wrong_nb_choices_warning:
        //TODO other?
    }
}

impl ToNumbas for QuestionPartChooseMultiple {
    type NumbasType = numbas::exam::ExamQuestionPartChooseMultiple;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            // TODO: below is duplicated in CHooseOne
            let (choices, marking_matrix, distractors) = match self.answer_data.unwrap() {
                MultipleChoiceAnswerData::ItemBased(answers) => (
                    VariableValued::Value(
                        answers
                            .iter()
                            .map(|a| a.statement.clone().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .to_numbas(locale)
                    .unwrap(),
                    Some(
                        VariableValued::Value(
                            answers
                                .iter()
                                .map(|a| a.marks.clone().unwrap())
                                .collect::<Vec<_>>(),
                        )
                        .to_numbas(locale)
                        .unwrap(),
                    ),
                    Some(
                        answers
                            .iter()
                            .map(|a| {
                                a.feedback.clone().unwrap() //TODO
                            })
                            .collect::<Vec<_>>()
                            .to_numbas(locale)
                            .unwrap(),
                    ),
                ),
                MultipleChoiceAnswerData::NumbasLike(data) => (
                    data.answers.to_numbas(locale).unwrap(),
                    Some(data.marks.to_numbas(locale).unwrap()),
                    data.feedback
                        .map(|f| f.to_numbas(locale).unwrap())
                        .flatten(),
                ),
            };
            Ok(numbas::exam::ExamQuestionPartChooseMultiple {
                part_data: self.to_numbas_shared_data(locale),
                min_answers: Some(self.should_select_at_least.clone().unwrap()),
                max_answers: self
                    .should_select_at_most
                    .clone()
                    .map(|s| s.to_numbas(locale).unwrap())
                    .flatten(),
                //.map(|a| a)
                //.unwrap_or(None),
                min_marks: Some(0usize), // todo?
                max_marks: Some(0usize.into()),
                shuffle_answers: self.shuffle_answers.unwrap(),
                choices,
                display_columns: self.columns.unwrap().into(),
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix,
                distractors,
            })
        } else {
            Err(check)
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
        answer_data: MultipleChoiceMatchAnswerData,
        shuffle_answers: bool,
        shuffle_items: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
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
    fn to_numbas(&self, locale: &str) -> NumbasResult<String> {
        // TODO: check if translation exists?
        Ok(self.clone().to_string(locale).unwrap())
        /*self
        .iter()
        .map(|a| a.clone().unwrap().to_string(&locale).unwrap())
        .collect()
            */
    }
}

impl ToNumbas for QuestionPartMatchAnswersWithItems {
    type NumbasType = numbas::exam::ExamQuestionPartMatchAnswersWithChoices;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            let (answers, choices, marking_matrix) = match self.answer_data.unwrap() {
                MultipleChoiceMatchAnswerData::ItemBased(data) => (
                    VariableValued::Value(data.answers.clone())
                        .to_numbas(locale)
                        .unwrap(),
                    VariableValued::Value(
                        data.items
                            .clone()
                            .map(|v| {
                                v.iter()
                                    .map(|a| a.clone().unwrap().statement)
                                    .collect::<Vec<_>>()
                            })
                            .unwrap(),
                    )
                    .to_numbas(locale)
                    .unwrap(),
                    Some(
                        VariableValued::Value(
                            data.items // TODO: better handling
                                .unwrap(),
                        )
                        .map(|v| {
                            v.iter()
                                .map(|i| {
                                    data.answers
                                        .clone()
                                        .unwrap()
                                        .iter()
                                        .map(|a| {
                                            i.unwrap()
                                                .answer_marks
                                                .unwrap()
                                                .iter()
                                                .find(|am| am.answer.unwrap() == a.unwrap())
                                                .map_or_else(|| 0usize.into(), |v| v.marks.unwrap())
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .to_numbas(locale)
                        .unwrap(),
                    ),
                ),
                MultipleChoiceMatchAnswerData::NumbasLike(data) => (
                    data.answers.to_numbas(locale).unwrap(),
                    data.choices.to_numbas(locale).unwrap(),
                    Some(data.marks.to_numbas(locale).unwrap()),
                ),
            };
            Ok(numbas::exam::ExamQuestionPartMatchAnswersWithChoices {
                part_data: self.to_numbas_shared_data(locale),
                min_answers: Some(self.should_select_at_least.clone().unwrap()),
                max_answers: self
                    .should_select_at_most
                    .clone()
                    .map(|s| s.to_numbas(locale).unwrap())
                    .flatten(),
                min_marks: Some(0),
                max_marks: Some(0),
                shuffle_answers: self.shuffle_answers.unwrap(),
                shuffle_choices: self.shuffle_items.unwrap(),
                answers,
                choices,
                wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
                layout: self.layout.clone().unwrap(),
                show_cell_answer_state: self.show_cell_answer_state.unwrap(),
                marking_matrix,
                display_type: self.display.unwrap().to_numbas(locale).unwrap(),
            })
        } else {
            Err(check)
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

impl ToNumbas for MatchAnswerWithItemsDisplay {
    type NumbasType = numbas::exam::MatchAnswersWithChoicesDisplayType;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            MatchAnswerWithItemsDisplay::Check => {
                numbas::exam::MatchAnswersWithChoicesDisplayType::Check
            }
            MatchAnswerWithItemsDisplay::Radio => {
                numbas::exam::MatchAnswersWithChoicesDisplayType::Radio
            }
        })
    }
}

impl ToRumbas for numbas::exam::MatchAnswersWithChoicesDisplayType {
    type RumbasType = MatchAnswerWithItemsDisplay;
    fn to_rumbas(&self) -> Self::RumbasType {
        match self {
            numbas::exam::MatchAnswersWithChoicesDisplayType::Check => {
                MatchAnswerWithItemsDisplay::Check
            }
            numbas::exam::MatchAnswersWithChoicesDisplayType::Radio => {
                MatchAnswerWithItemsDisplay::Radio
            }
        }
    }
}

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum MultipleChoiceMatchAnswerData {
        ItemBased(MultipleChoiceMatchAnswers),
        NumbasLike(MultipleChoiceMatchAnswerDataNumbasLike)
    }
}

optional_overwrite! {
    pub struct MultipleChoiceMatchAnswerDataNumbasLike {
        answers: VariableValued<Vec<TranslatableString>>,
        choices: VariableValued<Vec<TranslatableString>>,
        marks: VariableValued<Vec<Vec<numbas::exam::Primitive>>>
    }
}

optional_overwrite! {
    pub struct MultipleChoiceMatchAnswers {
        /// Values of the answers
        answers: Vec<Value<TranslatableString>>,
        /// Items for which the answer can be selected
        items: Vec<Value<MatchAnswersItem>>
    }
}
