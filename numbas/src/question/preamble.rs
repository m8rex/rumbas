use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Preamble {
    pub js: String,
    pub css: String,
}

impl std::default::Default for Preamble {
    fn default() -> Self {
        Self { js: "".to_string(), css: "".to_string() }
    }
}