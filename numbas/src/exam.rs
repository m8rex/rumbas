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
            for whole_item in s.split(',') {
                let (item, value) = if whole_item.starts_with("!") {
                    let mut chars = whole_item.chars();
                    chars.next();
                    (chars.as_str(), false)
                } else {
                    (whole_item, true)
                };
                let new_item = match item.to_lowercase().as_ref() {
                    "all" => Ok(AnswerSimplificationType::All(value)),
                    "basic" => Ok(AnswerSimplificationType::Basic(value)),
                    "unitfactor" => Ok(AnswerSimplificationType::UnitFactor(value)),
                    "unitpower" => Ok(AnswerSimplificationType::UnitPower(value)),
                    "unitdenominator" => Ok(AnswerSimplificationType::UnitDenominator(value)),
                    "zerofactor" => Ok(AnswerSimplificationType::ZeroFactor(value)),
                    "zeroterm" => Ok(AnswerSimplificationType::ZeroTerm(value)),
                    "zeropower" => Ok(AnswerSimplificationType::ZeroPower(value)),
                    "collectnumbers" => Ok(AnswerSimplificationType::CollectNumbers(value)),
                    "zerobase" => Ok(AnswerSimplificationType::ZeroBase(value)),
                    "constantsfirst" => Ok(AnswerSimplificationType::ConstantsFirst(value)),
                    "sqrtproduct" => Ok(AnswerSimplificationType::SqrtProduct(value)),
                    "sqrtfivision" => Ok(AnswerSimplificationType::SqrtDivision(value)),
                    "sqrtdquare" => Ok(AnswerSimplificationType::SqrtSquare(value)),
                    "othernumbers" => Ok(AnswerSimplificationType::OtherNumbers(value)),
                    "timesdot" => Ok(AnswerSimplificationType::TimesDot(value)),
                    "expandbrackets" => Ok(AnswerSimplificationType::ExpandBrackets(value)),
                    "noleadingminus" => Ok(AnswerSimplificationType::NoLeadingMinus(value)),
                    "trig" => Ok(AnswerSimplificationType::Trigonometric(value)),
                    "collectlikefractions" => {
                        Ok(AnswerSimplificationType::CollectLikeFractions(value))
                    }
                    "canonicalorder" => Ok(AnswerSimplificationType::CanonicalOrder(value)),
                    "cancelfactors" => Ok(AnswerSimplificationType::CancelFactors(value)),
                    "cancelterms" => Ok(AnswerSimplificationType::CancelTerms(value)),
                    "simplifyfractions" => Ok(AnswerSimplificationType::Fractions(value)),
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
                AnswerSimplificationType::All(true) => "all",
                AnswerSimplificationType::All(false) => "!all",
                AnswerSimplificationType::Basic(true) => "basic",
                AnswerSimplificationType::Basic(false) => "!basic",
                AnswerSimplificationType::UnitFactor(true) => "unitFactor",
                AnswerSimplificationType::UnitFactor(false) => "!unitFactor",
                AnswerSimplificationType::UnitPower(true) => "unitPower",
                AnswerSimplificationType::UnitPower(false) => "!unitPower",
                AnswerSimplificationType::UnitDenominator(true) => "unitDenominator",
                AnswerSimplificationType::UnitDenominator(false) => "!unitDenominator",
                AnswerSimplificationType::ZeroFactor(true) => "zeroFactor",
                AnswerSimplificationType::ZeroFactor(false) => "!zeroFactor",
                AnswerSimplificationType::ZeroTerm(true) => "zeroTerm",
                AnswerSimplificationType::ZeroTerm(false) => "!zeroTerm",
                AnswerSimplificationType::ZeroPower(true) => "zeroPower",
                AnswerSimplificationType::ZeroPower(false) => "!zeroPower",
                AnswerSimplificationType::CollectNumbers(true) => "collectNumbers",
                AnswerSimplificationType::CollectNumbers(false) => "!collectNumbers",
                AnswerSimplificationType::ZeroBase(true) => "zeroBase",
                AnswerSimplificationType::ZeroBase(false) => "!zeroBase",
                AnswerSimplificationType::ConstantsFirst(true) => "constantsFirst",
                AnswerSimplificationType::ConstantsFirst(false) => "!constantsFirst",
                AnswerSimplificationType::SqrtProduct(true) => "sqrtProduct",
                AnswerSimplificationType::SqrtProduct(false) => "!sqrtProduct",
                AnswerSimplificationType::SqrtDivision(true) => "sqrtDivision",
                AnswerSimplificationType::SqrtDivision(false) => "!sqrtDivision",
                AnswerSimplificationType::SqrtSquare(true) => "sqrtSquare",
                AnswerSimplificationType::SqrtSquare(false) => "!sqrtSquare",
                AnswerSimplificationType::OtherNumbers(true) => "otherNumbers",
                AnswerSimplificationType::OtherNumbers(false) => "!otherNumbers",
                AnswerSimplificationType::TimesDot(true) => "timesDot",
                AnswerSimplificationType::TimesDot(false) => "!timesDot",
                AnswerSimplificationType::ExpandBrackets(true) => "expandBrackets",
                AnswerSimplificationType::ExpandBrackets(false) => "!expandBrackets",
                AnswerSimplificationType::NoLeadingMinus(true) => "noLeadingMinus",
                AnswerSimplificationType::NoLeadingMinus(false) => "!noLeadingMinus",
                AnswerSimplificationType::Trigonometric(true) => "trig",
                AnswerSimplificationType::Trigonometric(false) => "!trig",
                AnswerSimplificationType::CollectLikeFractions(true) => "collectLikeFractions",
                AnswerSimplificationType::CollectLikeFractions(false) => "!collectLikeFractions",
                AnswerSimplificationType::CanonicalOrder(true) => "canonicalOrder",
                AnswerSimplificationType::CanonicalOrder(false) => "!canonicalOrder",
                AnswerSimplificationType::CancelFactors(true) => "cancelFactors",
                AnswerSimplificationType::CancelFactors(false) => "!cancelFactors",
                AnswerSimplificationType::CancelTerms(true) => "cancelTerms",
                AnswerSimplificationType::CancelTerms(false) => "!cancelTerms",
                AnswerSimplificationType::Fractions(true) => "simplifyFractions",
                AnswerSimplificationType::Fractions(false) => "!simplifyFractions",
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
    pub basic_settings: BasicExamSettings,
    pub resources: Vec<[String; 2]>,
    pub extensions: Vec<String>,
    pub custom_part_types: Vec<CustomPartType>,

    pub navigation: ExamNavigation,
    pub timing: ExamTiming,
    pub feedback: ExamFeedback,

    //rulesets: HashMap<String, String>, //TODO + Type
    pub functions: Option<HashMap<String, ExamFunction>>,
    pub variables: Option<HashMap<String, ExamVariable>>,
    pub question_groups: Vec<ExamQuestionGroup>,
    //contributors TODO
    //metadata TODO
    pub diagnostic: Option<ExamDiagnostic>,
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
        diagnostic: Option<ExamDiagnostic>,
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
            diagnostic,
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
    pub name: String,
    #[serde(rename = "duration")]
    pub duration_in_seconds: Option<usize>,
    #[serde(rename = "percentPass", deserialize_with = "from_str_optional")]
    pub percentage_needed_to_pass: Option<f64>,
    #[serde(rename = "showQuestionGroupNames")]
    pub show_question_group_names: Option<bool>,
    #[serde(rename = "showstudentname")]
    pub show_student_name: Option<bool>,
    #[serde(rename = "allowPrinting")]
    /// Whether students are ammpwed to print an exam transcript
    pub allow_printing: Option<bool>,
}

impl BasicExamSettings {
    pub fn new(
        name: String,
        duration_in_seconds: Option<usize>,
        percentage_needed_to_pass: Option<f64>,
        show_question_group_names: Option<bool>,
        show_student_name: Option<bool>,
        allow_printing: Option<bool>,
    ) -> BasicExamSettings {
        BasicExamSettings {
            name,
            duration_in_seconds,
            percentage_needed_to_pass,
            show_question_group_names,
            show_student_name,
            allow_printing,
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
    pub allow_regenerate: bool,
    #[serde(rename = "navigatemode")]
    pub navigation_mode: ExamNavigationMode,
    pub reverse: Option<bool>,
    #[serde(rename = "browse")]
    pub browsing_enabled: Option<bool>,
    #[serde(rename = "allowsteps")]
    pub allow_steps: Option<bool>,
    #[serde(rename = "showfrontpage")]
    pub show_frontpage: bool,
    #[serde(rename = "showresultspage")]
    pub show_results_page: Option<ExamShowResultsPage>,
    #[serde(rename = "preventleave")]
    pub prevent_leaving: Option<bool>,
    #[serde(rename = "onleave")]
    pub on_leave: Option<ExamLeaveAction>,
    #[serde(rename = "startpassword")]
    pub start_password: Option<String>, //TODO: if empty string -> also None
}

impl ExamNavigation {
    pub fn new(
        allow_regenerate: bool,
        navigation_mode: ExamNavigationMode,
        reverse: Option<bool>,
        browsing_enabled: Option<bool>,
        allow_steps: Option<bool>,
        show_frontpage: bool,
        show_results_page: Option<ExamShowResultsPage>,
        prevent_leaving: Option<bool>,
        on_leave: Option<ExamLeaveAction>,
        start_password: Option<String>,
    ) -> ExamNavigation {
        ExamNavigation {
            allow_regenerate,
            navigation_mode,
            reverse,
            browsing_enabled,
            allow_steps,
            show_frontpage,
            show_results_page,
            prevent_leaving,
            on_leave,
            start_password,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExamNavigationMode {
    Sequence,
    Menu,
    Diagnostic,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuestionNavigation {
    #[serde(rename = "allowregen")]
    pub allow_regenerate: bool,
    #[serde(rename = "showfrontpage")]
    pub show_frontpage: bool,
    #[serde(rename = "preventleave")]
    pub prevent_leaving: Option<bool>,
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
    pub allow_pause: bool,
    pub timeout: ExamTimeoutAction, // Action to do on timeout
    #[serde(rename = "timedwarning")]
    pub timed_warning: ExamTimeoutAction, // Action to do five minutes before timeout
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
    pub show_actual_mark: bool, // show student's score
    #[serde(rename = "showtotalmark")]
    pub show_total_mark: bool, // show total marks available
    #[serde(rename = "showanswerstate")]
    pub show_answer_state: bool, // Show whether answer was correct
    #[serde(rename = "allowrevealanswer")]
    pub allow_reveal_answer: bool,
    #[serde(flatten)]
    pub review: Option<ExamReview>,
    pub advice: Option<String>,
    pub intro: String,
    #[serde(rename = "feedbackmessages")]
    pub feedback_messages: Vec<ExamFeedbackMessage>,
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
    pub show_score: Option<bool>,
    #[serde(rename = "reviewshowfeedback")]
    pub show_feedback: Option<bool>,
    #[serde(rename = "reviewshowexpectedanswer")]
    pub show_expected_answer: Option<bool>,
    #[serde(rename = "reviewshowadvice")]
    pub show_advice: Option<bool>,
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
    pub message: String,
    pub threshold: String, //TODO type
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
    pub parameters: Vec<ExamFunctionParameter>,
    #[serde(rename = "type")]
    pub output_type: ExamFunctionType,
    pub definition: String,
    pub language: ExamFunctionLanguage,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum ExamFunctionLanguage {
    #[serde(rename = "jme")]
    JME,
    #[serde(rename = "javascript")]
    JavaScript,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
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
    Natural,
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
    pub name: String,
    pub definition: String,
    pub description: String,
    #[serde(rename = "templateType")]
    pub template_type: ExamVariableTemplateType,
    pub group: String,
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
    pub name: Option<String>,
    #[serde(flatten)]
    pub picking_strategy: ExamQuestionGroupPickingStrategy,
    pub questions: Vec<ExamQuestion>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BuiltinConstants(pub std::collections::HashMap<String, bool>);

impl Default for BuiltinConstants {
    fn default() -> Self {
        BuiltinConstants(
            vec![
                ("e".to_string(), true),
                ("pi,\u{03c0}".to_string(), true),
                ("i".to_string(), true),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestion {
    //TODO
    pub name: String,
    pub statement: String,
    pub advice: String,
    pub parts: Vec<ExamQuestionPart>,
    #[serde(default)]
    pub builtin_constants: BuiltinConstants,
    #[serde(default)]
    pub constants: Vec<ExamQuestionConstant>,
    pub variables: HashMap<String, ExamVariable>,
    #[serde(rename = "variablesTest")]
    pub variables_test: ExamQuestionVariablesTest,
    pub functions: HashMap<String, ExamFunction>,
    pub ungrouped_variables: Vec<String>,
    pub variable_groups: Vec<String>,      //TODO: type
    pub rulesets: HashMap<String, String>, //TODO Type
    pub preamble: Preamble,
    //contributors TODO
    pub navigation: QuestionNavigation,
    //custom part types TODO
    pub extensions: Vec<String>,
    //metadata TODO
    //resources TODO
    //TODO type: question?
    /// Tags starting with 'skill: ' are used in diagnostic mode to specify a topic
    pub tags: Vec<String>,
}

impl ExamQuestion {
    pub fn new(
        name: String,
        statement: String,
        advice: String,
        parts: Vec<ExamQuestionPart>,
        builtin_constants: std::collections::HashMap<String, bool>,
        constants: Vec<ExamQuestionConstant>,
        variables: HashMap<String, ExamVariable>,
        variables_test: ExamQuestionVariablesTest,
        functions: HashMap<String, ExamFunction>,
        ungrouped_variables: Vec<String>,
        variable_groups: Vec<String>,
        rulesets: HashMap<String, String>,
        preamble: Preamble,
        navigation: QuestionNavigation,
        extensions: Vec<String>,
        tags: Vec<String>,
    ) -> ExamQuestion {
        ExamQuestion {
            name,
            statement,
            advice,
            parts,
            builtin_constants: BuiltinConstants(builtin_constants),
            constants,
            variables,
            variables_test,
            functions,
            ungrouped_variables,
            variable_groups,
            rulesets,
            preamble,
            navigation,
            extensions,
            tags,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preamble {
    pub js: String,
    pub css: String,
}

impl Preamble {
    pub fn new(js: String, css: String) -> Preamble {
        Preamble { js, css }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionConstant {
    pub name: String,
    pub value: String,
    pub tex: String,
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
    pub marks: Option<Primitive>,
    pub prompt: Option<String>, //TODO option? Maybe not in this type, but in other. Some types require this, other's not?
    #[serde(rename = "useCustomName")]
    pub use_custom_name: Option<bool>,
    #[serde(rename = "customName")]
    pub custom_name: Option<String>,
    #[serde(
        rename = "stepsPenalty",
        default,
        deserialize_with = "from_str_optional"
    )]
    pub steps_penalty: Option<usize>,
    #[serde(rename = "enableMinimumMarks")]
    pub enable_minimum_marks: Option<bool>,
    #[serde(rename = "minimumMarks")]
    pub minimum_marks: Option<usize>,
    #[serde(rename = "showCorrectAnswer")]
    pub show_correct_answer: bool,
    #[serde(rename = "showFeedbackIcon")]
    pub show_feedback_icon: Option<bool>,
    #[serde(rename = "variableReplacementStrategy")]
    pub variable_replacement_strategy: VariableReplacementStrategy,
    #[serde(rename = "adaptiveMarkingPenalty")]
    pub adaptive_marking_penalty: Option<usize>,
    #[serde(rename = "customMarkingAlgorithm")]
    pub custom_marking_algorithm: Option<String>,
    #[serde(rename = "extendBaseMarkingAlgorithm")]
    pub extend_base_marking_algorithm: Option<bool>,
    pub steps: Option<Vec<ExamQuestionPart>>,
    //scripts TODO
    //[serde(rename= "variableReplacements")]
}

impl ExamQuestionPartSharedData {
    pub fn new(
        marks: Option<Primitive>,
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
    pub part_data: ExamQuestionPartSharedData,
    pub answer: String,
    #[serde(
        rename = "answerSimplification",
        default,
        deserialize_with = "answer_simplification_deserialize",
        serialize_with = "answer_simplification_serialize"
    )]
    pub answer_simplification: Option<Vec<AnswerSimplificationType>>, //comma separated list
    #[serde(rename = "showPreview")]
    pub show_preview: bool,
    #[serde(rename = "checkingType")]
    #[serde(flatten)]
    pub checking_type: JMECheckingType,
    #[serde(rename = "failureRate")]
    pub failure_rate: f64,
    #[serde(rename = "vsetRange")]
    pub vset_range: [f64; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    #[serde(rename = "vsetRangePoints")]
    pub vset_range_points: usize,
    #[serde(rename = "checkVariableNames")]
    pub check_variable_names: bool,
    #[serde(rename = "singleLetterVariables")]
    pub single_letter_variables: Option<bool>,
    #[serde(rename = "allowUnknownFunctions")]
    pub allow_unknown_functions: Option<bool>,
    #[serde(rename = "implicitFunctionComposition")]
    pub implicit_function_composition: Option<bool>,
    #[serde(rename = "maxlength")]
    pub max_length: Option<JMELengthRestriction>, // TODO: all restrictions to one flattended struct?
    #[serde(rename = "minlength")]
    pub min_length: Option<JMELengthRestriction>,
    #[serde(rename = "musthave")]
    pub must_have: Option<JMEStringRestriction>,
    #[serde(rename = "notallowed")]
    pub may_not_have: Option<JMEStringRestriction>,
    #[serde(rename = "mustmatchpattern")]
    pub must_match_pattern: Option<JMEPatternRestriction>,
    #[serde(rename = "valuegenerators")]
    pub value_generators: Option<Vec<JMEValueGenerator>>,
}

impl ExamQuestionPartJME {
    pub fn new(
        part_data: ExamQuestionPartSharedData,
        answer: String,
        answer_simplification: Option<Vec<AnswerSimplificationType>>,
        show_preview: bool,
        checking_type: JMECheckingType,
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
    All(bool),
    Basic(bool),
    UnitFactor(bool),
    UnitPower(bool),
    UnitDenominator(bool),
    ZeroFactor(bool),
    ZeroTerm(bool),
    ZeroPower(bool),
    CollectNumbers(bool),
    ZeroBase(bool),
    ConstantsFirst(bool),
    SqrtProduct(bool),
    SqrtDivision(bool),
    SqrtSquare(bool),
    OtherNumbers(bool),
    TimesDot(bool),
    ExpandBrackets(bool),
    CollectLikeFractions(bool),
    CanonicalOrder(bool),
    NoLeadingMinus(bool),
    Fractions(bool),
    Trigonometric(bool),
    CancelTerms(bool),
    CancelFactors(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMECheckingTypeData<T> {
    #[serde(rename = "checkingAccuracy")]
    pub checking_accuracy: T,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "checkingType")]
pub enum JMECheckingType {
    #[serde(rename = "reldiff")]
    RelativeDifference(JMECheckingTypeData<f64>),
    #[serde(rename = "absdiff")]
    AbsoluteDifference(JMECheckingTypeData<f64>),
    #[serde(rename = "dp")]
    DecimalPlaces(JMECheckingTypeData<usize>),
    #[serde(rename = "sigfig")]
    SignificantFigures(JMECheckingTypeData<usize>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMERestriction {
    pub name: String,
    pub strings: Vec<String>,
    #[serde(rename = "partialCredit")]
    pub partial_credit: f64, //TODO: maybe usize?
    pub message: String,
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
    pub restriction: JMERestriction,
    pub length: Option<usize>,
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
    pub restriction: JMERestriction,
    #[serde(rename = "showStrings")]
    pub show_strings: bool,
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
    #[serde(rename = "partialCredit")]
    pub partial_credit: f64, //TODO: maybe usize?
    pub message: String,
    pub pattern: String, //TODO type?
    #[serde(rename = "nameToCompare")]
    pub name_to_compare: String,
}

impl JMEPatternRestriction {
    pub fn new(
        partial_credit: f64,
        message: String,
        pattern: String,
        name_to_compare: String,
    ) -> JMEPatternRestriction {
        JMEPatternRestriction {
            partial_credit,
            message,
            pattern,
            name_to_compare,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEValueGenerator {
    pub name: String,
    pub value: String,
}

impl JMEValueGenerator {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionVariablesTest {
    pub condition: String,
    #[serde(rename = "maxRuns")]
    pub max_runs: SaveNatural,
}

impl ExamQuestionVariablesTest {
    pub fn new(condition: String, max_runs: usize) -> ExamQuestionVariablesTest {
        ExamQuestionVariablesTest {
            condition,
            max_runs: SaveNatural(max_runs),
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
    /*#[serde(rename = "checkingType")]
    pub checking_type: Option<CheckingType>,*/ //TODO: check if being used
    /*#[serde(rename = "inputStep")]
    pub input_step: Option<usize>,*/ //TODO: check if being used
    #[serde(rename = "mustBeReduced")]
    pub fractions_must_be_reduced: Option<bool>,
    #[serde(rename = "mustBeReducedPC")]
    pub partial_credit_if_fraction_not_reduced: Option<Primitive>,
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
        #[serde(rename = "minValue")]
        min_value: Primitive,
        #[serde(rename = "maxValue")]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "Primitive")]
/// A natural number (unsigned int) that can be parsed from primitive
pub struct SaveNatural(pub usize);

impl std::convert::TryFrom<Primitive> for SaveNatural {
    type Error = String;
    fn try_from(p: Primitive) -> Result<Self, Self::Error> {
        match p {
            Primitive::Natural(n) => Ok(SaveNatural(n)),
            Primitive::Float(_n) => Err("Please use an unsigned integer.".to_string()),
            Primitive::String(n) => n.parse().map(|n| SaveNatural(n)).map_err(|e| e.to_string()),
        }
    }
}

impl std::convert::From<usize> for SaveNatural {
    fn from(u: usize) -> Self {
        SaveNatural(u)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(String),
    Value(T),
}

impl<T> VariableValued<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> VariableValued<U> {
        match self {
            VariableValued::Variable(x) => VariableValued::Variable(x),
            VariableValued::Value(x) => VariableValued::Value(f(x)),
        }
    }
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
    pub answers: VariableValued<Vec<VariableValued<String>>>,
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: ChooseOneDisplayType, // How to display the response selectors
    #[serde(rename = "displayColumns")]
    pub columns: SaveNatural, // How many columns to use to display the choices. Not usefull when dropdown -> optional? TODO
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<VariableValued<Primitive>>>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    //TODO: type (contains only strings...)
    pub distractors: Option<Vec<String>>,
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
    pub max_marks: Option<SaveNatural>, // Is there a maximum number of marks the student can get?
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<usize>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<usize>, // Maximum number of responses the student can select
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayColumns")]
    pub display_columns: SaveNatural, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: VariableValued<Vec<String>>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<MultipleChoiceMatrix>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    pub distractors: Option<VariableValued<MultipleChoiceMatrix>>,
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
    pub choices: VariableValued<Vec<String>>,
    pub answers: VariableValued<Vec<String>>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<MultipleChoiceMatrix>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
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
    Matrix(Vec<VariableValued<Vec<Primitive>>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Primitive {
    String(String),
    Natural(usize),
    Float(f64),
}

impl std::convert::From<usize> for Primitive {
    fn from(u: usize) -> Self {
        Primitive::Natural(u)
    }
}

impl std::convert::From<f64> for Primitive {
    fn from(f: f64) -> Self {
        Primitive::Float(f)
    }
}

impl std::convert::From<String> for Primitive {
    fn from(s: String) -> Self {
        Primitive::String(s)
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "{}", s),
            Primitive::Natural(n) => write!(f, "{}", n),
            Primitive::Float(fl) => write!(f, "{}", fl),
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamDiagnostic {
    pub knowledge_graph: ExamDiagnosticKnowledgeGraph,
    pub script: ExamDiagnosticScript,
    #[serde(rename = "customScript")]
    pub custom_script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamDiagnosticKnowledgeGraph {
    pub topics: Vec<ExamDiagnosticKnowledgeGraphTopic>,
    pub learning_objectives: Vec<ExamDiagnosticKnowledgeGraphLearningObjective>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamDiagnosticKnowledgeGraphTopic {
    pub name: String,
    pub description: String,
    pub learning_objectives: Vec<String>,
    pub depends_on: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamDiagnosticKnowledgeGraphLearningObjective {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExamDiagnosticScript {
    Mastery,
    Diagnosys,
    Custom,
}
