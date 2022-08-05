use crate::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use crate::question::part::match_answers::MultipleChoiceWarningType;
use crate::question::part::QuestionPartSharedData;
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
    #[serde(default)]
    /// If the student would have scored less than this many marks, they are instead awarded this many. Useful in combination with negative marking.
    pub min_marks: SafeNatural, //TODO; what is difference with minimum_marks?
    #[serde(rename = "maxMarks")]
    #[serde(default)]
    /// If the student would have scored more than this many marks, they are instead awarded this many. The value 0 means “no maximum mark”.
    pub max_marks: SafeNatural, // Is there a maximum number of marks the student can get?
    #[serde(rename = "minAnswers")]
    #[serde(default)]
    /// For choose several from a list and match choices with answers parts, the student must select at least this many choices. The value 0 means “no minimum”, though the student must make at least one choice to submit the part.
    pub min_answers: SafeNatural, // Minimum number of responses the student can select
    #[serde(rename = "maxAnswers")]
    #[serde(default)]
    /// For choose several from a list and match choices with answers parts, the student must select at most this many choices. The value 0 means “no maximum”.
    pub max_answers: SafeNatural, // Maximum number of responses the student can select
    #[serde(rename = "warningType", default)]
    pub wrong_nb_answers_warning: MultipleChoiceWarningType, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "shuffleChoices")]
    #[serde(default)]
    pub shuffle_answers: bool,
    #[serde(rename = "displayColumns")]
    #[serde(default = "crate::util::safe_natural_one")]
    pub display_columns: SafeNatural, // How many columns to use to display the choices.

    #[serde(rename = "showCellAnswerState")]
    #[serde(default = "crate::util::bool_true")]
    /// If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If not ticked, the ticked choices will be given a neutral highlight regardless of their scores.
    pub show_cell_answer_state: bool,

    /// This is either a list of embraced jme strings or a jme expression
    #[serde(default)]
    pub choices: VariableValued<Vec<ContentAreaString>>,
    #[serde(rename = "matrix")]
    #[serde(default)]
    /// This is either a list of jme strings or a jme expression
    pub marking_matrix: VariableValued<Vec<JMEString>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    /// This is optional if marking_matrix# is a JMEString
    #[serde(default)]
    pub distractors: Vec<ContentAreaString>,
    /// This determines how the student’s score is determined, based on their selections and the marking matrix.
    #[serde(rename = "markingMethod")]
    #[serde(default)]
    pub marking_method: MultipleChoiceMarkingMethod,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MultipleChoiceMarkingMethod {
    #[serde(rename = "sum ticked cells")]
    SumTickedCells,
    #[serde(rename = "score per matched cell")]
    ScorePerMatchedCell,
    #[serde(rename = "all-or-nothing")]
    AllOrNothing,
}

impl Default for MultipleChoiceMarkingMethod {
    fn default() -> Self {
        Self::SumTickedCells // only option before more marking methods were added }
    }
}
