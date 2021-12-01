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
impl_examples!(i8, i16, i32, i64, i128, isize: vec![-1, 0, 1, 10]);

impl_examples!(f32, f64: vec![-0.4, 0.4, -1.0, 1.0, 10.5]);

impl_examples!(String: vec!["text".to_string(), "other text".to_string()]);

impl_examples!(bool: vec![true, false]);

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

#[cfg(test)]
mod valuetype_test {
    use super::*;
    #[test]
    fn valuetype_test() {
        let examples = bool::examples();
        assert_eq!(
            ValueType::<bool>::examples(),
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
    use super::*;
    #[test]
    fn value_test() {
        let valuetype_examples = ValueType::<usize>::examples();
        assert_eq!(
            Value::<usize>::examples(),
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

// TODO: macro
impl<A: Examples, B: Examples, C: Examples> Examples for (A, B, C) {
    fn examples() -> Vec<Self> {
        let mut examples_a = A::examples();
        let mut examples_b = B::examples();
        let mut examples_c = C::examples();

        let mut max_examples = 0;
        max_examples = std::cmp::max(max_examples, examples_a.len());
        max_examples = std::cmp::max(max_examples, examples_b.len());
        max_examples = std::cmp::max(max_examples, examples_c.len());

        // Make sure that all elements in the longer vector have corresponding items in the shorter
        // one
        while examples_a.len() < max_examples {
            examples_a.extend(A::examples().into_iter());
        }
        while examples_b.len() < max_examples {
            examples_b.extend(B::examples().into_iter());
        }
        while examples_c.len() < max_examples {
            examples_c.extend(C::examples().into_iter());
        }

        let mut iterator_a = examples_a.into_iter();
        let mut iterator_b = examples_b.into_iter();
        let mut iterator_c = examples_c.into_iter();

        let mut result = Vec::new();
        loop {
            let a_opt = iterator_a.next();
            if a_opt.is_none() {
                break;
            }
            let b_opt = iterator_b.next();
            if b_opt.is_none() {
                break;
            }
            let c_opt = iterator_c.next();
            if c_opt.is_none() {
                break;
            }
            result.push((a_opt.unwrap(), b_opt.unwrap(), c_opt.unwrap()))
        }
        result
    }
}

impl<A: Examples, B: Examples> Examples for (A, B) {
    fn examples() -> Vec<Self> {
        let mut examples_a = A::examples();
        let mut examples_b = B::examples();

        // Make sure that all elements in the longer vector have corresponding items in the shorter
        // one
        if examples_a.len() > examples_b.len() {
            while examples_a.len() > examples_b.len() {
                examples_b.extend(B::examples().into_iter());
            }
        } else if examples_a.len() < examples_b.len() {
            while examples_a.len() < examples_b.len() {
                examples_a.extend(A::examples().into_iter());
            }
        }

        examples_a.into_iter().zip(examples_b.into_iter()).collect()
    }
}

#[cfg(test)]
mod tuples_test {
    use super::*;
    #[test]
    fn tuple_test() {
        let usize_examples = usize::examples();
        let string_examples = String::examples();
        assert_eq!(
            <(usize, String)>::examples(),
            vec![
                (usize_examples[0], string_examples[0].clone()),
                (usize_examples[1], string_examples[1].clone()),
                (usize_examples[2], string_examples[0].clone()),
                (usize_examples[3], string_examples[1].clone())
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
            if parts.len() == 0 {
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
    #[test]
    fn array_test_one() {
        let isize_examples = isize::examples();
        assert_eq!(
            <[isize; 1]>::examples(),
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
        let isize_examples = isize::examples();
        assert_eq!(
            <[isize; 2]>::examples(),
            vec![
                [isize_examples[0], isize_examples[1]],
                [isize_examples[2], isize_examples[3]],
            ]
        )
    }
    #[test]
    fn array_test_three() {
        let isize_examples = isize::examples();
        assert_eq!(
            <[isize; 3]>::examples(),
            vec![
                [isize_examples[0], isize_examples[1], isize_examples[2]],
                [isize_examples[3], isize_examples[0], isize_examples[1]],
            ]
        )
    }
    #[test]
    fn array_test_four() {
        let isize_examples = isize::examples();
        assert_eq!(
            <[isize; 4]>::examples(),
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
        let isize_examples = isize::examples();
        assert_eq!(
            <[isize; 5]>::examples(),
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
                for i in 0..examples_a.len() {
                    if let Some(value) = b.next() {
                        map.insert(examples_a[i].clone(), value);
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
    #[test]
    fn hashmap_test_exact_multiple_of_values() {
        let string_examples = String::examples();
        let usize_examples = usize::examples();
        assert_eq!(usize_examples.len() % string_examples.len(), 0);
        assert_eq!(
            <std::collections::HashMap<String, usize>>::examples(),
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
        let usize_examples = usize::examples();
        let string_examples = String::examples();
        assert!(usize_examples.len() > string_examples.len());
        assert_eq!(
            <std::collections::HashMap<usize, String>>::examples(),
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
        let string_examples = String::examples();
        let f32_examples = f32::examples();
        assert!(f32_examples.len() % string_examples.len() != 0);
        assert!(f32_examples.len() > string_examples.len());
        assert_eq!(
            <std::collections::HashMap<String, f32>>::examples(),
            vec![
                vec![
                    (string_examples[0].clone(), f32_examples[0]),
                    (string_examples[1].clone(), f32_examples[1])
                ]
                .into_iter()
                .collect(),
                vec![
                    (string_examples[0].clone(), f32_examples[2]),
                    (string_examples[1].clone(), f32_examples[3])
                ]
                .into_iter()
                .collect(),
                vec![(string_examples[0].clone(), f32_examples[4]),]
                    .into_iter()
                    .collect()
            ]
        )
    }
}

impl<T: Examples> Examples for Box<T> {
    fn examples() -> Vec<Self> {
        T::examples().into_iter().map(|e| Box::new(e)).collect()
    }
}

#[cfg(test)]
mod box_test {
    use super::*;
    #[test]
    fn box_test() {
        let examples = bool::examples();
        assert_eq!(
            Box::<bool>::examples(),
            vec![Box::new(examples[0]), Box::new(examples[1]),]
        )
    }
}
