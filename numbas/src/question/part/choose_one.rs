use crate::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use crate::question::part::QuestionPartSharedData;
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

    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: ChooseOneDisplayType, // How to display the response selectors
    #[serde(rename = "displayColumns")]
    ///For choose one from a list and choose several from a list parts, this dictates how many columns the choices are displayed in. If 0, the choices are displayed on a single line, wrapped at the edges of the screen.
    /// Can't be a variable / jme expression
    pub columns: SafeNatural, // How many columns to use to display the choices. Not usefull when dropdown -> optional?

    #[serde(rename = "showCellAnswerState")]
    /// If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If not ticked, the ticked choices will be given a neutral highlight regardless of their scores.
    pub show_cell_answer_state: Option<bool>,

    /// This is either a list of embraced jme strings or a jme expression
    pub choices: VariableValued<Vec<ContentAreaString>>,
    #[serde(rename = "matrix")]
    /// This is either a list of jme strings or a jme expression
    pub marking_matrix: Option<VariableValued<Vec<JMEString>>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    /// This is optional if marking_matrix is a JMEString
    pub distractors: Option<Vec<EmbracedJMEString>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ChooseOneDisplayType {
    #[serde(rename = "radiogroup")]
    Radio,
    #[serde(rename = "dropdownlist")]
    DropDown,
}
