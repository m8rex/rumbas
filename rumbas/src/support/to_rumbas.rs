use crate::question::part::question_part::QuestionPart;
use crate::question::part::question_part::{AdaptiveMarking, CustomMarking};
use crate::support::file_reference::{FileString, JMEFileString};
use crate::support::noneable::Noneable;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::JMETranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use numbas::support::primitive::Number;

pub trait ToRumbas<RumbasType>: Clone {
    fn to_rumbas(&self) -> RumbasType;
}

impl_to_rumbas!(String, bool, f64, usize, [f64; 2]);
impl_to_rumbas!(numbas::support::primitive::Number);
impl_to_rumbas!(numbas::jme::JMEString);
impl_to_rumbas!(numbas::jme::EmbracedJMEString);
impl_to_rumbas!(numbas::jme::ContentAreaString);

impl<T, O: ToRumbas<T>> ToRumbas<Vec<T>> for Vec<O> {
    fn to_rumbas(&self) -> Vec<T> {
        self.iter().map(|item| item.to_rumbas()).collect()
    }
}

impl<K: Clone + std::cmp::Ord, S, O: ToRumbas<S>> ToRumbas<std::collections::BTreeMap<K, S>>
    for std::collections::BTreeMap<K, O>
{
    fn to_rumbas(&self) -> std::collections::BTreeMap<K, S> {
        self.iter()
            .map(|(k, v)| (k.to_owned(), v.to_rumbas()))
            .collect()
    }
}

impl<K: Clone + std::hash::Hash + std::cmp::Eq, S, O: ToRumbas<S>>
    ToRumbas<std::collections::HashMap<K, S>> for std::collections::HashMap<K, O>
{
    fn to_rumbas(&self) -> std::collections::HashMap<K, S> {
        self.iter()
            .map(|(k, v)| (k.to_owned(), v.to_rumbas()))
            .collect()
    }
}

impl<K, L: ToRumbas<K>, S, O: ToRumbas<S>> ToRumbas<(K, S)> for (L, O) {
    fn to_rumbas(&self) -> (K, S) {
        (self.0.to_rumbas(), self.1.to_rumbas())
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

impl ToRumbas<JMEFileString> for String {
    fn to_rumbas(&self) -> JMEFileString {
        JMEFileString::s(self)
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
    pd: &numbas::question::part::QuestionPartSharedData,
) -> numbas::support::primitive::Number {
    pd.marks.clone()
}

pub fn extract_part_common_prompt(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> ContentAreaTranslatableString {
    pd.prompt.clone().to_rumbas()
}

pub fn extract_part_common_use_custom_name(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> bool {
    pd.use_custom_name
}

pub fn extract_part_common_custom_name(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> Noneable<String> {
    if pd.custom_name.is_empty() {
        Noneable::None
    } else {
        Noneable::NotNone(pd.custom_name.clone())
    }
}

pub fn extract_part_common_steps_penalty(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> Number {
    pd.steps_penalty.clone()
}

pub fn extract_part_common_show_correct_answer(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> bool {
    pd.show_correct_answer
}

pub fn extract_part_common_show_feedback_icon(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> bool {
    pd.show_feedback_icon
}

pub fn extract_part_common_steps(
    pd: &numbas::question::part::QuestionPartSharedData,
) -> Vec<QuestionPart> {
    pd.steps.to_rumbas()
}

/// Macro used to create a question part type for numbas
/// Usage: create_question_part! { PartName with &self.part_data => { field1: val1, field2, val2 }  }
macro_rules! create_question_part {
    (
        $type: ident with $part_data: expr => {
            $(
                $field: ident$(: $val: expr)?
            ),*
        }
    ) => {
        {
            let part_data = $part_data;
            $type {
                // Default section
                marks: extract_part_common_marks(&part_data).to_rumbas(),
                prompt: extract_part_common_prompt(&part_data),
                part_name: extract_part_common_custom_name(&part_data),
                steps_penalty: extract_part_common_steps_penalty(&part_data).to_rumbas(),
                show_correct_answer: extract_part_common_show_correct_answer(&part_data)
                    .to_rumbas(),
                show_feedback_icon: extract_part_common_show_feedback_icon(&part_data).to_rumbas(),
                adaptive_marking: part_data.adaptive_marking.to_rumbas(),
                custom_marking: part_data.custom_marking.to_rumbas(),
                steps: extract_part_common_steps(&part_data),
                $(
                    $field$(: $val)?
                ),*
            }
        }
    }
}
pub(crate) use create_question_part;

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
