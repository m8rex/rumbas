pub trait OptionalOverwrite {
    type Item;

    fn empty_fields(&self) -> Vec<String>;
    fn overwrite(&mut self, other: &Self::Item);
}

impl OptionalOverwrite for String {
    type Item = String;
    fn empty_fields(&self) -> Vec<String> {
        Vec::new()
    }
    fn overwrite(&mut self, _other: &String) {}
}

macro_rules! impl_optional_overwrite {
    ($($type: ty), *) => {
        $(
        impl OptionalOverwrite for $type {
            type Item = $type;
            fn empty_fields(&self) -> Vec<String> {
                Vec::new()
            }
            fn overwrite(&mut self, _other: &$type) {}
        }
        )*
    };
}
impl_optional_overwrite!(bool, f64, usize);

macro_rules! optional_overwrite {
    // This macro creates a struct with all optional fields
    // It also adds a method to overwrite all fields with None value with the values of another object of the same type
    // It also adds a method to list the fields that are None
    ($struct: ident, $($field: ident: $type: ty), *) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct $struct {
            $(
                pub $field: Option<$type>
            ),*
        }
        impl OptionalOverwrite for $struct {
            type Item = $struct;
            fn empty_fields(&self) -> Vec<String> {
                let mut empty = Vec::new();
                $(

                    if let Some(val) = &self.$field {
                        let extra_empty = val.empty_fields();
                        for extra in extra_empty.iter() {
                            empty.push(format!("{}.{}", stringify!($field), extra));
                        }
                    } else{
                        empty.push(stringify!($field).to_string());
                    }
                )*
                empty
            }
            fn overwrite(&mut self, other: &$struct) {
                $(
                    if let Some(ref mut val) = self.$field {
                        if let Some(other_val) = &other.$field {
                            val.overwrite(&other_val);
                        }
                    } else {
                        self.$field = other.$field.clone();
                    }
                )*
            }

        }
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
}
