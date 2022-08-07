use crate::jme::EmbracedJMEString;
use crate::jme::JMEString;
use crate::question::answer_simplification::{AnswerSimplificationRule, AnswerSimplificationType};
use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::{SafeFloat, SafeNatural};
use crate::support::serde_functions::{
    answer_simplification_deserialize_string, answer_simplification_serialize_string,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

pub fn default_answer_simplification() -> Vec<AnswerSimplificationType> {
    let v: Vec<AnswerSimplificationRule> = vec![
        AnswerSimplificationRule::Basic(true),
        AnswerSimplificationRule::CancelUnitFactors(true),
        AnswerSimplificationRule::CancelUnitPowers(true),
        AnswerSimplificationRule::CancelUnitDenominators(true),
        AnswerSimplificationRule::CancelZeroFactors(true),
        AnswerSimplificationRule::OmitZeroTerms(true),
        AnswerSimplificationRule::CancelZeroFactors(true),
        AnswerSimplificationRule::OmitZeroTerms(true),
        AnswerSimplificationRule::CancelZeroPowers(true),
        AnswerSimplificationRule::NoLeadingMinus(true),
        AnswerSimplificationRule::CollectNumbers(true),
        AnswerSimplificationRule::Fractions(true),
        AnswerSimplificationRule::CancelPowersWithBaseZero(true),
        AnswerSimplificationRule::ConstantsFirst(true),
        AnswerSimplificationRule::CollectSqrtProducts(true),
        AnswerSimplificationRule::CollectSqrtDivisions(true),
        AnswerSimplificationRule::CancelSqrtSquares(true),
        AnswerSimplificationRule::Trigonometric(true),
        AnswerSimplificationRule::EvaluatePowersOfNumbers(true),
        AnswerSimplificationRule::CollectTerms(true),
        AnswerSimplificationRule::CollectPowersOfCommonFactors(true),
        AnswerSimplificationRule::CollectLikeFractions(true),
    ];
    v.into_iter()
        .map(AnswerSimplificationType::Rule)
        .collect::<Vec<_>>()
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartJME {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    pub answer: EmbracedJMEString,
    #[serde(
        rename = "answerSimplification",
        default = "default_answer_simplification",
        deserialize_with = "answer_simplification_deserialize_string",
        serialize_with = "answer_simplification_serialize_string"
    )]
    pub answer_simplification: Vec<AnswerSimplificationType>, //comma separated list
    #[serde(rename = "showPreview")]
    #[serde(alias = "showpreview")]
    #[serde(default = "crate::util::bool_true")]
    pub show_preview: bool,
    #[serde(rename = "checkingType")]
    #[serde(alias = "checkingtype")]
    #[serde(flatten, default)]
    pub checking_type: JMECheckingType,
    /// If the comparison fails this many times or more, the student’s answer is marked as wrong.
    #[serde(rename = "failureRate")]
    #[serde(default = "crate::util::float_one")]
    pub failure_rate: f64,
    #[serde(rename = "vsetRange")]
    #[serde(alias = "vsetrange")]
    #[serde(default = "default_vset_range")]
    pub vset_range: [SafeFloat; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    #[serde(rename = "vsetRangePoints")]
    #[serde(alias = "vsetrangepoints")]
    #[serde(default = "default_vset_points")]
    pub vset_range_points: SafeNatural,
    #[serde(rename = "checkVariableNames")]
    #[serde(alias = "checkvariablenames")]
    #[serde(default)]
    pub check_variable_names: bool,
    #[serde(rename = "singleLetterVariables")]
    #[serde(default)]
    pub single_letter_variables: bool,
    #[serde(rename = "allowUnknownFunctions")]
    #[serde(default = "crate::util::bool_true")]
    pub allow_unknown_functions: bool,
    #[serde(rename = "implicitFunctionComposition")]
    #[serde(default)]
    pub implicit_function_composition: bool,
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
    #[serde(rename = "valuegenerators", default)]
    pub value_generators: Vec<JMEValueGenerator>,
}

fn default_vset_range() -> [SafeFloat; 2] {
    [0.0.into(), 1.0.into()]
}

fn default_vset_points() -> SafeNatural {
    5.into()
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

impl std::default::Default for JMECheckingType {
    fn default() -> Self {
        Self::RelativeDifference(JMECheckingTypeData {
            checking_accuracy: 0.0.into(),
        })
    }
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
