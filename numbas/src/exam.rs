//Currently based on https://github.com/numbas/Numbas/blob/f420421a7ef3c2cd4c39e43f377d2a363ae2f81e/bin/exam.py
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

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
                    "timesDot" => Ok(AnswerSimplificationType::TimesDot),
                    "expandBrackets" => Ok(AnswerSimplificationType::ExpandBrackets),
                    "noLeadingMinus" => Ok(AnswerSimplificationType::NoLeadingMinus),
                    "trig" => Ok(AnswerSimplificationType::Trigonometric),
                    "collectLikeFractions" => Ok(AnswerSimplificationType::CollectLikeFractions),
                    "canonicalOrder" => Ok(AnswerSimplificationType::CanonicalOrder),
                    "cancelFactors" => Ok(AnswerSimplificationType::CancelFactors),
                    "cancelTerms" => Ok(AnswerSimplificationType::CancelTerms),
                    "simplifyFractions" => Ok(AnswerSimplificationType::Fractions),
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

fn answer_simplification_serialize<S>(
    values_o: &Option<Vec<AnswerSimplificationType>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(values) = values_o {
        let mut parts: Vec<String> = Vec::new();
        for value in values {
            let new_item = match value {
                AnswerSimplificationType::Basic => "basic",
                AnswerSimplificationType::UnitFactor => "unitFactor",
                AnswerSimplificationType::UnitPower => "unitPower",
                AnswerSimplificationType::UnitDenominator => "unitDenominator",
                AnswerSimplificationType::ZeroFactor => "zeroFactor",
                AnswerSimplificationType::ZeroTerm => "zeroTerm",
                AnswerSimplificationType::ZeroPower => "zeroPower",
                AnswerSimplificationType::CollectNumbers => "collectNumbers",
                AnswerSimplificationType::ZeroBase => "zeroBase",
                AnswerSimplificationType::ConstantsFirst => "constantsFirst",
                AnswerSimplificationType::SqrtProduct => "sqrtProduct",
                AnswerSimplificationType::SqrtDivision => "sqrtDivision",
                AnswerSimplificationType::SqrtSquare => "sqrtSquare",
                AnswerSimplificationType::OtherNumbers => "otherNumbers",
                AnswerSimplificationType::TimesDot => "timesDot",
                AnswerSimplificationType::ExpandBrackets => "expandBrackets",
                AnswerSimplificationType::NoLeadingMinus => "noLeadingMinus",
                AnswerSimplificationType::Trigonometric => "trig",
                AnswerSimplificationType::CollectLikeFractions => "collectLikeFractions",
                AnswerSimplificationType::CanonicalOrder => "canonicalOrder",
                AnswerSimplificationType::CancelFactors => "cancelFactors",
                AnswerSimplificationType::CancelTerms => "cancelTerms",
                AnswerSimplificationType::Fractions => "simplifyFractions",
            };
            parts.push(new_item.to_string());
        }
        s.serialize_str(&parts.join(",")[..])
    } else {
        s.serialize_str("")
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Exam {
    #[serde(flatten)]
    basic_settings: BasicExamSettings,
    resources: Vec<[String; 2]>,
    extensions: Vec<String>,
    custom_part_types: Vec<CustomPartType>,

    navigation: ExamNavigation,
    timing: ExamTiming,
    feedback: ExamFeedback,

    //rulesets: HashMap<String, String>, //TODO + Type
    functions: Option<HashMap<String, ExamFunction>>,
    variables: Option<HashMap<String, ExamVariable>>,
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

    pub fn new(
        basic_settings: BasicExamSettings,
        resources: Vec<[String; 2]>,
        extensions: Vec<String>,
        custom_part_types: Vec<CustomPartType>,
        navigation: ExamNavigation,
        timing: ExamTiming,
        feedback: ExamFeedback,
        functions: Option<HashMap<String, ExamFunction>>,
        variables: Option<HashMap<String, ExamVariable>>,
        question_groups: Vec<ExamQuestionGroup>,
    ) -> Exam {
        Exam {
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
        }
    }

    //TODO: return Result type instead of printing errors
    pub fn write(&self, file_name: &str) {
        match serde_json::to_string(self) {
            Ok(s) => match std::fs::write(
                file_name,
                format!(
                    r#"// Numbas version: exam_results_page_options
{}"#,
                    s
                ),
            ) {
                Ok(_) => println!("Saved {}", file_name),
                Err(e) => println!("Error saving {}: {}", file_name, e),
            },
            Err(e) => println!("Error generating {}: {}", file_name, e),
        }
    }
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BasicExamSettings {
    name: String,
    #[serde(rename = "duration")]
    duration_in_seconds: Option<usize>,
    #[serde(rename = "percentPass", deserialize_with = "from_str_optional")]
    percentage_needed_to_pass: Option<f64>,
    #[serde(rename = "showQuestionGroupNames")]
    show_question_group_names: Option<bool>,
    #[serde(rename = "showstudentname")]
    show_student_name: Option<bool>,
}

impl BasicExamSettings {
    pub fn new(
        name: String,
        duration_in_seconds: Option<usize>,
        percentage_needed_to_pass: Option<f64>,
        show_question_group_names: Option<bool>,
        show_student_name: Option<bool>,
    ) -> BasicExamSettings {
        BasicExamSettings {
            name,
            duration_in_seconds,
            percentage_needed_to_pass,
            show_question_group_names,
            show_student_name,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartType {} //TODO: add fields

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamNavigation {
    #[serde(rename = "allowregen")]
    allow_regenerate: bool,
    reverse: Option<bool>,
    #[serde(rename = "browse")]
    browsing_enabled: Option<bool>,
    #[serde(rename = "navigatemode")]
    navigation_mode: Option<ExamNavigationMode>,
    #[serde(rename = "allowsteps")]
    allow_steps: Option<bool>,
    #[serde(rename = "showfrontpage")]
    show_frontpage: bool,
    #[serde(rename = "showresultspage")]
    show_results_page: Option<ExamShowResultsPage>,
    #[serde(rename = "preventleave")]
    prevent_leaving: Option<bool>,
    #[serde(rename = "onleave")]
    on_leave: Option<ExamLeaveAction>,
    #[serde(rename = "startpassword")]
    start_password: Option<String>, //TODO: if empty string -> also None
}

impl ExamNavigation {
    pub fn new(
        allow_regenerate: bool,
        reverse: Option<bool>,
        browsing_enabled: Option<bool>,
        navigation_mode: Option<ExamNavigationMode>,
        allow_steps: Option<bool>,
        show_frontpage: bool,
        show_results_page: Option<ExamShowResultsPage>,
        prevent_leaving: Option<bool>,
        on_leave: Option<ExamLeaveAction>,
        start_password: Option<String>,
    ) -> ExamNavigation {
        ExamNavigation {
            allow_regenerate,
            reverse,
            browsing_enabled,
            navigation_mode,
            allow_steps,
            show_frontpage,
            show_results_page,
            prevent_leaving,
            on_leave,
            start_password,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuestionNavigation {
    #[serde(rename = "allowregen")]
    allow_regenerate: bool,
    #[serde(rename = "showfrontpage")]
    show_frontpage: bool,
    #[serde(rename = "preventleave")]
    prevent_leaving: Option<bool>,
}

impl QuestionNavigation {
    pub fn new(
        allow_regenerate: bool,
        show_frontpage: bool,
        prevent_leaving: Option<bool>,
    ) -> QuestionNavigation {
        QuestionNavigation {
            allow_regenerate,
            show_frontpage,
            prevent_leaving,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "action")]
pub enum ExamLeaveAction {
    #[serde(rename = "none")]
    None { message: String }, //This message doesn't do anything
    #[serde(rename = "warnifunattempted")]
    WarnIfNotAttempted { message: String }, // Show a warning message if a user moves away from a question that is not attempted
    #[serde(rename = "preventifunattempted")]
    PreventIfNotAttempted { message: String }, // Prevent a user from moving away from a question that is not attempted
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "action")]
pub enum ExamTimeoutAction {
    #[serde(rename = "none")]
    None { message: String }, //This message doesn't do anything
    #[serde(rename = "warn")]
    Warn { message: String }, // Show a warning message
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExamNavigationMode {
    #[serde(rename = "sequence")]
    Sequence,
    #[serde(rename = "menu")]
    Menu,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExamShowResultsPage {
    #[serde(rename = "oncompletion")]
    OnCompletion,
    #[serde(rename = "never")]
    Never,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamTiming {
    #[serde(rename = "allowPause")]
    allow_pause: bool,
    timeout: ExamTimeoutAction, // Action to do on timeout
    #[serde(rename = "timedwarning")]
    timed_warning: ExamTimeoutAction, // Action to do five minutes before timeout
}

impl ExamTiming {
    pub fn new(
        allow_pause: bool,
        timeout: ExamTimeoutAction,
        timed_warning: ExamTimeoutAction,
    ) -> ExamTiming {
        ExamTiming {
            allow_pause,
            timeout,
            timed_warning,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl ExamFeedback {
    pub fn new(
        show_actual_mark: bool,
        show_total_mark: bool,
        show_answer_state: bool,
        allow_reveal_answer: bool,
        review: Option<ExamReview>,
        advice: Option<String>,
        intro: String,
        feedback_messages: Vec<ExamFeedbackMessage>,
    ) -> ExamFeedback {
        ExamFeedback {
            show_actual_mark,
            show_total_mark,
            show_answer_state,
            allow_reveal_answer,
            review,
            advice,
            intro,
            feedback_messages,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl ExamReview {
    pub fn new(
        show_score: Option<bool>,
        show_feedback: Option<bool>,
        show_expected_answer: Option<bool>,
        show_advice: Option<bool>,
    ) -> ExamReview {
        ExamReview {
            show_score,
            show_feedback,
            show_expected_answer,
            show_advice,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamFeedbackMessage {
    message: String,
    threshold: String, //TODO type
}

impl ExamFeedbackMessage {
    pub fn new(message: String, threshold: String) -> ExamFeedbackMessage {
        ExamFeedbackMessage { message, threshold }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamFunction {
    //TODO
    parameters: Vec<ExamFunctionParameter>,
    #[serde(rename = "type")]
    output_type: ExamFunctionType,
    definition: String,
    language: ExamFunctionLanguage,
}

impl ExamFunction {
    pub fn new(
        parameters: Vec<ExamFunctionParameter>,
        output_type: ExamFunctionType,
        definition: String,
        language: ExamFunctionLanguage,
    ) -> ExamFunction {
        ExamFunction {
            parameters,
            output_type,
            definition,
            language,
        }
    }
}

pub type ExamFunctionParameter = (String, ExamFunctionType);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExamFunctionLanguage {
    #[serde(rename = "jme")]
    JME,
    #[serde(rename = "javascript")]
    JavaScript,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamVariable {
    name: String,
    definition: String,
    description: String,
    #[serde(rename = "templateType")]
    template_type: ExamVariableTemplateType,
    group: String,
}

impl ExamVariable {
    pub fn new(
        name: String,
        definition: String,
        description: String,
        template_type: ExamVariableTemplateType,
        group: String,
    ) -> ExamVariable {
        ExamVariable {
            name,
            definition,
            description,
            template_type,
            group,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExamVariableTemplateType {
    #[serde(rename = "anything")]
    Anything, //JME
    #[serde(rename = "list of numbers")]
    ListOfNumbers,
    #[serde(rename = "list of strings")]
    ListOfStrings,
    #[serde(rename = "long string")]
    LongString,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "randrange")]
    RandomRange, // Chooses a value from the range
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "string")]
    r#String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionGroup {
    //TODO
    name: Option<String>,
    #[serde(flatten)]
    picking_strategy: ExamQuestionGroupPickingStrategy,
    questions: Vec<ExamQuestion>,
}

impl ExamQuestionGroup {
    pub fn new(
        name: Option<String>,
        picking_strategy: ExamQuestionGroupPickingStrategy,
        questions: Vec<ExamQuestion>,
    ) -> ExamQuestionGroup {
        ExamQuestionGroup {
            name,
            picking_strategy,
            questions,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "pickingStrategy")]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    variable_groups: Vec<String>,      //TODO: type
    rulesets: HashMap<String, String>, //TODO Type
    preamble: Preamble,
    //contributors TODO
    navigation: QuestionNavigation,
    //custom part types TODO
    extensions: Vec<String>,
    //metadata TODO
    //resources TODO
    //TODO type: question?
}

impl ExamQuestion {
    pub fn new(
        name: String,
        statement: String,
        advice: String,
        parts: Vec<ExamQuestionPart>,
        variables: HashMap<String, ExamVariable>,
        variables_test: ExamQuestionVariablesTest,
        functions: HashMap<String, ExamFunction>,
        ungrouped_variables: Vec<String>,
        variable_groups: Vec<String>,
        rulesets: HashMap<String, String>,
        preamble: Preamble,
        navigation: QuestionNavigation,
        extensions: Vec<String>,
    ) -> ExamQuestion {
        ExamQuestion {
            name,
            statement,
            advice,
            parts,
            variables,
            variables_test,
            functions,
            ungrouped_variables,
            variable_groups,
            rulesets,
            preamble,
            navigation,
            extensions,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preamble {
    js: String,
    css: String,
}

impl Preamble {
    pub fn new(js: String, css: String) -> Preamble {
        Preamble { js, css }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    ChooseOne(ExamQuestionPartChooseOne),
    #[serde(rename = "m_n_2")]
    ChooseMultiple(ExamQuestionPartChooseMultiple),
    #[serde(rename = "m_n_x")]
    MatchAnswersWithChoices(ExamQuestionPartMatchAnswersWithChoices),
    #[serde(rename = "gapfill")]
    GapFill(ExamQuestionPartGapFill),
    #[serde(rename = "information")]
    Information(ExamQuestionPartInformation),
    #[serde(rename = "extension")]
    Extension(ExamQuestionPartExtension),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl ExamQuestionPartSharedData {
    pub fn new(
        marks: Option<usize>,
        prompt: Option<String>,
        use_custom_name: Option<bool>,
        custom_name: Option<String>,
        steps_penalty: Option<usize>,
        enable_minimum_marks: Option<bool>,
        minimum_marks: Option<usize>,
        show_correct_answer: bool,
        show_feedback_icon: Option<bool>,
        variable_replacement_strategy: VariableReplacementStrategy,
        adaptive_marking_penalty: Option<usize>,
        custom_marking_algorithm: Option<String>,
        extend_base_marking_algorithm: Option<bool>,
        steps: Option<Vec<ExamQuestionPart>>,
    ) -> ExamQuestionPartSharedData {
        ExamQuestionPartSharedData {
            marks,
            prompt,
            use_custom_name,
            custom_name,
            steps_penalty,
            enable_minimum_marks,
            minimum_marks,
            show_correct_answer,
            show_feedback_icon,
            variable_replacement_strategy,
            adaptive_marking_penalty,
            custom_marking_algorithm,
            extend_base_marking_algorithm,
            steps,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "originalfirst")]
    OriginalFirst,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartJME {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    answer: String,
    #[serde(
        rename = "answerSimplification",
        default,
        deserialize_with = "answer_simplification_deserialize",
        serialize_with = "answer_simplification_serialize"
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
    #[serde(rename = "valuegenerators")]
    value_generators: Option<Vec<JMEValueGenerator>>,
}

impl ExamQuestionPartJME {
    pub fn new(
        part_data: ExamQuestionPartSharedData,
        answer: String,
        answer_simplification: Option<Vec<AnswerSimplificationType>>,
        show_preview: bool,
        checking_type: JMECheckingType,
        checking_accuracy: f64,
        failure_rate: f64,
        vset_range: [f64; 2],
        vset_range_points: usize,
        check_variable_names: bool,
        single_letter_variables: Option<bool>,
        allow_unknown_functions: Option<bool>,
        implicit_function_composition: Option<bool>,
        max_length: Option<JMELengthRestriction>,
        min_length: Option<JMELengthRestriction>,
        must_have: Option<JMEStringRestriction>,
        may_not_have: Option<JMEStringRestriction>,
        must_match_pattern: Option<JMEPatternRestriction>,
        value_generators: Option<Vec<JMEValueGenerator>>,
    ) -> ExamQuestionPartJME {
        ExamQuestionPartJME {
            part_data,
            answer,
            answer_simplification,
            show_preview,
            checking_type,
            checking_accuracy,
            failure_rate,
            vset_range,
            vset_range_points,
            check_variable_names,
            single_letter_variables,
            allow_unknown_functions,
            implicit_function_composition,
            max_length,
            min_length,
            must_have,
            may_not_have,
            must_match_pattern,
            value_generators,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AnswerSimplificationType {
    //TODO casing?
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
    TimesDot,
    ExpandBrackets,
    CollectLikeFractions,
    CanonicalOrder,
    NoLeadingMinus,
    Fractions,
    Trigonometric,
    CancelTerms,
    CancelFactors,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMERestriction {
    name: String,
    strings: Vec<String>,
    #[serde(rename = "partialCredit")]
    partial_credit: f64, //TODO: maybe usize?
    message: String,
}

impl JMERestriction {
    pub fn new(
        name: String,
        strings: Vec<String>,
        partial_credit: f64,
        message: String,
    ) -> JMERestriction {
        JMERestriction {
            name,
            strings,
            partial_credit,
            message,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMELengthRestriction {
    #[serde(flatten)]
    restriction: JMERestriction,
    length: Option<usize>,
}

impl JMELengthRestriction {
    pub fn new(restriction: JMERestriction, length: Option<usize>) -> JMELengthRestriction {
        JMELengthRestriction {
            restriction,
            length,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEStringRestriction {
    #[serde(flatten)]
    restriction: JMERestriction,
    #[serde(rename = "showStrings")]
    show_strings: bool,
}

impl JMEStringRestriction {
    pub fn new(restriction: JMERestriction, show_strings: bool) -> JMEStringRestriction {
        JMEStringRestriction {
            restriction,
            show_strings,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEPatternRestriction {
    #[serde(flatten)]
    restriction: JMERestriction,
    pattern: String, //TODO type?
    #[serde(rename = "nameToCompare")]
    name_to_compare: String,
}

impl JMEPatternRestriction {
    pub fn new(
        restriction: JMERestriction,
        pattern: String,
        name_to_compare: String,
    ) -> JMEPatternRestriction {
        JMEPatternRestriction {
            restriction,
            pattern,
            name_to_compare,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEValueGenerator {
    name: String,
    value: String,
}

impl JMEValueGenerator {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionVariablesTest {
    condition: String,
    #[serde(rename = "maxRuns")]
    max_runs: usize,
}

impl ExamQuestionVariablesTest {
    pub fn new(condition: String, max_runs: usize) -> ExamQuestionVariablesTest {
        ExamQuestionVariablesTest {
            condition,
            max_runs,
        }
    }
}

//TODO: docs https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/numberentry.js#L101
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartNumberEntry {
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "correctAnswerFraction")]
    pub correct_answer_fraction: bool,
    #[serde(rename = "correctAnswerStyle")]
    pub correct_answer_style: Option<AnswerStyle>,
    #[serde(rename = "allowFractions")]
    pub allow_fractions: bool,
    #[serde(rename = "notationStyles")]
    pub notation_styles: Option<Vec<AnswerStyle>>,
    #[serde(rename = "checkingType")]
    pub checking_type: Option<CheckingType>, //TODO: check if being used
    /*#[serde(rename = "inputStep")]
    pub input_step: Option<usize>,*/ //TODO: check if being used
    #[serde(rename = "mustBeReduced")]
    pub fractions_must_be_reduced: Option<bool>,
    #[serde(rename = "mustBeReducedPC")]
    pub partial_credit_if_fraction_not_reduced: Option<f64>,
    #[serde(flatten)]
    pub precision: Option<QuestionPrecision>,
    #[serde(rename = "showPrecisionHint")]
    pub show_precision_hint: Option<bool>,
    #[serde(rename = "showFractionHint")]
    pub show_fraction_hint: Option<bool>,
    #[serde(flatten)]
    pub answer: NumberEntryAnswerType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CheckingType {
    #[serde(rename = "range")]
    Range,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QuestionPrecisionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dp")]
    DecimalPlaces,
    #[serde(rename = "sigfig")]
    SignificantFigures,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartPatternMatch {
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: bool,
    #[serde(rename = "partialCredit")]
    pub partial_credit: f64,
    pub answer: Primitive,
    #[serde(rename = "displayAnswer")]
    pub display_answer: Option<Primitive>,
    #[serde(rename = "matchMode")]
    pub match_mode: PatternMatchMode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum PatternMatchMode {
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "exact")]
    Exact, //TODO: check all options
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartChooseOne {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<usize>, // Minimum number of responses the student must select
    #[serde(rename = "choices")]
    pub answers: Vec<String>,
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: ChooseOneDisplayType, // How to display the response selectors
    #[serde(rename = "displayColumns")]
    pub columns: u8, // How many columns to use to display the choices. Not usefull when dropdown -> optional? TODO
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<MultipleChoiceMatrix>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    pub distractors: Option<MultipleChoiceMatrix>, //TODO: type (contains only strings...)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChooseOneDisplayType {
    #[serde(rename = "radiogroup")]
    Radio,
    #[serde(rename = "dropdownlist")]
    DropDown,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartChooseMultiple {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "minMarks")]
    pub min_marks: Option<usize>, //TODO; what is difference with minimum_marks?
    #[serde(rename = "maxMarks")]
    pub max_marks: Option<usize>, // Is there a maximum number of marks the student can get?
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<usize>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<usize>, // Maximum number of responses the student can select
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayColumns")]
    pub display_columns: usize, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: Vec<String>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<MultipleChoiceMatrix>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    pub distractors: Option<MultipleChoiceMatrix>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartMatchAnswersWithChoices {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "minMarks")]
    pub min_marks: Option<usize>, //TODO; what is difference with minimum_marks? -> not for 1_n_2
    #[serde(rename = "maxMarks")]
    pub max_marks: Option<usize>, // Is there a maximum number of marks the student can get? -> not for 1_n_2
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<usize>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<usize>, // Maximum number of responses the student can select -> always one for 1_n_2
    #[serde(rename = "shuffleChoices")]
    pub shuffle_choices: bool,
    #[serde(rename = "shuffleAnswers")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: MatchAnswersWithChoicesDisplayType, // How to display the response selectors -> only for 1_n_2?
    //#[serde(rename = "displayColumns")] //TODO?
    //pub displayed_columns: usize, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses?
    pub layout: MatchAnswersWithChoicesLayout,
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: Vec<String>,
    pub answers: Vec<String>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<MultipleChoiceMatrix>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesDisplayType {
    #[serde(rename = "checkbox")]
    Check,
    #[serde(rename = "radiogroup")]
    Radio,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MultipleChoiceWarningType {
    #[serde(rename = "none")]
    None,
    //TODO: also prevent and warn -> same as leave actions?
    //https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L493
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesLayoutType {
    #[serde(rename = "all")]
    All,
    //TODO: https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L766
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MatchAnswersWithChoicesLayout {
    r#type: MatchAnswersWithChoicesLayoutType,
    expression: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MultipleChoiceMatrix {
    //TODO use specific type for the three types
    Item(Primitive),
    Row(Vec<Primitive>),
    Matrix(Vec<Vec<Primitive>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Primitive {
    String(String),
    Integer(usize),
    Float(f64),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartGapFill {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
    #[serde(rename = "sortAnswers")]
    sort_answers: Option<bool>,
    gaps: Vec<ExamQuestionPart>,
}

impl ExamQuestionPartGapFill {
    pub fn new(
        part_data: ExamQuestionPartSharedData,
        sort_answers: Option<bool>,
        gaps: Vec<ExamQuestionPart>,
    ) -> ExamQuestionPartGapFill {
        ExamQuestionPartGapFill {
            part_data,
            sort_answers,
            gaps,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartInformation {
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartExtension {
    #[serde(flatten)]
    part_data: ExamQuestionPartSharedData,
}
