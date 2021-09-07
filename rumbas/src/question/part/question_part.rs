use crate::question::part::extension::QuestionPartExtension;
use crate::question::part::extension::QuestionPartExtensionInput;
use crate::question::part::gapfill::QuestionPartGapFill;
use crate::question::part::gapfill::QuestionPartGapFillInput;
use crate::question::part::information::QuestionPartInformation;
use crate::question::part::information::QuestionPartInformationInput;
use crate::question::part::jme::QuestionPartJME;
use crate::question::part::jme::QuestionPartJMEInput;
use crate::question::part::matrix::QuestionPartMatrix;
use crate::question::part::matrix::QuestionPartMatrixInput;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultiple;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultipleInput;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOne;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOneInput;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItems;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItemsInput;
use crate::question::part::number_entry::QuestionPartNumberEntry;
use crate::question::part::number_entry::QuestionPartNumberEntryInput;
use crate::question::part::pattern_match::QuestionPartPatternMatch;
use crate::question::part::pattern_match::QuestionPartPatternMatchInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::{ContentAreaTranslatableString, JMETranslatableString};
use crate::support::translatable::{
    ContentAreaTranslatableStringInput, JMETranslatableStringInput,
};
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum QuestionPart {
        Builtin(QuestionPartBuiltin),
        Custom(QuestionPartCustom)
    }
}

pub type QuestionPartsInput = Vec<Value<QuestionPartInput>>;
pub type QuestionParts = Vec<QuestionPart>;

impl ToNumbas<numbas::question::part::QuestionPart> for QuestionPart {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPart {
        match self {
            QuestionPart::Builtin(b) => {
                numbas::question::part::QuestionPart::Builtin(b.to_numbas(locale))
            }
            QuestionPart::Custom(b) => {
                numbas::question::part::QuestionPart::Custom(b.to_numbas(locale))
            }
        }
    }
}

impl ToRumbas<QuestionPart> for numbas::question::part::QuestionPart {
    fn to_rumbas(&self) -> QuestionPart {
        match self {
            numbas::question::part::QuestionPart::Builtin(bqp) => {
                QuestionPart::Builtin(bqp.to_rumbas())
            }
            numbas::question::part::QuestionPart::Custom(cqp) => {
                QuestionPart::Custom(cqp.to_rumbas())
            }
        }
    }
}

impl QuestionPartInput {
    pub fn get_steps(&mut self) -> &mut Value<Vec<Value<QuestionPartInput>>> {
        match self {
            QuestionPartInput::Builtin(b) => b.get_steps(),
            QuestionPartInput::Custom(b) => b.get_steps(),
        }
    }
}

optional_overwrite_enum! {
    #[serde(tag="type")]
    pub enum QuestionPartBuiltin {
        #[serde(rename = "jme")]
        JME(QuestionPartJME),
        #[serde(rename = "gapfill")]
        GapFill(QuestionPartGapFill),
        #[serde(rename = "choose_one")]
        ChooseOne(QuestionPartChooseOne),
        #[serde(rename = "choose_multiple")]
        ChooseMultiple(QuestionPartChooseMultiple),
        #[serde(rename= "match_answers")]
        MatchAnswersWithItems(QuestionPartMatchAnswersWithItems),
        #[serde(rename = "number_entry")]
        NumberEntry(QuestionPartNumberEntry),
        #[serde(rename = "pattern_match")]
        PatternMatch(QuestionPartPatternMatch),
        #[serde(rename = "information")]
        Information(QuestionPartInformation),
        #[serde(rename = "extension")]
        Extension(QuestionPartExtension),
        #[serde(rename = "matrix")]
        Matrix(QuestionPartMatrix)
    }
}

impl ToNumbas<numbas::question::part::QuestionPartBuiltin> for QuestionPartBuiltin {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPartBuiltin {
        match self {
            QuestionPartBuiltin::JME(d) => {
                numbas::question::part::QuestionPartBuiltin::JME(d.to_numbas(locale))
            }
            QuestionPartBuiltin::GapFill(d) => {
                numbas::question::part::QuestionPartBuiltin::GapFill(d.to_numbas(locale))
            }
            QuestionPartBuiltin::ChooseOne(d) => {
                numbas::question::part::QuestionPartBuiltin::ChooseOne(d.to_numbas(locale))
            }
            QuestionPartBuiltin::ChooseMultiple(d) => {
                numbas::question::part::QuestionPartBuiltin::ChooseMultiple(d.to_numbas(locale))
            }
            QuestionPartBuiltin::MatchAnswersWithItems(d) => {
                numbas::question::part::QuestionPartBuiltin::MatchAnswersWithChoices(
                    d.to_numbas(locale),
                )
            }
            QuestionPartBuiltin::NumberEntry(d) => {
                numbas::question::part::QuestionPartBuiltin::NumberEntry(d.to_numbas(locale))
            }
            QuestionPartBuiltin::PatternMatch(d) => {
                numbas::question::part::QuestionPartBuiltin::PatternMatch(d.to_numbas(locale))
            }
            QuestionPartBuiltin::Information(d) => {
                numbas::question::part::QuestionPartBuiltin::Information(d.to_numbas(locale))
            }
            QuestionPartBuiltin::Extension(d) => {
                numbas::question::part::QuestionPartBuiltin::Extension(d.to_numbas(locale))
            }
            QuestionPartBuiltin::Matrix(d) => {
                numbas::question::part::QuestionPartBuiltin::Matrix(Box::new(d.to_numbas(locale)))
            }
        }
    }
}

impl ToRumbas<QuestionPartBuiltin> for numbas::question::part::QuestionPartBuiltin {
    fn to_rumbas(&self) -> QuestionPartBuiltin {
        match self {
            numbas::question::part::QuestionPartBuiltin::JME(p) => {
                QuestionPartBuiltin::JME(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::NumberEntry(p) => {
                QuestionPartBuiltin::NumberEntry(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::Matrix(p) => {
                QuestionPartBuiltin::Matrix((**p).to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::PatternMatch(p) => {
                QuestionPartBuiltin::PatternMatch(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::ChooseOne(p) => {
                QuestionPartBuiltin::ChooseOne(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::ChooseMultiple(p) => {
                QuestionPartBuiltin::ChooseMultiple(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::MatchAnswersWithChoices(p) => {
                QuestionPartBuiltin::MatchAnswersWithItems(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::GapFill(p) => {
                QuestionPartBuiltin::GapFill(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::Information(p) => {
                QuestionPartBuiltin::Information(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::Extension(p) => {
                QuestionPartBuiltin::Extension(p.to_rumbas())
            }
        }
    }
}

impl QuestionPartBuiltinInput {
    pub fn get_steps(&mut self) -> &mut Value<Vec<Value<QuestionPartInput>>> {
        match self {
            QuestionPartBuiltinInput::JME(d) => d.get_steps(),
            QuestionPartBuiltinInput::GapFill(d) => d.get_steps(),
            QuestionPartBuiltinInput::ChooseOne(d) => d.get_steps(),
            QuestionPartBuiltinInput::ChooseMultiple(d) => d.get_steps(),
            QuestionPartBuiltinInput::MatchAnswersWithItems(d) => d.get_steps(),
            QuestionPartBuiltinInput::NumberEntry(d) => d.get_steps(),
            QuestionPartBuiltinInput::PatternMatch(d) => d.get_steps(),
            QuestionPartBuiltinInput::Information(d) => d.get_steps(),
            QuestionPartBuiltinInput::Extension(d) => d.get_steps(),
            QuestionPartBuiltinInput::Matrix(d) => d.get_steps(),
        }
    }
}

pub type JMENotesVecInput = Vec<Value<JMENoteInput>>;
pub type JMENotesVec = Vec<JMENote>;

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "JMENotesInput")]
#[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
pub struct JMENotes(pub JMENotesVec);

impl ToNumbas<numbas::jme::JMENotesString> for JMENotes {
    fn to_numbas(&self, locale: &str) -> numbas::jme::JMENotesString {
        self.0
            .iter()
            .map(|v| {
                let description = if let Noneable::NotNone(d) = &v.description {
                    format!("({})", d)
                } else {
                    "".to_string()
                };
                format!(
                    "{}{}:{}",
                    v.name,
                    description,
                    v.expression.to_numbas(locale)
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
            .try_into()
            .unwrap()
    }
}

impl ToRumbas<JMENotes> for numbas::jme::JMENotesString {
    fn to_rumbas(&self) -> JMENotes {
        JMENotes(if let Some(ref notes) = self.notes {
            notes
                .iter()
                .map(|n| JMENote {
                    name: n.name.to_string().to_rumbas(),
                    description: n.description.to_rumbas(),
                    expression: n.expression_string().to_rumbas(),
                })
                .collect()
        } else {
            vec![]
        })
    }
}

impl Default for JMENotes {
    fn default() -> Self {
        JMENotes(vec![])
    }
}

optional_overwrite! {
    pub struct JMENote {
        name: RumbasString,
        description: NoneableString,
        expression: JMETranslatableString
    }
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartMarkingNote> for JMENote {
    fn to_numbas(&self, locale: &str) -> numbas::question::custom_part_type::CustomPartMarkingNote {
        numbas::question::custom_part_type::CustomPartMarkingNote {
            name: self.name.to_numbas(locale),
            definition: self.expression.to_numbas(&locale),
            description: self.description.unwrap_or("".to_string()),
        }
    }
}

impl ToRumbas<JMENote> for numbas::question::custom_part_type::CustomPartMarkingNote {
    fn to_rumbas(&self) -> JMENote {
        JMENote {
            name: self.name.to_rumbas(),
            expression: self.definition.to_rumbas(),
            description: Noneable::NotNone(self.description.to_rumbas()),
        }
    }
}

// TODO major refactor: add fields that are used to right ones
macro_rules! question_part_type {
    (
        $(#[$outer:meta])*
        pub struct $struct: ident {
            $($(
                $(#[$inner:meta])*
                $field: ident: $type: ty
             ),+)?
        }
    )=> {
        optional_overwrite! {
            $(#[$outer])*
            pub struct $struct {
                marks: Primitive, // TODO: strict?
                prompt: ContentAreaTranslatableString,
                use_custom_name: RumbasBool,
                custom_name: RumbasString, //TODO Translatable?
                steps_penalty: RumbasNatural,
                enable_minimum_marks: RumbasBool,
                minimum_marks: RumbasNatural, //TODO: separate?
                show_correct_answer: RumbasBool,
                show_feedback_icon: RumbasBool,
                variable_replacement_strategy: VariableReplacementStrategy,
                adaptive_marking_penalty: RumbasNatural,
                custom_marking_algorithm_notes: JMENotes,
                extend_base_marking_algorithm: RumbasBool,
                steps: QuestionParts
                $(,
                $(
                    $(#[$inner])*
                    $field: $type
                ),+
                )?
            }
        }
        impl ToNumbas<numbas::question::part::QuestionPartSharedData> for $struct {
            fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPartSharedData {
                numbas::question::part::QuestionPartSharedData {
                    marks: Some(self.marks.to_numbas(locale)),
                    prompt: Some(self.prompt.to_numbas(locale)),
                    use_custom_name: Some(self.use_custom_name.to_numbas(locale)),
                    custom_name: Some(self.custom_name.to_numbas(locale)),
                    steps_penalty: Some(self.steps_penalty.to_numbas(locale)),
                    enable_minimum_marks:Some(self.enable_minimum_marks.to_numbas(locale)),
                    minimum_marks: Some(self.minimum_marks.to_numbas(locale)),
                    show_correct_answer: self.show_correct_answer.to_numbas(locale),
                    show_feedback_icon: Some(self.show_feedback_icon.to_numbas(locale)),
                    variable_replacement_strategy: self.variable_replacement_strategy.to_numbas(&locale),
                    adaptive_marking_penalty:Some(self.adaptive_marking_penalty.to_numbas(locale)),
                    custom_marking_algorithm: Some(self.custom_marking_algorithm_notes.to_numbas(&locale)),
                    extend_base_marking_algorithm: Some(self.extend_base_marking_algorithm.to_numbas(locale)),
                    steps: Some(self.steps.to_numbas(&locale)),
                }

            }
        }
        paste::paste! {
            impl [<$struct Input>] {
                pub fn get_steps(&mut self) -> &mut Value<Vec<Value<QuestionPartInput>>> {
                    &mut self.steps
                }
            }
        }
    }
}

question_part_type! {
    pub struct QuestionPartCustom {
        r#type: RumbasString,
        settings: MapStringToCustomPartInputTypeValue
    }
}

type MapStringToCustomPartInputTypeValueInput =
    std::collections::HashMap<String, CustomPartInputTypeValueInput>;
type MapStringToCustomPartInputTypeValue =
    std::collections::HashMap<String, CustomPartInputTypeValue>;

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum CustomPartInputTypeValue {
        CheckBox(RumbasBool),
        Code(RumbasString)
    }
}

impl ToNumbas<numbas::question::part::CustomPartInputTypeValue> for CustomPartInputTypeValue {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::CustomPartInputTypeValue {
        match self {
            CustomPartInputTypeValue::CheckBox(v) => {
                numbas::question::part::CustomPartInputTypeValue::CheckBox(*v)
            }
            CustomPartInputTypeValue::Code(v) => {
                numbas::question::part::CustomPartInputTypeValue::Code(v.clone().into())
            }
        }
    }
}

impl ToRumbas<CustomPartInputTypeValue> for numbas::question::part::CustomPartInputTypeValue {
    fn to_rumbas(&self) -> CustomPartInputTypeValue {
        match self {
            numbas::question::part::CustomPartInputTypeValue::CheckBox(v) => {
                CustomPartInputTypeValue::CheckBox(*v)
            }
            numbas::question::part::CustomPartInputTypeValue::Code(v) => {
                CustomPartInputTypeValue::Code(v.to_string())
            }
        }
    }
}

impl ToNumbas<numbas::question::part::QuestionPartCustom> for QuestionPartCustom {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPartCustom {
        numbas::question::part::QuestionPartCustom {
            part_data: self.to_numbas(locale),
            r#type: self.r#type.to_numbas(locale),
            settings: self.settings.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartCustom> for numbas::question::part::QuestionPartCustom {
    fn to_rumbas(&self) -> QuestionPartCustom {
        let custom_marking_algorithm_notes: Option<_> =
            self.part_data.custom_marking_algorithm.to_rumbas();

        QuestionPartCustom {
            // Default section
            marks: extract_part_common_marks(&self.part_data),
            prompt: extract_part_common_prompt(&self.part_data),
            use_custom_name: extract_part_common_use_custom_name(&self.part_data),
            custom_name: extract_part_common_custom_name(&self.part_data),
            steps_penalty: extract_part_common_steps_penalty(&self.part_data),
            enable_minimum_marks: extract_part_common_enable_minimum_marks(&self.part_data),
            minimum_marks: extract_part_common_minimum_marks(&self.part_data),
            show_correct_answer: extract_part_common_show_correct_answer(&self.part_data),
            show_feedback_icon: extract_part_common_show_feedback_icon(&self.part_data),
            variable_replacement_strategy: self.part_data.variable_replacement_strategy.to_rumbas(),
            adaptive_marking_penalty: extract_part_common_adaptive_marking_penalty(&self.part_data),
            custom_marking_algorithm_notes: custom_marking_algorithm_notes.unwrap_or_default(),
            extend_base_marking_algorithm: extract_part_common_extend_base_marking_algorithm(
                &self.part_data,
            ),
            steps: extract_part_common_steps(&self.part_data),

            r#type: self.r#type.clone(),
            settings: self.settings.to_rumbas(),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    OriginalFirst,
}
impl_optional_overwrite!(VariableReplacementStrategy);

impl ToNumbas<numbas::question::part::VariableReplacementStrategy> for VariableReplacementStrategy {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::VariableReplacementStrategy {
        match self {
            VariableReplacementStrategy::OriginalFirst => {
                numbas::question::part::VariableReplacementStrategy::OriginalFirst
            }
        }
    }
}

impl ToRumbas<VariableReplacementStrategy> for numbas::question::part::VariableReplacementStrategy {
    fn to_rumbas(&self) -> VariableReplacementStrategy {
        match self {
            numbas::question::part::VariableReplacementStrategy::OriginalFirst => {
                VariableReplacementStrategy::OriginalFirst
            }
        }
    }
}
