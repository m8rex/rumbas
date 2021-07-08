use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

pub trait EmptyFields {
    fn empty_fields(&self) -> Vec<String>;
}

pub trait OptionalOverwrite<Item>: Clone + DeserializeOwned + EmptyFields {
    fn overwrite(&mut self, other: &Item);
    fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
//TODO: improve, all strings (not only none are seen as empty)
pub enum Noneable<T> {
    None(String),
    NotNone(T),
}

macro_rules! impl_optional_overwrite_value_only {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl$(< $($gen: EmptyFields ),* >)? EmptyFields for Value<$type> {
            fn empty_fields(&self) -> Vec<String> {
                if let Some(ValueType::Normal(val)) = &self.0 {
                    val.empty_fields()
                }
                else if let Some(ValueType::Template(ts)) = &self.0 {
                    vec![ts.yaml()]
                }
                else {
                    vec!["".to_string()]
                }
            }
        }
        impl$(< $($gen: OptionalOverwrite<$gen> + DeserializeOwned ),* >)? OptionalOverwrite<Value<$type>> for Value<$type> {
            fn overwrite(&mut self, other: &Value<$type>) {
                if let Some(ValueType::Normal(ref mut val)) = self.0 {
                    if let Some(ValueType::Normal(other_val)) = &other.0 {
                        val.overwrite(&other_val);
                    }
                } else if self.0.is_none() {
                    *self = other.clone();
                }
            }
            fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value){
                if let Some(ValueType::Template(ts)) = &self.0 {
                    if ts.key == Some(key.clone()) {
                        *self=Value::Normal(serde_yaml::from_value(val.clone()).unwrap());
                    }
                } else if let Some(ValueType::Normal(ref mut v)) = &mut self.0 {
                    v.insert_template_value(key, val);
                }
            }
        }

        )*
    };
}

macro_rules! impl_optional_overwrite_value {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl_optional_overwrite_value_only!($type$([ $($gen),* ])?);
        impl$(< $($gen: EmptyFields ),* >)? EmptyFields for Noneable<$type> {
            fn empty_fields(&self) -> Vec<String> {
                if let Noneable::NotNone(val) = &self {
                    return val.empty_fields()
                }
                else {
                    return vec![]
                }
            }
        }
        impl$(< $($gen: OptionalOverwrite<$gen> ),* >)? OptionalOverwrite<Noneable<$type>> for Noneable<$type> {
            fn overwrite(&mut self, other: &Noneable<$type>) {
                if let Noneable::NotNone(ref mut val) = self {
                    if let Noneable::NotNone(other_val) = &other {
                        val.overwrite(&other_val);
                    }
                } else {
                    // Do nothing, none is a valid value
                }
            }
            fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value){
                if let Noneable::NotNone(item) = self {
                    item.insert_template_value(&key, &val);
                }
            }
        }
        impl_optional_overwrite_value_only!(Noneable<$type>$([ $($gen),* ])?);
        )*
    };
}

impl<O: EmptyFields> EmptyFields for Vec<O> {
    fn empty_fields(&self) -> Vec<String> {
        let mut empty = Vec::new();
        for (i, item) in self.iter().enumerate() {
            let extra_empty = item.empty_fields();
            for extra in extra_empty.iter() {
                empty.push(format!("{}.{}", i, extra));
            }
        }
        empty
    }
}
impl<O: OptionalOverwrite<O>> OptionalOverwrite<Vec<O>> for Vec<O> {
    fn overwrite(&mut self, _other: &Vec<O>) {}
    fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value) {
        for (_i, item) in self.iter_mut().enumerate() {
            item.insert_template_value(&key, &val);
        }
    }
}
impl_optional_overwrite_value!(Vec<U>[U]);

macro_rules! impl_optional_overwrite {
    ($($type: ty), *) => {
        $(
        impl EmptyFields for $type {
            fn empty_fields(&self) -> Vec<String> {
                Vec::new()
            }
        }
        impl OptionalOverwrite<$type> for $type {
            fn overwrite(&mut self, _other: &$type) {}
            fn insert_template_value(&mut self, _key: &String, _val: &serde_yaml::Value) {}
        }
        impl_optional_overwrite_value!($type);
        )*
    };
}
impl_optional_overwrite!(String, bool, f64, usize, [f64; 2]);

impl<T: EmptyFields> EmptyFields for HashMap<String, T> {
    fn empty_fields(&self) -> Vec<String> {
        let mut empty = Vec::new();
        // Key is not displayable, so show an index, just to differentiate
        for (i, (_key, item)) in self.iter().enumerate() {
            let extra_empty = item.empty_fields();
            for extra in extra_empty.iter() {
                empty.push(format!("{}.{}", i, extra));
            }
        }
        empty
    }
}
impl<T: OptionalOverwrite<T>> OptionalOverwrite<HashMap<String, T>> for HashMap<String, T> {
    fn overwrite(&mut self, _other: &HashMap<String, T>) {}
    fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value) {
        for (_i, (_key, item)) in self.iter_mut().enumerate() {
            item.insert_template_value(&key, &val);
        }
    }
}
impl_optional_overwrite_value!(HashMap < String, T > [T]);

macro_rules! optional_overwrite {
    // This macro creates a struct with all optional fields
    // It also adds a method to overwrite all fields with None value with the values of another object of the same type
    // It also adds a method to list the fields that are None
    (
        $(#[$outer:meta])*
        pub struct $struct: ident {
            $(
                $(#[$inner:meta])*
                $field: ident: $type: ty
            ),+
        }
    ) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        $(
            #[$outer]
        )*
        pub struct $struct {
            $(
                $(#[$inner])*
                pub $field: Value<$type>
            ),*
        }
        impl EmptyFields for $struct {
            fn empty_fields(&self) -> Vec<String> {
                let mut empty = Vec::new();
                $(
                    let extra_empty = &self.$field.empty_fields();
                    if extra_empty.len() == 1 && extra_empty[0] == "" {
                        empty.push(stringify!($field).to_string());
                    }
                    else {
                        for extra in extra_empty.iter() {
                            empty.push(format!("{}.{}", stringify!($field), extra));
                        }
                    }
                )*
                empty
            }
        }
        impl OptionalOverwrite<$struct> for $struct {
            fn overwrite(&mut self, other: &$struct) {
                $(
                    self.$field.overwrite(&other.$field);
                )*
            }
            fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value){
                $(
                    self.$field.insert_template_value(&key, &val);
                )*
            }
        }

        impl std::convert::From<$struct> for Value<$struct> {
            fn from(val: $struct) -> Self {
                Value::Normal(val)
            }
        }
        impl_optional_overwrite_value!($struct);
    }
}

macro_rules! optional_overwrite_enum {
    (

        $(#[$outer:meta])*
        pub enum $enum: ident {
            $(
                $(#[$inner:meta])*
                $field: ident($type: ty)
            ),+
        }
    ) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        $(
            #[$outer]
        )*
        pub enum $enum {
            $(
                $(
                    #[$inner]
                )*
                $field($type)
            ),*
        }
        impl EmptyFields for $enum {
            fn empty_fields(&self) -> Vec<String> {
                match self {
                $(
                    $enum::$field(val) => val.empty_fields()
                ),*
                }
            }
        }
        impl OptionalOverwrite<$enum> for $enum {
            fn overwrite(&mut self, other: &$enum) {
                match (self, other) {
                $(
                    (&mut $enum::$field(ref mut val), &$enum::$field(ref valo)) => val.overwrite(&valo)
                ),*
                    , _ => ()
                };
            }
            fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value){
                match self {
                $(
                    &mut $enum::$field(ref mut enum_val) => enum_val.insert_template_value(&key, &val)
                ),*
                };
            }
        }
        impl_optional_overwrite_value!($enum);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;
    optional_overwrite! {
        pub struct Temp {
        name: String,
        test: String
        }
    }

    optional_overwrite! {
        pub struct Temp2 {
        other: String,
        t: Temp
        }
    }
    //TODO: template
    #[test]
    fn empty_fields_simple_structs() {
        let t = Temp {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        assert_eq!(t.empty_fields(), vec!["test"]);
        let t = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        assert_eq!(t.empty_fields().len(), 0);
        let t = Temp {
            name: Value::None(),
            test: Value::None(),
        };
        assert_eq!(t.empty_fields(), vec!["name", "test"]);
    }

    #[test]
    fn empty_fields_complex_structs() {
        let t = Temp2 {
            other: Value::Normal("val".to_string()),
            t: Value::Normal(Temp {
                name: Value::Normal("val".to_string()),
                test: Value::Normal("name".to_string()),
            }),
        };
        assert_eq!(t.empty_fields().len(), 0);
        let t = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t.name"]);
        let t = Temp2 {
            other: Value::None(),
            t: Value::None(),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t"]);
        let t = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::None(),
            }),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t.name", "t.test"]);
    }

    #[test]
    fn overwrite_simple_structs() {
        let mut t = Temp {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        let t2 = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        t.overwrite(&t2);
        assert_eq!(
            t,
            Temp {
                name: t.clone().name,
                test: t2.clone().test,
            }
        );
    }

    #[test]
    fn overwrite_nested_structs() {
        let t3 = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let mut t4 = Temp2 {
            other: Value::None(),
            t: Value::None(),
        };
        t4.overwrite(&t3);
        assert_eq!(
            t4,
            Temp2 {
                other: Value::None(),
                t: t3.clone().t
            }
        );
        let t5 = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::Normal("test".to_string()),
                test: Value::Normal("name2".to_string()),
            }),
        };
        t4.overwrite(&t5);
        assert_eq!(
            t4,
            Temp2 {
                other: Value::None(),
                t: Value::Normal(Temp {
                    name: t5.t.unwrap().name,
                    test: t3.t.unwrap().test
                }),
            }
        );
    }

    #[test]
    fn empty_fields_vec_of_simple_structs() {
        let t1 = Temp {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        let t2 = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        let t3 = Temp {
            name: Value::None(),
            test: Value::None(),
        };
        let v = vec![t1, t2, t3];
        assert_eq!(v.empty_fields(), vec!["0.test", "2.name", "2.test"]);
    }

    #[test]
    fn empty_fields_vec_ofcomplex_structs() {
        let t1 = Temp2 {
            other: Value::Normal("val".to_string()),
            t: Value::Normal(Temp {
                name: Value::Normal("val".to_string()),
                test: Value::Normal("name".to_string()),
            }),
        };
        let t2 = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let t3 = Temp2 {
            other: Value::None(),
            t: Value::None(),
        };
        let t4 = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::None(),
            }),
        };
        let v = vec![t1, t2, t3, t4];
        assert_eq!(
            v.empty_fields(),
            vec!["1.other", "1.t.name", "2.other", "2.t", "3.other", "3.t.name", "3.t.test"]
        );
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(String),
    Value(T),
}

impl<T: EmptyFields> EmptyFields for VariableValued<T> {
    fn empty_fields(&self) -> Vec<String> {
        match self {
            VariableValued::Variable(s) => s.empty_fields(),
            VariableValued::Value(v) => v.empty_fields(),
        }
    }
}
impl<T: OptionalOverwrite<T> + DeserializeOwned> OptionalOverwrite<VariableValued<T>>
    for VariableValued<T>
{
    fn overwrite(&mut self, other: &VariableValued<T>) {
        match (self, other) {
            (&mut VariableValued::Variable(ref mut val), &VariableValued::Variable(ref valo)) => {
                val.overwrite(&valo)
            }
            (&mut VariableValued::Value(ref mut val), &VariableValued::Value(ref valo)) => {
                val.overwrite(&valo)
            }
            _ => (),
        };
    }
    fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value) {
        match self {
            &mut VariableValued::Variable(ref mut s) => s.insert_template_value(&key, &val),
            &mut VariableValued::Value(ref mut v) => v.insert_template_value(&key, &val),
        };
    }
}
impl_optional_overwrite_value!(VariableValued<T>[T]);

impl<T: ToNumbas + EmptyFields> ToNumbas for VariableValued<T> {
    type NumbasType = numbas::exam::VariableValued<T::NumbasType>;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(match self {
                VariableValued::Variable(v) => numbas::exam::VariableValued::Variable(v.clone()),
                VariableValued::Value(v) => {
                    numbas::exam::VariableValued::Value(v.to_numbas(locale).unwrap())
                }
            })
        } else {
            Err(empty_fields)
        }
    }
}

impl<T> VariableValued<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> VariableValued<U> {
        match self {
            VariableValued::Variable(x) => VariableValued::Variable(x),
            VariableValued::Value(x) => VariableValued::Value(f(x)),
        }
    }
}

impl<T: ToRumbas> ToRumbas for numbas::exam::VariableValued<T> {
    type RumbasType = VariableValued<T::RumbasType>;
    fn to_rumbas(&self) -> Self::RumbasType {
        match self {
            numbas::exam::VariableValued::Variable(v) => VariableValued::Variable(v.clone()),
            numbas::exam::VariableValued::Value(v) => VariableValued::Value(v.to_rumbas()),
        }
    }
}
