use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

//TODO: defaults
question_part_type! {
    QuestionPartChooseOne,
    answers: Vec<MultipleChoiceAnswer>,
    shuffle_answers: bool,
    show_cell_answer_state: bool,
    should_select_at_least: usize, // TODO 0 or 1?
    display: ChooseOneDisplay: serde(flatten)
    //TODO wrong_nb_choices_warning:
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
                    answers
                        .iter()
                        .map(|a| numbas::exam::Primitive::Float(a.marks.clone().unwrap()))
                        .collect(),
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

optional_overwrite! {
    MultipleChoiceAnswer,
    statement: TranslatableString,
    feedback: TranslatableString,
    marks: f64 //TODO; float or not?
}

question_part_type! {
    QuestionPartChooseMultiple,
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
                    answers
                        .iter()
                        .map(|a| numbas::exam::Primitive::Float(a.marks.clone().unwrap()))
                        .collect(),
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
