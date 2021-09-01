#[macro_use]
extern crate rumbas_support_derive;

include! {"macros.rs.include"}

use rumbas_support::input::Input;
use rumbas_support::input::InputCheckResult;
use rumbas_support::input::InputInverse;
use rumbas_support::overwrite::Overwrite;
use rumbas_support::value::Value;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Input, Overwrite)]
#[input(name = "TempInput")]
pub struct Temp {
    name: String,
    test: String,
}

#[derive(Debug, PartialEq, Deserialize, Input, Overwrite)]
#[input(name = "Temp2Input")]
pub struct Temp2 {
    other: String,
    t: Temp,
}

#[derive(Debug, PartialEq, Deserialize, Input, Overwrite)]
#[input(name = "TempEnumInput")]
pub enum TempEnum {
    Unit,
    Tuple(Temp, Temp2),
    Struct { a: Temp, b: bool },
}

//TODO: template
#[test]
fn check_simple_structs() {
    let t = TempInput {
        name: Value::Normal("test".to_string()),
        test: Value::None(),
    };
    assert_missing_fields!(t, vec!["test"]);
    let t = TempInput {
        name: Value::Normal("test2".to_string()),
        test: Value::Normal("name".to_string()),
    };
    assert_no_missing!(t);
    let t = TempInput {
        name: Value::None(),
        test: Value::None(),
    };
    assert_missing_fields!(t, vec!["name", "test"]);
}

#[test]
fn check_complex_structs() {
    let t = Temp2Input {
        other: Value::Normal("val".to_string()),
        t: Value::Normal(TempInput {
            name: Value::Normal("val".to_string()),
            test: Value::Normal("name".to_string()),
        }),
    };
    assert_no_missing!(t);
    let t = Temp2Input {
        other: Value::None(),
        t: Value::Normal(TempInput {
            name: Value::None(),
            test: Value::Normal("name".to_string()),
        }),
    };
    assert_missing_fields!(t, vec!["other", "t.name"]);
    let t = Temp2Input {
        other: Value::None(),
        t: Value::None(),
    };
    assert_missing_fields!(t, vec!["other", "t"]);
    let t = Temp2Input {
        other: Value::None(),
        t: Value::Normal(TempInput {
            name: Value::None(),
            test: Value::None(),
        }),
    };
    assert_missing_fields!(t, vec!["other", "t.name", "t.test"]);
}

#[test]
fn overwrite_simple_structs() {
    let mut t = TempInput {
        name: Value::Normal("test".to_string()),
        test: Value::None(),
    };
    let t2 = TempInput {
        name: Value::Normal("test2".to_string()),
        test: Value::Normal("name".to_string()),
    };
    t.overwrite(&t2);
    assert_eq!(
        t,
        TempInput {
            name: t.clone().name,
            test: t2.test,
        }
    );
}

#[test]
fn overwrite_nested_structs() {
    let t3 = Temp2Input {
        other: Value::None(),
        t: Value::Normal(TempInput {
            name: Value::None(),
            test: Value::Normal("name".to_string()),
        }),
    };
    let mut t4 = Temp2Input {
        other: Value::None(),
        t: Value::None(),
    };
    t4.overwrite(&t3);
    assert_eq!(
        t4,
        Temp2Input {
            other: Value::None(),
            t: t3.clone().t
        }
    );
    let t5 = Temp2Input {
        other: Value::None(),
        t: Value::Normal(TempInput {
            name: Value::Normal("test".to_string()),
            test: Value::Normal("name2".to_string()),
        }),
    };
    t4.overwrite(&t5);
    assert_eq!(
        t4,
        Temp2Input {
            other: Value::None(),
            t: Value::Normal(TempInput {
                name: t5.t.unwrap().name,
                test: t3.t.unwrap().test
            }),
        }
    );
}

#[test]
fn check_vec_of_simple_structs() {
    let t1 = TempInput {
        name: Value::Normal("test".to_string()),
        test: Value::None(),
    };
    let t2 = TempInput {
        name: Value::Normal("test2".to_string()),
        test: Value::Normal("name".to_string()),
    };
    let t3 = TempInput {
        name: Value::None(),
        test: Value::None(),
    };
    let v = vec![t1, t2, t3];
    assert_missing_fields!(v, vec!["0.test", "2.name", "2.test"]);
}

#[test]
fn check_vec_ofcomplex_structs() {
    let t1 = Temp2Input {
        other: Value::Normal("val".to_string()),
        t: Value::Normal(TempInput {
            name: Value::Normal("val".to_string()),
            test: Value::Normal("name".to_string()),
        }),
    };
    let t2 = Temp2Input {
        other: Value::None(),
        t: Value::Normal(TempInput {
            name: Value::None(),
            test: Value::Normal("name".to_string()),
        }),
    };
    let t3 = Temp2Input {
        other: Value::None(),
        t: Value::None(),
    };
    let t4 = Temp2Input {
        other: Value::None(),
        t: Value::Normal(TempInput {
            name: Value::None(),
            test: Value::None(),
        }),
    };
    let v = vec![t1, t2, t3, t4];
    assert_missing_fields!(
        v,
        vec!["1.other", "1.t.name", "2.other", "2.t", "3.other", "3.t.name", "3.t.test"]
    );
}
