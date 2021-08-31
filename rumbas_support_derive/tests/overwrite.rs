#[macro_use]
extern crate rumbas_support_derive;

#[cfg(test)]
mod test {

    use rumbas_support::input::Input;
    use rumbas_support::input::InputCheckResult;
    use rumbas_support::input::InputInverse;
    use rumbas_support::overwrite::Overwrite;
    use rumbas_support::value::Value;
    use serde::Deserialize;

    #[derive(Debug, PartialEq, Deserialize, Input, Overwrite)]
    #[input(name = "TempInput")]
    pub struct Temp {
        name: String,
        test: String,
    }

    #[derive(Debug, PartialEq, Deserialize, Input, Overwrite)]
    #[input(name = "Temp2Input")]
    pub struct Temp2 {
        other: String,
        t: Temp,
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
