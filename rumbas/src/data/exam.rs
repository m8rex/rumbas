use crate::data::optional_overwrite::OptionalOverwrite;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;

type NumbasResult<T> = Result<T, Vec<String>>;

optional_overwrite! {
    Exam,
    name: String,
    navigation: Navigation,
    timing: Timing,
    feedback: Feedback
}

optional_overwrite! {
    Navigation,
    allow_regenerate: bool,
    reverse: bool,
    browsing_enabled: bool,
    allow_steps: bool,
    show_frontpage: bool,
    show_results_page: ShowResultsPage,
    prevent_leaving: bool,
    on_leave: Action,
    start_password: String,
    show_names_of_question_groups: bool
}

impl Navigation {
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamNavigation::new(
                self.allow_regenerate.unwrap(),
                self.reverse,
                self.browsing_enabled,
                self.allow_steps,
                self.show_frontpage.unwrap(),
                self.show_results_page.map(|s| s.to_numbas()),
                self.prevent_leaving,
                self.on_leave.clone().map(|s| s.to_numbas()),
                self.start_password.clone(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShowResultsPage {
    OnCompletion,
    Never,
}
impl_optional_overwrite!(ShowResultsPage);
impl ShowResultsPage {
    fn to_numbas(&self) -> numbas::exam::ExamShowResultsPage {
        match self {
            ShowResultsPage::OnCompletion => numbas::exam::ExamShowResultsPage::OnCompletion,
            ShowResultsPage::Never => numbas::exam::ExamShowResultsPage::Never,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum Action {
    None { message: String },
}
impl_optional_overwrite!(Action);
impl Action {
    fn to_numbas(&self) -> numbas::exam::ExamAction {
        match self {
            Action::None { message } => numbas::exam::ExamAction::None {
                message: message.to_string(),
            },
        }
    }
}

optional_overwrite! {
    Timing,
    duration_in_seconds: usize,
    allow_pause: bool,
    on_timeout: Action,
    timed_warning: Action
}

impl Timing {
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamTiming> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamTiming::new(
                self.allow_pause.unwrap(),
                self.on_timeout.clone().unwrap().to_numbas(),
                self.timed_warning.clone().unwrap().to_numbas(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    Feedback,
    percentage_needed_to_pass: f64,
    show_name_of_student: bool,
    show_actual_mark: bool,
    show_total_mark: bool,
    show_answer_state: bool,
    allow_reveal_answer: bool,
    review: Review,
    advice: String,
    intro: String,
    feedback_messages: Vec<FeedbackMessage>
}

impl Feedback {
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamFeedback> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamFeedback::new(
                self.show_actual_mark.unwrap(),
                self.show_total_mark.unwrap(),
                self.show_answer_state.unwrap(),
                self.allow_reveal_answer.unwrap(),
                self.review.clone().map(|s| s.to_numbas().unwrap()),
                self.advice.clone(),
                self.intro.clone().unwrap(),
                self.feedback_messages
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_numbas())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    Review,
    show_score: bool,
    show_feedback: bool,
    show_expected_answer: bool,
    show_advice: bool
}

impl Review {
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamReview> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamReview::new(
                self.show_score,
                self.show_feedback,
                self.show_expected_answer,
                self.show_advice,
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    message: String,
    threshold: String, //TODO type
}

impl FeedbackMessage {
    fn to_numbas(&self) -> numbas::exam::ExamFeedbackMessage {
        numbas::exam::ExamFeedbackMessage::new(self.message.clone(), self.threshold.clone())
    }
}

impl Exam {
    pub fn to_numbas(&self) -> NumbasResult<numbas::exam::Exam> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let basic_settings = numbas::exam::BasicExamSettings::new(
                self.name.clone().unwrap(),
                self.timing.clone().unwrap().duration_in_seconds,
                self.feedback.clone().unwrap().percentage_needed_to_pass,
                self.navigation
                    .clone()
                    .unwrap()
                    .show_names_of_question_groups,
                self.feedback.clone().unwrap().show_name_of_student,
            );

            //TODO
            let resources: Vec<[String; 2]> = Vec::new();

            //TODO
            let extensions: Vec<String> = Vec::new();

            //TODO
            let custom_part_types: Vec<numbas::exam::CustomPartType> = Vec::new();

            //TODO
            let navigation = self.navigation.clone().unwrap().to_numbas().unwrap();

            //TODO
            let timing = self.timing.clone().unwrap().to_numbas().unwrap();

            //TODO
            let feedback = self.feedback.clone().unwrap().to_numbas().unwrap();

            //TODO
            let functions = None;

            //TODO
            let variables = None;

            //TODO
            let question_groups: Vec<numbas::exam::ExamQuestionGroup> = Vec::new();

            Ok(numbas::exam::Exam::new(
                basic_settings,
                resources,
                extensions,
                custom_part_types,
                navigation,
                timing,
                feedback,
                functions,
                variables,
                question_groups,
            ))
        } else {
            Err(empty_fields)
        }
    }

    pub fn from_file(file: &Path) -> serde_json::Result<Exam> {
        let json = fs::read_to_string(file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_json::from_str(&json)
    }
}

macro_rules! exam_from {
    ($($func_name: ident: $var: ident: $type: ty), *) => {
        impl Exam {
            $(
                pub fn $func_name($var: $type) -> Exam {
                    let mut empty: Exam = serde_json::from_str("{}").unwrap(); //TODO is this to hacky?
                    empty.$var = Some($var);
                    empty
                }
            )*
        }
    };
}

exam_from! {
    from_navigation: navigation: Navigation,
    from_timing: timing: Timing,
    from_feedback: feedback: Feedback
}
