#[macro_use]
extern crate rumbas_support_derive;

include! {"macros.rs.include"}

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
    Unit,
    Tuple(TestOverwrite, bool, String),
    Struct { a: f64 },
    TupleOne(f64),
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
#[derive(Debug, Clone, Deserialize, Examples)]
///  Hi there
pub enum TestOverwrite {
    Unit,
    Tuple(bool, f64),
    Struct { field1: bool, field2: f64 },
}

#[test]
fn create_test2() {
    let _test2 = Test2 {
        field1: vec![Test::Unit],
        field2: 65.0,
    };

    let _test2 = Test2Input {
        field1: Value::Normal(vec![Value::Normal(TestInput::Struct {
            a: Value::Normal(5.8),
        })]),
        field2: Value::Normal(65.0),
    };
}

#[test]
fn find_missing() {
    assert_no_missing!(TestInput::Unit);

    assert_no_missing!(TestInput::Tuple(
        TestOverwriteInput::Struct {
            field1: Value::Normal(true),
            field2: Value::Normal(5.8)
        },
        true,
        "s".to_owned()
    ));

    assert_no_missing!(TestInput::Struct {
        a: Value::Normal(5.8)
    });

    assert_missing_fields!(TestInput::Struct { a: Value::None() }, vec!["a"]);

    assert_missing_fields!(
        TestInput::Tuple(
            TestOverwriteInput::Struct {
                field1: Value::None(),
                field2: Value::Normal(5.8)
            },
            true,
            "s".to_owned()
        ),
        vec!["0.field1"]
    );
}

#[test]
fn examples() {
    Test::examples();
    TestInput::examples();
    Test2::examples();
    Test2Input::examples();
    TestOverwrite::examples();
    TestOverwriteInput::examples();
}
