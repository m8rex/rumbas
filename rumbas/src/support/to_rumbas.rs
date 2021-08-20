use crate::question::part::question_part::QuestionPart;
use crate::support::file_reference::FileString;
use crate::support::template::Value;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::JMETranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use std::convert::TryInto;

pub trait ToRumbas<RumbasType>: Clone {
    fn to_rumbas(&self) -> RumbasType;
}

macro_rules! impl_to_rumbas {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl$(< $($gen : Clone),* >)? ToRumbas<$type> for $type {
            fn to_rumbas(&self) -> $type {
                self.clone()
            }
        }
        )*
    };
}
pub(crate) use impl_to_rumbas;

impl_to_rumbas!(bool, f64, usize, String, [f64; 2]);
impl_to_rumbas!(numbas::exam::Primitive);
impl_to_rumbas!(numbas::jme::JMEString);
impl_to_rumbas!(numbas::jme::EmbracedJMEString);
impl_to_rumbas!(numbas::jme::ContentAreaString);

impl<T, O: ToRumbas<T>> ToRumbas<Value<T>> for O {
    fn to_rumbas(&self) -> Value<T> {
        Value::Normal(self.to_rumbas())
    }
}

impl<T, O: ToRumbas<T>> ToRumbas<Vec<T>> for Vec<O> {
    fn to_rumbas(&self) -> Vec<T> {
        self.iter().map(|item| item.to_rumbas()).collect()
    }
}

impl<T, O: ToRumbas<T>> ToRumbas<Option<T>> for Option<O> {
    fn to_rumbas(&self) -> Option<T> {
        self.clone().map(|item| item.to_rumbas())
    }
}

impl ToRumbas<FileString> for String {
    fn to_rumbas(&self) -> FileString {
        FileString::s(self)
    }
}

impl ToRumbas<TranslatableString> for String {
    fn to_rumbas(&self) -> TranslatableString {
        self.clone().into()
    }
}

impl ToRumbas<JMETranslatableString> for JMEString {
    fn to_rumbas(&self) -> JMETranslatableString {
        self.clone().into()
    }
}

impl ToRumbas<EmbracedJMETranslatableString> for EmbracedJMEString {
    fn to_rumbas(&self) -> EmbracedJMETranslatableString {
        self.clone().into()
    }
}

impl ToRumbas<ContentAreaTranslatableString> for ContentAreaString {
    fn to_rumbas(&self) -> ContentAreaTranslatableString {
        self.clone().into()
    }
}

pub fn extract_part_common_marks(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> numbas::exam::Primitive {
    pd.marks
        .clone()
        .unwrap_or(numbas::exam::Primitive::Natural(DEFAULTS.part_common_marks))
}

pub fn extract_part_common_prompt(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> ContentAreaTranslatableString {
    pd.prompt
        .clone()
        .unwrap_or_else(|| "".to_string().try_into().unwrap())
        .to_rumbas()
}

pub fn extract_part_common_use_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.use_custom_name
        .unwrap_or(DEFAULTS.part_common_use_custom_name)
}

pub fn extract_part_common_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.custom_name.clone().unwrap_or_default()
}

pub fn extract_part_common_steps_penalty(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.steps_penalty
        .unwrap_or(DEFAULTS.part_common_steps_penalty)
}

pub fn extract_part_common_enable_minimum_marks(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.enable_minimum_marks
        .unwrap_or(DEFAULTS.part_common_enable_minimum_marks)
}

pub fn extract_part_common_minimum_marks(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.minimum_marks
        .unwrap_or(DEFAULTS.part_common_minimum_marks)
}

pub fn extract_part_common_show_correct_answer(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.show_correct_answer
}

pub fn extract_part_common_show_feedback_icon(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.show_feedback_icon
        .unwrap_or(DEFAULTS.part_common_show_feedback_icon)
}

pub fn extract_part_common_adaptive_marking_penalty(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> usize {
    pd.adaptive_marking_penalty
        .unwrap_or(DEFAULTS.part_common_adaptive_marking_penalty)
}

pub fn extract_part_common_extend_base_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.extend_base_marking_algorithm
        .unwrap_or(DEFAULTS.part_common_extend_base_marking_algorithm)
}

pub fn extract_part_common_steps(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> Vec<QuestionPart> {
    pd.steps.clone().unwrap_or_default().to_rumbas()
}

/// Macro used to create a question part type for numbas
/// Usage: create_question_part! { PartName with &self.part_data => { field1: val1, field2, val2 }  }
macro_rules! create_question_part {
    (
        $type: ident with $part_data: expr => {
            $(
                $field: ident: $val: expr
            ),*
        }
    ) => {
        {
            let part_data = $part_data;
            let custom_marking_algorithm_notes: Value<JMENotes> = part_data
                .custom_marking_algorithm
                .clone()
                .unwrap_or_default()
                .to_rumbas();
            $type {
                // Default section
                marks: extract_part_common_marks(&part_data).to_rumbas(),
                prompt: extract_part_common_prompt(&part_data).to_rumbas(),
                use_custom_name: extract_part_common_use_custom_name(&part_data).to_rumbas(),
                custom_name: extract_part_common_custom_name(&part_data).to_rumbas(),
                steps_penalty: extract_part_common_steps_penalty(&part_data).to_rumbas(),
                enable_minimum_marks: extract_part_common_enable_minimum_marks(&part_data)
                    .to_rumbas(),
                minimum_marks: extract_part_common_minimum_marks(&part_data).to_rumbas(),
                show_correct_answer: extract_part_common_show_correct_answer(&part_data)
                    .to_rumbas(),
                show_feedback_icon: extract_part_common_show_feedback_icon(&part_data).to_rumbas(),
                variable_replacement_strategy: part_data.variable_replacement_strategy.to_rumbas(),
                adaptive_marking_penalty: extract_part_common_adaptive_marking_penalty(&part_data)
                    .to_rumbas(),
                custom_marking_algorithm_notes,
                extend_base_marking_algorithm: extract_part_common_extend_base_marking_algorithm(
                    &part_data,
                )
                .to_rumbas(),
                steps: Value::Normal(extract_part_common_steps(&part_data)),
                $(
                    $field: $val
                ),*
            }
        }
    }
}
pub(crate) use create_question_part;
