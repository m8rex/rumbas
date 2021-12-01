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
pub enum Test {
    First,
    Second,
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
            field1: vec![Test::First],
            field2: 65.0,
        };

        let _test2 = Test2Input {
            field1: Value::Normal(vec![Value::Normal(TestInput::First)]),
            field2: Value::Normal(65.0),
        };
    }
    #[test]
    fn examples() {
        Test::examples();
        TestInput::examples();
        Test2::examples();
        Test2Input::examples();
    }
}
