use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "FeedbackInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct Feedback {
    pub percentage_needed_to_pass: Noneable<f64>, // if "none" (or 0) -> no percentage shown in frontpage, otherwise it is shown
    pub show_name_of_student: bool,
    /// Whether current marks are shown during exam or not (show_actual_mark in numbas)
    pub show_current_marks: bool,
    /// Whether the maximal mark for a question (or the total exam) is shown (show_total_mark of numbas)
    pub show_maximum_marks: bool,
    /// Whether answer feedback is shown (right or wrong etc)
    pub show_answer_state: bool,
    /// Whether the 'reveal answer' button is present
    pub allow_reveal_answer: bool,
    pub review: Review, // If none, everything is true???
    pub advice: TranslatableString,
    pub intro: TranslatableString,
    pub feedback_messages: Vec<FeedbackMessage>,
}

impl ToNumbas<numbas::exam::feedback::Feedback> for Feedback {
    fn to_numbas(&self, locale: &str) -> numbas::exam::feedback::Feedback {
        numbas::exam::feedback::Feedback {
            show_actual_mark: self.show_current_marks.to_numbas(locale),
            show_total_mark: self.show_maximum_marks.to_numbas(locale),
            show_answer_state: self.show_answer_state.to_numbas(locale),
            allow_reveal_answer: self.allow_reveal_answer.to_numbas(locale),
            review: self.review.clone().to_numbas(locale),
            advice: self.advice.clone().to_string(locale),
            intro: self.intro.clone().to_numbas(locale),
            feedback_messages: self.feedback_messages.to_numbas(locale),
        }
    }
}

impl ToRumbas<Feedback> for numbas::exam::Exam {
    fn to_rumbas(&self) -> Feedback {
        let review: Review = self.feedback.review.to_rumbas();
        Feedback {
            percentage_needed_to_pass: self.basic_settings.percentage_needed_to_pass.to_rumbas(),
            show_name_of_student: self.basic_settings.show_student_name.to_rumbas(),
            show_current_marks: self.feedback.show_actual_mark.to_rumbas(),
            show_maximum_marks: self.feedback.show_total_mark.to_rumbas(),
            show_answer_state: self.feedback.show_answer_state.to_rumbas(),
            allow_reveal_answer: self.feedback.allow_reveal_answer.to_rumbas(),
            review,
            advice: self.feedback.advice.clone().unwrap_or_default().to_rumbas(),
            intro: self.feedback.intro.to_rumbas(),
            feedback_messages: self.feedback.feedback_messages.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "ReviewInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct Review {
    /// Whether to show score in result overview page
    pub show_score: bool,
    /// Show feedback while reviewing
    pub show_feedback: bool,
    /// Show expected answer while reviewing
    pub show_expected_answer: bool,
    /// Show advice while reviewing
    pub show_advice: bool,
}

impl ToNumbas<numbas::exam::feedback::Review> for Review {
    fn to_numbas(&self, locale: &str) -> numbas::exam::feedback::Review {
        numbas::exam::feedback::Review {
            show_score: self.show_score.to_numbas(locale),
            show_feedback: self.show_feedback.to_numbas(locale),
            show_expected_answer: self.show_expected_answer.to_numbas(locale),
            show_advice: self.show_advice.to_numbas(locale),
        }
    }
}

impl ToRumbas<Review> for numbas::exam::feedback::Review {
    fn to_rumbas(&self) -> Review {
        Review {
            show_score: self.show_score.to_rumbas(),
            show_feedback: self.show_feedback.to_rumbas(),
            show_expected_answer: self.show_expected_answer.to_rumbas(),
            show_advice: self.show_advice.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "FeedbackMessageInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct FeedbackMessage {
    pub message: String,   //TODO: inputstring or filestring?
    pub threshold: String, //TODO type
}

impl ToNumbas<numbas::exam::feedback::FeedbackMessage> for FeedbackMessage {
    fn to_numbas(&self, locale: &str) -> numbas::exam::feedback::FeedbackMessage {
        numbas::exam::feedback::FeedbackMessage {
            message: self.message.to_numbas(locale),
            threshold: self.threshold.to_numbas(locale),
        }
    }
}

impl ToRumbas<FeedbackMessage> for numbas::exam::feedback::FeedbackMessage {
    fn to_rumbas(&self) -> FeedbackMessage {
        FeedbackMessage {
            message: self.message.clone(),
            threshold: self.threshold.clone(),
        }
    }
}
