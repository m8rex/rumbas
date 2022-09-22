use crate::jme::ContentAreaString;
use crate::jme::JMENotesString;
use crate::support::primitive::Number;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Error;
use serde_with::skip_serializing_none;

pub mod choose_multiple;
pub mod choose_one;
pub mod extension;
pub mod gapfill;
pub mod information;
pub mod jme;
pub mod match_answers;
pub mod matrix;
pub mod number_entry;
pub mod pattern_match;

use choose_multiple::QuestionPartChooseMultiple;
use choose_one::QuestionPartChooseOne;
use extension::QuestionPartExtension;
use gapfill::QuestionPartGapFill;
use information::QuestionPartInformation;
use jme::QuestionPartJME;
use match_answers::QuestionPartMatchAnswersWithChoices;
use matrix::QuestionPartMatrix;
use number_entry::QuestionPartNumberEntry;
use pattern_match::QuestionPartPatternMatch;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(try_from = "std::collections::BTreeMap<String, serde_json::Value>")]
#[serde(untagged)]
pub enum QuestionPart {
    Builtin(QuestionPartBuiltin),
    Custom(QuestionPartCustom),
}
impl std::convert::TryFrom<std::collections::BTreeMap<String, serde_json::Value>> for QuestionPart {
    type Error = serde_json::Error;
    fn try_from(
        map: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Result<QuestionPart, Error> {
        if let Some(serde_json::Value::String(r#type)) = map.get("type") {
            if [
                "jme",
                "numberentry",
                "matrix",
                "patternmatch",
                "1_n_2",
                "m_n_2",
                "m_n_x",
                "gapfill",
                "information",
                "extension",
            ]
            .contains(&&r#type[..])
            {
                QuestionPartBuiltin::deserialize(serde_json::Value::Object(
                    map.into_iter().collect(),
                ))
                .map_err(serde::de::Error::custom)
                .map(QuestionPart::Builtin)
            } else {
                QuestionPartCustom::deserialize(serde_json::Value::Object(
                    map.into_iter().collect(),
                ))
                .map_err(serde::de::Error::custom)
                .map(QuestionPart::Custom)
            }
        } else {
            Err(serde::de::Error::custom("Missing type field"))
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum QuestionPartBuiltin {
    #[serde(rename = "jme")]
    JME(QuestionPartJME),
    #[serde(rename = "numberentry")]
    NumberEntry(QuestionPartNumberEntry),
    #[serde(rename = "matrix")]
    Matrix(Box<QuestionPartMatrix>),
    #[serde(rename = "patternmatch")]
    PatternMatch(QuestionPartPatternMatch),
    #[serde(rename = "1_n_2")]
    ChooseOne(QuestionPartChooseOne),
    #[serde(rename = "m_n_2")]
    ChooseMultiple(QuestionPartChooseMultiple),
    #[serde(rename = "m_n_x")]
    MatchAnswersWithChoices(QuestionPartMatchAnswersWithChoices),
    #[serde(rename = "gapfill")]
    GapFill(QuestionPartGapFill),
    #[serde(rename = "information")]
    Information(QuestionPartInformation),
    #[serde(rename = "extension")]
    Extension(QuestionPartExtension),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartSharedData {
    /// A content area used to prompt the student for an answer.
    #[serde(default)]
    pub prompt: ContentAreaString, //TODO option? Maybe not in this type, but in other. Some types require this, other's not?
    /// The number of marks to award for answering the part correctly.
    pub marks: Number,
    /// An optional list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.
    #[serde(default)]
    pub steps: Vec<QuestionPart>,
    #[serde(rename = "stepsPenalty", default)]
    /// If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3×46=2 marks after revealing steps.
    pub steps_penalty: Number,
    #[serde(rename = "showCorrectAnswer")]
    #[serde(default = "crate::util::bool_true")]
    /// When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.
    pub show_correct_answer: bool,
    #[serde(rename = "showFeedbackIcon")]
    #[serde(default = "crate::util::bool_true")]
    /// After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.
    pub show_feedback_icon: bool,
    // TODO: "Score_counts_toward_objective"
    #[serde(rename = "customMarkingAlgorithm")]
    #[serde(default)]
    pub custom_marking_algorithm: JMENotesString,
    #[serde(rename = "extendBaseMarkingAlgorithm")]
    #[serde(default = "crate::util::bool_true")]
    /// If this is ticked, all marking notes provided by the part’s standard marking algorithm will be available. If the same note is defined in both the standard algorithm and your custom algorithm, your version will be used.
    pub extend_base_marking_algorithm: bool,

    // TODO below not listed in
    // https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html?highlight=content%20area#generic-part-properties
    #[serde(rename = "useCustomName")]
    #[serde(default)]
    pub use_custom_name: bool,
    #[serde(rename = "customName")]
    #[serde(default)]
    pub custom_name: String,
    #[serde(rename = "enableMinimumMarks")]
    #[serde(default = "crate::util::bool_true")]
    pub enable_minimum_marks: bool,
    #[serde(rename = "minimumMarks")]
    #[serde(default)]
    pub minimum_marks: usize,

    #[serde(rename = "variableReplacementStrategy")]
    #[serde(default)]
    /// The circumstances under which the variable replacements are used, and adaptive marking is applied.
    pub variable_replacement_strategy: VariableReplacementStrategy,
    #[serde(rename = "adaptiveMarkingPenalty")]
    #[serde(default)]
    /// If adaptive marking is used, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for using adaptive marking is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without the penalty is instead worth 3×46=2 marks when adaptive marking is used.
    pub adaptive_marking_penalty: usize,
    //scripts TODO
    //https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html?highlight=content%20area#scripts
    //[serde(rename= "variableReplacements")]
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartCustom {
    pub r#type: String,
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(default)]
    pub settings: std::collections::BTreeMap<String, CustomPartInputTypeValue>,
}

// TODO: other types
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CustomPartInputTypeValue {
    CheckBox(bool),
    Code(String),
}

impl std::convert::From<CustomPartInputTypeValue> for String {
    fn from(v: CustomPartInputTypeValue) -> Self {
        match v {
            CustomPartInputTypeValue::CheckBox(v) => v.to_string(),
            CustomPartInputTypeValue::Code(v) => v,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "originalfirst")]
    /// The student’s answer is first marked using the original values of the question variables. If the credit given by this method is less than the maximum available, the marking is repeated using the defined variable replacements. If the credit gained with variable replacements is greater than the credit gained under the original marking, that score is used, and the student is told that their answers to previous parts have been used in the marking for this part.
    OriginalFirst,
    //#[serde(rename = "always")] // TODO: check name etc
    // /// The student’s answer is only marked once, with the defined variable replacements applied.
    //Always,
}

impl std::default::Default for VariableReplacementStrategy {
    fn default() -> Self {
        Self::OriginalFirst
    }
}
