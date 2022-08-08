use crate::question::extension::Extensions;
use crate::question::part::question_part::JMENotes;
use crate::support::file_manager::*;
use crate::support::noneable::Noneable;
use crate::support::sanitize::sanitize;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::JMETranslatableString;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::{YamlError, YamlResult};
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::{Into, TryInto};
use std::path::PathBuf;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeDefinitionInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct CustomPartTypeDefinition {
    pub type_name: TranslatableString,
    pub description: TranslatableString,
    pub settings: Vec<CustomPartTypeSetting>,
    pub can_be_gap: bool,
    pub can_be_step: bool,
    pub marking_notes: JMENotes,
    pub help_url: TranslatableString,
    pub published: bool,
    pub extensions: Extensions,
    pub input_widget: CustomPartInputWidget, //TODO source
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartType> for CustomPartTypeDefinition {
    fn to_numbas(&self, _locale: &str) -> numbas::question::custom_part_type::CustomPartType {
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
    fn to_numbas_with_name(
        &self,
        locale: &str,
        name: String,
    ) -> numbas::question::custom_part_type::CustomPartType {
        numbas::question::custom_part_type::CustomPartType {
            short_name: name,
            name: self.type_name.clone().to_string(locale).unwrap(),
            description: self.description.clone().to_string(locale).unwrap(),
            settings: self.settings.to_numbas(locale),
            help_url: self.help_url.clone().to_string(locale).unwrap(),
            public_availability: numbas::question::custom_part_type::CustomPartAvailability::Always,
            marking_script: self.marking_notes.to_numbas(locale),
            can_be_gap: self.can_be_gap,
            can_be_step: self.can_be_step,
            marking_notes: self.marking_notes.0.to_numbas(locale),
            published: self.published,
            extensions: self.extensions.to_numbas(locale),
            input_widget: self.input_widget.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartTypeDefinition> for numbas::question::custom_part_type::CustomPartType {
    fn to_rumbas(&self) -> CustomPartTypeDefinition {
        CustomPartTypeDefinition {
            type_name: self.name.to_rumbas(),
            description: self.description.to_rumbas(),
            settings: self.settings.to_rumbas(),
            help_url: self.help_url.to_rumbas(),
            // public_availability: numbas::question::custom_part_type::CustomPartAvailability::Always,
            can_be_gap: self.can_be_gap,
            can_be_step: self.can_be_step,
            marking_notes: JMENotes(self.marking_notes.clone().to_rumbas()),
            published: self.published,
            extensions: Extensions::from(&self.extensions),
            input_widget: self.input_widget.to_rumbas(),
        }
    }
}

impl CustomPartTypeDefinitionInput {
    pub fn from_str(yaml: &str, file: PathBuf) -> YamlResult<Self> {
        serde_yaml::from_str(yaml).map_err(|e| YamlError::from(e, file))
    }
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl CustomPartTypeDefinition {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        CustomPartTypeDefinitionInput::from_normal(self.to_owned()).to_yaml()
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "input_type")]
#[serde(rename_all = "snake_case")]
pub enum CustomPartTypeSetting {
    CheckBox(CustomPartTypeSettingCheckBox),
    Code(CustomPartTypeSettingCode),
    MathematicalExpression(CustomPartTypeSettingMathematicalExpression),
    String(CustomPartTypeSettingString),
    DropDown(CustomPartTypeSettingDropDown),
    Percentage(CustomPartTypeSettingPercentage),
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSetting> for CustomPartTypeSetting {
    fn to_numbas(&self, locale: &str) -> numbas::question::custom_part_type::CustomPartTypeSetting {
        match self {
            Self::CheckBox(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::CheckBox(
                    c.to_numbas(locale),
                )
            }
            Self::Code(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::Code(c.to_numbas(locale))
            }
            Self::MathematicalExpression(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::MathematicalExpression(
                    c.to_numbas(locale),
                )
            }
            Self::String(c) => numbas::question::custom_part_type::CustomPartTypeSetting::String(
                c.to_numbas(locale),
            ),
            Self::DropDown(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::DropDown(
                    c.to_numbas(locale),
                )
            }
            Self::Percentage(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::Percentage(
                    c.to_numbas(locale),
                )
            }
        }
    }
}

impl ToRumbas<CustomPartTypeSetting> for numbas::question::custom_part_type::CustomPartTypeSetting {
    fn to_rumbas(&self) -> CustomPartTypeSetting {
        match self {
            Self::CheckBox(c) => CustomPartTypeSetting::CheckBox(c.to_rumbas()),
            Self::Code(c) => CustomPartTypeSetting::Code(c.to_rumbas()),
            Self::MathematicalExpression(c) => {
                CustomPartTypeSetting::MathematicalExpression(c.to_rumbas())
            }
            Self::String(c) => CustomPartTypeSetting::String(c.to_rumbas()),
            Self::DropDown(c) => CustomPartTypeSetting::DropDown(c.to_rumbas()),
            Self::Percentage(c) => CustomPartTypeSetting::Percentage(c.to_rumbas()),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingSharedDataInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingSharedData {
    /// A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.
    name: TranslatableString,
    /// The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.
    numbas_label: TranslatableString,
    /// The address of documentation explaining this setting in further depth.
    documentation_url: Noneable<TranslatableString>,
    /// Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.
    numbas_hint: TranslatableString,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingSharedData>
    for CustomPartTypeSettingSharedData
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingSharedData {
        numbas::question::custom_part_type::CustomPartTypeSettingSharedData {
            name: self.name.to_numbas(locale),
            label: self.numbas_label.to_numbas(locale),
            hint: self.numbas_hint.to_numbas(locale),
            help_url: self.documentation_url.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingSharedData>
    for numbas::question::custom_part_type::CustomPartTypeSettingSharedData
{
    fn to_rumbas(&self) -> CustomPartTypeSettingSharedData {
        CustomPartTypeSettingSharedData {
            name: self.name.to_rumbas(),
            numbas_label: self.label.to_rumbas(),
            numbas_hint: self.hint.to_rumbas(),
            documentation_url: self.help_url.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingStringInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingString {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the text when the question is run. Otherwise, the string will be untouched.
    evaluate_enclosed_expressions: bool,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.
    default_value: Noneable<String>,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingString>
    for CustomPartTypeSettingString
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingString {
        numbas::question::custom_part_type::CustomPartTypeSettingString {
            shared_data: self.shared_data.to_numbas(locale),
            evaluate_enclosed_expressions: self.evaluate_enclosed_expressions.to_numbas(locale),
            default_value: self.default_value.to_numbas(locale).unwrap_or_default(), // TODO implement String to Noneable<String> where it is None if string is empty
        }
    }
}

impl ToRumbas<CustomPartTypeSettingString>
    for numbas::question::custom_part_type::CustomPartTypeSettingString
{
    fn to_rumbas(&self) -> CustomPartTypeSettingString {
        CustomPartTypeSettingString {
            shared_data: self.shared_data.to_rumbas(),
            evaluate_enclosed_expressions: self.evaluate_enclosed_expressions.to_rumbas(),
            default_value: if self.default_value.is_empty() {
                Noneable::None
            } else {
                Noneable::NotNone(self.default_value.to_rumbas())
            },
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingMathematicalExpressionInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingMathematicalExpression {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    ///  If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the string.
    evaluate_enclosed_expressions: bool,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.
    default_value: Noneable<EmbracedJMETranslatableString>,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingMathematicalExpression>
    for CustomPartTypeSettingMathematicalExpression
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingMathematicalExpression {
        numbas::question::custom_part_type::CustomPartTypeSettingMathematicalExpression {
            shared_data: self.shared_data.to_numbas(locale),
            evaluate_enclosed_expressions: self.evaluate_enclosed_expressions.to_numbas(locale),
            default_value: self.default_value.to_numbas(locale).unwrap_or_default(),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingMathematicalExpression>
    for numbas::question::custom_part_type::CustomPartTypeSettingMathematicalExpression
{
    fn to_rumbas(&self) -> CustomPartTypeSettingMathematicalExpression {
        CustomPartTypeSettingMathematicalExpression {
            shared_data: self.shared_data.to_rumbas(),
            evaluate_enclosed_expressions: self.evaluate_enclosed_expressions.to_rumbas(),
            default_value: if self.default_value.is_empty() {
                Noneable::None
            } else {
                Noneable::NotNone(self.default_value.to_rumbas())
            },
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingCodeInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingCode {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.
    default_value: Noneable<JMETranslatableString>,
    evaluate: bool,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingCode>
    for CustomPartTypeSettingCode
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingCode {
        numbas::question::custom_part_type::CustomPartTypeSettingCode {
            shared_data: self.shared_data.to_numbas(locale),
            evaluate: self.evaluate.to_numbas(locale),
            default_value: self.default_value.to_numbas(locale).unwrap_or_default(),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingCode>
    for numbas::question::custom_part_type::CustomPartTypeSettingCode
{
    fn to_rumbas(&self) -> CustomPartTypeSettingCode {
        CustomPartTypeSettingCode {
            shared_data: self.shared_data.to_rumbas(),
            evaluate: self.evaluate.to_rumbas(),
            default_value: if self.default_value.is_empty() {
                Noneable::None
            } else {
                Noneable::NotNone(self.default_value.to_rumbas())
            },
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingCheckBoxInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingCheckBox {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor.
    default_value: bool,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingCheckBox>
    for CustomPartTypeSettingCheckBox
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingCheckBox {
        numbas::question::custom_part_type::CustomPartTypeSettingCheckBox {
            shared_data: self.shared_data.to_numbas(locale),
            default_value: self.default_value.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingCheckBox>
    for numbas::question::custom_part_type::CustomPartTypeSettingCheckBox
{
    fn to_rumbas(&self) -> CustomPartTypeSettingCheckBox {
        CustomPartTypeSettingCheckBox {
            shared_data: self.shared_data.to_rumbas(),
            default_value: self.default_value.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingDropDownInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingDropDown {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.
    default_value: Noneable<TranslatableString>,
    choices: Vec<CustomPartTypeSettingDropDownChoice>,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingDropDown>
    for CustomPartTypeSettingDropDown
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingDropDown {
        numbas::question::custom_part_type::CustomPartTypeSettingDropDown {
            shared_data: self.shared_data.to_numbas(locale),
            default_value: self
                .default_value
                .to_numbas(locale)
                .unwrap_or_else(String::new),
            choices: self.choices.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingDropDown>
    for numbas::question::custom_part_type::CustomPartTypeSettingDropDown
{
    fn to_rumbas(&self) -> CustomPartTypeSettingDropDown {
        CustomPartTypeSettingDropDown {
            shared_data: self.shared_data.to_rumbas(),
            default_value: if self.default_value.to_string().is_empty() {
                Noneable::None
            } else {
                Noneable::NotNone(self.default_value.to_string().to_rumbas())
            },
            choices: self.choices.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingDropDownChoiceInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingDropDownChoice {
    value: TranslatableString,
    label: TranslatableString,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingDropDownChoice>
    for CustomPartTypeSettingDropDownChoice
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingDropDownChoice {
        numbas::question::custom_part_type::CustomPartTypeSettingDropDownChoice {
            value: self.value.to_numbas(locale),
            label: self.label.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingDropDownChoice>
    for numbas::question::custom_part_type::CustomPartTypeSettingDropDownChoice
{
    fn to_rumbas(&self) -> CustomPartTypeSettingDropDownChoice {
        CustomPartTypeSettingDropDownChoice {
            value: self.value.to_string().into(),
            label: self.label.to_string().into(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeSettingPercentageInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartTypeSettingPercentage {
    #[serde(flatten)]
    shared_data: CustomPartTypeSettingSharedData,
    /// The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.
    default_value: Noneable<f64>,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartTypeSettingPercentage>
    for CustomPartTypeSettingPercentage
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartTypeSettingPercentage {
        numbas::question::custom_part_type::CustomPartTypeSettingPercentage {
            shared_data: self.shared_data.to_numbas(locale),
            default_value: self
                .default_value
                .clone()
                .map(|n| n.to_string())
                .unwrap_or_else(String::new),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingPercentage>
    for numbas::question::custom_part_type::CustomPartTypeSettingPercentage
{
    fn to_rumbas(&self) -> CustomPartTypeSettingPercentage {
        CustomPartTypeSettingPercentage {
            shared_data: self.shared_data.to_rumbas(),
            default_value: if self.default_value.is_empty() {
                Noneable::None
            } else {
                Noneable::NotNone(
                    self.default_value
                        .parse()
                        .expect("Floating point percentage in custom part type"),
                )
            },
        }
    }
}

macro_rules! create_input_option_value {
    ($struct: ident, $input: literal, $type: ty, $numbas_subtype: ty) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples)]
        #[input(name = $input)]
        #[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
        pub struct $struct {
            /// The value
            pub value: $type,
            /// A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.
            #[serde(rename = "static")]
            pub is_static: bool,
        }

        impl
            ToNumbas<
                numbas::question::custom_part_type::CustomPartInputOptionValue<$numbas_subtype>,
            > for $struct
        {
            fn to_numbas(
                &self,
                locale: &str,
            ) -> numbas::question::custom_part_type::CustomPartInputOptionValue<$numbas_subtype>
            {
                numbas::question::custom_part_type::CustomPartInputOptionValue {
                    value: self.value.clone().to_numbas(locale),
                    is_static: self.is_static,
                }
            }
        }

        impl ToRumbas<$struct>
            for numbas::question::custom_part_type::CustomPartInputOptionValue<$numbas_subtype>
        {
            fn to_rumbas(&self) -> $struct {
                $struct {
                    value: self.value.clone().to_rumbas(),
                    is_static: self.is_static,
                }
            }
        }
    };
}

create_input_option_value!(
    CustomPartInputOptionValueBool,
    "CustomPartInputOptionValueBoolInput",
    bool,
    bool
);

create_input_option_value!(
    CustomPartInputOptionValueTranslatableString,
    "CustomPartInputOptionValueTranslatableStringInput",
    TranslatableString,
    String
);

create_input_option_value!(
    CustomPartInputOptionValueTranslatableStrings,
    "CustomPartInputOptionValueTranslatableStringsInput",
    Vec<TranslatableString>,
    Vec<String>
);

create_input_option_value!(
    CustomPartInputOptionValueAnswerStyles,
    "CustomPartInputOptionValueAnswerStylesInput",
    Vec<crate::question::part::number_entry::AnswerStyle>,
    Vec<numbas::support::answer_style::AnswerStyle>
);

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartInputWidgetInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(tag = "type")]
pub enum CustomPartInputWidget {
    //TODO other types: https://numbas-editor.readthedocs.io/en/latest/custom-part-types/reference.html
    #[serde(rename = "string")]
    /// The student enters a single line of text.
    String(CustomPartStringInputOptions),
    #[serde(rename = "number")]
    /// The student enters a number, using any of the allowed notation styles. If the student’s answer is not a valid number, they are shown a warning and can not submit the part.
    Number(CustomPartNumberInputOptions),
    #[serde(rename = "radiogroup")]
    /// The student chooses one from a list of choices by selecting a radio button.
    RadioGroup(CustomPartRadioGroupInputOptions),
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartInputWidget> for CustomPartInputWidget {
    fn to_numbas(&self, locale: &str) -> numbas::question::custom_part_type::CustomPartInputWidget {
        match self {
            CustomPartInputWidget::String(s) => {
                numbas::question::custom_part_type::CustomPartInputWidget::String(
                    s.to_numbas(locale),
                )
            }
            CustomPartInputWidget::Number(s) => {
                numbas::question::custom_part_type::CustomPartInputWidget::Number(
                    s.to_numbas(locale),
                )
            }
            CustomPartInputWidget::RadioGroup(s) => {
                numbas::question::custom_part_type::CustomPartInputWidget::RadioButtons(
                    s.to_numbas(locale),
                )
            }
        }
    }
}

impl ToRumbas<CustomPartInputWidget> for numbas::question::custom_part_type::CustomPartInputWidget {
    fn to_rumbas(&self) -> CustomPartInputWidget {
        match self {
            numbas::question::custom_part_type::CustomPartInputWidget::String(s) => {
                CustomPartInputWidget::String(s.to_rumbas())
            }
            numbas::question::custom_part_type::CustomPartInputWidget::Number(s) => {
                CustomPartInputWidget::Number(s.to_rumbas())
            }
            numbas::question::custom_part_type::CustomPartInputWidget::RadioButtons(s) => {
                CustomPartInputWidget::RadioGroup(s.to_rumbas())
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartStringInputOptionsInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct CustomPartStringInputOptions {
    //TODO? hint & correctAnswer is shared for all..., use macro?
    /// A string displayed next to the input field, giving any necessary information about how to enter their answer.
    pub hint: CustomPartInputOptionValueTranslatableString,
    /// A JME expression which evaluates to the expected answer to the part.
    pub correct_answer: JMETranslatableString,
    /// If false, the part will only be marked if their answer is non-empty.
    pub allow_empty: CustomPartInputOptionValueBool,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartStringInputOptions>
    for CustomPartStringInputOptions
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartStringInputOptions {
        numbas::question::custom_part_type::CustomPartStringInputOptions {
            hint: self.hint.to_numbas(locale),
            correct_answer: self
                .correct_answer
                .to_string(locale)
                .unwrap()
                .try_into()
                .unwrap(),
            allow_empty: self.allow_empty.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartStringInputOptions>
    for numbas::question::custom_part_type::CustomPartStringInputOptions
{
    fn to_rumbas(&self) -> CustomPartStringInputOptions {
        CustomPartStringInputOptions {
            hint: self.hint.to_rumbas(),
            correct_answer: self.correct_answer.to_rumbas(),
            allow_empty: self.allow_empty.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartNumberInputOptionsInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct CustomPartNumberInputOptions {
    /// A string displayed next to the input field, giving any necessary information about how to enter their answer.
    pub hint: CustomPartInputOptionValueTranslatableString,
    /// A JME expression which evaluates to the expected answer to the part.
    pub correct_answer: JMETranslatableString,
    ///Allow the student to enter their answer as a fraction?
    pub allow_fractions: CustomPartInputOptionValueBool,
    pub allowed_notation_styles: CustomPartInputOptionValueAnswerStyles,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartNumberInputOptions>
    for CustomPartNumberInputOptions
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartNumberInputOptions {
        numbas::question::custom_part_type::CustomPartNumberInputOptions {
            hint: self.hint.to_numbas(locale),
            correct_answer: self
                .correct_answer
                .to_string(locale)
                .unwrap()
                .try_into()
                .unwrap(),
            allow_fractions: self.allow_fractions.to_numbas(locale),
            allowed_notation_styles: self.allowed_notation_styles.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartNumberInputOptions>
    for numbas::question::custom_part_type::CustomPartNumberInputOptions
{
    fn to_rumbas(&self) -> CustomPartNumberInputOptions {
        CustomPartNumberInputOptions {
            hint: self.hint.to_rumbas(),
            correct_answer: self.correct_answer.to_rumbas(),
            allow_fractions: self.allow_fractions.to_rumbas(),
            allowed_notation_styles: self.allowed_notation_styles.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartRadioGroupInputOptionsInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct CustomPartRadioGroupInputOptions {
    /// A string displayed next to the input field, giving any necessary information about how to enter their answer.
    pub hint: CustomPartInputOptionValueTranslatableString,
    /// A JME expression which evaluates to the expected answer to the part.
    pub correct_answer: JMETranslatableString,
    /// The labels for the choices to offer to the student.
    pub choices: CustomPartInputOptionValueTranslatableStrings,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartRadioButtonsInputOptions>
    for CustomPartRadioGroupInputOptions
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartRadioButtonsInputOptions {
        numbas::question::custom_part_type::CustomPartRadioButtonsInputOptions {
            hint: self.hint.to_numbas(locale),
            correct_answer: self
                .correct_answer
                .to_string(locale)
                .unwrap()
                .try_into()
                .unwrap(),
            choices: self.choices.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomPartRadioGroupInputOptions>
    for numbas::question::custom_part_type::CustomPartRadioButtonsInputOptions
{
    fn to_rumbas(&self) -> CustomPartRadioGroupInputOptions {
        CustomPartRadioGroupInputOptions {
            hint: self.hint.to_rumbas(),
            correct_answer: self.correct_answer.to_rumbas(),
            choices: self.choices.to_rumbas(),
        }
    }
}

crate::support::file_manager::create_from_string_type!(
    CustomPartTypeDefinitionPath,
    CustomPartTypeDefinitionPathInput,
    CustomPartTypeDefinition,
    CustomPartTypeDefinitionInput,
    CustomPartTypeFileToRead,
    numbas::question::custom_part_type::CustomPartType,
    "CustomPartTypeDefinitionPath",
    |_, _| (),
    short_name
);
