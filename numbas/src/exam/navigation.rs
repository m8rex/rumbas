use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Navigation {
    #[serde(rename = "startpassword")]
    pub start_password: Option<String>, //TODO: if empty string -> also None
    #[serde(rename = "allowregen")]
    pub allow_regenerate: bool,
    #[serde(flatten)]
    pub navigation_mode: NavigationMode,
    #[serde(rename = "allowsteps")]
    pub allow_steps: Option<bool>,
    #[serde(rename = "showfrontpage")]
    pub show_frontpage: bool,
    #[serde(rename = "preventleave")]
    pub confirm_when_leaving: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "navigatemode")]
pub enum NavigationMode {
    #[serde(rename = "sequence")]
    Sequential(NavigationModeSequential),
    Menu,
    Diagnostic(NavigationModeDiagnostic),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct NavigationModeSequential {
    #[serde(rename = "onleave")]
    pub on_leave: LeaveAction,
    #[serde(rename = "showresultspage")]
    pub show_results_page: ShowResultsPage,
    #[serde(rename = "reverse")]
    pub can_move_to_previous: bool,
    #[serde(rename = "browse")]
    pub browsing_enabled: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct NavigationModeDiagnostic {
    #[serde(rename = "onleave")]
    pub on_leave: LeaveAction,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "action")]
pub enum LeaveAction {
    #[serde(rename = "none")]
    None { message: String }, //This message doesn't do anything
    #[serde(rename = "warnifunattempted")]
    WarnIfNotAttempted { message: String }, // Show a warning message if a user moves away from a question that is not attempted
    #[serde(rename = "preventifunattempted")]
    PreventIfNotAttempted { message: String }, // Prevent a user from moving away from a question that is not attempted
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ShowResultsPage {
    #[serde(rename = "oncompletion")]
    OnCompletion,
    #[serde(rename = "never")]
    Never,
}
