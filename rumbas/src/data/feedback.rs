use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use crate::data::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
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

impl ToNumbas for Feedback {
    type NumbasType = numbas::exam::ExamFeedback;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamFeedback> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamFeedback {
                show_actual_mark: self.show_current_marks.unwrap(),
                show_total_mark: self.show_maximum_marks.unwrap(),
                show_answer_state: self.show_answer_state.unwrap(),
                allow_reveal_answer: self.allow_reveal_answer.unwrap(),
                review: self.review.clone().map(|o| o.to_numbas(locale).unwrap()),
                advice: self.advice.clone().map(|o| o.to_string(locale)).flatten(),
                intro: self.intro.clone().unwrap().to_string(locale).unwrap(),
                feedback_messages: self.feedback_messages.to_numbas(locale).unwrap(),
            })
        } else {
            Err(check)
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
            advice: Value::Normal(TranslatableString::s(
                &self.feedback.advice.clone().unwrap_or_default(),
            )),
            intro: Value::Normal(TranslatableString::s(&self.feedback.intro)),
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

impl ToNumbas for Review {
    type NumbasType = numbas::exam::ExamReview;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<numbas::exam::ExamReview> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamReview {
                show_score: Some(self.show_score.clone().unwrap()),
                show_feedback: Some(self.show_feedback.clone().unwrap()),
                show_expected_answer: Some(self.show_expected_answer.clone().unwrap()),
                show_advice: Some(self.show_advice.clone().unwrap()),
            })
        } else {
            Err(check)
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    pub message: String,   //TODO: inputstring or filestring?
    pub threshold: String, //TODO type
}
impl_optional_overwrite!(FeedbackMessage);

impl ToNumbas for FeedbackMessage {
    type NumbasType = numbas::exam::ExamFeedbackMessage;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<numbas::exam::ExamFeedbackMessage> {
        Ok(numbas::exam::ExamFeedbackMessage {
            message: self.message.clone(),
            threshold: self.threshold.clone(),
        })
    }
}
