use crate::jme::EmbracedJMEString;
use crate::jme::JMEString;
use crate::question::answer_simplification::AnswerSimplificationType;
use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::{SafeFloat, SafeNatural};
use crate::support::serde_functions::{
    answer_simplification_deserialize_string, answer_simplification_serialize_string,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartJME {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    pub answer: EmbracedJMEString,
    #[serde(
        rename = "answerSimplification",
        default,
        deserialize_with = "answer_simplification_deserialize_string",
        serialize_with = "answer_simplification_serialize_string"
    )]
    pub answer_simplification: Option<Vec<AnswerSimplificationType>>, //comma separated list
    #[serde(rename = "showPreview")]
    #[serde(alias = "showpreview")]
    pub show_preview: bool,
    #[serde(rename = "checkingType")]
    #[serde(alias = "checkingtype")]
    #[serde(flatten)]
    pub checking_type: JMECheckingType,
    /// If the comparison fails this many times or more, the student’s answer is marked as wrong.
    #[serde(rename = "failureRate")]
    pub failure_rate: Option<f64>,
    #[serde(rename = "vsetRange")]
    #[serde(alias = "vsetrange")]
    pub vset_range: [SafeFloat; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    #[serde(rename = "vsetRangePoints")]
    #[serde(alias = "vsetrangepoints")]
    pub vset_range_points: SafeNatural,
    #[serde(rename = "checkVariableNames")]
    #[serde(alias = "checkvariablenames")]
    pub check_variable_names: bool,
    #[serde(rename = "singleLetterVariables")]
    pub single_letter_variables: Option<bool>,
    #[serde(rename = "allowUnknownFunctions")]
    pub allow_unknown_functions: Option<bool>,
    #[serde(rename = "implicitFunctionComposition")]
    pub implicit_function_composition: Option<bool>,
    #[serde(rename = "maxlength")]
    pub max_length: Option<JMELengthRestriction>, // TODO: all restrictions to one flattended struct?
    #[serde(rename = "minlength")]
    pub min_length: Option<JMELengthRestriction>,
    #[serde(rename = "musthave")]
    pub must_have: Option<JMEStringRestriction>,
    #[serde(rename = "notallowed")]
    pub may_not_have: Option<JMEStringRestriction>,
    #[serde(rename = "mustmatchpattern")]
    pub must_match_pattern: Option<JMEPatternRestriction>,
    #[serde(rename = "valuegenerators")]
    pub value_generators: Option<Vec<JMEValueGenerator>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct JMECheckingTypeData<T> {
    #[serde(rename = "checkingAccuracy")]
    #[serde(alias = "checkingaccuracy")]
    pub checking_accuracy: T,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "checkingType")]
pub enum JMECheckingType {
    #[serde(rename = "reldiff")]
    RelativeDifference(JMECheckingTypeData<SafeFloat>),
    #[serde(rename = "absdiff")]
    AbsoluteDifference(JMECheckingTypeData<SafeFloat>),
    #[serde(rename = "dp")]
    DecimalPlaces(JMECheckingTypeData<usize>),
    #[serde(rename = "sigfig")]
    SignificantFigures(JMECheckingTypeData<usize>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct JMERestriction {
    //pub name: String,
    #[serde(rename = "partialCredit")]
    pub partial_credit: SafeFloat, //TODO: maybe SafeNatural?
    pub message: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct JMELengthRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    pub length: Option<SafeNatural>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct JMEStringRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    #[serde(rename = "showStrings")]
    pub show_strings: bool,
    pub strings: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct JMEPatternRestriction {
    #[serde(rename = "partialCredit")]
    pub partial_credit: SafeFloat, //TODO: maybe SafeNatural?
    pub message: String,
    pub pattern: String, //TODO type?
    #[serde(rename = "nameToCompare")]
    pub name_to_compare: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct JMEValueGenerator {
    pub name: String,
    pub value: JMEString,
}
