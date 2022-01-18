#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::preamble::*;
use serde::Deserialize;
use serde::Serialize;
use comparable::Comparable;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "TestInput")]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq)]
pub struct Test {
    field1: bool,
    field2: f64,
}

type TestInputs = Vec<Test>;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "Test2Input")]
#[input(test)]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq)]
pub struct Test2 {
    field1: TestInputs,
    field2: f64,
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "TestOverwriteInput")]
#[derive(Clone, Deserialize)]
pub struct TestOverwrite {
    field1: bool,
    field2: f64,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_test2() {
        let _test2 = Test2 {
            field1: vec![Test {
                field1: true,
                field2: 64.8,
            }],
            field2: 65.0,
        };

        let _test2 = Test2Input {
            field1: Value::Normal(vec![ValueType::Normal(TestInput {
                field1: Value::Normal(true),
                field2: Value::Normal(64.8),
            })]),
            field2: Value::Normal(65.0),
        };
    }

    #[test]
    fn parse_yaml_test_no_fields_ok_for_normal() {
        let fail: Result<Test2Input, _> = serde_yaml::from_str(
            r"---
other_field1: true
",
        );
        assert!(fail.is_ok());
    }

    #[test]
    fn parse_yaml_test_no_fields() {
        let fail: Result<Test2InputEnum, _> = serde_yaml::from_str(
            r"---
other_field1: true
",
        );
        assert!(fail.is_err());
    }

    #[test]
    fn parse_yaml_test() {
        let ok: Result<Test2InputEnum, _> = serde_yaml::from_str(
            r"---
field1: true
",
        );

        assert!(ok.is_ok());
    }
    #[test]
    fn examples() {
        TestInput::examples();
        TestInputEnum::examples();
        Test2Input::examples();
        Test2InputEnum::examples();
    }
}
