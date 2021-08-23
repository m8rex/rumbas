use crate::question::part::QuestionPartSharedData;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartInformation {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
}
