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
    #[serde(default)]
    pub correct_answer_fractions: bool,
    #[serde(rename = "numRows", default="crate::util::variable_safe_natural_three")]
    pub num_rows: VariableValued<SafeNatural>,
    #[serde(rename = "numColumns", default="crate::util::variable_safe_natural_three")]
    pub num_columns: VariableValued<SafeNatural>,
    #[serde(rename = "allowResize", default="crate::util::bool_true")]
    pub allow_resize: bool,
    #[serde(rename = "minColumns", default)]
    pub min_columns: VariableValued<usize>,
    #[serde(rename = "maxColumns", default)]
    pub max_columns: VariableValued<usize>,
    #[serde(rename = "minRows", default)]
    pub min_rows: VariableValued<usize>,
    #[serde(rename = "maxRows", default)]
    pub max_rows: VariableValued<usize>,
    #[serde(rename = "tolerance", default)]
    pub tolerance: f64,
    #[serde(rename = "markPerCell", default)]
    pub mark_per_cell: bool,
    #[serde(rename = "allowFractions")]
    #[serde(default)]
    pub allow_fractions: bool,
    //#[serde(flatten)]  // todo
    //precision: QuestionPrecision,
}
