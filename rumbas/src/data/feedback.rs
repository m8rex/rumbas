use crate::data::template::{Value, ValueType};
use crate::data::translatable::TranslatableString;
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Feedback {
        percentage_needed_to_pass: Noneable<f64>, // if "none" (or 0) -> no percentage shown in frontpage, otherwise it is shown
        show_name_of_student: bool,
        /// Whether current marks are shown during exam or not (show_actual_mark in numbas)
        show_current_marks: bool,
        /// Whether the maximal mark for a question (or the total exam) is shown (show_total_mark of numbas)
        show_maximum_marks: bool,
        /// Whether answer feedback is shown (right or wrong etc)
        show_answer_state: bool,
        /// Whether the 'reveal answer' button is present
        allow_reveal_answer: bool,
        review: Review, // If none, everything is true???
        advice: TranslatableString,
        intro: TranslatableString,
        feedback_messages: Vec<Value<FeedbackMessage>>
    }
}

impl ToNumbas<numbas::exam::ExamFeedback> for Feedback {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamFeedback {
        numbas::exam::ExamFeedback {
            show_actual_mark: self.show_current_marks.to_numbas(locale),
            show_total_mark: self.show_maximum_marks.to_numbas(locale),
            show_answer_state: self.show_answer_state.to_numbas(locale),
            allow_reveal_answer: self.allow_reveal_answer.to_numbas(locale),
            review: self.review.clone().map(|o| o.to_numbas(locale)),
            advice: self.advice.clone().map(|o| o.to_string(locale)).flatten(),
            intro: self.intro.clone().to_numbas(locale),
            feedback_messages: self.feedback_messages.to_numbas(locale),
        }
    }
}

impl ToRumbas<Feedback> for numbas::exam::Exam {
    fn to_rumbas(&self) -> Feedback {
        Feedback {
            percentage_needed_to_pass: Value::Normal(
                self.basic_settings
                    .percentage_needed_to_pass
                    .map(Noneable::NotNone)
                    .unwrap_or_else(Noneable::nn),
            ),
            show_name_of_student: Value::Normal(
                self.basic_settings
                    .show_student_name
                    .unwrap_or(DEFAULTS.basic_settings_show_student_name),
            ),
            show_current_marks: Value::Normal(self.feedback.show_actual_mark),
            show_maximum_marks: Value::Normal(self.feedback.show_total_mark),
            show_answer_state: Value::Normal(self.feedback.show_answer_state),
            allow_reveal_answer: Value::Normal(self.feedback.allow_reveal_answer),
            review: Value::Normal(self.feedback.review.to_rumbas().unwrap()),
            advice: Value::Normal(self.feedback.advice.clone().unwrap_or_default().into()),
            intro: Value::Normal(self.feedback.intro.clone().into()),
            feedback_messages: Value::Normal(
                self.feedback
                    .feedback_messages
                    .clone()
                    .into_iter()
                    .map(|m| {
                        Value::Normal(FeedbackMessage {
                            message: m.message,
                            threshold: m.threshold,
                        })
                    })
                    .collect(),
            ),
        }
    }
}

optional_overwrite! {
    pub struct Review {
        /// Whether to show score in result overview page
        show_score: bool,
        /// Show feedback while reviewing
        show_feedback: bool,
        /// Show expected answer while reviewing
        show_expected_answer: bool,
        /// Show advice while reviewing
        show_advice: bool
    }
}

impl ToNumbas<numbas::exam::ExamReview> for Review {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamReview {
        numbas::exam::ExamReview {
            show_score: Some(self.show_score.clone().to_numbas(locale)),
            show_feedback: Some(self.show_feedback.clone().to_numbas(locale)),
            show_expected_answer: Some(self.show_expected_answer.clone().to_numbas(locale)),
            show_advice: Some(self.show_advice.clone().to_numbas(locale)),
        }
    }
}

impl ToRumbas<Review> for numbas::exam::ExamReview {
    fn to_rumbas(&self) -> Review {
        Review {
            show_score: Value::Normal(
                self.show_score
                    .unwrap_or(DEFAULTS.feedback_review_show_score),
            ),
            show_feedback: Value::Normal(
                self.show_feedback
                    .unwrap_or(DEFAULTS.feedback_review_show_feedback),
            ),
            show_expected_answer: Value::Normal(
                self.show_expected_answer
                    .unwrap_or(DEFAULTS.feedback_review_show_expected_answer),
            ),
            show_advice: Value::Normal(
                self.show_advice
                    .unwrap_or(DEFAULTS.feedback_review_show_advice),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    pub message: String,   //TODO: inputstring or filestring?
    pub threshold: String, //TODO type
}
impl_optional_overwrite!(FeedbackMessage);

impl ToNumbas<numbas::exam::ExamFeedbackMessage> for FeedbackMessage {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamFeedbackMessage {
        numbas::exam::ExamFeedbackMessage {
            message: self.message.clone(),
            threshold: self.threshold.clone(),
        }
    }
}
