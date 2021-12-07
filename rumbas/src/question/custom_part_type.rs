use crate::question::extension::Extensions;
use crate::question::part::question_part::JMENotes;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::JMETranslatableString;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::{YamlError, YamlResult};
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomPartTypeDefinitionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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
            settings: self.settings.to_numbas(&locale),
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

impl CustomPartTypeDefinitionInput {
    pub fn from_name(name: &str) -> YamlResult<Self> {
        let file =
            std::path::Path::new(crate::CUSTOM_PART_TYPES_FOLDER).join(format!("{}.yaml", name));
        let yaml = std::fs::read_to_string(&file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_yaml::from_str(&yaml).map_err(|e| YamlError::from(e, file.to_path_buf()))
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
                    c.to_numbas(&locale),
                )
            }
            Self::Code(c) => numbas::question::custom_part_type::CustomPartTypeSetting::Code(
                c.to_numbas(&locale),
            ),
            Self::MathematicalExpression(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::MathematicalExpression(
                    c.to_numbas(&locale),
                )
            }
            Self::String(c) => numbas::question::custom_part_type::CustomPartTypeSetting::String(
                c.to_numbas(&locale),
            ),
            Self::DropDown(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::DropDown(
                    c.to_numbas(&locale),
                )
            }
            Self::Percentage(c) => {
                numbas::question::custom_part_type::CustomPartTypeSetting::Percentage(
                    c.to_numbas(&locale),
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
            default_value: self
                .default_value
                .to_numbas(locale)
                .unwrap_or_else(String::new), // TODO implement String to Noneable<String> where it is None if string is empty
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
                .unwrap_or_else(String::new)
                .into(),
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
            value: self.value.to_numbas(locale).into(),
            label: self.label.to_numbas(locale).into(),
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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
                .map(|n| n.into())
                .unwrap_or_else(|| String::new().into()),
        }
    }
}

impl ToRumbas<CustomPartTypeSettingPercentage>
    for numbas::question::custom_part_type::CustomPartTypeSettingPercentage
{
    fn to_rumbas(&self) -> CustomPartTypeSettingPercentage {
        CustomPartTypeSettingPercentage {
            shared_data: self.shared_data.to_rumbas(),
            default_value: match self.default_value {
                numbas::support::primitive::Primitive::Float(f) => Noneable::NotNone(f),
                numbas::support::primitive::Primitive::Natural(f) => Noneable::NotNone(f as f64),
                numbas::support::primitive::Primitive::String(s) => Noneable::None,
            },
        }
    }
}

macro_rules! create_input_option_value {
    ($struct: ident, $input: literal, $type: ty, $numbas_subtype: ty) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples)]
        #[input(name = $input)]
        #[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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

#[derive(Input, Overwrite, RumbasCheck, JsonSchema, Examples)]
#[input(name = "CustomPartTypeDefinitionPathInput")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct CustomPartTypeDefinitionPath {
    pub custom_part_type_name: String,
    pub custom_part_type_data: CustomPartTypeDefinition,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartType> for CustomPartTypeDefinitionPath {
    fn to_numbas(&self, locale: &str) -> numbas::question::custom_part_type::CustomPartType {
        self.custom_part_type_data
            .clone()
            .to_numbas_with_name(locale, self.custom_part_type_name.clone())
    }
}

impl ToRumbas<CustomPartTypeDefinitionPath> for numbas::question::custom_part_type::CustomPartType {
    fn to_rumbas(&self) -> CustomPartTypeDefinitionPath {
        CustomPartTypeDefinitionPath {
            custom_part_type_data: CustomPartTypeDefinition {
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
            },
            custom_part_type_name: self.short_name.clone(),
        }
    }
}

impl JsonSchema for CustomPartTypeDefinitionPathInput {
    fn schema_name() -> String {
        "CustomPartTypeDefinitionPath".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

impl std::convert::TryFrom<String> for CustomPartTypeDefinitionPathInput {
    type Error = YamlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let custom_part_type_data = CustomPartTypeDefinitionInput::from_name(&s).map_err(|e| e)?;
        Ok(CustomPartTypeDefinitionPathInput {
            custom_part_type_name: Value::Normal(s),
            custom_part_type_data: Value::Normal(custom_part_type_data),
        })
    }
}

impl std::convert::From<CustomPartTypeDefinitionPathInput> for String {
    fn from(q: CustomPartTypeDefinitionPathInput) -> Self {
        q.custom_part_type_name.unwrap()
    }
}

pub enum CustomPartTypeDefinitionError {
    Yaml(YamlError),
    Empty(&'static str),
}

impl std::fmt::Display for CustomPartTypeDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml(y) => write!(f, "{}", y),
            Self::Empty(e) => write!(f, "{}", e),
        }
    }
}

// Remove these impl's etc, should not ser / deser
impl std::convert::TryFrom<String> for CustomPartTypeDefinitionPath {
    type Error = CustomPartTypeDefinitionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let data: CustomPartTypeDefinitionPathInput = s.try_into().map_err(Self::Error::Yaml)?;
        /*let data: CustomPartTypeDefinitionPathInput =
        data.try_into().map_err(Self::Error::Empty)?; */
        Ok(data.to_normal())
    }
}

// Remove these impl's etc, should not ser / deser
impl std::convert::From<CustomPartTypeDefinitionPath> for String {
    fn from(q: CustomPartTypeDefinitionPath) -> Self {
        q.custom_part_type_name
    }
}

impl Hash for CustomPartTypeDefinitionPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.custom_part_type_name.hash(state);
    }
}
impl PartialEq for CustomPartTypeDefinitionPath {
    fn eq(&self, other: &Self) -> bool {
        self.custom_part_type_name == other.custom_part_type_name
    }
}
impl Eq for CustomPartTypeDefinitionPath {}
