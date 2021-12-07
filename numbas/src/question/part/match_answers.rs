use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::Primitive;
use crate::support::primitive::SafeNatural;
use crate::support::primitive::VariableValued;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartMatchAnswersWithChoices {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "minMarks")]
    pub min_marks: Option<SafeNatural>, //TODO; what is difference with minimum_marks? -> not for 1_n_2
    #[serde(rename = "maxMarks")]
    pub max_marks: Option<SafeNatural>, // Is there a maximum number of marks the student can get? -> not for 1_n_2
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<SafeNatural>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<SafeNatural>, // Maximum number of responses the student can select -> always one for 1_n_2
    #[serde(rename = "shuffleChoices")]
    pub shuffle_choices: bool,
    #[serde(rename = "shuffleAnswers")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: MatchAnswersWithChoicesDisplayType, // How to display the response selectors -> only for 1_n_2?
    //#[serde(rename = "displayColumns")] //TODO?
    //pub displayed_columns: usize, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: MultipleChoiceWarningType, // What to do if the student picks the wrong number of responses?
    pub layout: MatchAnswersWithChoicesLayout,
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: VariableValued<Vec<String>>, // todo jme
    pub answers: VariableValued<Vec<String>>, // todo jme
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<Vec<Primitive>>>>, // Marks for each answer/choice pair. Arranged as `matrix[choice][answer]
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesDisplayType {
    #[serde(rename = "checkbox")]
    Check,
    #[serde(rename = "radiogroup")]
    Radio,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Copy)]
pub enum MultipleChoiceWarningType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "prevent")]
    Prevent,
    //TODO: also prevent and warn -> same as leave actions?
    //https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L493
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesLayoutType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "lowertriangle")]
    LowerTriangle,
    //TODO: https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L766
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
/// Define which choices are available to be picked. If Custom expression is selected, give either a list of lists of boolean values, or a matrix with as many rows as the part has choices and as many columns as the part has answers. Any non-zero value in the matrix indicates that the corresponding choice-answer pair should be available to the student.
pub struct MatchAnswersWithChoicesLayout {
    pub r#type: MatchAnswersWithChoicesLayoutType,
    pub expression: String, // TODO: expression only needed for custom type?
}

/* TODO: remove */
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MultipleChoiceMatrix {
    //TODO use specific type for the three types
    Item(Primitive),
    Row(Vec<Primitive>),
    Matrix(Vec<VariableValued<Vec<Primitive>>>),
}
