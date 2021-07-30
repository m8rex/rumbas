use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::convert::TryInto;
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
            T::from_str(&s)
                .map_err(serde::de::Error::custom)
                .map(Option::from)
        }
        Ok(v) => Err(serde::de::Error::custom(format!(
            "string or number expected but found something else: {}",
            v
        ))),
        Err(_) => Ok(None),
    }
}

impl std::convert::TryFrom<&str> for AnswerSimplificationType {
    type Error = &'static str;
    fn try_from(whole_item: &str) -> Result<Self, Self::Error> {
        let (item, value) = if whole_item.starts_with('!') {
            let mut chars = whole_item.chars();
            chars.next();
            (chars.as_str(), false)
        } else {
            (whole_item, true)
        };
        match item.to_lowercase().as_ref() {
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
            "collectlikefractions" => Ok(AnswerSimplificationType::CollectLikeFractions(value)),
            "canonicalorder" => Ok(AnswerSimplificationType::CanonicalOrder(value)),
            "cancelfactors" => Ok(AnswerSimplificationType::CancelFactors(value)),
            "cancelterms" => Ok(AnswerSimplificationType::CancelTerms(value)),
            "simplifyfractions" => Ok(AnswerSimplificationType::Fractions(value)),
            _ => {
                /*       Err(serde::de::Error::custom(format!(
                    "unknown answer simplification type {}",
                    item
                )))*/
                Ok(AnswerSimplificationType::Unknown((item.to_string(), value)))
            }
        }
    }
}

impl std::fmt::Display for AnswerSimplificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnswerSimplificationType::All(true) => "all".to_string(),
                AnswerSimplificationType::All(false) => "!all".to_string(),
                AnswerSimplificationType::Basic(true) => "basic".to_string(),
                AnswerSimplificationType::Basic(false) => "!basic".to_string(),
                AnswerSimplificationType::UnitFactor(true) => "unitFactor".to_string(),
                AnswerSimplificationType::UnitFactor(false) => "!unitFactor".to_string(),
                AnswerSimplificationType::UnitPower(true) => "unitPower".to_string(),
                AnswerSimplificationType::UnitPower(false) => "!unitPower".to_string(),
                AnswerSimplificationType::UnitDenominator(true) => "unitDenominator".to_string(),
                AnswerSimplificationType::UnitDenominator(false) => "!unitDenominator".to_string(),
                AnswerSimplificationType::ZeroFactor(true) => "zeroFactor".to_string(),
                AnswerSimplificationType::ZeroFactor(false) => "!zeroFactor".to_string(),
                AnswerSimplificationType::ZeroTerm(true) => "zeroTerm".to_string(),
                AnswerSimplificationType::ZeroTerm(false) => "!zeroTerm".to_string(),
                AnswerSimplificationType::ZeroPower(true) => "zeroPower".to_string(),
                AnswerSimplificationType::ZeroPower(false) => "!zeroPower".to_string(),
                AnswerSimplificationType::CollectNumbers(true) => "collectNumbers".to_string(),
                AnswerSimplificationType::CollectNumbers(false) => "!collectNumbers".to_string(),
                AnswerSimplificationType::ZeroBase(true) => "zeroBase".to_string(),
                AnswerSimplificationType::ZeroBase(false) => "!zeroBase".to_string(),
                AnswerSimplificationType::ConstantsFirst(true) => "constantsFirst".to_string(),
                AnswerSimplificationType::ConstantsFirst(false) => "!constantsFirst".to_string(),
                AnswerSimplificationType::SqrtProduct(true) => "sqrtProduct".to_string(),
                AnswerSimplificationType::SqrtProduct(false) => "!sqrtProduct".to_string(),
                AnswerSimplificationType::SqrtDivision(true) => "sqrtDivision".to_string(),
                AnswerSimplificationType::SqrtDivision(false) => "!sqrtDivision".to_string(),
                AnswerSimplificationType::SqrtSquare(true) => "sqrtSquare".to_string(),
                AnswerSimplificationType::SqrtSquare(false) => "!sqrtSquare".to_string(),
                AnswerSimplificationType::OtherNumbers(true) => "otherNumbers".to_string(),
                AnswerSimplificationType::OtherNumbers(false) => "!otherNumbers".to_string(),
                AnswerSimplificationType::TimesDot(true) => "timesDot".to_string(),
                AnswerSimplificationType::TimesDot(false) => "!timesDot".to_string(),
                AnswerSimplificationType::ExpandBrackets(true) => "expandBrackets".to_string(),
                AnswerSimplificationType::ExpandBrackets(false) => "!expandBrackets".to_string(),
                AnswerSimplificationType::NoLeadingMinus(true) => "noLeadingMinus".to_string(),
                AnswerSimplificationType::NoLeadingMinus(false) => "!noLeadingMinus".to_string(),
                AnswerSimplificationType::Trigonometric(true) => "trig".to_string(),
                AnswerSimplificationType::Trigonometric(false) => "!trig".to_string(),
                AnswerSimplificationType::CollectLikeFractions(true) => {
                    "collectLikeFractions".to_string()
                }
                AnswerSimplificationType::CollectLikeFractions(false) => {
                    "!collectLikeFractions".to_string()
                }
                AnswerSimplificationType::CanonicalOrder(true) => "canonicalOrder".to_string(),
                AnswerSimplificationType::CanonicalOrder(false) => "!canonicalOrder".to_string(),
                AnswerSimplificationType::CancelFactors(true) => "cancelFactors".to_string(),
                AnswerSimplificationType::CancelFactors(false) => "!cancelFactors".to_string(),
                AnswerSimplificationType::CancelTerms(true) => "cancelTerms".to_string(),
                AnswerSimplificationType::CancelTerms(false) => "!cancelTerms".to_string(),
                AnswerSimplificationType::Fractions(true) => "simplifyFractions".to_string(),
                AnswerSimplificationType::Fractions(false) => "!simplifyFractions".to_string(),
                AnswerSimplificationType::Unknown((n, true)) => n.to_string(),
                AnswerSimplificationType::Unknown((n, false)) => format!("!{}", n),
            }
        )
    }
}

impl std::convert::From<AnswerSimplificationType> for String {
    fn from(a: AnswerSimplificationType) -> String {
        format!("{}", a)
    }
}

fn answer_simplification_deserialize_string<'de, D>(
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
                let new_item = whole_item.try_into();
                match new_item {
                    Ok(a) => r.push(a),
                    Err(m) => {
                        return Err(serde::de::Error::custom(format!(
                            "unknown answer simplification type {}",
                            m
                        )))
                    }
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

fn answer_simplification_serialize_string<S>(
    values_o: &Option<Vec<AnswerSimplificationType>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(values) = values_o {
        let mut parts: Vec<String> = Vec::new();
        for value in values {
            let new_item = value.to_string();
            parts.push(new_item);
        }
        s.serialize_str(&parts.join(",")[..])
    } else {
        s.serialize_str("")
    }
}

/*
fn answer_simplification_deserialize_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<AnswerSimplificationType>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(serde_json::Value::Array(v)) => {
            let mut r = Vec::new();
            for value in v.iter() {
                match value {
                    serde_json::Value::String(whole_item) => {
                        let new_item = AnswerSimplificationType::from_str(whole_item);
                        match new_item {
                            Ok(a) => r.push(a),
                            Err(m) => {
                                return Err(serde::de::Error::custom(format!(
                                    "unknown answer simplification type {}",
                                    m
                                )))
                            }
                        }
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "string expected but found something else: {}",
                            value
                        )))
                    }
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

impl Serialize for Vec<AnswerSimplificationType> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for value in self.iter() {
            seq.serialize_element(&value.to_string())?;
        }
        seq.end()
    }
}
*/

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Exam {
    #[serde(flatten)]
    pub basic_settings: BasicExamSettings,
    pub resources: Vec<Resource>,
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
    pub fn from_exam_str(s: &str) -> serde_json::Result<Exam> {
        let json = if s.starts_with("// Numbas version: exam_results_page_options") {
            s.splitn(2, '\n').collect::<Vec<_>>()[1]
        } else {
            s
        };
        serde_json::from_str(json)
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartType {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub settings: Vec<CustomPartTypeSetting>,
    pub help_url: String,
    pub public_availability: CustomPartAvailability,
    pub marking_script: String,
    pub can_be_gap: bool,
    pub can_be_step: bool,
    pub marking_notes: Vec<CustomPartMarkingNotes>,
    pub published: bool,
    pub extensions: Vec<String>,
    #[serde(flatten)]
    pub input_widget: CustomPartInputWidget,
    //TODO source
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartMarkingNotes {
    pub name: String,
    pub definition: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "input_widget", content = "input_options")]
pub enum CustomPartInputWidget {
    //TODO other types: https://numbas-editor.readthedocs.io/en/latest/custom-part-types/reference.html
    #[serde(rename = "string")]
    /// The student enters a single line of text.
    String(CustomPartStringInputOptions),
    #[serde(rename = "number")]
    /// The student enters a number, using any of the allowed notation styles. If the student’s answer is not a valid number, they are shown a warning and can not submit the part.
    Number(CustomPartNumberInputOptions),
    #[serde(rename = "radios")]
    /// The student chooses one from a list of choices by selecting a radio button.
    RadioButtons(CustomPartRadioButtonsInputOptions),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartInputOptionValue<T: Clone> {
    pub value: T,
    /// A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.
    #[serde(rename = "static")]
    pub is_static: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartStringInputOptions {
    //TODO? hint & correctAnswer is shared for all...
    pub hint: CustomPartInputOptionValue<String>, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    #[serde(rename = "correctAnswer")]
    pub correct_answer: String, // A JME expression which evaluates to the expected answer to the part.
    #[serde(rename = "allowEmpty")]
    pub allow_empty: CustomPartInputOptionValue<bool>, // If false, the part will only be marked if their answer is non-empty.
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartNumberInputOptions {
    //TODO? hint & correctAnswer is shared for all...
    pub hint: CustomPartInputOptionValue<String>, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    #[serde(rename = "correctAnswer")]
    pub correct_answer: String, // A JME expression which evaluates to the expected answer to the part.
    #[serde(rename = "allowFractions")]
    pub allow_fractions: CustomPartInputOptionValue<bool>, //Allow the student to enter their answer as a fraction?
    #[serde(rename = "allowedNotationStyles")]
    pub allowed_notation_styles: CustomPartInputOptionValue<Vec<AnswerStyle>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartRadioButtonsInputOptions {
    //TODO? hint & correctAnswer is shared for all...
    pub hint: CustomPartInputOptionValue<String>, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    #[serde(rename = "correctAnswer")]
    pub correct_answer: String, // A JME expression which evaluates to the expected answer to the part.
    /// The labels for the choices to offer to the student.
    pub choices: CustomPartInputOptionValue<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CustomPartAvailability {
    #[serde(rename = "always")]
    Always,
}

// TODO: other
// https://docs.numbas.org.uk/en/latest/custom-part-types/reference.html?highlight=Custom#setting-types

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "input_type")]
pub enum CustomPartTypeSetting {
    #[serde(rename = "checkbox")]
    CheckBox(CustomPartTypeSettingCheckBox),
    #[serde(rename = "code")]
    Code(CustomPartTypeSettingCode),
    #[serde(rename = "mathematical_expression")]
    MathematicalExpression(CustomPartTypeSettingMathematicalExpression),
    #[serde(rename = "string")]
    String(CustomPartTypeSettingString),
    #[serde(rename = "dropdown")]
    DropDown(CustomPartTypeSettingDropDown),
    #[serde(rename = "percent")]
    Percentage(CustomPartTypeSettingPercentage),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingSharedData {
    /// A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting, but doesn’t need to be very descriptive - the label can do that.
    name: String,
    /// The label shown next to the setting in the question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.
    label: String,
    /// The address of documentation explaining this setting in further depth.
    help_url: Option<String>,
    /// Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.
    hint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingString {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    #[serde(rename = "subvars")]
    /// If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the text when the question is run. Otherwise, the string will be untouched.
    evaluate_enclosed_expressions: bool,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    default_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingMathematicalExpression {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    #[serde(rename = "subvars")]
    ///  If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the string.
    evaluate_enclosed_expressions: bool,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    default_value: Primitive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingCode {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    default_value: Primitive,
    evaluate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingCheckBox {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    default_value: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingDropDown {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    default_value: Primitive,
    choices: Vec<CustomPartTypeSettingDropDownChoice>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingDropDownChoice {
    value: Primitive,
    label: Primitive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingPercentage {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    default_value: Primitive,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamNavigation {
    #[serde(rename = "startpassword")]
    pub start_password: Option<String>, //TODO: if empty string -> also None
    #[serde(rename = "allowregen")]
    pub allow_regenerate: bool,
    #[serde(flatten)]
    pub navigation_mode: ExamNavigationMode,
    #[serde(rename = "allowsteps")]
    pub allow_steps: Option<bool>,
    #[serde(rename = "showfrontpage")]
    pub show_frontpage: bool,
    #[serde(rename = "preventleave")]
    pub confirm_when_leaving: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "navigatemode")]
pub enum ExamNavigationMode {
    #[serde(rename = "sequence")]
    Sequential(ExamNavigationModeSequential),
    Menu,
    Diagnostic(ExamNavigationModeDiagnostic),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamNavigationModeSequential {
    #[serde(rename = "onleave")]
    pub on_leave: ExamLeaveAction,
    #[serde(rename = "showresultspage")]
    pub show_results_page: ExamShowResultsPage,
    #[serde(rename = "reverse")]
    pub can_move_to_previous: bool,
    #[serde(rename = "browse")]
    pub browsing_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamNavigationModeDiagnostic {
    #[serde(rename = "onleave")]
    pub on_leave: ExamLeaveAction,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuestionNavigation {
    #[serde(rename = "allowregen")]
    pub allow_regenerate: bool,
    #[serde(rename = "showfrontpage")]
    pub show_frontpage: bool,
    #[serde(rename = "preventleave")]
    pub confirm_when_leaving: Option<bool>,
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
    #[serde(default)]
    pub intro: String,
    #[serde(rename = "feedbackmessages")]
    #[serde(default)]
    pub feedback_messages: Vec<ExamFeedbackMessage>,
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamFeedbackMessage {
    pub message: String,
    pub threshold: String, //TODO type
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
    #[serde(rename = "ggbapplet")]
    ExtensionGeogebraApplet,
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
    /// If this is ticked, then when an exam uses this question the author can override the value
    /// of this variable with their own choice.
    #[serde(default)]
    pub can_override: bool,
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
    pub variable_groups: Vec<ExamQuestionVariableGroup>,
    pub rulesets: HashMap<String, Vec<AnswerSimplificationType>>,
    pub preamble: Preamble,
    //contributors TODO
    pub navigation: QuestionNavigation,
    //custom part types TODO
    pub extensions: Vec<String>, // todo: enum
    //metadata TODO
    pub resources: Vec<Resource>,
    //TODO type: question?
    /// Tags starting with 'skill: ' are used in diagnostic mode to specify a topic
    pub tags: Vec<String>,
    pub custom_part_types: Vec<CustomPartType>,
}

#[derive(Debug, Deserialize)]
struct ExamQuestionInput<'a> {
    #[serde(borrow)]
    question_groups: [ExamQuestionInputQuestionGroups<'a>; 1],
}
#[derive(Debug, Deserialize)]
struct ExamQuestionInputQuestionGroups<'a> {
    #[serde(borrow)]
    questions: [HashMap<&'a str, serde_json::Value>; 1],
}
impl ExamQuestion {
    pub fn from_question_exam_str(s: &str) -> serde_json::Result<ExamQuestion> {
        let json = if s.starts_with("// Numbas version: exam_results_page_options") {
            s.splitn(2, '\n').collect::<Vec<_>>()[1]
        } else {
            s
        };
        let exam: HashMap<String, serde_json::Value> = serde_json::from_str(json)?;
        let question_input: ExamQuestionInput = serde_json::from_str(json)?;
        let mut question = question_input.question_groups[0].questions[0].clone();
        for key in ["resources", "extensions", "custom_part_types", "navigation"] {
            if let Some(value) = exam.get(key) {
                question.insert(key, value.to_owned());
            }
        }
        let new_json = serde_json::to_string_pretty(&question).unwrap();
        serde_json::from_str(&new_json)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preamble {
    pub js: String,
    pub css: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionConstant {
    pub name: String,
    pub value: String,
    pub tex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ExamQuestionPart {
    Builtin(ExamQuestionPartBuiltin),
    Custom(ExamQuestionPartCustom),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ExamQuestionPartBuiltin {
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartCustom {
    pub r#type: String,
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    pub settings: std::collections::HashMap<String, CustomPartInputTypeValue>,
}

// TODO: other types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CustomPartInputTypeValue {
    CheckBox(bool),
    Code(Primitive),
}

impl std::convert::From<CustomPartInputTypeValue> for String {
    fn from(v: CustomPartInputTypeValue) -> Self {
        match v {
            CustomPartInputTypeValue::CheckBox(v) => v.to_string(),
            CustomPartInputTypeValue::Code(v) => v.to_string(),
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
        deserialize_with = "answer_simplification_deserialize_string",
        serialize_with = "answer_simplification_serialize_string"
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
    pub vset_range: [SafeFloat; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    #[serde(rename = "vsetRangePoints")]
    pub vset_range_points: SafeNatural,
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
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
    Unknown((String, bool)),
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
    RelativeDifference(JMECheckingTypeData<SafeFloat>),
    #[serde(rename = "absdiff")]
    AbsoluteDifference(JMECheckingTypeData<SafeFloat>),
    #[serde(rename = "dp")]
    DecimalPlaces(JMECheckingTypeData<usize>),
    #[serde(rename = "sigfig")]
    SignificantFigures(JMECheckingTypeData<usize>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMERestriction {
    //pub name: String,
    #[serde(rename = "partialCredit")]
    pub partial_credit: SafeFloat, //TODO: maybe SafeNatural?
    pub message: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMELengthRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    pub length: Option<SafeNatural>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEStringRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    #[serde(rename = "showStrings")]
    pub show_strings: bool,
    pub strings: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEPatternRestriction {
    #[serde(rename = "partialCredit")]
    pub partial_credit: SafeFloat, //TODO: maybe SafeNatural?
    pub message: String,
    pub pattern: String, //TODO type?
    #[serde(rename = "nameToCompare")]
    pub name_to_compare: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JMEValueGenerator {
    pub name: String,
    pub value: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionVariablesTest {
    pub condition: String,
    #[serde(rename = "maxRuns")]
    pub max_runs: SafeNatural,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionVariableGroup {
    pub variables: Vec<String>,
    pub name: String,
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

// See https://github.com/numbas/Numbas/blob/26e5c25be75f5bb1a7d6b625bc8ed0c6a59224e5/runtime/scripts/util.js#L1259
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AnswerStyle {
    /// English style - commas separate thousands, dot for decimal point
    #[serde(rename = "en")]
    English,
    /// Plain English style - no thousands separator, dot for decimal point
    #[serde(rename = "plain")]
    EnglishPlain,
    /// English SI style - spaces separate thousands, dot for decimal point
    #[serde(rename = "si-en")]
    EnglishSI,
    /// Continental European style - dots separate thousands, comma for decimal poin
    #[serde(rename = "eu")]
    European,
    /// Plain French style - no thousands separator, comma for decimal point
    #[serde(rename = "plain-eu")]
    EuropeanPlain,
    /// French SI style - spaces separate thousands, comma for decimal point
    #[serde(rename = "si-fr")]
    FrenchSI,
    /// Indian style - commas separate groups, dot for decimal point. The rightmost group is three digits, other groups are two digits.
    #[serde(rename = "in")]
    Indian,
    /// Significand-exponent ("scientific") style
    #[serde(rename = "scientific")]
    Scientific,
    /// Swiss style - apostrophes separate thousands, dot for decimal point
    #[serde(rename = "ch")]
    Swiss,
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
    precision: SafeNatural,
    #[serde(rename = "precisionPartialCredit")]
    precision_partial_credit: SafeNatural,
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
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "correctAnswer")]
    pub correct_answer: Primitive,
    #[serde(rename = "correctAnswerFractions")]
    pub correct_answer_fractions: bool,
    #[serde(rename = "numRows")]
    pub num_rows: VariableValued<SafeNatural>,
    #[serde(rename = "numColumns")]
    pub num_columns: VariableValued<SafeNatural>,
    #[serde(rename = "allowResize")]
    pub allow_resize: bool,
    #[serde(rename = "minColumns")]
    pub min_columns: VariableValued<usize>,
    #[serde(rename = "maxColumns")]
    pub max_columns: VariableValued<usize>,
    #[serde(rename = "minRows")]
    pub min_rows: VariableValued<usize>,
    #[serde(rename = "maxRows")]
    pub max_rows: VariableValued<usize>,
    #[serde(rename = "tolerance")]
    pub tolerance: f64,
    #[serde(rename = "markPerCell")]
    pub mark_per_cell: bool,
    #[serde(rename = "allowFractions")]
    pub allow_fractions: bool,
    //#[serde(flatten)]  // todo
    //precision: QuestionPrecision,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartPatternMatch {
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    #[serde(rename = "partialCredit")]
    pub partial_credit: Option<SafeFloat>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(try_from = "Primitive")]
/// A natural number (unsigned int) that can be parsed from primitive
pub struct SafeNatural(pub usize);

impl std::convert::TryFrom<Primitive> for SafeNatural {
    type Error = String;
    fn try_from(p: Primitive) -> Result<Self, Self::Error> {
        match p {
            Primitive::Natural(n) => Ok(SafeNatural(n)),
            Primitive::Float(_n) => Err("Please use an unsigned integer.".to_string()),
            Primitive::String(n) => n.parse().map(SafeNatural).map_err(|e| e.to_string()),
        }
    }
}

impl std::convert::From<usize> for SafeNatural {
    fn from(u: usize) -> Self {
        SafeNatural(u)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(try_from = "Primitive")]
/// A decimal number (float) that can be parsed from primitive
pub struct SafeFloat(pub f64);

impl std::convert::TryFrom<Primitive> for SafeFloat {
    type Error = String;
    fn try_from(p: Primitive) -> Result<Self, Self::Error> {
        match p {
            Primitive::Natural(n) => Ok(SafeFloat(n as f64)),
            Primitive::Float(n) => Ok(SafeFloat(n)),
            Primitive::String(n) => n.parse().map(SafeFloat).map_err(|e| e.to_string()),
        }
    }
}

impl std::convert::From<f64> for SafeFloat {
    fn from(v: f64) -> Self {
        SafeFloat(v)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "BooledPrimitive")]
/// A boolean that can be parsed from (booled) primitive
pub struct SafeBool(pub bool);

impl std::convert::TryFrom<BooledPrimitive> for SafeBool {
    type Error = String;
    fn try_from(p: BooledPrimitive) -> Result<Self, Self::Error> {
        match p {
            BooledPrimitive::Natural(_n) => Err("Please use a boolean value.".to_string()),
            BooledPrimitive::Float(_n) => Err("Please use a boolean value.".to_string()),
            BooledPrimitive::String(n) => n.parse().map(SafeBool).map_err(|e| e.to_string()),
            BooledPrimitive::Bool(b) => Ok(SafeBool(b)),
        }
    }
}

impl std::convert::From<bool> for SafeBool {
    fn from(b: bool) -> Self {
        SafeBool(b)
    }
}

impl std::fmt::Display for SafeBool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    pub answers: VariableValued<Vec<String>>,
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: ChooseOneDisplayType, // How to display the response selectors
    #[serde(rename = "displayColumns")]
    pub columns: SafeNatural, // How many columns to use to display the choices. Not usefull when dropdown -> optional? TODO
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: Option<MultipleChoiceWarningType>, // What to do if the student picks the wrong number of responses? TODO: not used for this type?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: Option<bool>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<Primitive>>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
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
    pub max_marks: Option<SafeNatural>, // Is there a maximum number of marks the student can get?
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<SafeNatural>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<SafeNatural>, // Maximum number of responses the student can select
    #[serde(rename = "shuffleChoices")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayColumns")]
    pub display_columns: SafeNatural, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: MultipleChoiceWarningType, // What to do if the student picks the wrong number of responses?
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: VariableValued<Vec<String>>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<Primitive>>>, // Marks for each answer/choice pair. Arranged as `matrix[answer][choice]
    pub distractors: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExamQuestionPartMatchAnswersWithChoices {
    //TODO -> Split for different types
    #[serde(flatten)]
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "minMarks")]
    pub min_marks: Option<SafeNatural>, //TODO; what is difference with minimum_marks? -> not for 1_n_2
    #[serde(rename = "maxMarks")]
    pub max_marks: Option<SafeNatural>, // Is there a maximum number of marks the student can get? -> not for 1_n_2
    #[serde(rename = "minAnswers")]
    pub min_answers: Option<SafeNatural>, // Minimum number of responses the student must select
    #[serde(rename = "maxAnswers")]
    pub max_answers: Option<SafeNatural>, // Maximum number of responses the student can select -> always one for 1_n_2
    #[serde(rename = "shuffleChoices")]
    pub shuffle_choices: bool,
    #[serde(rename = "shuffleAnswers")]
    pub shuffle_answers: bool,
    #[serde(rename = "displayType")]
    pub display_type: MatchAnswersWithChoicesDisplayType, // How to display the response selectors -> only for 1_n_2?
    //#[serde(rename = "displayColumns")] //TODO?
    //pub displayed_columns: usize, // How many columns to use to display the choices.
    #[serde(rename = "warningType")]
    pub wrong_nb_choices_warning: MultipleChoiceWarningType, // What to do if the student picks the wrong number of responses?
    pub layout: MatchAnswersWithChoicesLayout,
    #[serde(rename = "showCellAnswerState")]
    pub show_cell_answer_state: bool,
    pub choices: VariableValued<Vec<String>>,
    pub answers: VariableValued<Vec<String>>,
    #[serde(rename = "matrix")]
    pub marking_matrix: Option<VariableValued<Vec<Vec<Primitive>>>>, // Marks for each answer/choice pair. Arranged as `matrix[choice][answer]
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesDisplayType {
    #[serde(rename = "checkbox")]
    Check,
    #[serde(rename = "radiogroup")]
    Radio,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum MultipleChoiceWarningType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "prevent")]
    Prevent,
    //TODO: also prevent and warn -> same as leave actions?
    //https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L493
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MatchAnswersWithChoicesLayoutType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "lowertriangle")]
    LowerTriangle,
    //TODO: https://github.com/numbas/Numbas/blob/master/runtime/scripts/parts/multipleresponse.js#L766
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MatchAnswersWithChoicesLayout {
    r#type: MatchAnswersWithChoicesLayoutType,
    expression: String,
}

/* TODO: remove */
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BooledPrimitive {
    String(String),
    Natural(usize),
    Float(f64),
    Bool(bool),
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
    pub part_data: ExamQuestionPartSharedData,
    #[serde(rename = "sortAnswers")]
    pub sort_answers: Option<bool>,
    pub gaps: Vec<ExamQuestionPart>,
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
    pub part_data: ExamQuestionPartSharedData,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resource(pub [String; 2]);
