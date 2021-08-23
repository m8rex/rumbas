use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub definition: String, // TODO: jme?
    pub description: String,
    #[serde(rename = "templateType")]
    pub template_type: VariableTemplateType,
    pub group: String,
    /// If this is ticked, then when an exam uses this question the author can override the value
    /// of this variable with their own choice.
    #[serde(default)]
    pub can_override: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum VariableTemplateType {
    #[serde(rename = "anything")]
    Anything, //JME
    #[serde(rename = "list of numbers")]
    ListOfNumbers,
    #[serde(rename = "list of strings")]
    ListOfStrings,
    #[serde(rename = "long string")]
    LongString,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "randrange")]
    RandomRange, // Chooses a value from the range
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "string")]
    r#String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct VariableGroup {
    pub variables: Vec<String>,
    pub name: String,
}
