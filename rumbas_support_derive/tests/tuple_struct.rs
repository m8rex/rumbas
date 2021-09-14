#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::overwrite::Overwrite;
use rumbas_support::rumbas_check::RumbasCheck;
use rumbas_support::rumbas_check::RumbasCheckResult;
use rumbas_support::value::Value;
use rumbas_support::value::ValueType;
use serde::Deserialize;

#[derive(Input, RumbasCheck)]
#[input(name = "TestInput")]
#[derive(Clone, Deserialize)]
pub struct Test(bool, f64);

type TestInputs = Vec<Test>;

#[derive(Input, RumbasCheck)]
#[input(name = "Test2Input")]
#[derive(Clone, Deserialize)]
pub struct Test2(TestInputs, f64);

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "TestOverwriteInput")]
#[derive(Clone, Deserialize)]
pub struct TestOverwrite(bool, f64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_test2() {
        let _test2 = Test2(vec![Test(true, 64.8)], 65.0);

        let _test2 = Test2Input(
            ValueType::Normal(vec![Value::Normal(TestInput(
                ValueType::Normal(true),
                ValueType::Normal(64.8),
            ))]),
            ValueType::Normal(65.0),
        );
    }
}
