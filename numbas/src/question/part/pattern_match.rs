use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::Primitive;
use crate::support::primitive::SafeFloat;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartPatternMatch {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    #[serde(rename = "partialCredit")]
    pub partial_credit: Option<SafeFloat>,
    pub answer: Primitive,
    #[serde(rename = "displayAnswer")]
    pub display_answer: Option<Primitive>,
    #[serde(rename = "matchMode")]
    pub match_mode: PatternMatchMode,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq)]
pub enum PatternMatchMode {
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "exact")]
    Exact, //TODO: check all options
}
