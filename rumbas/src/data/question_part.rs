use crate::data::extension::QuestionPartExtension;
use crate::data::gapfill::QuestionPartGapFill;
use crate::data::information::QuestionPartInformation;
use crate::data::jme::QuestionPartJME;
use crate::data::matrix::QuestionPartMatrix;
use crate::data::multiple_choice::QuestionPartChooseMultiple;
use crate::data::multiple_choice::QuestionPartChooseOne;
use crate::data::multiple_choice::QuestionPartMatchAnswersWithItems;
use crate::data::number_entry::QuestionPartNumberEntry;
use crate::data::optional_overwrite::*;
use crate::data::pattern_match::QuestionPartPatternMatch;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::*;
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum QuestionPart {
        Builtin(QuestionPartBuiltin),
        Custom(QuestionPartCustom)
    }
}

impl ToNumbas for QuestionPart {
    type NumbasType = numbas::exam::ExamQuestionPart;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamQuestionPart> {
        match self {
            QuestionPart::Builtin(b) => b
                .to_numbas(locale)
                .map(numbas::exam::ExamQuestionPart::Builtin),
            QuestionPart::Custom(b) => b
                .to_numbas(locale)
                .map(numbas::exam::ExamQuestionPart::Custom),
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

impl ToNumbas for QuestionPartBuiltin {
    type NumbasType = numbas::exam::ExamQuestionPartBuiltin;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamQuestionPartBuiltin> {
        Ok(match self {
            QuestionPartBuiltin::JME(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::JME(n)
            }
            QuestionPartBuiltin::GapFill(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::GapFill(n)
            }
            QuestionPartBuiltin::ChooseOne(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::ChooseOne(n)
            }
            QuestionPartBuiltin::ChooseMultiple(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::ChooseMultiple(n)
            }
            QuestionPartBuiltin::MatchAnswersWithItems(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::MatchAnswersWithChoices(n)
            }
            QuestionPartBuiltin::NumberEntry(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::NumberEntry(n)
            }
            QuestionPartBuiltin::PatternMatch(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::PatternMatch(n)
            }
            QuestionPartBuiltin::Information(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::Information(n)
            }
            QuestionPartBuiltin::Extension(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::Extension(n)
            }
            QuestionPartBuiltin::Matrix(d) => {
                let n = d.to_numbas(locale)?;
                numbas::exam::ExamQuestionPartBuiltin::Matrix(Box::new(n))
            }
        })
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
                prompt: TranslatableString,
                use_custom_name: bool,
                custom_name: String, //Translatable?
                steps_penalty: usize,
                enable_minimum_marks: bool,
                minimum_marks: usize, //TODO: separate?
                show_correct_answer: bool,
                show_feedback_icon: bool,
                variable_replacement_strategy: VariableReplacementStrategy,
                adaptive_marking_penalty: usize,
                custom_marking_algorithm: String, // TODO? empty string -> none?, from file?
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
                    marks: Some(self.marks.clone().unwrap().into()),
                    prompt: self.prompt.clone().map(|s| s.to_string(&locale)).flatten(),
                    use_custom_name: Some(self.use_custom_name.clone().unwrap()),
                    custom_name: Some(self.custom_name.clone().unwrap()),
                    steps_penalty: Some(self.steps_penalty.clone().unwrap()),
                    enable_minimum_marks:Some(self.enable_minimum_marks.clone().unwrap()),
                    minimum_marks: Some(self.minimum_marks.clone().unwrap()),
                    show_correct_answer: self.show_correct_answer.clone().unwrap(),
                    show_feedback_icon: Some(self.show_feedback_icon.clone().unwrap()),
                    variable_replacement_strategy: self.variable_replacement_strategy.clone().unwrap().to_numbas(&locale).unwrap(),
                    adaptive_marking_penalty:Some(self.adaptive_marking_penalty.clone().unwrap()),
                    custom_marking_algorithm: Some(self.custom_marking_algorithm.clone().unwrap()),
                    extend_base_marking_algorithm: Some(self.extend_base_marking_algorithm.clone().unwrap()),
                    steps: self.steps.clone().map(|v| v.iter().map(|s| s.to_numbas(&locale).unwrap()).collect()),
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

impl ToNumbas for CustomPartInputTypeValue {
    type NumbasType = numbas::exam::CustomPartInputTypeValue;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(match self {
                CustomPartInputTypeValue::CheckBox(v) => {
                    numbas::exam::CustomPartInputTypeValue::CheckBox(*v)
                }
                CustomPartInputTypeValue::Code(v) => {
                    numbas::exam::CustomPartInputTypeValue::Code(v.clone().into())
                }
            })
        } else {
            Err(check)
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

impl ToNumbas for QuestionPartCustom {
    type NumbasType = numbas::exam::ExamQuestionPartCustom;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(locale),
                r#type: self.r#type.unwrap(),
                settings: self
                    .settings
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_numbas(locale).unwrap()))
                    .collect(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionPartCustom> for numbas::exam::ExamQuestionPartCustom {
    fn to_rumbas(&self) -> QuestionPartCustom {
        QuestionPartCustom {
            // Default section
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
                &self.part_data,
            ))),
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
            custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
                &self.part_data,
            )),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    OriginalFirst,
}
impl_optional_overwrite!(VariableReplacementStrategy);

impl ToNumbas for VariableReplacementStrategy {
    type NumbasType = numbas::exam::VariableReplacementStrategy;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            VariableReplacementStrategy::OriginalFirst => {
                numbas::exam::VariableReplacementStrategy::OriginalFirst
            }
        })
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
