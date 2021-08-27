use crate::question::extension::Extensions;
use crate::question::extension::ExtensionsInput;
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::Value;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::JMETranslatableString;
use crate::support::translatable::JMETranslatableStringInput;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
use crate::support::translatable::TranslatableStrings;
use crate::support::translatable::TranslatableStringsInput;
use crate::support::yaml::{YamlError, YamlResult};
use numbas::question::custom_part_type::CustomPartTypeSetting as NCustomPartTypeSetting;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};

optional_overwrite! {
    pub struct CustomPartTypeDefinition {
        type_name: TranslatableString,
        description: TranslatableString,
        settings: NCustomPartTypeSettings, // TODO
        can_be_gap: RumbasBool,
        can_be_step: RumbasBool,
        marking_notes: JMENotes,
        help_url: TranslatableString,
        published: RumbasBool,
        extensions: Extensions,
        input_widget: CustomPartInputWidget
        //TODO source
    }
}

type NCustomPartTypeSettingsInput = Vec<Value<NCustomPartTypeSetting>>;
type NCustomPartTypeSettings = Vec<NCustomPartTypeSetting>;
impl_optional_overwrite!(NCustomPartTypeSetting);

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
            settings: self.settings.clone(), // .to_numbas(&locale).unwrap(),
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

#[derive(Debug, Clone)]
pub struct CustomPartInputOptionValue<T: Clone> {
    /// The value
    value: T,
    /// A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.
    is_static: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct CustomPartInputOptionValueInput<T: Clone + OptionalCheck + Input> {
    /// The value
    value: Value<T>,
    /// A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.
    #[serde(rename = "static")]
    is_static: bool,
}

impl<T: Input + OptionalCheck + Clone> Input for CustomPartInputOptionValueInput<T>
where
    T::Normal: Clone,
{
    type Normal = CustomPartInputOptionValue<<T as Input>::Normal>;
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            value: Value::Normal(T::from_normal(normal.value)),
            is_static: normal.is_static,
        }
    }
    fn to_normal(&self) -> Self::Normal {
        Self::Normal {
            value: self.value.to_normal(),
            is_static: self.is_static,
        }
    }
}

impl<T: OptionalOverwrite<T> + OptionalCheck + Clone>
    OptionalOverwrite<CustomPartInputOptionValueInput<T>> for CustomPartInputOptionValueInput<T>
where
    T::Normal: Clone,
{
    fn overwrite(&mut self, other: &Self) {
        self.value.overwrite(&other.value);
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.value.insert_template_value(&key, &val);
    }
}
// TODO
//impl_optional_overwrite_value!(CustomPartInputOptionValueInput<T>[T]);

impl<N: Clone + OptionalCheck + Input> OptionalCheck for CustomPartInputOptionValueInput<N> {
    fn find_missing(&self) -> OptionalCheckResult {
        self.value.find_missing() // TODO: extend path...
    }
}

impl<N: Clone> RumbasCheck for CustomPartInputOptionValue<N> {
    fn check(&self, _locale: &str) -> RumbasCheckResult {
        RumbasCheckResult::empty()
    }
}

impl<N: Clone + RumbasCheck, T: Clone + ToNumbas<N>>
    ToNumbas<numbas::question::custom_part_type::CustomPartInputOptionValue<N>>
    for CustomPartInputOptionValue<T>
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::custom_part_type::CustomPartInputOptionValue<N> {
        numbas::question::custom_part_type::CustomPartInputOptionValue {
            value: self.value.clone().to_numbas(locale),
            is_static: self.is_static,
        }
    }
}

impl<V, T: Clone + ToRumbas<V>> ToRumbas<CustomPartInputOptionValue<V>>
    for numbas::question::custom_part_type::CustomPartInputOptionValue<T>
where
    V: Clone,
{
    fn to_rumbas(&self) -> CustomPartInputOptionValue<V> {
        CustomPartInputOptionValue {
            value: self.value.clone().to_rumbas(),
            is_static: self.is_static,
        }
    }
}

optional_overwrite_enum! {
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
        RadioGroup(CustomPartRadioGroupInputOptions)
    }
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

optional_overwrite! {
    pub struct CustomPartStringInputOptions {
        //TODO? hint & correctAnswer is shared for all..., use macro?
        /// A string displayed next to the input field, giving any necessary information about how to enter their answer.
        hint: CustomPartInputOptionValueTranslatableString,
        /// A JME expression which evaluates to the expected answer to the part.
        correct_answer: JMETranslatableString,
        /// If false, the part will only be marked if their answer is non-empty.
        allow_empty: CustomPartInputOptionValueBool
    }
}

type CustomPartInputOptionValueTranslatableString = CustomPartInputOptionValue<TranslatableString>;
type CustomPartInputOptionValueTranslatableStringInput =
    CustomPartInputOptionValueInput<TranslatableStringInput>;

type CustomPartInputOptionValueTranslatableStrings =
    CustomPartInputOptionValue<TranslatableStrings>;
type CustomPartInputOptionValueTranslatableStringsInput =
    CustomPartInputOptionValueInput<TranslatableStringsInput>;

type CustomPartInputOptionValueBool = CustomPartInputOptionValue<RumbasBool>;
type CustomPartInputOptionValueBoolInput = CustomPartInputOptionValueInput<RumbasBool>;

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

optional_overwrite! {
    pub struct CustomPartNumberInputOptions {
        /// A string displayed next to the input field, giving any necessary information about how to enter their answer.
        hint: CustomPartInputOptionValueTranslatableString,
        /// A JME expression which evaluates to the expected answer to the part.
        correct_answer: JMETranslatableString,
        ///Allow the student to enter their answer as a fraction?
        allow_fractions: CustomPartInputOptionValueBool,
        allowed_notation_styles:
            CustomPartInputOptionValueAnswerStyles
    }
}

type CustomPartInputOptionValueAnswerStyles =
    CustomPartInputOptionValue<Vec<crate::question::part::number_entry::AnswerStyle>>;
type CustomPartInputOptionValueAnswerStylesInput =
    CustomPartInputOptionValueInput<Vec<crate::question::part::number_entry::AnswerStyle>>; // TODO

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

optional_overwrite! {
    pub struct CustomPartRadioGroupInputOptions {
        /// A string displayed next to the input field, giving any necessary information about how to enter their answer.
        hint: CustomPartInputOptionValueTranslatableString,
        /// A JME expression which evaluates to the expected answer to the part.
        correct_answer: JMETranslatableString,
        /// The labels for the choices to offer to the student.
        choices: CustomPartInputOptionValueTranslatableStrings
    }
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

optional_overwrite! {
    #[serde(try_from = "String")]
    #[serde(into = "String")]
    pub struct CustomPartTypeDefinitionPath {
        custom_part_type_name: RumbasString,
        custom_part_type_data: CustomPartTypeDefinition
    }
}

pub type CustomPartTypeDefinitionPaths = Vec<CustomPartTypeDefinitionPath>;
pub type CustomPartTypeDefinitionPathsInput = Vec<Value<CustomPartTypeDefinitionPathInput>>;

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
                settings: self.settings.clone(),
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

impl JsonSchema for CustomPartTypeDefinitionPath {
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
