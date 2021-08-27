pub use super::optional_overwrite_macros::*;
pub use crate::support::noneable::Noneable;
pub use crate::support::optional_check::{OptionalCheck, OptionalCheckResult};
pub use crate::support::rumbas_check::{RumbasCheck, RumbasCheckResult};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait OptionalOverwrite<Item>: Clone + DeserializeOwned + OptionalCheck + Input {
    fn overwrite(&mut self, other: &Item);
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value);
}

pub trait Input: Clone + OptionalCheck {
    type Normal;
    /// This method assumes that it is called by a function that is initially called from `to_normal_safe`
    fn to_normal(&self) -> Self::Normal;

    /// Method that safely convets the input type to the normal type
    fn to_normal_safe(&self) -> Result<Self::Normal, OptionalCheckResult> {
        let check = self.find_missing();
        if check.is_empty() {
            Ok(self.to_normal())
        } else {
            Err(check)
        }
    }

    fn from_normal(normal: Self::Normal) -> Self;
}

impl<O: Input> Input for Vec<O> {
    type Normal = Vec<<O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter().map(|a| a.to_normal()).collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal.into_iter().map(<O as Input>::from_normal).collect()
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

impl<O: Input> Input for HashMap<String, O> {
    type Normal = HashMap<String, <O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter()
            .map(|(s, a)| (s.to_owned(), a.to_normal()))
            .collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal
            .into_iter()
            .map(|(s, a)| (s, <O as Input>::from_normal(a)))
            .collect()
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

impl<O: Input> Input for Box<O> {
    type Normal = Box<<O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        Box::new((**self).to_normal())
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        Box::new(<O as Input>::from_normal(*normal))
    }
}

impl<T: OptionalOverwrite<T>> OptionalOverwrite<Box<T>> for Box<T> {
    fn overwrite(&mut self, other: &Box<T>) {
        (**self).overwrite(&*other)
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        (**self).insert_template_value(key, val)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::rumbas_types::*;
    use crate::support::template::Value;
    use schemars::JsonSchema;
    use serde::Deserialize;
    use serde::Serialize;

    optional_overwrite! {
        #[derive(PartialEq)]
        pub struct Temp {
        name: RumbasString,
        test: RumbasString
        }
    }

    optional_overwrite! {
        #[derive(PartialEq)]
        pub struct Temp2 {
        other: RumbasString,
        t: Temp
        }
    }
    //TODO: template
    #[test]
    fn check_simple_structs() {
        let t = TempInput {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        let check = t.find_missing();
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["test"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
        let t = TempInput {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        assert_eq!(t.find_missing().is_empty(), true);
        let t = TempInput {
            name: Value::None(),
            test: Value::None(),
        };
        let check = t.find_missing();
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
        let t = Temp2Input {
            other: Value::Normal("val".to_string()),
            t: Value::Normal(TempInput {
                name: Value::Normal("val".to_string()),
                test: Value::Normal("name".to_string()),
            }),
        };
        assert_eq!(t.find_missing().is_empty(), true);
        let t = Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let check = t.find_missing();
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t.name"],
        );
        assert_eq!(check.invalid_yaml_fields().is_empty(), true);
        let t = Temp2Input {
            other: Value::None(),
            t: Value::None(),
        };
        let check = t.find_missing();
        assert_eq!(
            check
                .missing_fields()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
            vec!["other", "t"],
        );
        assert_eq!(check.invalid_yaml_fields().len(), 0);
        let t = Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: Value::None(),
                test: Value::None(),
            }),
        };
        let check = t.find_missing();
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
        let mut t = TempInput {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        let t2 = TempInput {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        t.overwrite(&t2);
        assert_eq!(
            t,
            TempInput {
                name: t.clone().name,
                test: t2.test,
            }
        );
    }

    #[test]
    fn overwrite_nested_structs() {
        let t3 = Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let mut t4 = Temp2Input {
            other: Value::None(),
            t: Value::None(),
        };
        t4.overwrite(&t3);
        assert_eq!(
            t4,
            Temp2Input {
                other: Value::None(),
                t: t3.clone().t
            }
        );
        let t5 = Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: Value::Normal("test".to_string()),
                test: Value::Normal("name2".to_string()),
            }),
        };
        t4.overwrite(&t5);
        assert_eq!(
            t4,
            Temp2Input {
                other: Value::None(),
                t: Value::Normal(TempInput {
                    name: t5.t.unwrap().name,
                    test: t3.t.unwrap().test
                }),
            }
        );
    }

    #[test]
    fn check_vec_of_simple_structs() {
        let t1 = TempInput {
            name: Value::Normal("test".to_string()),
            test: Value::None(),
        };
        let t2 = TempInput {
            name: Value::Normal("test2".to_string()),
            test: Value::Normal("name".to_string()),
        };
        let t3 = TempInput {
            name: Value::None(),
            test: Value::None(),
        };
        let v = vec![t1, t2, t3];
        let check = v.find_missing();
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
        let t1 = Temp2Input {
            other: Value::Normal("val".to_string()),
            t: Value::Normal(TempInput {
                name: Value::Normal("val".to_string()),
                test: Value::Normal("name".to_string()),
            }),
        };
        let t2 = Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: Value::None(),
                test: Value::Normal("name".to_string()),
            }),
        };
        let t3 = Temp2Input {
            other: Value::None(),
            t: Value::None(),
        };
        let t4 = Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: Value::None(),
                test: Value::None(),
            }),
        };
        let v = vec![t1, t2, t3, t4];
        let check = v.find_missing();
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
