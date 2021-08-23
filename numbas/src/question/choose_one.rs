use crate::question::match_answers::MultipleChoiceWarningType;
use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::Primitive;
use crate::support::primitive::SafeNatural;
use crate::support::primitive::VariableValued;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartChooseOne {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<usize>, // Minimum number of responses the student must select
    #[serde(rename = "choices")]
    pub answers: VariableValued<Vec<String>>, // TODO: jme?
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: ChooseOneDisplayType, // How to display the response selectors
    #[serde(rename = "displayColumns")]
    pub columns: SafeNatural, // How many columns to use to display the choices. Not usefull when dropdown -> optional? TODO
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses? TODO: not used for this type?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: Option<bool>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<Primitive>>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    //TODO: type (contains only strings...)
    pub distractors: Option<Vec<String>>, // TODO: jme?
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ChooseOneDisplayType {
    #[serde(rename = "radiogroup")]
    Radio,
    #[serde(rename = "dropdownlist")]
    DropDown,
}
