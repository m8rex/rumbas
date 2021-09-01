#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::overwrite::Overwrite;
use rumbas_support::value::Value;
use serde::Deserialize;

#[derive(Clone, Deserialize, Input)]
#[input(name = "TestInput")]
pub enum Test {
    First,
    Second,
}

type TestInputs = Vec<Test>;

#[derive(Clone, Deserialize, Input)]
#[input(name = "Test2Input")]
pub struct Test2 {
    field1: TestInputs,
    field2: f64,
}

/*
#[derive(Clone, Input, Deserialize, Overwrite)]
#[input(name = "TestOverwriteInput")]
pub struct TestOverwrite {
    field1: bool,
    field2: f64,
}*/

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
            field1: Value::Normal(vec![TestInput::First]),
            field2: Value::Normal(65.0),
        };
    }
}
