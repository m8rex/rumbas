use crate::jme::JMEString;
use crate::question::part::QuestionPartSharedData;
use crate::support::answer_style::AnswerStyle;
use crate::support::primitive::{Number, SafeNatural, VariableValued};
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
    #[serde(default = "bool::default")]
    pub correct_answer_fraction: bool,
    #[serde(rename = "correctAnswerStyle", default)]
    pub correct_answer_style: AnswerStyle,
    #[serde(rename = "allowFractions")]
    #[serde(default = "bool::default")]
    pub allow_fractions: bool,
    #[serde(rename = "notationStyles", default = "default_notation_styles")]
    pub notation_styles: Vec<AnswerStyle>,
    /*#[serde(rename = "checkingType")]
    pub checking_type: Option<CheckingType>,*/ //TODO: check if being used
    /*#[serde(rename = "inputStep")]
    pub input_step: Option<usize>,*/ //TODO: check if being used
    #[serde(rename = "mustBeReduced", default)]
    pub fractions_must_be_reduced: bool,
    #[serde(rename = "mustBeReducedPC", default)]
    pub partial_credit_if_fraction_not_reduced: Number,
    #[serde(flatten, default)]
    pub precision: QuestionPrecision,
    #[serde(rename = "showPrecisionHint", default="crate::util::bool_true")]
    pub show_precision_hint: bool,
    #[serde(rename = "showFractionHint", default="crate::util::bool_true")]
    pub show_fraction_hint: bool,
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
        #[serde(rename = "minValue", alias="minvalue")]
        min_value: JMEString,
        #[serde(rename = "maxValue", alias="maxvalue")]
        max_value: JMEString,
    },
    Answer {
        answer: JMEString,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPrecision {
    #[serde(rename = "precisionType", default)]
    precision_type: QuestionPrecisionType,
    #[serde(rename = "precision", default)]
    precision: VariableValued<SafeNatural>,
    #[serde(rename = "precisionPartialCredit", default)]
    precision_partial_credit: SafeNatural,
    #[serde(rename = "precisionMessage", default)]
    precision_message: String,
    #[serde(rename = "strictPrecision", default="crate::util::bool_true")]
    strict_precision: bool,
}

impl std::default::Default for QuestionPrecision {
    fn default() -> Self {
        Self { precision_type: Default::default(), precision: VariableValued::Value(0.into()), precision_partial_credit: 0.into(), precision_message: String::new(), strict_precision: true }
    }
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

impl std::default::Default for QuestionPrecisionType {
    fn default() -> Self {
        Self::None
    }
}

fn default_notation_styles() -> Vec<AnswerStyle> {
    vec![
        AnswerStyle::English, AnswerStyle::EnglishSI, AnswerStyle::EnglishPlain
    ]
}