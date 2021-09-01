#[macro_use]
extern crate rumbas_support_derive;

include! {"macros.rs.include"}

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::overwrite::Overwrite;
use rumbas_support::value::Value;
use serde::Deserialize;

#[derive(Clone, Deserialize, Input)]
#[input(name = "TestInput")]
pub enum Test {
    Unit,
    Tuple(f64, bool, String),
    Struct { a: f64 },
}

type TestInputs = Vec<Test>;

#[derive(Clone, Deserialize, Input)]
#[input(name = "Test2Input")]
pub struct Test2 {
    field1: TestInputs,
    field2: f64,
}

#[derive(Clone, Input, Deserialize, Overwrite)]
#[input(name = "TestOverwriteInput")]
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
        field1: Value::Normal(vec![TestInput::Struct {
            a: Value::Normal(5.8),
        }]),
        field2: Value::Normal(65.0),
    };
}

#[test]
fn find_missing() {
    assert_no_missing!(TestInput::Unit);

    assert_no_missing!(TestInput::Tuple(
        Value::Normal(5.8),
        Value::Normal(true),
        Value::Normal("s".to_owned())
    ));
    assert_missing_fields!(
        TestInput::Tuple(
            Value::None(),
            Value::Normal(true),
            Value::Normal("s".to_owned())
        ),
        vec!["0"] // TODO
    );
    assert_missing_fields!(
        TestInput::Tuple(
            Value::Normal(5.8),
            Value::None(),
            Value::Normal("s".to_owned())
        ),
        vec!["1"]
    );
    assert_missing_fields!(
        TestInput::Tuple(Value::Normal(5.8), Value::Normal(true), Value::None()),
        vec!["2"]
    );
    assert_missing_fields!(
        TestInput::Tuple(Value::None(), Value::Normal(true), Value::None()),
        vec!["0", "2"]
    );

    assert_no_missing!(TestInput::Struct {
        a: Value::Normal(5.8)
    });

    assert_missing_fields!(TestInput::Struct { a: Value::None() }, vec!["a"]);
}
