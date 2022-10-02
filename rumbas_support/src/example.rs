use crate::value::{Value, ValueType};
use std::convert::Into;
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

impl_examples!(u8, u16, u32, u64, u128, usize: vec![2]);
impl_examples!(i8, i16, i32, i64, i128, isize: vec![-1]);

impl_examples!(f32, f64: vec![1.2]);

impl_examples!(String: vec!["nonjmetext§".to_string(),]);

impl_examples!(bool: vec![false]);

impl<T: Examples> Examples for ValueType<T> {
    fn examples() -> Vec<Self> {
        T::examples()
            .into_iter()
            .map(ValueType::Normal)
            .chain(
                vec![
                    ValueType::Template("template:template_key".to_string().try_into().unwrap()),
                    /*ValueType::TemplateWithDefault(crate::value::TemplateWithDefault {
                        template_key: "template_key".to_string(),
                        default_value: None, // TODO T::examples().get(0).map(|k| k.to_owned()),
                    }),*/
                ]
                .into_iter(),
            )
            .collect()
    }
}

#[cfg(test)]
mod valuetype_test {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Bool(bool);
    impl Examples for Bool {
        fn examples() -> Vec<Self> {
            vec![Bool(true), Bool(false)]
        }
    }

    #[test]
    fn valuetype_test() {
        let examples = Bool::examples();
        assert_eq!(
            ValueType::<Bool>::examples(),
            vec![
                ValueType::Normal(examples[0]),
                ValueType::Normal(examples[1]),
                ValueType::Template("template:template_key".to_string().try_into().unwrap())
            ]
        )
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

#[cfg(test)]
mod value_test {
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Usize(usize);
    impl Examples for Usize {
        fn examples() -> Vec<Self> {
            vec![Usize(0), Usize(1), Usize(5), Usize(10)]
        }
    }

    use super::*;
    #[test]
    fn value_test() {
        let valuetype_examples = ValueType::<Usize>::examples();
        assert_eq!(
            Value::<Usize>::examples(),
            vec![
                Value(Some(valuetype_examples[0].clone())),
                Value(Some(valuetype_examples[1].clone())),
                Value(Some(valuetype_examples[2].clone())),
                Value(Some(valuetype_examples[3].clone())),
                Value(Some(valuetype_examples[4].clone())),
                Value(None)
            ]
        )
    }
}

rumbas_support_derive::impl_examples_for_tuple!((A,));
rumbas_support_derive::impl_examples_for_tuple!((A, B));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F, G));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F, G, H));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F, G, H, I));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F, G, H, I, J));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F, G, H, I, J, K));
rumbas_support_derive::impl_examples_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L));

#[cfg(test)]
mod tuples_test {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Usize(usize);
    impl Examples for Usize {
        fn examples() -> Vec<Self> {
            vec![Usize(0), Usize(1), Usize(5), Usize(10)]
        }
    }
    #[derive(Clone, PartialEq, Debug)]
    struct Text(String);
    impl Examples for Text {
        fn examples() -> Vec<Self> {
            vec![
                Text("some text".to_string()),
                Text("other text".to_string()),
            ]
        }
    }

    #[test]
    fn tuple_test_one() {
        let usize_examples = Usize::examples();
        assert_eq!(
            <(Usize,)>::examples(),
            vec![
                (usize_examples[0],),
                (usize_examples[1],),
                (usize_examples[2],),
                (usize_examples[3],)
            ]
        )
    }
    #[test]
    fn tuple_test_two() {
        let usize_examples = Usize::examples();
        let string_examples = Text::examples();
        assert_eq!(
            <(Usize, Text)>::examples(),
            vec![
                (usize_examples[0], string_examples[0].clone()),
                (usize_examples[1], string_examples[1].clone()),
                (usize_examples[2], string_examples[0].clone()),
                (usize_examples[3], string_examples[1].clone())
            ]
        )
    }
    #[test]
    fn tuple_test_three() {
        let usize_examples = Usize::examples();
        let string_examples = Text::examples();
        assert_eq!(
            <(Usize, Text, Text)>::examples(),
            vec![
                (
                    usize_examples[0],
                    string_examples[0].clone(),
                    string_examples[0].clone()
                ),
                (
                    usize_examples[1],
                    string_examples[1].clone(),
                    string_examples[1].clone()
                ),
                (
                    usize_examples[2],
                    string_examples[0].clone(),
                    string_examples[0].clone()
                ),
                (
                    usize_examples[3],
                    string_examples[1].clone(),
                    string_examples[1].clone()
                )
            ]
        )
    }
}

fn convert_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl<A: Examples, const N: usize> Examples for [A; N] {
    fn examples() -> Vec<Self> {
        let mut examples_a = A::examples();

        // Make sure we have a multiple of N
        while examples_a.len() % N != 0 {
            for example in A::examples().into_iter() {
                examples_a.push(example);
                if examples_a.len() % N == 0 {
                    break;
                }
            }
        }

        let mut iterator = examples_a.into_iter();
        let mut result = Vec::new();
        loop {
            let mut parts: Vec<_> = Vec::new();
            for e in &mut iterator {
                parts.push(e);
                if parts.len() == N {
                    break;
                }
            }
            if parts.is_empty() {
                break;
            }
            result.push(convert_vec_to_array(parts))
        }
        result
    }
}

#[cfg(test)]
mod array_test {
    use super::*;
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Isize(isize);
    impl Examples for Isize {
        fn examples() -> Vec<Self> {
            vec![Isize(-1), Isize(1), Isize(-10), Isize(10)]
        }
    }
    #[test]
    fn array_test_one() {
        let isize_examples = Isize::examples();
        assert_eq!(
            <[Isize; 1]>::examples(),
            vec![
                [isize_examples[0]],
                [isize_examples[1]],
                [isize_examples[2]],
                [isize_examples[3]],
            ]
        )
    }
    #[test]
    fn array_test_two() {
        let isize_examples = Isize::examples();
        assert_eq!(
            <[Isize; 2]>::examples(),
            vec![
                [isize_examples[0], isize_examples[1]],
                [isize_examples[2], isize_examples[3]],
            ]
        )
    }
    #[test]
    fn array_test_three() {
        let isize_examples = Isize::examples();
        assert_eq!(
            <[Isize; 3]>::examples(),
            vec![
                [isize_examples[0], isize_examples[1], isize_examples[2]],
                [isize_examples[3], isize_examples[0], isize_examples[1]],
            ]
        )
    }
    #[test]
    fn array_test_four() {
        let isize_examples = Isize::examples();
        assert_eq!(
            <[Isize; 4]>::examples(),
            vec![[
                isize_examples[0],
                isize_examples[1],
                isize_examples[2],
                isize_examples[3]
            ],]
        )
    }
    #[test]
    fn array_test_five() {
        let isize_examples = Isize::examples();
        assert_eq!(
            <[Isize; 5]>::examples(),
            vec![[
                isize_examples[0],
                isize_examples[1],
                isize_examples[2],
                isize_examples[3],
                isize_examples[0]
            ],]
        )
    }
}

impl<A: Examples> Examples for Vec<A> {
    fn examples() -> Vec<Self> {
        vec![A::examples()]
    }
}

#[cfg(test)]
mod vec_test {
    use super::*;
    #[test]
    fn vec_test() {
        let f64_examples = f64::examples();
        assert_eq!(<Vec<f64>>::examples(), vec![f64_examples])
    }
}

impl<A: Examples + std::hash::Hash + std::cmp::Eq + Clone, B: Examples> Examples
    for std::collections::HashMap<A, B>
{
    fn examples() -> Vec<Self> {
        let examples_a = A::examples();
        let examples_b = B::examples();

        if examples_a.len() >= examples_b.len() {
            // We have enough keys
            vec![<(A, B)>::examples().into_iter().collect()]
        } else {
            let nb_maps: usize = if examples_b.len() % examples_a.len() == 0 {
                examples_b.len() / examples_a.len()
            } else {
                examples_b.len() / examples_a.len() + 1
            };
            let mut b = examples_b.into_iter();
            let mut maps = Vec::new();
            for _ in 0..nb_maps {
                let mut map = std::collections::HashMap::new();
                let examples_a = A::examples();
                for example_a in examples_a.into_iter() {
                    if let Some(value) = b.next() {
                        map.insert(example_a, value);
                    } else {
                        break;
                    }
                }
                maps.push(map);
            }
            maps
        }
    }
}

impl<A: Examples + std::cmp::Ord + Clone, B: Examples> Examples
    for std::collections::BTreeMap<A, B>
{
    fn examples() -> Vec<Self> {
        let examples_a = A::examples();
        let examples_b = B::examples();

        if examples_a.len() >= examples_b.len() {
            // We have enough keys
            vec![<(A, B)>::examples().into_iter().collect()]
        } else {
            let nb_maps: usize = if examples_b.len() % examples_a.len() == 0 {
                examples_b.len() / examples_a.len()
            } else {
                examples_b.len() / examples_a.len() + 1
            };
            let mut b = examples_b.into_iter();
            let mut maps = Vec::new();
            for _ in 0..nb_maps {
                let mut map = std::collections::BTreeMap::new();
                let examples_a = A::examples();
                for example_a in examples_a.into_iter() {
                    if let Some(value) = b.next() {
                        map.insert(example_a, value);
                    } else {
                        break;
                    }
                }
                maps.push(map);
            }
            maps
        }
    }
}

#[cfg(test)]
mod hashmap_test {
    use super::*;
    #[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
    struct Usize(usize);
    impl Examples for Usize {
        fn examples() -> Vec<Self> {
            vec![Usize(0), Usize(1), Usize(5), Usize(10)]
        }
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Usize2(usize);
    impl Examples for Usize2 {
        fn examples() -> Vec<Self> {
            vec![Usize2(0), Usize2(1), Usize2(5), Usize2(10), Usize2(15)]
        }
    }
    #[derive(Clone, PartialEq, Debug, Hash, Eq)]
    struct Text(String);
    impl Examples for Text {
        fn examples() -> Vec<Self> {
            vec![
                Text("some text".to_string()),
                Text("other text".to_string()),
            ]
        }
    }
    #[test]
    fn hashmap_test_exact_multiple_of_values() {
        let string_examples = Text::examples();
        let usize_examples = Usize::examples();
        assert_eq!(usize_examples.len() % string_examples.len(), 0);
        assert_eq!(
            <std::collections::HashMap<Text, Usize>>::examples(),
            vec![
                vec![
                    (string_examples[0].clone(), usize_examples[0]),
                    (string_examples[1].clone(), usize_examples[1])
                ]
                .into_iter()
                .collect(),
                vec![
                    (string_examples[0].clone(), usize_examples[2]),
                    (string_examples[1].clone(), usize_examples[3])
                ]
                .into_iter()
                .collect()
            ]
        )
    }
    #[test]
    fn hashmap_test_less_values() {
        let usize_examples = Usize::examples();
        let string_examples = Text::examples();
        assert!(usize_examples.len() > string_examples.len());
        assert_eq!(
            <std::collections::HashMap<Usize, Text>>::examples(),
            vec![vec![
                (usize_examples[0], string_examples[0].clone()),
                (usize_examples[1], string_examples[1].clone()),
                (usize_examples[2], string_examples[0].clone()),
                (usize_examples[3], string_examples[1].clone())
            ]
            .into_iter()
            .collect()]
        )
    }
    #[test]
    fn hashmap_test_non_exact_multiple_of_values() {
        let string_examples = Text::examples();
        let usize_examples = Usize2::examples();
        assert!(usize_examples.len() % string_examples.len() != 0);
        assert!(usize_examples.len() > string_examples.len());
        assert_eq!(
            <std::collections::HashMap<Text, Usize2>>::examples(),
            vec![
                vec![
                    (string_examples[0].clone(), usize_examples[0]),
                    (string_examples[1].clone(), usize_examples[1])
                ]
                .into_iter()
                .collect(),
                vec![
                    (string_examples[0].clone(), usize_examples[2]),
                    (string_examples[1].clone(), usize_examples[3])
                ]
                .into_iter()
                .collect(),
                vec![(string_examples[0].clone(), usize_examples[4]),]
                    .into_iter()
                    .collect()
            ]
        )
    }
}

impl<T: Examples> Examples for Box<T> {
    fn examples() -> Vec<Self> {
        T::examples().into_iter().map(Box::new).collect()
    }
}

#[cfg(test)]
mod box_test {
    use super::*;
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Bool(bool);
    impl Examples for Bool {
        fn examples() -> Vec<Self> {
            vec![Bool(true), Bool(false)]
        }
    }
    #[test]
    fn box_test() {
        let examples = Bool::examples();
        assert_eq!(
            Box::<Bool>::examples(),
            vec![Box::new(examples[0]), Box::new(examples[1]),]
        )
    }
}

impl_examples!(numbas::jme::JMEString: vec!["x^5".to_string().try_into().unwrap()]);
impl_examples!(numbas::support::primitive::Number: vec![1usize.into(), 1.5.into()]);
