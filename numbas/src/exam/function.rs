use crate::jme::JMEString;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Function {
    //TODO
    pub parameters: Vec<FunctionParameter>,
    #[serde(rename = "type")]
    pub output_type: FunctionType,
    #[serde(flatten)]
    pub definition: FunctionDefinition,
}

pub type FunctionParameter = (String, FunctionType);

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "language")]
pub enum FunctionDefinition {
    #[serde(rename = "jme")]
    JME { definition: JMEString },
    #[serde(rename = "javascript")]
    Javascript { definition: String },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Copy)]
pub enum FunctionType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "dict")]
    Dict,
    #[serde(rename = "expression")]
    Expression,
    #[serde(rename = "html")]
    HTML,
    #[serde(rename = "integer")]
    Natural,
    #[serde(rename = "keypair")]
    KeyPair,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "matrix")]
    Matrix,
    #[serde(rename = "nothing")]
    Nothing,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "rational")]
    Rational,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "string")]
    r#String,
    #[serde(rename = "vector")]
    Vector,
    #[serde(rename = "ggbapplet")]
    ExtensionGeogebraApplet,
}
