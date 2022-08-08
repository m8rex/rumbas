use crate::jme::EmbracedJMEString;
use crate::jme::JMENotesString;
use crate::jme::JMEString;
use crate::support::answer_style::AnswerStyle;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartType {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub settings: Vec<CustomPartTypeSetting>,
    pub help_url: String,
    pub public_availability: CustomPartAvailability,
    pub marking_script: JMENotesString,
    pub can_be_gap: bool,
    pub can_be_step: bool,
    pub marking_notes: Vec<CustomPartMarkingNote>,
    pub published: bool,
    pub extensions: Vec<String>,
    #[serde(flatten)]
    pub input_widget: CustomPartInputWidget,
    //TODO source
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartMarkingNote {
    pub name: String,
    pub definition: JMEString,
    pub description: String,
}

impl std::convert::From<crate::jme::ast::Note> for CustomPartMarkingNote {
    fn from(note: crate::jme::ast::Note) -> Self {
        CustomPartMarkingNote {
            name: note.name.to_string(),
            definition: note.expression_string(),
            description: note.description.unwrap_or_else(|| "".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartInputOptionValue<T: Clone> {
    pub value: T,
    /// A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.
    #[serde(rename = "static")]
    pub is_static: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartStringInputOptions {
    //TODO? hint & correctAnswer is shared for all..., macro?
    pub hint: CustomPartInputOptionValue<String>, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    #[serde(rename = "correctAnswer")]
    pub correct_answer: JMEString, // A JME expression which evaluates to the expected answer to the part.
    #[serde(rename = "allowEmpty")]
    pub allow_empty: CustomPartInputOptionValue<bool>, // If false, the part will only be marked if their answer is non-empty.
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartNumberInputOptions {
    pub hint: CustomPartInputOptionValue<String>, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    #[serde(rename = "correctAnswer")]
    pub correct_answer: JMEString, // A JME expression which evaluates to the expected answer to the part.
    #[serde(rename = "allowFractions")]
    pub allow_fractions: CustomPartInputOptionValue<bool>, //Allow the student to enter their answer as a fraction?
    #[serde(rename = "allowedNotationStyles")]
    pub allowed_notation_styles: CustomPartInputOptionValue<Vec<AnswerStyle>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartRadioButtonsInputOptions {
    pub hint: CustomPartInputOptionValue<String>, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    #[serde(rename = "correctAnswer")]
    pub correct_answer: JMEString, // A JME expression which evaluates to the expected answer to the part.
    /// The labels for the choices to offer to the student.
    pub choices: CustomPartInputOptionValue<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub enum CustomPartAvailability {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "restricted")]
    Restricted,
}

// TODO: other
// https://docs.numbas.org.uk/en/latest/custom-part-types/reference.html?highlight=Custom#setting-types
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
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
    // TODO see https://numbas-editor.readthedocs.io/en/latest/custom-part-types/reference.html?highlight=content%20area#setting-types
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingSharedData {
    /// A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting, but doesn’t need to be very descriptive - the label can do that.
    pub name: String,
    /// The label shown next to the setting in the question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.
    pub label: String,
    /// The address of documentation explaining this setting in further depth.
    pub help_url: Option<String>,
    /// Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.
    pub hint: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingString {
    #[serde(flatten)]
    pub shared_data: CustomPartTypeSettingSharedData,
    #[serde(rename = "subvars")]
    /// If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the text when the question is run. Otherwise, the string will be untouched.
    pub evaluate_enclosed_expressions: bool,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    pub default_value: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingMathematicalExpression {
    #[serde(flatten)]
    pub shared_data: CustomPartTypeSettingSharedData,
    #[serde(rename = "subvars")]
    ///  If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the string.
    pub evaluate_enclosed_expressions: bool,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    pub default_value: EmbracedJMEString,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingCode {
    #[serde(flatten)]
    pub shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    pub default_value: JMEString,
    pub evaluate: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingCheckBox {
    #[serde(flatten)]
    pub shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    pub default_value: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingDropDown {
    #[serde(flatten)]
    pub shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    pub default_value: String,
    pub choices: Vec<CustomPartTypeSettingDropDownChoice>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingDropDownChoice {
    pub value: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CustomPartTypeSettingPercentage {
    #[serde(flatten)]
    pub shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, leave this blank.
    pub default_value: String,
}
