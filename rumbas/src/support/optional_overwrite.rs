pub use super::optional_overwrite_macros::*;
pub use crate::support::noneable::Noneable;
pub use crate::support::rumbas_check::{RumbasCheck, RumbasCheckResult};
use crate::support::template::{Value, ValueType};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait OptionalOverwrite<Item>: Clone + DeserializeOwned + RumbasCheck {
    fn overwrite(&mut self, other: &Item);
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value);
}
impl_optional_overwrite!(String, bool, f64, usize, [f64; 2]);

impl<O: OptionalOverwrite<O>> OptionalOverwrite<Vec<O>> for Vec<O> {
    fn overwrite(&mut self, _other: &Vec<O>) {}
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_i, item) in self.iter_mut().enumerate() {
            item.insert_template_value(key, val);
        }
    }
}
impl_optional_overwrite_value!(Vec<U>[U]);

impl<T: OptionalOverwrite<T>> OptionalOverwrite<HashMap<String, T>> for HashMap<String, T> {
    fn overwrite(&mut self, _other: &HashMap<String, T>) {}
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_i, (_key, item)) in self.iter_mut().enumerate() {
            item.insert_template_value(key, val);
        }
    }
}
impl_optional_overwrite_value!(HashMap < String, T > [T]);

#[cfg(test)]
mod test {
    use super::*;
    use schemars::JsonSchema;
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
        let check = t.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["test"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
        let t = Temp {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        assert_eq!(t.check("").is_empty(), true);
        let t = Temp {
            name: Value::None(),
            test: Value::None(),
        };
        let check = t.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["name", "test"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
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
        assert_eq!(t.check("").is_empty(), true);
        let t = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let check = t.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t.name"],
        );
        assert_eq!(check.invalid_yaml_fields().is_empty(), true);
        let t = Temp2 {
            other: Value::None(),
            t: Value::None(),
        };
        let check = t.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
        let t = Temp2 {
            other: Value::None(),
            t: Value::Normal(Temp {
                name: Value::None(),
                test: Value::None(),
            }),
        };
        let check = t.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t.name", "t.test"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
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
        let check = v.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["0.test", "2.name", "2.test"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
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
        let check = v.check("");
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["1.other", "1.t.name", "2.other", "2.t", "3.other", "3.t.name", "3.t.test"]
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
    }
}
