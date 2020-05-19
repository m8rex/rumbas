//Currently based on https://github.com/numbas/Numbas/blob/f420421a7ef3c2cd4c39e43f377d2a363ae2f81e/bin/exam.py
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc

fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(serde_json::Value::String(s)) => T::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(serde_json::Value::Number(n)) => {
            let s = n.to_string();
            let r = T::from_str(&s)
                .map_err(serde::de::Error::custom)
                .map(Option::from);
            r
        }
        Ok(v) => Err(serde::de::Error::custom(format!(
            "string or number expected but found something else: {}",
            v
        ))),
        Err(_) => Ok(None),
    }
}

fn answer_simplification_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<AnswerSimplificationType>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(serde_json::Value::String(s)) => {
            let mut r = Vec::new();
            for item in s.split(',') {
                let new_item = match item {
                    "basic" => Ok(AnswerSimplificationType::Basic),
                    "unitFactor" => Ok(AnswerSimplificationType::UnitFactor),
                    "unitPower" => Ok(AnswerSimplificationType::UnitPower),
                    "unitDenominator" => Ok(AnswerSimplificationType::UnitDenominator),
                    "zeroFactor" => Ok(AnswerSimplificationType::ZeroFactor),
                    "zeroTerm" => Ok(AnswerSimplificationType::ZeroTerm),
                    "zeroPower" => Ok(AnswerSimplificationType::ZeroPower),
                    "collectNumbers" => Ok(AnswerSimplificationType::CollectNumbers),
                    "zeroBase" => Ok(AnswerSimplificationType::ZeroBase),
                    "constantsFirst" => Ok(AnswerSimplificationType::ConstantsFirst),
                    "sqrtProduct" => Ok(AnswerSimplificationType::SqrtProduct),
                    "sqrtDivision" => Ok(AnswerSimplificationType::SqrtDivision),
                    "sqrtSquare" => Ok(AnswerSimplificationType::SqrtSquare),
                    "otherNumbers" => Ok(AnswerSimplificationType::OtherNumbers),
                    _ => Err(serde::de::Error::custom(format!(
                        "unknown answer simplification type {}",
                        item
                    ))),
                };
                match new_item {
                    Ok(a) => r.push(a),
                    Err(m) => return Err(m),
                }
            }
            Ok(Some(r))
        }
        Ok(v) => Err(serde::de::Error::custom(format!(
            "string expected but found something else: {}",
            v
        ))),
        Err(_) => Ok(None),
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Exam {
    name: String,
    #[serde(rename = "duration")]
    duration_in_seconds: Option<u32>, // in seconds
    #[serde(rename = "percentPass", deserialize_with = "from_str_optional")]
    percentage_needed_to_pass: Option<u32>, //TODO: is this a float?
    resources: Vec<[String; 2]>,
    extensions: Vec<String>,
    custom_part_types: Vec<CustomPartType>,
    #[serde(rename = "showQuestionGroupNames")]
    show_question_group_names: Option<bool>,
    #[serde(rename = "showstudentname")]
    show_student_name: Option<bool>,

    navigation: ExamNavigation,
    timing: ExamTiming,
    feedback: ExamFeedback,

    // rulesets: TODO
    functions: Option<HashMap<String, ExamFunction>>,
    variables: Option<HashMap<String, ExamFunction>>,
    question_groups: Vec<ExamQuestionGroup>,
    //contributors TODO
    //metadata TODO
}

impl Exam {
    pub fn from_str(s: &str) -> serde_json::Result<Exam> {
        let json = if s.starts_with("// Numbas version: exam_results_page_options") {
            s.splitn(2, "\n").collect::<Vec<_>>()[1]
        } else {
            s
        };
        serde_json::from_str(json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomPartType {} //TODO: add fields
#[derive(Serialize, Deserialize, Debug)]
pub struct ExamNavigation {
    #[serde(rename = "allowregen")]
    allow_regenerate: bool,
    reverse: Option<bool>,
    #[serde(rename = "browse")]
    browsing_enabled: Option<bool>,
    #[serde(rename = "allowsteps")]
    allow_steps: Option<bool>,
    #[serde(rename = "showfrontpage")]
    show_frontpage: bool,
    #[serde(rename = "showresultspage")]
    show_results_page: Option<ExamShowResultsPage>,
    #[serde(rename = "preventleave")]
    prevent_leaving: Option<bool>,
    #[serde(rename = "onleave")]
    on_leave: Option<ExamAction>,
    #[serde(rename = "startpassword")]
    start_password: Option<String>, //TODO: if empty string -> also None
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ExamAction {
    #[serde(rename = "none")]
    None { message: String },
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ExamShowResultsPage {
    #[serde(rename = "oncompletion")]
    OnCompletion,
    #[serde(rename = "never")]
    Never,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExamTiming {
    #[serde(rename = "allowPause")]
    allow_pause: bool,
    timeout: ExamAction,
    #[serde(rename = "timedwarning")]
    timed_warning: ExamAction,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExamFeedback {
    #[serde(rename = "showactualmark")]
    show_actual_mark: bool, // show student's score
    #[serde(rename = "showtotalmark")]
    show_total_mark: bool, // show total marks available
    #[serde(rename = "showanswerstate")]
    show_answer_state: bool, // Show whether answer was correct
    #[serde(rename = "allowrevealanswer")]
    allow_reveal_answer: bool,
    #[serde(flatten)]
    review: Option<ExamReview>,
    advice: Option<String>,
    intro: String,
    #[serde(rename = "feedbackmessages")]
    feedback_messages: Vec<ExamFeedbackMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamReview {
    #[serde(rename = "reviewshowscore")]
    show_score: Option<bool>,
    #[serde(rename = "reviewshowfeedback")]
    show_feedback: Option<bool>,
    #[serde(rename = "reviewshowexpectedanswer")]
    show_expected_answer: Option<bool>,
    #[serde(rename = "reviewshowadvice")]
    show_advice: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamFeedbackMessage {
    message: String,
    threshold: String, //TODO type
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamFunction {
    //TODO
    parameters: Vec<ExamFunctionParameter>,
    #[serde(rename = "type")]
    output_type: ExamFunctionType,
    definition: String,
    language: ExamFunctionLanguage,
}

pub type ExamFunctionParameter = (String, ExamFunctionType);

#[derive(Serialize, Deserialize, Debug)]
pub enum ExamFunctionLanguage {
    #[serde(rename = "jme")]
    JME,
    #[serde(rename = "javascript")]
    JavaScript,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExamFunctionType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "dict")]
    Dict,
    #[serde(rename = "expression")]
    Expression,
    #[serde(rename = "html")]
    HTML,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "keypair")]
    KeyPair,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "matrix")]
    Matrix,
    #[serde(rename = "nothing")]
    Nothing,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "rational")]
    Rational,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "string")]
    r#String,
    #[serde(rename = "vector")]
    Vector,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamVariable {
    name: String,
    definition: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionGroup {
    //TODO
    name: Option<String>,
    #[serde(rename = "pickingStrategy")]
    picking_strategy: ExamQuestionGroupPickingStrategy,
    questions: Vec<ExamQuestion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExamQuestionGroupPickingStrategy {
    #[serde(rename = "all-ordered")]
    AllOrdered,
    #[serde(rename = "all-shuffled")]
    AllShuffled,
    #[serde(rename = "random-subset")]
    RandomSubset {
        #[serde(rename = "pickQuestions")]
        pick_questions: usize,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestion {
    //TODO
    name: String,
    statement: String,
    advice: String,
    parts: Vec<ExamQuestionPart>,
    variables: HashMap<String, ExamVariable>,
    #[serde(rename = "variablesTest")]
    variables_test: ExamQuestionVariablesTest,
    functions: HashMap<String, ExamFunction>,
    ungrouped_variables: Vec<String>,
    //variable_groups
    //rulesets TODO
    //preamble TODO
    //contributors TODO
    navigation: ExamNavigation,
    //custom part types TODO
    extensions: Vec<String>,
    //metadata TODO
    //resources TODO
    //TODO type: question?
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ExamQuestionPart {
    //TODO: custom_part_constructor types?
    #[serde(rename = "jme")]
    JME(ExamQuestionPartJME),
    #[serde(rename = "numberentry")]
    NumberEntry(ExamQuestionPartNumberEntry),
    #[serde(rename = "matrix")]
    Matrix(ExamQuestionPartMatrix),
    #[serde(rename = "patternmatch")]
    PatternMatch(ExamQuestionPartPatternMatch),
    #[serde(rename = "1_n_2")]
    OneNTwo(ExamQuestionPartMultipleChoice),
    #[serde(rename = "m_n_2")]
    MNTwo(ExamQuestionPartMultipleChoice),
    #[serde(rename = "m_n_x")]
    MNX(ExamQuestionPartMultipleChoice),
    #[serde(rename = "gapfill")]
    GapFill(ExamQuestionPartGapFill),
    #[serde(rename = "information")]
    Information(ExamQuestionPartInformation),
    #[serde(rename = "extension")]
    Extension(ExamQuestionPartExtension),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartSharedData {
    marks: Option<usize>,
    prompt: Option<String>, //TODO option? Maybe not in this type, but in other. Some types require this, other's not?
    #[serde(rename = "useCustomName")]
    use_custom_name: Option<bool>,
    #[serde(rename = "customName")]
    custom_name: Option<String>,
    #[serde(
        rename = "stepsPenalty",
        default,
        deserialize_with = "from_str_optional"
    )]
    steps_penalty: Option<usize>,
    #[serde(rename = "enableMinimumMarks")]
    enable_minimum_marks: Option<bool>,
    #[serde(rename = "minimumMarks")]
    minimum_marks: Option<usize>,
    #[serde(rename = "showCorrectAnswer")]
    show_correct_answer: bool,
    #[serde(rename = "showFeedbackIcon")]
    show_feedback_icon: Option<bool>,
    #[serde(rename = "variableReplacementStrategy")]
    variable_replacement_strategy: VariableReplacementStrategy,
    #[serde(rename = "adaptiveMarkingPenalty")]
    adaptive_marking_penalty: Option<usize>,
    #[serde(rename = "customMarkingAlgorithm")]
    custom_marking_algorithm: Option<String>,
    #[serde(rename = "extendBaseMarkingAlgorithm")]
    extend_base_marking_algorithm: Option<bool>,
    steps: Option<Vec<ExamQuestionPart>>,
    //scripts TODO
    //[serde(rename= "variableReplacements")]
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "originalfirst")]
    OriginalFirst,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartJME {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    answer: String,
    #[serde(
        rename = "answerSimplification",
        default,
        deserialize_with = "answer_simplification_deserialize"
    )]
    answer_simplification: Option<Vec<AnswerSimplificationType>>, //comma separated list
    #[serde(rename = "showPreview")]
    show_preview: bool,
    #[serde(rename = "checkingType")]
    checking_type: JMECheckingType,
    #[serde(rename = "checkingAccuracy")]
    checking_accuracy: f64,
    #[serde(rename = "failureRate")]
    failure_rate: f64,
    #[serde(rename = "vsetRange")]
    vset_range: [f64; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    #[serde(rename = "vsetRangePoints")]
    vset_range_points: usize,
    #[serde(rename = "checkVariableNames")]
    check_variable_names: bool,
    #[serde(rename = "singleLetterVariables")]
    single_letter_variables: Option<bool>,
    #[serde(rename = "allowUnknownFunctions")]
    allow_unknown_functions: Option<bool>,
    #[serde(rename = "implicitFunctionComposition")]
    implicit_function_composition: Option<bool>,
    #[serde(rename = "maxlength")]
    max_length: Option<JMELengthRestriction>, // TODO: all restrictions to one flattended struct?
    #[serde(rename = "minlength")]
    min_length: Option<JMELengthRestriction>,
    #[serde(rename = "musthave")]
    must_have: Option<JMEStringRestriction>,
    #[serde(rename = "notallowed")]
    may_not_have: Option<JMEStringRestriction>,
    #[serde(rename = "mustmatchpattern")]
    must_match_pattern: Option<JMEPatternRestriction>,
    //TODO: valuegenerators
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnswerSimplificationType {
    Basic,
    UnitFactor,
    UnitPower,
    UnitDenominator,
    ZeroFactor,
    ZeroTerm,
    ZeroPower,
    CollectNumbers,
    ZeroBase,
    ConstantsFirst,
    SqrtProduct,
    SqrtDivision,
    SqrtSquare,
    OtherNumbers,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JMECheckingType {
    //TODO: other items (dp and sigfig)
    #[serde(rename = "reldiff")]
    RelativeDifference,
    #[serde(rename = "absdiff")]
    AbsoluteDifference,
    #[serde(rename = "dp")]
    DecimalPlaces,
    #[serde(rename = "sigfig")]
    SignificantFigures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JMERestriction {
    name: String,
    strings: Vec<String>,
    #[serde(rename = "partialCredit")]
    partial_credit: String, //TODO: type
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JMELengthRestriction {
    #[serde(flatten)]
    restriction: JMERestriction,
    length: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JMEStringRestriction {
    #[serde(flatten)]
    restriction: JMERestriction,
    #[serde(rename = "showStrings")]
    show_strings: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JMEPatternRestriction {
    #[serde(flatten)]
    restriction: JMERestriction,
    pattern: String, //TODO type?
    #[serde(rename = "nameToCompare")]
    name_to_compare: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionVariablesTest {
    condition: String,
    #[serde(rename = "maxRuns")]
    max_runs: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartNumberEntry {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    #[serde(rename = "correctAnswerFraction")]
    correct_answer_fraction: bool,
    #[serde(rename = "correctAnswerStyle")]
    correct_answer_style: Option<AnswerStyle>,
    #[serde(rename = "allowFractions")]
    allow_fractions: bool,
    #[serde(rename = "notationStyles")]
    notation_styles: Option<Vec<AnswerStyle>>,
    #[serde(rename = "checkingType")]
    checking_type: Option<CheckingType>,
    #[serde(rename = "inputStep")]
    input_step: Option<usize>,
    #[serde(rename = "mustBeReduced")]
    must_be_reduced: Option<bool>,
    #[serde(rename = "mustBeReducedPC")]
    must_be_reduced_pc: Option<usize>,
    #[serde(flatten)]
    precision: Option<QuestionPrecision>,
    #[serde(rename = "showPrecisionHint")]
    show_precision_hint: Option<bool>,
    #[serde(rename = "showFractionHint")]
    show_fraction_hint: Option<bool>,
    #[serde(flatten)]
    answer: Option<NumberEntryAnswerType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnswerStyle {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "si-en")]
    SiEn,
    #[serde(rename = "si-fr")]
    SiFr,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CheckingType {
    #[serde(rename = "range")]
    Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NumberEntryAnswerType {
    MinMax {
        #[serde(rename = "minvalue")]
        min_value: Primitive,
        #[serde(rename = "maxvalue")]
        max_value: Primitive,
    },
    Answer {
        answer: Primitive,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionPrecision {
    #[serde(rename = "precisionType")]
    precision_type: String, //TODO: enum ('none',...)
    #[serde(rename = "precision")]
    precision: usize,
    #[serde(rename = "precisionPartialCredit")]
    precision_partial_credit: usize,
    #[serde(rename = "precisionMessage")]
    precision_message: String,
    #[serde(rename = "strictPrecision")]
    strict_precision: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum QuestionPrecisionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dp")]
    DecimalPlaces,
    #[serde(rename = "sigfig")]
    SignificantFigures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartMatrix {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    #[serde(rename = "correctAnswer")]
    correct_answer: Primitive,
    #[serde(rename = "correctAnswerFractions")]
    correct_answer_fractions: bool,
    #[serde(rename = "numRows")]
    num_rows: usize,
    #[serde(rename = "numColumns")]
    num_columns: usize,
    #[serde(rename = "allowResize")]
    allow_resize: bool,
    #[serde(rename = "minColumns")]
    min_columns: usize,
    #[serde(rename = "maxColumns")]
    max_columns: usize,
    #[serde(rename = "minRows")]
    min_rows: usize,
    #[serde(rename = "maxRows")]
    max_rows: usize,
    #[serde(rename = "tolerance")]
    tolerance: f64,
    #[serde(rename = "markPerCell")]
    mark_per_cell: bool,
    #[serde(rename = "allowFractions")]
    allow_fractions: bool,
    #[serde(flatten)]
    precision: QuestionPrecision,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartPatternMatch {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    #[serde(rename = "caseSensitive")]
    case_sensitive: bool,
    #[serde(rename = "partialCredit")]
    partial_credit: usize,
    answer: Primitive,
    #[serde(rename = "displayAnswer")]
    display_answer: Option<Primitive>,
    #[serde(rename = "matchMode")]
    match_mode: PatternMatchMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatternMatchMode {
    #[serde(rename = "regex")]
    Regex,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartMultipleChoice {
    //TODO
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    #[serde(rename = "minMarks")]
    min_marks: Option<usize>,
    #[serde(rename = "maxMarks")]
    max_marks: Option<usize>,
    #[serde(rename = "minAnswers")]
    min_answers: Option<usize>,
    #[serde(rename = "maxAnswers")]
    max_answers: Option<usize>,
    #[serde(rename = "shuffleChoices")]
    shuffle_choices: bool,
    #[serde(rename = "shuffleAnswers")]
    shuffle_answers: Option<bool>,
    #[serde(rename = "displayType")]
    display_type: MultipleChoiceDisplayType,
    #[serde(rename = "displayColumns")]
    display_columns: usize,
    #[serde(rename = "warningType")]
    warning_type: Option<MultipleChoiceWarningType>,
    #[serde(flatten)]
    layout: Option<MultipleChoiceLayout>,
    #[serde(rename = "showCellAnswerState")]
    show_cell_answer_state: bool,
    choices: Vec<String>,
    answers: Option<Vec<String>>,
    matrix: Option<MultipleChoiceMatrix>,
    distractors: Option<MultipleChoiceMatrix>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MultipleChoiceDisplayType {
    #[serde(rename = "radiogroup")]
    Radio,
    #[serde(rename = "checkbox")]
    Check,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum MultipleChoiceWarningType {
    #[serde(rename = "none")]
    None,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum MultipleChoiceLayoutType {
    #[serde(rename = "all")]
    All,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleChoiceLayout {
    r#type: MultipleChoiceLayoutType,
    expression: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MultipleChoiceMatrix {
    Item(Primitive),
    Row(Vec<Primitive>),
    Matrix(Vec<Vec<Primitive>>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Primitive {
    String(String),
    Integer(usize),
    Float(f64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartGapFill {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    #[serde(rename = "sortAnswers")]
    sort_answers: Option<bool>,
    gaps: Vec<ExamQuestionPart>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartInformation {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExamQuestionPartExtension {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
}
