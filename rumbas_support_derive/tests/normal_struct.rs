#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::example::Examples;
use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::overwrite::Overwrite;
use rumbas_support::rumbas_check::RumbasCheck;
use rumbas_support::rumbas_check::RumbasCheckResult;
use rumbas_support::value::Value;
use serde::Deserialize;

#[derive(Input, RumbasCheck)]
#[input(name = "TestInput")]
#[derive(Clone, Deserialize, Examples)]
pub struct Test {
    field1: bool,
    field2: f64,
}

type TestInputs = Vec<Test>;

#[derive(Input, RumbasCheck)]
#[input(name = "Test2Input")]
#[derive(Clone, Deserialize, Examples)]
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
            field1: Value::Normal(vec![Value::Normal(TestInput {
                field1: Value::Normal(true),
                field2: Value::Normal(64.8),
            })]),
            field2: Value::Normal(65.0),
        };
    }

    #[test]
    fn parse_yaml_test_no_fields() {
        let fail: Result<Test2Input, _> = serde_yaml::from_str(
            r"---
other_field1: true
",
        );
        assert!(fail.is_err());
    }

    #[test]
    fn parse_yaml_test() {
        let ok: Result<Test2Input, _> = serde_yaml::from_str(
            r"---
field1: true
",
        );

        assert!(ok.is_ok());
    }
    #[test]
    fn examples() {
        Test::examples();
        TestInput::examples();
        Test2::examples();
        Test2Input::examples();
    }
}
