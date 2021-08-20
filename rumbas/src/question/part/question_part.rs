use crate::question::part::extension::QuestionPartExtension;
use crate::question::part::gapfill::QuestionPartGapFill;
use crate::question::part::information::QuestionPartInformation;
use crate::question::part::jme::QuestionPartJME;
use crate::question::part::matrix::QuestionPartMatrix;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultiple;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOne;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItems;
use crate::question::part::number_entry::QuestionPartNumberEntry;
use crate::question::part::pattern_match::QuestionPartPatternMatch;
use crate::support::template::{Value, ValueType};
use crate::support::translatable::{ContentAreaTranslatableString, JMETranslatableString};
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
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

impl ToNumbas<numbas::exam::ExamQuestionPart> for QuestionPart {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPart {
        match self {
            QuestionPart::Builtin(b) => {
                numbas::exam::ExamQuestionPart::Builtin(b.to_numbas(locale))
            }
            QuestionPart::Custom(b) => numbas::exam::ExamQuestionPart::Custom(b.to_numbas(locale)),
        }
    }
}

impl ToRumbas<QuestionPart> for numbas::exam::ExamQuestionPart {
    fn to_rumbas(&self) -> QuestionPart {
        match self {
            numbas::exam::ExamQuestionPart::Builtin(bqp) => QuestionPart::Builtin(bqp.to_rumbas()),
            numbas::exam::ExamQuestionPart::Custom(cqp) => QuestionPart::Custom(cqp.to_rumbas()),
        }
    }
}

impl QuestionPart {
    pub fn get_steps(&mut self) -> &mut Value<Vec<QuestionPart>> {
        match self {
            QuestionPart::Builtin(b) => b.get_steps(),
            QuestionPart::Custom(b) => b.get_steps(),
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

impl ToNumbas<numbas::exam::ExamQuestionPartBuiltin> for QuestionPartBuiltin {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartBuiltin {
        match self {
            QuestionPartBuiltin::JME(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::JME(n)
            }
            QuestionPartBuiltin::GapFill(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::GapFill(n)
            }
            QuestionPartBuiltin::ChooseOne(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::ChooseOne(n)
            }
            QuestionPartBuiltin::ChooseMultiple(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::ChooseMultiple(n)
            }
            QuestionPartBuiltin::MatchAnswersWithItems(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::MatchAnswersWithChoices(n)
            }
            QuestionPartBuiltin::NumberEntry(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::NumberEntry(n)
            }
            QuestionPartBuiltin::PatternMatch(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::PatternMatch(n)
            }
            QuestionPartBuiltin::Information(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::Information(n)
            }
            QuestionPartBuiltin::Extension(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::Extension(n)
            }
            QuestionPartBuiltin::Matrix(d) => {
                let n = d.to_numbas(locale);
                numbas::exam::ExamQuestionPartBuiltin::Matrix(Box::new(n))
            }
        }
    }
}

impl ToRumbas<QuestionPartBuiltin> for numbas::exam::ExamQuestionPartBuiltin {
    fn to_rumbas(&self) -> QuestionPartBuiltin {
        match self {
            numbas::exam::ExamQuestionPartBuiltin::JME(p) => {
                QuestionPartBuiltin::JME(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::NumberEntry(p) => {
                QuestionPartBuiltin::NumberEntry(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::Matrix(p) => {
                QuestionPartBuiltin::Matrix(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::PatternMatch(p) => {
                QuestionPartBuiltin::PatternMatch(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::ChooseOne(p) => {
                QuestionPartBuiltin::ChooseOne(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::ChooseMultiple(p) => {
                QuestionPartBuiltin::ChooseMultiple(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::MatchAnswersWithChoices(p) => {
                QuestionPartBuiltin::MatchAnswersWithItems(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::GapFill(p) => {
                QuestionPartBuiltin::GapFill(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::Information(p) => {
                QuestionPartBuiltin::Information(p.to_rumbas())
            }
            numbas::exam::ExamQuestionPartBuiltin::Extension(p) => {
                QuestionPartBuiltin::Extension(p.to_rumbas())
            }
        }
    }
}

impl QuestionPartBuiltin {
    pub fn get_steps(&mut self) -> &mut Value<Vec<QuestionPart>> {
        match self {
            QuestionPartBuiltin::JME(d) => d.get_steps(),
            QuestionPartBuiltin::GapFill(d) => d.get_steps(),
            QuestionPartBuiltin::ChooseOne(d) => d.get_steps(),
            QuestionPartBuiltin::ChooseMultiple(d) => d.get_steps(),
            QuestionPartBuiltin::MatchAnswersWithItems(d) => d.get_steps(),
            QuestionPartBuiltin::NumberEntry(d) => d.get_steps(),
            QuestionPartBuiltin::PatternMatch(d) => d.get_steps(),
            QuestionPartBuiltin::Information(d) => d.get_steps(),
            QuestionPartBuiltin::Extension(d) => d.get_steps(),
            QuestionPartBuiltin::Matrix(d) => d.get_steps(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, JsonSchema, Deserialize, Serialize)]
pub struct JMENotes(pub Value<Vec<JMENote>>);

impl RumbasCheck for JMENotes {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        self.0.check(locale)
    }
}

impl OptionalOverwrite<JMENotes> for JMENotes {
    fn overwrite(&mut self, other: &JMENotes) {
        self.0.overwrite(&other.0)
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.0.insert_template_value(key, val)
    }
}
impl_optional_overwrite_value!(JMENotes);

impl ToNumbas<numbas::jme::JMENotesString> for JMENotes {
    fn to_numbas(&self, locale: &str) -> numbas::jme::JMENotesString {
        self.0
            .unwrap()
            .iter()
            .map(|v| {
                let description = if let Noneable::NotNone(d) = v.description.unwrap() {
                    format!("({})", d)
                } else {
                    "".to_string()
                };
                format!(
                    "{}{}:{}",
                    v.name.unwrap(),
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
        JMENotes(Value::Normal(if let Some(ref notes) = self.notes {
            notes
                .iter()
                .map(|n| JMENote {
                    name: Value::Normal(n.name.to_string()),
                    description: Value::Normal(
                        n.description
                            .clone()
                            .map_or(Noneable::nn(), Noneable::NotNone),
                    ),
                    expression: Value::Normal(n.expression_string().to_rumbas()),
                })
                .collect()
        } else {
            vec![]
        }))
    }
}

impl Default for JMENotes {
    fn default() -> Self {
        JMENotes(Value::Normal(vec![]))
    }
}

optional_overwrite! {
    pub struct JMENote {
        name: String,
        description: Noneable<String>,
        expression: JMETranslatableString
    }
}

impl ToNumbas<numbas::exam::CustomPartMarkingNote> for JMENote {
    fn to_numbas(&self, locale: &str) -> numbas::exam::CustomPartMarkingNote {
        numbas::exam::CustomPartMarkingNote {
            name: self.name.to_numbas(locale),
            definition: self.expression.to_numbas(&locale),
            description: self.description.unwrap().unwrap_or("".to_string()),
        }
    }
}

impl ToRumbas<JMENote> for numbas::exam::CustomPartMarkingNote {
    fn to_rumbas(&self) -> JMENote {
        JMENote {
            name: Value::Normal(self.name.clone()),
            expression: Value::Normal(self.definition.to_rumbas()),
            description: Value::Normal(Noneable::NotNone(self.description.clone())),
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
                marks: numbas::exam::Primitive, // TODO: strict?
                prompt: ContentAreaTranslatableString,
                use_custom_name: bool,
                custom_name: String, //Translatable?
                steps_penalty: usize,
                enable_minimum_marks: bool,
                minimum_marks: usize, //TODO: separate?
                show_correct_answer: bool,
                show_feedback_icon: bool,
                variable_replacement_strategy: VariableReplacementStrategy,
                adaptive_marking_penalty: usize,
                custom_marking_algorithm_notes: JMENotes,
                extend_base_marking_algorithm: bool,
                steps: Vec<QuestionPart>
                $(,
                $(
                    $(#[$inner])*
                    $field: $type
                ),+
                )?
            }
        }
        impl $struct {
            fn to_numbas_shared_data(&self, locale: &str) -> numbas::exam::ExamQuestionPartSharedData {
                numbas::exam::ExamQuestionPartSharedData {
                    marks: Some(self.marks.clone().to_numbas(locale)),
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

            pub fn get_steps(&mut self) -> &mut Value<Vec<QuestionPart>> {
                &mut self.steps
            }
        }
    }
}

question_part_type! {
    pub struct QuestionPartCustom {
        r#type: String,
        settings: std::collections::HashMap<String, CustomPartInputTypeValue>
    }
}

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum CustomPartInputTypeValue {
        CheckBox(bool),
        Code(String)
    }
}

impl ToNumbas<numbas::exam::CustomPartInputTypeValue> for CustomPartInputTypeValue {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::CustomPartInputTypeValue {
        match self {
            CustomPartInputTypeValue::CheckBox(v) => {
                numbas::exam::CustomPartInputTypeValue::CheckBox(*v)
            }
            CustomPartInputTypeValue::Code(v) => {
                numbas::exam::CustomPartInputTypeValue::Code(v.clone().into())
            }
        }
    }
}

impl ToRumbas<CustomPartInputTypeValue> for numbas::exam::CustomPartInputTypeValue {
    fn to_rumbas(&self) -> CustomPartInputTypeValue {
        match self {
            numbas::exam::CustomPartInputTypeValue::CheckBox(v) => {
                CustomPartInputTypeValue::CheckBox(*v)
            }
            numbas::exam::CustomPartInputTypeValue::Code(v) => {
                CustomPartInputTypeValue::Code(v.to_string())
            }
        }
    }
}

impl ToNumbas<numbas::exam::ExamQuestionPartCustom> for QuestionPartCustom {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartCustom {
        numbas::exam::ExamQuestionPartCustom {
            part_data: self.to_numbas_shared_data(locale),
            r#type: self.r#type.to_numbas(locale),
            settings: self.settings.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartCustom> for numbas::exam::ExamQuestionPartCustom {
    fn to_rumbas(&self) -> QuestionPartCustom {
        QuestionPartCustom {
            // Default section
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(extract_part_common_prompt(&self.part_data)),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&self.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&self.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&self.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &self.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&self.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &self.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &self.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                self.part_data.variable_replacement_strategy.to_rumbas(),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &self.part_data,
            )),
            custom_marking_algorithm_notes: Value::Normal(
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
            ),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),

            r#type: Value::Normal(self.r#type.clone()),
            settings: Value::Normal(
                self.settings
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_rumbas()))
                    .collect(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    OriginalFirst,
}
impl_optional_overwrite!(VariableReplacementStrategy);

impl ToNumbas<numbas::exam::VariableReplacementStrategy> for VariableReplacementStrategy {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::VariableReplacementStrategy {
        match self {
            VariableReplacementStrategy::OriginalFirst => {
                numbas::exam::VariableReplacementStrategy::OriginalFirst
            }
        }
    }
}

impl ToRumbas<VariableReplacementStrategy> for numbas::exam::VariableReplacementStrategy {
    fn to_rumbas(&self) -> VariableReplacementStrategy {
        match self {
            numbas::exam::VariableReplacementStrategy::OriginalFirst => {
                VariableReplacementStrategy::OriginalFirst
            }
        }
    }
}
