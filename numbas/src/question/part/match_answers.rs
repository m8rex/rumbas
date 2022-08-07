use crate::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use crate::question::part::choose_multiple::MultipleChoiceMarkingMethod;
use crate::question::part::QuestionPartSharedData;
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

    #[serde(rename = "shuffleChoices", default)]
    pub shuffle_choices: bool,
    #[serde(rename = "shuffleAnswers", default)]
    pub shuffle_answers: bool,

    #[serde(flatten, default)]
    pub display_type: MatchAnswersWithChoicesDisplayType, // How to display the response selectors -> only for 1_n_2?
    //#[serde(rename = "displayColumns")] //TODO?
    //pub displayed_columns: usize, // How many columns to use to display the choices.
    #[serde(default)]
    pub layout: MatchAnswersWithChoicesLayout,

    #[serde(rename = "showCellAnswerState", default = "crate::util::bool_true")]
    /// If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If not ticked, the ticked choices will be given a neutral highlight regardless of their scores.
    pub show_cell_answer_state: bool,

    #[serde(default)]
    pub choices: VariableValued<Vec<ContentAreaString>>,
    #[serde(default)]
    pub answers: VariableValued<Vec<ContentAreaString>>,
    #[serde(rename = "matrix", default)]
    pub marking_matrix: VariableValued<Vec<Vec<JMEString>>>, // Marks for each answer/choice pair. Arranged as `matrix[choice][answer]
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "displayType")]
pub enum MatchAnswersWithChoicesDisplayType {
    #[serde(rename = "checkbox")]
    Check(MatchAnswersWithChoicesDisplayTypeCheck),
    #[serde(rename = "radiogroup")]
    Radio,
}

impl std::default::Default for MatchAnswersWithChoicesDisplayType {
    fn default() -> Self {
        Self::Radio
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct MatchAnswersWithChoicesDisplayTypeCheck {
    #[serde(rename = "markingMethod")]
    #[serde(default)]
    pub marking_method: MultipleChoiceMarkingMethod,
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

impl std::default::Default for MultipleChoiceWarningType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesLayoutType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "lowertriangle")]
    LowerTriangle,
    //TODO: https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L766
}

impl std::default::Default for MatchAnswersWithChoicesLayoutType {
    fn default() -> Self {
        Self::All
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
/// Define which choices are available to be picked. If Custom expression is selected, give either a list of lists of boolean values, or a matrix with as many rows as the part has choices and as many columns as the part has answers. Any non-zero value in the matrix indicates that the corresponding choice-answer pair should be available to the student.
pub struct MatchAnswersWithChoicesLayout {
    #[serde(default)]
    pub r#type: MatchAnswersWithChoicesLayoutType,
    pub expression: String, // TODO: expression only needed for custom type?
}

impl std::default::Default for MatchAnswersWithChoicesLayout {
    fn default() -> Self {
        Self {
            r#type: MatchAnswersWithChoicesLayoutType::All,
            expression: String::new(),
        }
    }
}
