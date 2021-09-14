use crate::question::Question;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionGroup {
    //TODO
    pub name: Option<String>,
    #[serde(flatten)]
    pub picking_strategy: QuestionGroupPickingStrategy,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "pickingStrategy")]
pub enum QuestionGroupPickingStrategy {
    #[serde(rename = "all-ordered")]
    AllOrdered,
    #[serde(rename = "all-shuffled")]
    AllShuffled,
    #[serde(rename = "random-subset")]
    RandomSubset {
        #[serde(rename = "pickQuestions")]
        pick_questions: usize,
    },
}
