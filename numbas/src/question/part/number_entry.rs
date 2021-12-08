use crate::jme::JMEString;
use crate::question::part::QuestionPartSharedData;
use crate::support::answer_style::AnswerStyle;
use crate::support::primitive::{Number, SafeNatural};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

//TODO: docs https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/numberentry.js#L101
#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartNumberEntry {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "correctAnswerFraction")]
    pub correct_answer_fraction: bool,
    #[serde(rename = "correctAnswerStyle")]
    pub correct_answer_style: Option<AnswerStyle>,
    #[serde(rename = "allowFractions")]
    pub allow_fractions: bool,
    #[serde(rename = "notationStyles")]
    pub notation_styles: Option<Vec<AnswerStyle>>,
    /*#[serde(rename = "checkingType")]
    pub checking_type: Option<CheckingType>,*/ //TODO: check if being used
    /*#[serde(rename = "inputStep")]
    pub input_step: Option<usize>,*/ //TODO: check if being used
    #[serde(rename = "mustBeReduced")]
    pub fractions_must_be_reduced: Option<bool>,
    #[serde(rename = "mustBeReducedPC")]
    pub partial_credit_if_fraction_not_reduced: Option<Number>,
    #[serde(flatten)]
    pub precision: Option<QuestionPrecision>,
    #[serde(rename = "showPrecisionHint")]
    pub show_precision_hint: Option<bool>,
    #[serde(rename = "showFractionHint")]
    pub show_fraction_hint: Option<bool>,
    #[serde(flatten)]
    pub answer: NumberEntryAnswerType,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum CheckingType {
    #[serde(rename = "range")]
    Range,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NumberEntryAnswerType {
    MinMax {
        #[serde(rename = "minValue")]
        min_value: JMEString,
        #[serde(rename = "maxValue")]
        max_value: JMEString,
    },
    Answer {
        answer: JMEString,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPrecision {
    #[serde(rename = "precisionType")]
    precision_type: String, //TODO: enum ('none',...)
    #[serde(rename = "precision")]
    precision: SafeNatural,
    #[serde(rename = "precisionPartialCredit")]
    precision_partial_credit: SafeNatural,
    #[serde(rename = "precisionMessage")]
    precision_message: String,
    #[serde(rename = "strictPrecision")]
    strict_precision: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QuestionPrecisionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dp")]
    DecimalPlaces,
    #[serde(rename = "sigfig")]
    SignificantFigures,
}
