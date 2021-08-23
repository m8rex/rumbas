use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Navigation {
    #[serde(rename = "allowregen")]
    pub allow_regenerate: bool,
    #[serde(rename = "showfrontpage")]
    pub show_frontpage: bool,
    #[serde(rename = "preventleave")]
    pub confirm_when_leaving: Option<bool>,
}
