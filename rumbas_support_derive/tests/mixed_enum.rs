#[macro_use]
extern crate rumbas_support_derive;

include! {"macros.rs.include"}

use comparable::Comparable;
use rumbas_support::preamble::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "TestInput")]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq)]
#[serde(untagged)]
pub enum Test {
    Unit(Unit),
    Tuple(TestOverwrite, bool, String),
    Struct { a: f64 },
    TupleOne(TestOverwrite),
}

type TestInputs = Vec<Test>;

#[derive(Input, RumbasCheck, Examples)]
#[input(name = "Test2Input")]
#[input(test)]
#[derive(Clone, Debug, Deserialize, Serialize, Comparable, PartialEq)]
pub struct Test2 {
    field1: TestInputs,
    field2: f64,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "TestOverwriteInput")]
#[derive(Debug, Clone, Deserialize, Serialize, Comparable, PartialEq)]
///  Hi there
#[serde(untagged)]
pub enum TestOverwrite {
    Unit(Unit),
    Tuple(bool, f64),
    Struct { field1: bool, field2: f64 },
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "UnitInput")]
#[derive(Debug, Clone, Deserialize, Serialize, Comparable, PartialEq)]
pub enum Unit {
    Unit,
}

#[test]
fn create_test2() {
    let _test2 = Test2 {
        field1: vec![Test::Unit(Unit::Unit)],
        field2: 65.0,
    };

    let _test2 = Test2Input {
        field1: Value::Normal(vec![ValueType::Normal(TestInput::Struct {
            a: Value::Normal(5.8),
        })]),
        field2: Value::Normal(65.0),
    };
}

#[test]
fn find_missing() {
    assert_no_missing!(TestInput::Unit(UnitInput::Unit));

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
            "test".to_string()
        ),
        vec!["0.field1"]
    );

    assert_missing_fields!(
        TestInput::TupleOne(TestOverwriteInput::Struct {
            field1: Value::None(),
            field2: Value::Normal(5.8)
        },),
        vec!["field1"]
    );
}

#[test]
fn examples() {
    TestInput::examples();
    Test2Input::examples();
    Test2InputEnum::examples();
    TestOverwriteInput::examples();
}
