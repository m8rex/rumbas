use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct Timing {
    #[serde(rename = "allowPause", default = "crate::util::bool_true")]
    pub allow_pause: bool,
    #[serde(default)]
    pub timeout: TimeoutAction, // Action to do on timeout
    #[serde(rename = "timedwarning", default)]
    pub timed_warning: TimeoutAction, // Action to do five minutes before timeout
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "action")]
pub enum TimeoutAction {
    #[serde(rename = "none")]
    None { message: String }, //This message doesn't do anything
    #[serde(rename = "warn")]
    Warn { message: String }, // Show a warning message
}

impl std::default::Default for TimeoutAction {
    fn default() -> Self {
        Self::None {
            message: String::new(),
        }
    }
}
