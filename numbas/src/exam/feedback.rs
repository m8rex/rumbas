use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Feedback {
    #[serde(rename = "showactualmark")]
    pub show_actual_mark: bool, // show student's score
    #[serde(rename = "showtotalmark")]
    pub show_total_mark: bool, // show total marks available
    #[serde(rename = "showanswerstate")]
    pub show_answer_state: bool, // Show whether answer was correct
    #[serde(rename = "allowrevealanswer")]
    pub allow_reveal_answer: bool,
    #[serde(flatten)]
    pub review: Option<Review>,
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
    pub show_score: Option<bool>,
    #[serde(rename = "reviewshowfeedback")]
    pub show_feedback: Option<bool>,
    #[serde(rename = "reviewshowexpectedanswer")]
    pub show_expected_answer: Option<bool>,
    #[serde(rename = "reviewshowadvice")]
    pub show_advice: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    pub message: String,
    pub threshold: String, //TODO type
}
