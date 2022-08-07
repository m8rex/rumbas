use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Navigation {
    #[serde(rename = "allowregen", default)]
    pub allow_regenerate: bool,
    #[serde(rename = "showfrontpage", default = "crate::util::bool_true")]
    pub show_frontpage: bool,
    #[serde(rename = "preventleave", default = "crate::util::bool_true")]
    pub confirm_when_leaving: bool,
}
