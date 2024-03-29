#[macro_use]
extern crate rumbas_support_derive;

use comparable::Comparable;
use rumbas_support::preamble::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "TestInput")]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq, Eq)]
pub struct Test;

type TestInputs = Vec<Test>;

#[derive(Input, RumbasCheck)]
#[input(name = "Test2Input")]
#[derive(Clone, Deserialize)]
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
            field1: vec![Test],
            field2: 65.0,
        };

        let _test2 = Test2Input {
            field1: Value::Normal(vec![ValueType::Normal(TestInput)]),
            field2: Value::Normal(65.0),
        };
    }
    #[test]
    fn examples() {
        TestInput::examples();
    }
}
