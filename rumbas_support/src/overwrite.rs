use crate::input::Input;
use crate::value::{Value, ValueType};
use std::collections::HashMap;

pub trait Overwrite<Item>: Clone + Input {
    fn overwrite(&mut self, other: &Item);
}

impl<O: Overwrite<O>> Overwrite<Vec<O>> for Vec<O> {
    fn overwrite(&mut self, _other: &Vec<O>) {}
}

impl<T: Overwrite<T>> Overwrite<HashMap<String, T>> for HashMap<String, T> {
    fn overwrite(&mut self, _other: &HashMap<String, T>) {}
}

impl<T: Overwrite<T>> Overwrite<Box<T>> for Box<T> {
    fn overwrite(&mut self, other: &Box<T>) {
        (**self).overwrite(&*other)
    }
}

impl<T: Overwrite<T>> Overwrite<ValueType<T>> for ValueType<T>
where
    T: serde::de::DeserializeOwned,
{
    fn overwrite(&mut self, other: &ValueType<T>) {
        if let ValueType::Normal(ref mut val) = self {
            if let ValueType::Normal(other_val) = &other {
                val.overwrite(other_val);
            }
        }
    }
}

impl<T: Overwrite<T>> Overwrite<Value<T>> for Value<T>
where
    T: serde::de::DeserializeOwned,
{
    fn overwrite(&mut self, other: &Value<T>) {
        if let Some(ref mut val) = self.0 {
            if let Some(other_val) = &other.0 {
                val.overwrite(other_val);
            }
        } else if self.0.is_none() {
            *self = other.clone();
        }
    }
}

impl<A: Overwrite<A>, B: Overwrite<B>> Overwrite<(A, B)> for (A, B) {
    fn overwrite(&mut self, other: &(A, B)) {
        self.0.overwrite(&other.0);
        self.1.overwrite(&other.1);
    }
}

macro_rules! impl_overwrite {
    ($($t: ty),*) => {
        $(
        impl Overwrite<$t> for $t {
            fn overwrite(&mut self, _other: &$t){}

        }
        )*
    };
}

impl_overwrite!(String);
impl_overwrite!(f64, f32, [f64; 2]);
impl_overwrite!(u128, u64, u32, u16, u8, usize);
impl_overwrite!(i128, i64, i32, i16, i8, isize);
impl_overwrite!(bool);

impl_overwrite!(std::path::PathBuf);

impl_overwrite!(numbas::jme::ContentAreaString);
impl_overwrite!(numbas::jme::EmbracedJMEString);
impl_overwrite!(numbas::jme::JMEString);
impl_overwrite!(numbas::question::part::match_answers::MatchAnswersWithChoicesLayout);
impl_overwrite!(numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType);
impl_overwrite!(numbas::question::part::match_answers::MultipleChoiceWarningType);
impl_overwrite!(numbas::question::part::pattern_match::PatternMatchMode);
impl_overwrite!(numbas::support::answer_style::AnswerStyle);
impl_overwrite!(numbas::question::function::FunctionType);
impl_overwrite!(numbas::question::custom_part_type::CustomPartTypeSetting);
impl_overwrite!(numbas::support::primitive::Number);
