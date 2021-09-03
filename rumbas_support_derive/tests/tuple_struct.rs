#[macro_use]
extern crate rumbas_support_derive;

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::overwrite::Overwrite;
use rumbas_support::value::Value;
use serde::Deserialize;

#[derive(Input)]
#[input(name = "TestInput")]
#[derive(Clone, Deserialize)]
pub struct Test(bool, f64);

type TestInputs = Vec<Test>;

#[derive(Input)]
#[input(name = "Test2Input")]
#[derive(Clone, Deserialize)]
pub struct Test2(TestInputs, f64);

#[derive(Input, Overwrite)]
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
            Value::Normal(vec![TestInput(Value::Normal(true), Value::Normal(64.8))]),
            Value::Normal(65.0),
        );
    }
}
