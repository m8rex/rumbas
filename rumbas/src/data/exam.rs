use crate::data::optional_overwrite::OptionalOverwrite;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type NumbasResult<T> = Result<T, Vec<String>>;

optional_overwrite! {
    Exam,
    name: String,
    navigation: Navigation,
    timing: Timing,
    feedback: Feedback,
    question_groups: Vec<QuestionGroup>
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
impl_optional_overwrite!(FeedbackMessage);
impl FeedbackMessage {
    fn to_numbas(&self) -> numbas::exam::ExamFeedbackMessage {
        numbas::exam::ExamFeedbackMessage::new(self.message.clone(), self.threshold.clone())
    }
}

/*
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct QuestionPath {
    question: String,
    question_data: Option<Question>,
}*/

optional_overwrite! {
    QuestionPath: serde(try_from = "String"),
    question: String,
    question_data: Question
}

impl std::convert::TryFrom<String> for QuestionPath {
    type Error = serde_json::error::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let question_data = Question::from_name(&s)?;
        Ok(QuestionPath {
            question: Some(s),
            question_data: Some(question_data),
        })
    }
}

optional_overwrite! {
    QuestionGroup,
    name: String,
    picking_strategy: PickingStrategy,
    questions: Vec<QuestionPath>
}

impl QuestionGroup {
    pub fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionGroup> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionGroup::new(
                self.name.clone(),
                self.picking_strategy.clone().unwrap().to_numbas(),
                Vec::new(), //TODO
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PickingStrategy {
    #[serde(rename = "all_ordered")]
    AllOrdered,
    #[serde(rename = "all_shuffled")]
    AllShuffled,
    #[serde(rename = "random_subset")]
    RandomSubset { pick_questions: usize },
}
impl_optional_overwrite!(PickingStrategy);
impl PickingStrategy {
    pub fn to_numbas(&self) -> numbas::exam::ExamQuestionGroupPickingStrategy {
        match self {
            PickingStrategy::AllOrdered => {
                numbas::exam::ExamQuestionGroupPickingStrategy::AllOrdered
            }
            PickingStrategy::AllShuffled => {
                numbas::exam::ExamQuestionGroupPickingStrategy::AllShuffled
            }
            PickingStrategy::RandomSubset { pick_questions } => {
                numbas::exam::ExamQuestionGroupPickingStrategy::RandomSubset {
                    pick_questions: *pick_questions,
                }
            }
        }
    }
}

optional_overwrite! {
    Question,
    name: String,
    statement: String,
    advice: String,
    parts: Vec<QuestionPart>,
    variables: HashMap<String, Variable>, //TODO variables_test
    functions: HashMap<String, Function>,
    navigation: Navigation,
    extensions: Vec<String> //TODO: obj of bools
    //TODO al lot of options

}

impl Question {
    pub fn from_name(name: &String) -> serde_json::Result<Question> {
        let file = Path::new("questions").join(format!("{}.json", name));
        let json = fs::read_to_string(&file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_json::from_str(&json)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum QuestionPart {
    //TODO; other types
    //TODO: custom_part_constructor types?
    #[serde(rename = "jme")]
    JME(QuestionPartJME),
}
impl_optional_overwrite!(QuestionPart);

macro_rules! question_part_type {
    ($struct: ident, $($field: ident: $type: ty), *) => {
        optional_overwrite! {
            $struct,
            marks: usize,
            prompt: String,
            use_custom_name: bool,
            custom_name: String,
            steps_penalty: usize,
            enable_minimum_marks: bool,
            minimum_marks: usize, //TODO: separate?
            show_correct_answer: bool,
            show_feedback_icon: bool,
            variable_replacement_strategy: VariableReplacementStrategy,
            adaptive_marking_penalty: usize,
            // custom_marking_algorithm: String TO
            extend_base_marking_algorithm: bool,
            steps: Vec<QuestionPart>,
            $(
                $field: $type
            ),*
        }
    }
}
question_part_type! {
    QuestionPartJME,
    answer: String,
    answer_simplification: JMEAnswerSimplification,
    show_preview: bool,
    checking_type: CheckingType,
    checking_accuracy: f64,
    failure_rate: f64,
    vset_range: [f64; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    vset_range_points: usize,
    check_variable_names: bool,
    single_letter_variables: bool,
    allow_unknown_functions: bool,
    implicit_function_composition: bool,
    max_length: numbas::exam::JMELengthRestriction, // TODO: custom struct, (because of partialCredit)
    min_length: numbas::exam::JMELengthRestriction,
    must_have: numbas::exam::JMEStringRestriction,
    may_not_have: numbas::exam::JMEStringRestriction,
    must_match_pattern: numbas::exam::JMEPatternRestriction
}

//TODO: rename etc
optional_overwrite! {
    JMEAnswerSimplification,
    simplify_basic: bool,
    simplify_unit_factor: bool,
    simplify_unit_power: bool,
    simplify_unit_denominator: bool,
    simplify_zero_factor: bool,
    simplify_zero_term: bool,
    simplify_zero_power: bool,
    simplify_zero_base: bool,
    collect_numbers: bool,
    constants_first: bool,
    simplify_sqrt_products: bool,
    simplify_sqrt_division: bool,
    simplify_sqrt_square: bool,
    simplify_other_numbers: bool
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckingType {
    RelativeDifference,
    AbsoluteDifference,
    DecimalPlaces,
    SignificantFigures,
}
impl_optional_overwrite!(CheckingType);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    OriginalFirst,
}
impl_optional_overwrite!(VariableReplacementStrategy);

optional_overwrite! {
    Variable,
    definition: String,
    description: String,
    template_type: String, //TODO , "anything"
    group: String //TODO "Ungrouped variables" -> real optional? if not -> ungrouped?
}

optional_overwrite! {
    Function,
    parameters: HashMap<String, numbas::exam::ExamFunctionType>,
    output_type: numbas::exam::ExamFunctionType,
    definition: String,
    language: numbas::exam::ExamFunctionLanguage
}
impl_optional_overwrite!(
    numbas::exam::ExamFunctionType,
    numbas::exam::ExamFunctionLanguage,
    numbas::exam::JMELengthRestriction,
    numbas::exam::JMEStringRestriction,
    numbas::exam::JMEPatternRestriction,
    numbas::exam::CheckingType,
    numbas::exam::AnswerSimplificationType
);

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

            // Below from questions
            //TODO
            let resources: Vec<[String; 2]> = Vec::new();
            //TODO obj of bools
            let extensions: Vec<String> = Vec::new();
            //TODO
            let custom_part_types: Vec<numbas::exam::CustomPartType> = Vec::new();

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
