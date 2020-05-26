use std::collections::HashMap;

pub trait OptionalOverwrite: Clone {
    type Item;

    fn empty_fields(&self) -> Vec<String>;
    fn overwrite(&mut self, other: &Self::Item);
}

macro_rules! impl_optional_overwrite_option {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl$(< $($gen: OptionalOverwrite ),* >)? OptionalOverwrite for Option<$type> {
            type Item = Option<$type>;
            fn empty_fields(&self) -> Vec<String> {
                if let Some(val) = &self {
                    return val.empty_fields()
                }
                else {
                    return vec!["".to_string()]
                }
            }
            fn overwrite(&mut self, other: &Self::Item) {
                if let Some(ref mut val) = self {
                    if let Some(other_val) = &other {
                        val.overwrite(&other_val);
                    }
                } else {
                    *self = other.clone();
                }
            }
        }
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
}
impl_optional_overwrite_option!(Vec<U>[U]);

macro_rules! impl_optional_overwrite {
    ($($type: ty), *) => {
        $(
        impl OptionalOverwrite for $type {
            type Item = $type;
            fn empty_fields(&self) -> Vec<String> {
                Vec::new()
            }
            fn overwrite(&mut self, _other: &Self::Item) {}
        }
        impl_optional_overwrite_option!($type);
        )*
    };
}
impl_optional_overwrite!(String, bool, f64, usize, [f64; 2]);
//TODO: different if implements
impl<U: OptionalOverwrite, T: OptionalOverwrite> OptionalOverwrite for HashMap<U, T> {
    type Item = HashMap<U, T>;
    fn empty_fields(&self) -> Vec<String> {
        Vec::new()
    }
    fn overwrite(&mut self, _other: &Self::Item) {}
}
impl_optional_overwrite_option!(HashMap < U, T > [U, T]);

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
                pub $field: Option<$type>
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
        }
        impl_optional_overwrite_option!($struct);
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
        }
        impl_optional_overwrite_option!($enum);
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

    #[test]
    fn empty_fields_simple_structs() {
        let t = Temp {
            name: Some("test".to_string()),
            test: None,
        };
        assert_eq!(t.empty_fields(), vec!["test"]);
        let t = Temp {
            name: Some("test2".to_string()),
            test: Some("name".to_string()),
        };
        assert_eq!(t.empty_fields().len(), 0);
        let t = Temp {
            name: None,
            test: None,
        };
        assert_eq!(t.empty_fields(), vec!["name", "test"]);
    }

    #[test]
    fn empty_fields_complex_structs() {
        let t = Temp2 {
            other: Some("val".to_string()),
            t: Some(Temp {
                name: Some("val".to_string()),
                test: Some("name".to_string()),
            }),
        };
        assert_eq!(t.empty_fields().len(), 0);
        let t = Temp2 {
            other: None,
            t: Some(Temp {
                name: None,
                test: Some("name".to_string()),
            }),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t.name"]);
        let t = Temp2 {
            other: None,
            t: None,
        };
        assert_eq!(t.empty_fields(), vec!["other", "t"]);
        let t = Temp2 {
            other: None,
            t: Some(Temp {
                name: None,
                test: None,
            }),
        };
        assert_eq!(t.empty_fields(), vec!["other", "t.name", "t.test"]);
    }

    #[test]
    fn overwrite_simple_structs() {
        let mut t = Temp {
            name: Some("test".to_string()),
            test: None,
        };
        let t2 = Temp {
            name: Some("test2".to_string()),
            test: Some("name".to_string()),
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
            other: None,
            t: Some(Temp {
                name: None,
                test: Some("name".to_string()),
            }),
        };
        let mut t4 = Temp2 {
            other: None,
            t: None,
        };
        t4.overwrite(&t3);
        assert_eq!(
            t4,
            Temp2 {
                other: None,
                t: t3.clone().t
            }
        );
        let t5 = Temp2 {
            other: None,
            t: Some(Temp {
                name: Some("test".to_string()),
                test: Some("name2".to_string()),
            }),
        };
        t4.overwrite(&t5);
        assert_eq!(
            t4,
            Temp2 {
                other: None,
                t: Some(Temp {
                    name: t5.t.unwrap().name,
                    test: t3.t.unwrap().test
                }),
            }
        );
    }

    #[test]
    fn empty_fields_vec_of_simple_structs() {
        let t1 = Temp {
            name: Some("test".to_string()),
            test: None,
        };
        let t2 = Temp {
            name: Some("test2".to_string()),
            test: Some("name".to_string()),
        };
        let t3 = Temp {
            name: None,
            test: None,
        };
        let v = vec![t1, t2, t3];
        assert_eq!(v.empty_fields(), vec!["0.test", "2.name", "2.test"]);
    }

    #[test]
    fn empty_fields_vec_ofcomplex_structs() {
        let t1 = Temp2 {
            other: Some("val".to_string()),
            t: Some(Temp {
                name: Some("val".to_string()),
                test: Some("name".to_string()),
            }),
        };
        let t2 = Temp2 {
            other: None,
            t: Some(Temp {
                name: None,
                test: Some("name".to_string()),
            }),
        };
        let t3 = Temp2 {
            other: None,
            t: None,
        };
        let t4 = Temp2 {
            other: None,
            t: Some(Temp {
                name: None,
                test: None,
            }),
        };
        let v = vec![t1, t2, t3, t4];
        assert_eq!(
            v.empty_fields(),
            vec!["1.other", "1.t.name", "2.other", "2.t", "3.other", "3.t.name", "3.t.test"]
        );
    }
}
