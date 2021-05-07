use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    Feedback,
    percentage_needed_to_pass: Noneable<f64>, // if "none" (or 0) -> no percentage shown in frontpage, otherwise it is shown
    show_name_of_student: bool,
    allow_printing: bool, // better in review ...? Should map to allowPrinting in exam.json....
    show_current_marks: bool, // Whether current marks are shown during exam or not (show_actual_mark in numbas)
    show_maximum_marks: bool, // Whether the maximal mark for a question (or the total exam) is shown (show_total_mark of numbas)
    show_answer_state: bool, // Whether answer feedback is shown (right or wrong etc)
    allow_reveal_answer: bool, // Whether the 'reveal answer' button is present
    review: Review, // If none, everything is true???
    advice: TranslatableString,
    intro: TranslatableString,
    feedback_messages: Vec<Value<FeedbackMessage>>
}

impl ToNumbas for Feedback {
    type NumbasType = numbas::exam::ExamFeedback;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamFeedback> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamFeedback::new(
                self.show_current_marks.unwrap(),
                self.show_maximum_marks.unwrap(),
                self.show_answer_state.unwrap(),
                self.allow_reveal_answer.unwrap(),
                self.review.clone().map(|o| o.to_numbas(&locale).unwrap()),
                self.advice.clone().map(|o| o.to_string(&locale)).flatten(),
                self.intro.clone().unwrap().to_string(&locale).unwrap(),
                self.feedback_messages
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_numbas(&locale).unwrap())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    Review,
    show_score: bool, // Whether to show score in result overview page
    show_feedback: bool, // Show feedback while reviewing
    show_expected_answer: bool, // Show expected answer while reviewing
    show_advice: bool // Show advice while reviewing
}

impl ToNumbas for Review {
    type NumbasType = numbas::exam::ExamReview;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::ExamReview> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamReview::new(
                Some(self.show_score.clone().unwrap()),
                Some(self.show_feedback.clone().unwrap()),
                Some(self.show_expected_answer.clone().unwrap()),
                Some(self.show_advice.clone().unwrap()),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    message: String,   //TODO: inputstring or filestring?
    threshold: String, //TODO type
}
impl_optional_overwrite!(FeedbackMessage);

impl ToNumbas for FeedbackMessage {
    type NumbasType = numbas::exam::ExamFeedbackMessage;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::ExamFeedbackMessage> {
        Ok(numbas::exam::ExamFeedbackMessage::new(
            self.message.clone(),
            self.threshold.clone(),
        ))
    }
}
