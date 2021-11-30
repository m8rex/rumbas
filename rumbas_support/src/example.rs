use crate::value::{Value, ValueType};
use std::convert::TryInto;

pub trait Examples {
    /// Method that return a list of examples for the given structs
    fn examples() -> Vec<Self>
    where
        Self: Sized;
}

macro_rules! impl_examples {
    ($($type: ty),+ : $res: expr) => {
        $(
        impl Examples for $type {
            fn examples() -> Vec<Self> {
                $res
            }
        }
        )+
    };
}

impl_examples!(u8, u16, u32, u64, u128, usize: vec![0, 1, 2, 10]);

impl_examples!(f32, f64: vec![0.4, 0.8, 1.0, 10.5]);

impl_examples!(String: vec!["text".to_string(), "other text".to_string()]);

impl<T: Examples> Examples for ValueType<T> {
    fn examples() -> Vec<Self> {
        T::examples()
            .into_iter()
            .map(|e| ValueType::Normal(e))
            .chain(
                vec![ValueType::Template(
                    "template:template_key".to_string().try_into().unwrap(),
                )]
                .into_iter(),
            )
            .collect()
    }
}

impl<T: Examples> Examples for Value<T> {
    fn examples() -> Vec<Self> {
        ValueType::<T>::examples()
            .into_iter()
            .map(|e| Value(Some(e)))
            .chain(vec![Value(None)].into_iter())
            .collect()
    }
}
