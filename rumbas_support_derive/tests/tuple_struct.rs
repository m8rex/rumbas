#[macro_use]
extern crate rumbas_support_derive;

use comparable::Comparable;
use rumbas_support::preamble::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "TestInput")]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq)]
pub struct Test(bool, f64);

type TestInputs = Vec<Test>;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "Test2Input")]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq)]
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

        let _test2 = Test2Input(vec![ValueType::Normal(TestInput(true, 4.8))], 65.0);
    }
    #[test]
    fn examples() {
        TestInput::examples();
        Test2Input::examples();
    }
}
