#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;

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

pub struct InputCreated {
    test: TestInput,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_test2() {
        let test2 = Test2 {
            field1: vec![Test {
                field1: true,
                field2: 64.8,
            }],
            field2: 65.0,
        };

        let test2 = Test2Input {
            field1: vec![TestInput {
                field1: true,
                field2: 64.8,
            }],
            field2: 65.0,
        };
    }
}
