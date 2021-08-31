#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::value::Value;

#[derive(Clone, Input)]
#[input(name = "TestInput")]
pub struct Test {
    field1: bool,
    field2: f64,
}

type TestInputs = Vec<Test>;

#[derive(Clone, Input)]
#[input(name = "Test2Input")]
pub struct Test2 {
    field1: TestInputs,
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
            field1: Value::Normal(vec![TestInput {
                field1: Value::Normal(true),
                field2: Value::Normal(64.8),
            }]),
            field2: Value::Normal(65.0),
        };
    }
}
