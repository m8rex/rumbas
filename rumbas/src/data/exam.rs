use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

type NumbasResult<T> = Result<T, Vec<String>>;
pub trait ToNumbas: Clone {
    type NumbasType;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType>;
    fn to_numbas_with_name(&self, _name: String) -> NumbasResult<Self::NumbasType> {
        self.to_numbas()
    }
}

impl<T: ToNumbas + OptionalOverwrite> ToNumbas for Option<T> {
    type NumbasType = <T as ToNumbas>::NumbasType;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        match self {
            Some(val) => {
                let empty_fields = val.empty_fields();
                if empty_fields.is_empty() {
                    Ok(val.to_numbas().unwrap())
                } else {
                    Err(empty_fields)
                }
            }
            None => Err(vec!["".to_string()]),
        }
    }
}
impl<T: ToNumbas + OptionalOverwrite> ToNumbas for Noneable<T> {
    type NumbasType = Option<<T as ToNumbas>::NumbasType>;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        match self {
            Noneable::NotNone(val) => {
                let empty_fields = val.empty_fields();
                if empty_fields.is_empty() {
                    Ok(Some(val.clone().to_numbas().unwrap()))
                } else {
                    Err(empty_fields)
                }
            }
            _ => Ok(None),
        }
    }
}

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

impl ToNumbas for Navigation {
    type NumbasType = numbas::exam::ExamNavigation;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamNavigation::new(
                self.allow_regenerate.unwrap(),
                self.reverse,
                self.browsing_enabled,
                self.allow_steps,
                self.show_frontpage.unwrap(),
                self.show_results_page.map(|s| s.to_numbas().unwrap()),
                self.prevent_leaving,
                self.on_leave.clone().map(|s| s.to_numbas().unwrap()),
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
impl ToNumbas for ShowResultsPage {
    type NumbasType = numbas::exam::ExamShowResultsPage;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            ShowResultsPage::OnCompletion => numbas::exam::ExamShowResultsPage::OnCompletion,
            ShowResultsPage::Never => numbas::exam::ExamShowResultsPage::Never,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum Action {
    None { message: String },
}
impl_optional_overwrite!(Action);
impl ToNumbas for Action {
    type NumbasType = numbas::exam::ExamAction;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            Action::None { message } => numbas::exam::ExamAction::None {
                message: message.to_string(),
            },
        })
    }
}

optional_overwrite! {
    QuestionNavigation,
    allow_regenerate: bool,
    show_frontpage: bool,
    prevent_leaving: bool
}

impl ToNumbas for QuestionNavigation {
    type NumbasType = numbas::exam::QuestionNavigation;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::QuestionNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::QuestionNavigation::new(
                self.allow_regenerate.unwrap(),
                self.show_frontpage.unwrap(),
                self.prevent_leaving,
            ))
        } else {
            Err(empty_fields)
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

impl ToNumbas for Timing {
    type NumbasType = numbas::exam::ExamTiming;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamTiming> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamTiming::new(
                self.allow_pause.unwrap(),
                self.on_timeout.clone().unwrap().to_numbas().unwrap(),
                self.timed_warning.clone().unwrap().to_numbas().unwrap(),
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

impl ToNumbas for Feedback {
    type NumbasType = numbas::exam::ExamFeedback;
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
                    .map(|s| s.to_numbas().unwrap())
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

impl ToNumbas for Review {
    type NumbasType = numbas::exam::ExamReview;
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
impl ToNumbas for FeedbackMessage {
    type NumbasType = numbas::exam::ExamFeedbackMessage;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamFeedbackMessage> {
        Ok(numbas::exam::ExamFeedbackMessage::new(
            self.message.clone(),
            self.threshold.clone(),
        ))
    }
}

optional_overwrite! {
    QuestionPath: serde(try_from = "String"),
    question: String,
    question_data: Question
}

impl std::convert::TryFrom<String> for QuestionPath {
    type Error = JsonError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let question_data = Question::from_name(&s).map_err(|e| {
            println!("{}", e);
            e
        })?;
        Ok(QuestionPath {
            question: Some(s),
            question_data: Some(question_data),
        })
    }
}

optional_overwrite! {
    QuestionGroup,
    name: String,
    picking_strategy: PickingStrategy: serde(flatten),
    questions: Vec<QuestionPath>
}

impl ToNumbas for QuestionGroup {
    type NumbasType = numbas::exam::ExamQuestionGroup;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionGroup> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionGroup::new(
                self.name.clone(),
                self.picking_strategy.clone().unwrap().to_numbas().unwrap(),
                self.questions
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|q| {
                        q.question_data
                            .clone()
                            .unwrap()
                            .to_numbas_with_name(q.question.clone().unwrap())
                            .unwrap()
                    })
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "picking_strategy")]
pub enum PickingStrategy {
    #[serde(rename = "all_ordered")]
    AllOrdered,
    #[serde(rename = "all_shuffled")]
    AllShuffled,
    #[serde(rename = "random_subset")]
    RandomSubset { pick_questions: usize },
}
impl_optional_overwrite!(PickingStrategy);
impl ToNumbas for PickingStrategy {
    type NumbasType = numbas::exam::ExamQuestionGroupPickingStrategy;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionGroupPickingStrategy> {
        Ok(match self {
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
        })
    }
}

optional_overwrite! {
    Question,
    statement: String,
    advice: String,
    parts: Vec<QuestionPart>,
    variables: HashMap<String, Variable>,
    variables_test: VariablesTest,
    functions: HashMap<String, Function>,
    preamble: Preamble,
    navigation: QuestionNavigation,
    extensions: Extensions,
    ungrouped_variables: Vec<String>
    //TODO al lot of options

}

impl ToNumbas for Question {
    type NumbasType = numbas::exam::ExamQuestion;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        //TODO?
        Err(vec![
            "Should not happen, don't call this method Missing name".to_string(),
        ])
    }
    //TODO: add to_numbas on Option's to reduce burden?
    fn to_numbas_with_name(&self, name: String) -> NumbasResult<numbas::exam::ExamQuestion> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestion::new(
                name,
                self.statement.clone().unwrap(),
                self.advice.clone().unwrap(),
                self.parts
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| p.to_numbas().unwrap())
                    .collect(),
                self.variables
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), v.to_numbas_with_name(k).unwrap()))
                    .collect(),
                self.variables_test.clone().unwrap().to_numbas().unwrap(),
                self.functions
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_numbas().unwrap()))
                    .collect(),
                self.ungrouped_variables.clone().unwrap(),
                Vec::new(),     //TODO: calculate from variables
                HashMap::new(), //TODO: add to Question type
                self.preamble.clone().unwrap().to_numbas().unwrap(),
                self.navigation.clone().unwrap().to_numbas().unwrap(),
                self.extensions.clone().unwrap().to_numbas().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    VariablesTest,
    condition: String,
    max_runs: usize
}

impl ToNumbas for VariablesTest {
    type NumbasType = numbas::exam::ExamQuestionVariablesTest;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionVariablesTest> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionVariablesTest::new(
                self.condition.clone().unwrap(),
                self.max_runs.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    Preamble,
    js: String,
    css: String
}

impl ToNumbas for Preamble {
    type NumbasType = numbas::exam::Preamble;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::Preamble> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::Preamble::new(
                self.js.clone().unwrap(),
                self.css.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Debug)]
pub struct JsonError {
    error: serde_json::error::Error,
    file: PathBuf,
}

impl JsonError {
    pub fn from(error: serde_json::error::Error, file: PathBuf) -> JsonError {
        JsonError { error, file }
    }
}
impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error in {} on column {} of line {}. The type of the error is {:?}",
            self.file.display(),
            self.error.column(),
            self.error.line(),
            self.error.classify()
        ) // Better explanation: Eof -> end of file, Data: wrong datatype or missing field, Syntax: syntax error
    }
}
pub type JsonResult<T> = Result<T, JsonError>;

impl Question {
    pub fn from_name(name: &String) -> JsonResult<Question> {
        let file = Path::new("questions").join(format!("{}.json", name));
        let json = fs::read_to_string(&file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_json::from_str(&json).map_err(|e| JsonError::from(e, file.to_path_buf()))
    }
}

optional_overwrite_enum! {
    QuestionPart: serde(tag = "type"),
    JME: QuestionPartJME: serde(rename = "jme"),
    GapFill: QuestionPartGapFill: serde(rename = "gapfill")
}

impl ToNumbas for QuestionPart {
    type NumbasType = numbas::exam::ExamQuestionPart;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionPart> {
        match self {
            QuestionPart::JME(d) => {
                let n = d.to_numbas()?;
                Ok(numbas::exam::ExamQuestionPart::JME(n))
            }
            QuestionPart::GapFill(d) => {
                let n = d.to_numbas()?;
                Ok(numbas::exam::ExamQuestionPart::GapFill(n))
            }
        }
    }
}

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
            custom_marking_algorithm: String, // TODO? empty string -> none?
            extend_base_marking_algorithm: bool,
            steps: Vec<QuestionPart>,
            $(
                $field: $type
            ),*
        }
        impl $struct {
            fn to_numbas_shared_data(&self) -> numbas::exam::ExamQuestionPartSharedData {
                numbas::exam::ExamQuestionPartSharedData::new(
            self.marks,
            self.prompt.clone(),
            self.use_custom_name,
            self.custom_name.clone(),
            self.steps_penalty,
            self.enable_minimum_marks,
            self.minimum_marks,
            self.show_correct_answer.clone().unwrap(),
            self.show_feedback_icon,
            self.variable_replacement_strategy.clone().unwrap().to_numbas().unwrap(),
            self.adaptive_marking_penalty,
            self.custom_marking_algorithm.clone(),
            self.extend_base_marking_algorithm,
            self.steps.clone().map(|v| v.iter().map(|s| s.to_numbas().unwrap()).collect()),
                )
            }
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
    max_length: Noneable<JMELengthRestriction>,
    min_length: Noneable<JMELengthRestriction>,
    must_have: JMEStringRestriction,
    may_not_have: JMEStringRestriction,
    must_match_pattern: JMEPatternRestriction
}

impl ToNumbas for QuestionPartJME {
    type NumbasType = numbas::exam::ExamQuestionPartJME;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionPartJME> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionPartJME::new(
                self.to_numbas_shared_data(),
                self.answer.clone().unwrap(),
                Some(
                    self.answer_simplification
                        .clone()
                        .unwrap()
                        .to_numbas()
                        .unwrap(),
                ),
                self.show_preview.clone().unwrap(),
                self.checking_type.clone().unwrap().to_numbas().unwrap(),
                self.checking_accuracy.unwrap(),
                self.failure_rate.unwrap(),
                self.vset_range.unwrap(),
                self.vset_range_points.unwrap(),
                self.check_variable_names.unwrap(),
                self.single_letter_variables,
                self.allow_unknown_functions,
                self.implicit_function_composition,
                self.max_length
                    .clone()
                    .map(|v| v.to_numbas().unwrap())
                    .flatten(),
                self.min_length
                    .clone()
                    .map(|v| v.to_numbas().unwrap())
                    .flatten(),
                self.must_have.clone().map(|v| v.to_numbas().unwrap()),
                self.may_not_have.clone().map(|v| v.to_numbas().unwrap()),
                self.must_match_pattern
                    .clone()
                    .map(|v| v.to_numbas().unwrap()),
            ))
        } else {
            Err(empty_fields)
        }
    }
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

impl ToNumbas for JMEAnswerSimplification {
    type NumbasType = Vec<numbas::exam::AnswerSimplificationType>;
    fn to_numbas(&self) -> NumbasResult<Vec<numbas::exam::AnswerSimplificationType>> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let mut v = Vec::new();
            if self.simplify_basic.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::Basic);
            }
            if self.simplify_unit_factor.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::UnitFactor);
            }
            if self.simplify_unit_power.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::UnitPower);
            }
            if self.simplify_unit_denominator.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::UnitDenominator);
            }
            if self.simplify_zero_factor.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroFactor);
            }
            if self.simplify_zero_term.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroTerm);
            }
            if self.simplify_zero_power.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroPower);
            }
            if self.simplify_zero_base.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroBase);
            }
            if self.collect_numbers.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::CollectNumbers);
            }
            if self.constants_first.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ConstantsFirst);
            }
            if self.simplify_sqrt_products.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::SqrtProduct);
            }
            if self.simplify_sqrt_division.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::SqrtDivision);
            }
            if self.simplify_sqrt_square.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::SqrtSquare);
            }
            if self.simplify_other_numbers.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::OtherNumbers);
            }
            Ok(v)
        } else {
            Err(empty_fields)
        }
    }
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
impl ToNumbas for CheckingType {
    type NumbasType = numbas::exam::JMECheckingType;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            CheckingType::RelativeDifference => numbas::exam::JMECheckingType::RelativeDifference,
            CheckingType::AbsoluteDifference => numbas::exam::JMECheckingType::AbsoluteDifference,
            CheckingType::DecimalPlaces => numbas::exam::JMECheckingType::DecimalPlaces,
            CheckingType::SignificantFigures => numbas::exam::JMECheckingType::SignificantFigures,
        })
    }
}

optional_overwrite! {
    JMERestriction,
    name: String,
    strings: Vec<String>,
    partial_credit: f64, //TODO, is number, so maybe usize?
    message: String
}

impl ToNumbas for JMERestriction {
    type NumbasType = numbas::exam::JMERestriction;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::JMERestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMERestriction::new(
                self.name.clone().unwrap(),
                self.strings.clone().unwrap(),
                self.partial_credit.clone().unwrap(),
                self.message.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    JMELengthRestriction,
    restriction: JMERestriction: serde(flatten),
    length: usize
}

impl ToNumbas for JMELengthRestriction {
    type NumbasType = numbas::exam::JMELengthRestriction;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::JMELengthRestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMELengthRestriction::new(
                self.restriction.clone().unwrap().to_numbas().unwrap(),
                self.length,
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    JMEStringRestriction,
    restriction: JMERestriction: serde(flatten),
    show_strings: bool
}

impl ToNumbas for JMEStringRestriction {
    type NumbasType = numbas::exam::JMEStringRestriction;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::JMEStringRestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMEStringRestriction::new(
                self.restriction.clone().unwrap().to_numbas().unwrap(),
                self.show_strings.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    JMEPatternRestriction,
    restriction: JMERestriction: serde(flatten),
    pattern: String, //TODO type?
    name_to_compare: String
}

impl ToNumbas for JMEPatternRestriction {
    type NumbasType = numbas::exam::JMEPatternRestriction;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::JMEPatternRestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMEPatternRestriction::new(
                self.restriction.clone().unwrap().to_numbas().unwrap(),
                self.pattern.clone().unwrap(),
                self.name_to_compare.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    OriginalFirst,
}
impl_optional_overwrite!(VariableReplacementStrategy);

impl ToNumbas for VariableReplacementStrategy {
    type NumbasType = numbas::exam::VariableReplacementStrategy;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            VariableReplacementStrategy::OriginalFirst => {
                numbas::exam::VariableReplacementStrategy::OriginalFirst
            }
        })
    }
}

optional_overwrite! {
    Variable,
    definition: String,
    description: String,
    template_type: VariableTemplateType,
    group: String //TODO "Ungrouped variables" -> real optional? if not -> ungrouped?
}
impl ToNumbas for Variable {
    type NumbasType = numbas::exam::ExamVariable;
    fn to_numbas_with_name(&self, name: String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamVariable::new(
                name,
                self.definition.clone().unwrap(),
                self.description.clone().unwrap(),
                self.template_type.clone().unwrap().to_numbas().unwrap(),
                self.group.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        //TODO?
        Err(vec![
            "Should not happen, don't call this method Missing name".to_string(),
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VariableTemplateType {
    Anything,
    RandomRange,
}

impl ToNumbas for VariableTemplateType {
    type NumbasType = numbas::exam::ExamVariableTemplateType;
    fn to_numbas(&self) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            VariableTemplateType::Anything => numbas::exam::ExamVariableTemplateType::Anything,
            VariableTemplateType::RandomRange => {
                numbas::exam::ExamVariableTemplateType::RandomRange
            }
        })
    }
}
impl_optional_overwrite!(VariableTemplateType);

optional_overwrite! {
    Function,
    parameters: HashMap<String, numbas::exam::ExamFunctionType>,
    output_type: numbas::exam::ExamFunctionType,
    definition: String,
    language: numbas::exam::ExamFunctionLanguage
}
impl ToNumbas for Function {
    type NumbasType = numbas::exam::ExamFunction;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamFunction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamFunction::new(
                self.parameters.clone().unwrap().into_iter().collect(),
                self.output_type.clone().unwrap(),
                self.definition.clone().unwrap(),
                self.language.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
impl_optional_overwrite!(
    numbas::exam::ExamFunctionType,
    numbas::exam::ExamFunctionLanguage,
    numbas::exam::AnswerSimplificationType
);

question_part_type! {
    QuestionPartGapFill,
    sort_answers: bool,
    gaps: Vec<QuestionPart>
}

impl ToNumbas for QuestionPartGapFill {
    type NumbasType = numbas::exam::ExamQuestionPartGapFill;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::ExamQuestionPartGapFill> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionPartGapFill::new(
                self.to_numbas_shared_data(),
                self.sort_answers,
                self.gaps
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|g| g.to_numbas().unwrap())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

impl ToNumbas for Exam {
    type NumbasType = numbas::exam::Exam;
    fn to_numbas(&self) -> NumbasResult<numbas::exam::Exam> {
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
            let functions = Some(HashMap::new());

            //TODO
            let variables = Some(HashMap::new());

            //TODO
            let question_groups: Vec<numbas::exam::ExamQuestionGroup> = self
                .question_groups
                .clone()
                .unwrap()
                .iter()
                .map(|qg| qg.clone().to_numbas().unwrap())
                .collect();

            // Below from questions
            //TODO
            let resources: Vec<[String; 2]> = Vec::new();
            //TODO from obj of bools
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
}
impl Exam {
    pub fn from_file(file: &Path) -> JsonResult<Exam> {
        let json = fs::read_to_string(file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_json::from_str(&json).map_err(|e| JsonError::from(e, file.to_path_buf()))
    }
}

//TODO: add other extensions
optional_overwrite! {
    Extensions,
    jsx_graph: bool
}

impl ToNumbas for Extensions {
    type NumbasType = Vec<String>;
    fn to_numbas(&self) -> NumbasResult<Vec<String>> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let mut extensions = Vec::new();
            if self.jsx_graph.unwrap() {
                extensions.push("jsx_graph".to_string());
            }
            Ok(extensions)
        } else {
            Err(empty_fields)
        }
    }
}
