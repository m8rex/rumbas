use crate::jme::JMEString;
use crate::question::part::QuestionPartSharedData;
use crate::support::primitive::{SafeNatural, VariableValued};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionPartMatrix {
    #[serde(flatten)]
    pub part_data: QuestionPartSharedData,
    #[serde(rename = "correctAnswer")]
    pub correct_answer: JMEString,
    #[serde(rename = "correctAnswerFractions")]
    pub correct_answer_fractions: bool,
    #[serde(rename = "numRows")]
    pub num_rows: VariableValued<SafeNatural>,
    #[serde(rename = "numColumns")]
    pub num_columns: VariableValued<SafeNatural>,
    #[serde(rename = "allowResize")]
    pub allow_resize: bool,
    #[serde(rename = "minColumns")]
    pub min_columns: VariableValued<usize>,
    #[serde(rename = "maxColumns")]
    pub max_columns: VariableValued<usize>,
    #[serde(rename = "minRows")]
    pub min_rows: VariableValued<usize>,
    #[serde(rename = "maxRows")]
    pub max_rows: VariableValued<usize>,
    #[serde(rename = "tolerance")]
    pub tolerance: f64,
    #[serde(rename = "markPerCell")]
    pub mark_per_cell: bool,
    #[serde(rename = "allowFractions")]
    pub allow_fractions: bool,
    //#[serde(flatten)]  // todo
    //precision: QuestionPrecision,
}
