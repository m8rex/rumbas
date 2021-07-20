use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckPath {
    parts: Vec<String>,
    last_part: Option<String>,
}

impl RumbasCheckPath {
    pub fn with_last(os: Option<String>) -> Self {
        RumbasCheckPath {
            parts: vec![],
            last_part: os,
        }
    }
    pub fn without_last() -> Self {
        Self::with_last(None)
    }
    pub fn add(&mut self, s: String) {
        self.parts.insert(0, s)
    }
}

impl std::fmt::Display for RumbasCheckPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let base = self.parts.join(".");
        write!(
            f,
            "{}",
            if let Some(ref e) = self.last_part {
                format!("{}.{}", base, e)
            } else {
                base
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckMissingData {
    path: RumbasCheckPath,
}

impl std::fmt::Display for RumbasCheckMissingData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckInvalidData {
    path: RumbasCheckPath,
    data: serde_yaml::Value,
}

impl std::fmt::Display for RumbasCheckInvalidData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p = self.path.to_string();
        write!(
            f,
            "{}",
            if let Ok(s) = serde_yaml::to_string(&self.data) {
                format!("{}\n With yaml:\n{}", p, s)
            } else {
                p
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckResult {
    // When adding a field, do also add it to is_empty
    missing_values: Vec<RumbasCheckMissingData>,
    invalid_values: Vec<RumbasCheckInvalidData>,
}

impl RumbasCheckResult {
    pub fn from_missing(os: Option<String>) -> RumbasCheckResult {
        RumbasCheckResult {
            missing_values: vec![RumbasCheckMissingData {
                path: RumbasCheckPath::with_last(os),
            }],
            invalid_values: vec![],
        }
    }
    pub fn from_invalid(v: &serde_yaml::Value) -> RumbasCheckResult {
        RumbasCheckResult {
            missing_values: vec![],
            invalid_values: vec![RumbasCheckInvalidData {
                path: RumbasCheckPath::without_last(),
                data: v.clone(),
            }],
        }
    }
    pub fn empty() -> RumbasCheckResult {
        RumbasCheckResult {
            missing_values: vec![],
            invalid_values: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.missing_values.len() == 0 && self.invalid_values.len() == 0
    }
    pub fn extend_path(&mut self, s: String) {
        for missing_value in self.missing_values.iter_mut() {
            missing_value.path.add(s.clone());
        }
        for invalid_value in self.invalid_values.iter_mut() {
            invalid_value.path.add(s.clone());
        }
    }
    pub fn union(&mut self, other: &Self) {
        self.missing_values.extend(other.missing_values.clone());
        self.invalid_values.extend(other.invalid_values.clone());
    }
    pub fn missing_fields(&self) -> Vec<RumbasCheckMissingData> {
        self.missing_values.clone()
    }
    pub fn invalid_fields(&self) -> Vec<RumbasCheckInvalidData> {
        self.invalid_values.clone()
    }
}

pub trait RumbasCheck {
    /// Check the read rumbas data
    fn check(&self) -> RumbasCheckResult;
}

pub trait OptionalOverwrite<Item>: Clone + DeserializeOwned + RumbasCheck {
    fn overwrite(&mut self, other: &Item);
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
//TODO: improve, all strings (not only none are seen as empty)
pub enum Noneable<T> {
    None(String),
    NotNone(T),
}

impl<T> Noneable<T> {
    // Create a None with string "none"
    pub fn nn() -> Self {
        Self::None("none".to_string())
    }
}

macro_rules! impl_optional_overwrite_value_only {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl$(< $($gen: RumbasCheck ),* >)? RumbasCheck for Value<$type> {
            fn check(&self) -> RumbasCheckResult {
                match &self.0 {
                    Some(ValueType::Normal(val)) => {
                        val.check()
                    },
                    Some(ValueType::Template(ts)) => {
                        RumbasCheckResult::from_missing(Some(ts.yaml()))
                    },
                    Some(ValueType::Invalid(v)) => { // TODO: report
                        RumbasCheckResult::from_invalid(v)
                    },
                        None => { RumbasCheckResult::from_missing(None)
                    }
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
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                if let Some(ValueType::Template(ts)) = &self.0 {
                    if ts.key == Some(key.to_string()) {
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
        impl$(< $($gen: RumbasCheck ),* >)? RumbasCheck for Noneable<$type> {
            fn check(&self) -> RumbasCheckResult {
                if let Noneable::NotNone(val) = &self {
                    val.check()
                }
                else {
                    RumbasCheckResult::empty()
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
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                if let Noneable::NotNone(item) = self {
                    item.insert_template_value(&key, &val);
                }
            }
        }
        impl_optional_overwrite_value_only!(Noneable<$type>$([ $($gen),* ])?);
        )*
    };
}

impl<O: RumbasCheck> RumbasCheck for Vec<O> {
    fn check(&self) -> RumbasCheckResult {
        let mut result = RumbasCheckResult::empty();
        for (i, item) in self.iter().enumerate() {
            let mut previous_result = item.check();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }
}
impl<O: OptionalOverwrite<O>> OptionalOverwrite<Vec<O>> for Vec<O> {
    fn overwrite(&mut self, _other: &Vec<O>) {}
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_i, item) in self.iter_mut().enumerate() {
            item.insert_template_value(key, val);
        }
    }
}
impl_optional_overwrite_value!(Vec<U>[U]);

macro_rules! impl_optional_overwrite {
    ($($type: ty), *) => {
        $(
        impl RumbasCheck for $type {
            fn check(&self) -> RumbasCheckResult {
                RumbasCheckResult::empty()
            }
        }
        impl OptionalOverwrite<$type> for $type {
            fn overwrite(&mut self, _other: &$type) {}
            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
        }
        impl_optional_overwrite_value!($type);
        )*
    };
}
impl_optional_overwrite!(String, bool, f64, usize, [f64; 2]);

impl<T: RumbasCheck> RumbasCheck for HashMap<String, T> {
    fn check(&self) -> RumbasCheckResult {
        let mut result = RumbasCheckResult::empty();
        // Key is not displayable, so show an index, just to differentiate
        for (i, (_key, item)) in self.iter().enumerate() {
            let mut previous_result = item.check();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }
}
impl<T: OptionalOverwrite<T>> OptionalOverwrite<HashMap<String, T>> for HashMap<String, T> {
    fn overwrite(&mut self, _other: &HashMap<String, T>) {}
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_i, (_key, item)) in self.iter_mut().enumerate() {
            item.insert_template_value(key, val);
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
        impl RumbasCheck for $struct {
            fn check(&self) -> RumbasCheckResult {
                let mut result = RumbasCheckResult::empty();
                $(
                    {
                    let mut previous_result = self.$field.check();
                    previous_result.extend_path(stringify!($field).to_string());
                    result.union(&previous_result);
                    }
                )*
                result
            }
        }
        impl OptionalOverwrite<$struct> for $struct {
            fn overwrite(&mut self, other: &$struct) {
                $(
                    self.$field.overwrite(&other.$field);
                )*
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
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
        impl RumbasCheck for $enum {
            fn check(&self) -> RumbasCheckResult {
                match self {
                $(
                    $enum::$field(val) => val.check() // TODO: stringify field or something? (to better show what failed)
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
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
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
    fn check_simple_structs() {
        let t = Temp {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        let check = t.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["test"],
        );
        assert_eq!(check.invalid_values.len(), 0);
        let t = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        assert_eq!(t.check().is_empty(), true);
        let t = Temp {
            name: Value::None(),
            test: Value::None(),
        };
        let check = t.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["name", "test"],
        );
        assert_eq!(check.invalid_values.len(), 0);
    }

    #[test]
    fn check_complex_structs() {
        let t = Temp2 {
            other: Value::Normal("val".to_string()),
            t: Value::Normal(Temp {
                name: Value::Normal("val".to_string()),
                test: Value::Normal("name".to_string()),
            }),
        };
        assert_eq!(t.check().is_empty(), true);
        let t = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let check = t.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t.name"],
        );
        assert_eq!(check.invalid_values.is_empty(), true);
        let t = Temp2 {
            other: Value::None(),
            t: Value::None(),
        };
        let check = t.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t"],
        );
        assert_eq!(check.invalid_values.len(), 0);
        let t = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::None(),
            }),
        };
        let check = t.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t.name", "t.test"],
        );
        assert_eq!(check.invalid_values.len(), 0);
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
                test: t2.test,
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
    fn check_vec_of_simple_structs() {
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
        let check = v.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["0.test", "2.name", "2.test"],
        );
        assert_eq!(check.invalid_values.len(), 0);
    }

    #[test]
    fn check_vec_ofcomplex_structs() {
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
        let check = v.check();
        assert_eq!(
            check
                .missing_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["1.other", "1.t.name", "2.other", "2.t", "3.other", "3.t.name", "3.t.test"]
        );
        assert_eq!(check.invalid_values.len(), 0);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(String),
    Value(T),
}

impl<T: RumbasCheck> RumbasCheck for VariableValued<T> {
    fn check(&self) -> RumbasCheckResult {
        match self {
            VariableValued::Variable(s) => s.check(),
            VariableValued::Value(v) => v.check(),
        }
    }
}
impl<T: OptionalOverwrite<T> + DeserializeOwned> OptionalOverwrite<VariableValued<T>>
    for VariableValued<T>
{
    fn overwrite(&mut self, other: &VariableValued<T>) {
        match (self, other) {
            (&mut VariableValued::Variable(ref mut val), &VariableValued::Variable(ref valo)) => {
                val.overwrite(valo)
            }
            (&mut VariableValued::Value(ref mut val), &VariableValued::Value(ref valo)) => {
                val.overwrite(valo)
            }
            _ => (),
        };
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match *self {
            VariableValued::Variable(ref mut s) => s.insert_template_value(key, val),
            VariableValued::Value(ref mut v) => v.insert_template_value(key, val),
        };
    }
}
impl_optional_overwrite_value!(VariableValued<T>[T]);

impl<T: ToNumbas + RumbasCheck> ToNumbas for VariableValued<T> {
    type NumbasType = numbas::exam::VariableValued<T::NumbasType>;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(match self {
                VariableValued::Variable(v) => numbas::exam::VariableValued::Variable(v.clone()),
                VariableValued::Value(v) => {
                    numbas::exam::VariableValued::Value(v.to_numbas(locale).unwrap())
                }
            })
        } else {
            Err(check)
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
