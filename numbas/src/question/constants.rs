use crate::jme::JMEString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct BuiltinConstants(pub std::collections::HashMap<String, bool>);

impl Default for BuiltinConstants {
    fn default() -> Self {
        BuiltinConstants(
            vec![
                ("e".to_string(), false),
                ("pi,\u{03c0}".to_string(), false),
                ("i".to_string(), false),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionConstant {
    pub name: String,
    pub value: JMEString,
    pub tex: String,
}
