use crate::question::part::match_answers::MultipleChoiceWarningType;
use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::Primitive;
use crate::support::primitive::SafeNatural;
use crate::support::primitive::VariableValued;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartChooseMultiple {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "minMarks")]
    pub min_marks: Option<usize>, //TODO; what is difference with minimum_marks?
    #[serde(rename = "maxMarks")]
    pub max_marks: Option<SafeNatural>, // Is there a maximum number of marks the student can get?
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<SafeNatural>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<SafeNatural>, // Maximum number of responses the student can select
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayColumns")]
    pub display_columns: SafeNatural, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: MultipleChoiceWarningType, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: VariableValued<Vec<String>>, // todo jme?
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<Primitive>>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    pub distractors: Option<Vec<String>>, // todo jme?
}
