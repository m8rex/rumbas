use crate::data::template::Value;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

pub trait OptionalOverwrite: Clone {
    type Item;

    fn empty_fields(&self) -> Vec<String>;
    fn overwrite(&mut self, other: &Self::Item);
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
        impl$(< $($gen: OptionalOverwrite + DeserializeOwned ),* >)? OptionalOverwrite for Value<$type> {
            type Item = Value<$type>;
            fn empty_fields(&self) -> Vec<String> {
                if let Value::Normal(val) = &self {
                    val.empty_fields()
                }
                else if let Value::Template(val) = &self {
                    vec![val.yaml()]
                }
                else {
                    vec!["".to_string()]
                }
            }
            fn overwrite(&mut self, other: &Self::Item) {
                if let Value::Normal(ref mut val) = self {
                    if let Value::Normal(other_val) = &other {
                        val.overwrite(&other_val);
                    }
                } else {
                    *self = other.clone();
                }
            }
            fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value){
                if let Value::Template(ts) = self {
                    if ts.key == Some(key.clone()) {
                        *self=Value::Normal(serde_yaml::from_value(val.clone()).unwrap());
                    }
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
        impl$(< $($gen: OptionalOverwrite ),* >)? OptionalOverwrite for Noneable<$type> {
            type Item = Noneable<$type>;
            fn empty_fields(&self) -> Vec<String> {
                if let Noneable::NotNone(val) = &self {
                    return val.empty_fields()
                }
                else {
                    return vec![]
                }
            }
            fn overwrite(&mut self, other: &Self::Item) {
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

impl<O: OptionalOverwrite> OptionalOverwrite for Vec<O> {
    type Item = Vec<O>;
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
    fn overwrite(&mut self, _other: &Self::Item) {}
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
        impl OptionalOverwrite for $type {
            type Item = $type;
            fn empty_fields(&self) -> Vec<String> {
                Vec::new()
            }
            fn overwrite(&mut self, _other: &Self::Item) {}
            fn insert_template_value(&mut self, _key: &String, _val: &serde_yaml::Value) {}
        }
        impl_optional_overwrite_value!($type);
        )*
    };
}
impl_optional_overwrite!(String, bool, f64, usize, [f64; 2]);

impl<T: OptionalOverwrite> OptionalOverwrite for HashMap<String, T> {
    type Item = HashMap<String, T>;
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
    fn overwrite(&mut self, _other: &Self::Item) {}
    fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value) {
        for (i, (_key, mut item)) in self.iter_mut().enumerate() {
            item.insert_template_value(&key, &val);
        }
    }
}
impl_optional_overwrite_value!(HashMap < String, T > [T]);

macro_rules! optional_overwrite {
    // This macro creates a struct with all optional fields
    // It also adds a method to overwrite all fields with None value with the values of another object of the same type
    // It also adds a method to list the fields that are None
    ($struct: ident$(: $container_attribute: meta)?, $($field: ident: $type: ty$(: $attribute: meta)?), *) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        $(
            #[$container_attribute]
        )?
        pub struct $struct {
            $(
                $(
                    #[$attribute]
                )?
                pub $field: Value<$type>
            ),*
        }
        impl OptionalOverwrite for $struct {
            type Item = $struct;
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
            fn overwrite(&mut self, other: &Self::Item) {
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
    ($enum: ident$(: $container_attribute: meta)?, $($field: ident: $type: ty$(: $attribute: meta)?), *) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        $(
            #[$container_attribute]
        )?
        pub enum $enum {
            $(
                $(
                    #[$attribute]
                )?
                $field($type)
            ),*
        }
        impl OptionalOverwrite for $enum {
            type Item = $enum;
            fn empty_fields(&self) -> Vec<String> {
                match self {
                $(
                    $enum::$field(val) => val.empty_fields()
                ),*
                }
            }
            fn overwrite(&mut self, other: &Self::Item) {
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
                    , _ => ()
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
        Temp,
        name: String,
        test: String
    }

    optional_overwrite! {
        Temp2,
        other: String,
        t: Temp
    }
    //TODO: template
    #[test]
    fn empty_fields_simple_structs() {
        let t = Temp {
            name: Value::Normal("test".to_string()),
            test: Value::None,
        };
        assert_eq!(t.empty_fields(), vec!["test"]);
        let t = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        assert_eq!(t.empty_fields().len(), 0);
        let t = Temp {
            name: Value::None,
            test: Value::None,
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
            other: Value::None,
            t: Value::Normal(Temp {
                name: Value::None,
                test: Value::Normal("name".to_string()),
            }),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t.name"]);
        let t = Temp2 {
            other: Value::None,
            t: Value::None,
        };
        assert_eq!(t.empty_fields(), vec!["other", "t"]);
        let t = Temp2 {
            other: Value::None,
            t: Value::Normal(Temp {
                name: Value::None,
                test: Value::None,
            }),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t.name", "t.test"]);
    }

    #[test]
    fn overwrite_simple_structs() {
        let mut t = Temp {
            name: Value::Normal("test".to_string()),
            test: Value::None,
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
            other: Value::None,
            t: Value::Normal(Temp {
                name: Value::None,
                test: Value::Normal("name".to_string()),
            }),
        };
        let mut t4 = Temp2 {
            other: Value::None,
            t: Value::None,
        };
        t4.overwrite(&t3);
        assert_eq!(
            t4,
            Temp2 {
                other: Value::None,
                t: t3.clone().t
            }
        );
        let t5 = Temp2 {
            other: Value::None,
            t: Value::Normal(Temp {
                name: Value::Normal("test".to_string()),
                test: Value::Normal("name2".to_string()),
            }),
        };
        t4.overwrite(&t5);
        assert_eq!(
            t4,
            Temp2 {
                other: Value::None,
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
            test: Value::None,
        };
        let t2 = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        let t3 = Temp {
            name: Value::None,
            test: Value::None,
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
            other: Value::None,
            t: Value::Normal(Temp {
                name: Value::None,
                test: Value::Normal("name".to_string()),
            }),
        };
        let t3 = Temp2 {
            other: Value::None,
            t: Value::None,
        };
        let t4 = Temp2 {
            other: Value::None,
            t: Value::Normal(Temp {
                name: Value::None,
                test: Value::None,
            }),
        };
        let v = vec![t1, t2, t3, t4];
        assert_eq!(
            v.empty_fields(),
            vec!["1.other", "1.t.name", "2.other", "2.t", "3.other", "3.t.name", "3.t.test"]
        );
    }
}
