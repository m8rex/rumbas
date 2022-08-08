use crate::jme::EmbracedJMEString;
use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::SafeFloat;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartPatternMatch {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "caseSensitive", default)]
    pub case_sensitive: bool,
    #[serde(rename = "partialCredit", default)]
    /// Partial credit for answer not matching case
    pub partial_credit: SafeFloat,
    /// The text or pattern the student must match.
    /// When Match test is Regular expression, this is a regular expression defining the strings to be accepted as correct. If there are several valid answers, separate them with a | character. If you’re using the full regular expression functionality, note that ^ and $ are automatically added to the start and end of the answer pattern to ensure that the student’s whole answer matches the pattern.
    pub answer: EmbracedJMEString,
    #[serde(rename = "displayAnswer")]
    // Only a value when Regex pattern mode
    pub display_answer: Option<EmbracedJMEString>,
    #[serde(rename = "matchMode", default)]
    pub match_mode: PatternMatchMode,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternMatchMode {
    #[serde(rename = "regex")]
    Regex, // TODO: only here we need the display_answer
    #[serde(rename = "exact")]
    Exact,
}

impl std::default::Default for PatternMatchMode {
    fn default() -> PatternMatchMode {
        Self::Regex
    }
}
