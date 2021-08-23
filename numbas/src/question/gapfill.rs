use crate::question::part::QuestionPart;
use crate::question::part::QuestionPartSharedData;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartGapFill {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "sortAnswers")]
    pub sort_answers: Option<bool>,
    pub gaps: Vec<QuestionPart>,
}
