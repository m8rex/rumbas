use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Feedback {
    #[serde(rename = "showactualmark")]
    #[serde(default = "crate::util::bool_true")]
    pub show_actual_mark: bool, // show student's score
    #[serde(rename = "showtotalmark")]
    #[serde(default = "crate::util::bool_true")]
    pub show_total_mark: bool, // show total marks available
    #[serde(rename = "showanswerstate")]
    #[serde(default = "crate::util::bool_true")]
    pub show_answer_state: bool, // Show whether answer was correct
    #[serde(rename = "allowrevealanswer")]
    #[serde(default = "crate::util::bool_true")]
    pub allow_reveal_answer: bool,
    #[serde(flatten)]
    pub review: Review,
    pub advice: Option<String>,
    #[serde(default)]
    pub intro: String,
    #[serde(rename = "feedbackmessages")]
    #[serde(default)]
    pub feedback_messages: Vec<FeedbackMessage>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Review {
    #[serde(rename = "reviewshowscore")]
    #[serde(default = "crate::util::bool_true")]
    pub show_score: bool,
    #[serde(rename = "reviewshowfeedback")]
    #[serde(default = "crate::util::bool_true")]
    pub show_feedback: bool,
    #[serde(rename = "reviewshowexpectedanswer")]
    #[serde(default = "crate::util::bool_true")]
    pub show_expected_answer: bool,
    #[serde(rename = "reviewshowadvice")]
    #[serde(default = "crate::util::bool_true")]
    pub show_advice: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    pub message: String,
    pub threshold: String, //TODO type
}
