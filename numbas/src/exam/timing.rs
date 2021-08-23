use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Timing {
    #[serde(rename = "allowPause")]
    pub allow_pause: bool,
    pub timeout: TimeoutAction, // Action to do on timeout
    #[serde(rename = "timedwarning")]
    pub timed_warning: TimeoutAction, // Action to do five minutes before timeout
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "action")]
pub enum TimeoutAction {
    #[serde(rename = "none")]
    None { message: String }, //This message doesn't do anything
    #[serde(rename = "warn")]
    Warn { message: String }, // Show a warning message
}
