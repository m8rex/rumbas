use crate::input::Input;
use crate::value::{Value, ValueType};
use serde::de::DeserializeOwned;
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

impl<T: Overwrite<T>> Overwrite<Value<T>> for Value<T>
where
    T: serde::de::DeserializeOwned,
{
    fn overwrite(&mut self, other: &Value<T>) {
        if let Some(ValueType::Normal(ref mut val)) = self.0 {
            if let Some(ValueType::Normal(other_val)) = &other.0 {
                val.overwrite(&other_val);
            }
        } else if self.0.is_none() {
            *self = other.clone();
        }
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
impl_overwrite!(f64, f32);
impl_overwrite!(u128, u64, u32, u16, u8, usize);
impl_overwrite!(i128, i64, i32, i16, i8, isize);
impl_overwrite!(bool);
