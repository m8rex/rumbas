use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
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
            Ok(numbas::exam::ExamFeedback::new(
                self.show_current_marks.unwrap(),
                self.show_maximum_marks.unwrap(),
                self.show_answer_state.unwrap(),
                self.allow_reveal_answer.unwrap(),
                self.review.clone().map(|o| o.to_numbas(locale).unwrap()),
                self.advice.clone().map(|o| o.to_string(locale)).flatten(),
                self.intro.clone().unwrap().to_string(locale).unwrap(),
                self.feedback_messages
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_numbas(locale).unwrap())
                    .collect(),
            ))
        } else {
            Err(check)
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
            Ok(numbas::exam::ExamReview::new(
                Some(self.show_score.clone().unwrap()),
                Some(self.show_feedback.clone().unwrap()),
                Some(self.show_expected_answer.clone().unwrap()),
                Some(self.show_advice.clone().unwrap()),
            ))
        } else {
            Err(check)
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
        Ok(numbas::exam::ExamFeedbackMessage::new(
            self.message.clone(),
            self.threshold.clone(),
        ))
    }
}
