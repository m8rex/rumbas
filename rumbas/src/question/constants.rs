use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::defaults::DEFAULTS;
use numbas::jme::JMEString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    /// Specify which builtin constants should be enabled
    pub struct BuiltinConstants {
        /// Whether the constant e is enabled
        e: bool,
        /// Whether the constant pi is enabled
        pi: bool,
        /// Whether the constant i is enabled-
        i: bool
    }
}

impl ToNumbas<std::collections::HashMap<String, bool>> for BuiltinConstants {
    fn to_numbas(&self, _locale: &str) -> std::collections::HashMap<String, bool> {
        let mut builtin = std::collections::HashMap::new();
        // TODO: use macro to make sure that this list always remains up to date
        builtin.insert("e".to_string(), self.e.unwrap());
        builtin.insert("pi,\u{03c0}".to_string(), self.pi.unwrap());
        builtin.insert("i".to_string(), self.i.unwrap());
        builtin
    }
}

impl ToRumbas<BuiltinConstants> for numbas::exam::BuiltinConstants {
    fn to_rumbas(&self) -> BuiltinConstants {
        BuiltinConstants {
            e: Value::Normal(
                *self
                    .0
                    .get(&"e".to_string())
                    .unwrap_or(&DEFAULTS.builtin_constants_e),
            ),
            pi: Value::Normal(
                *self
                    .0
                    .get(&"pi,\u{03c0}".to_string())
                    .unwrap_or(&DEFAULTS.builtin_constants_pi),
            ),
            i: Value::Normal(
                *self
                    .0
                    .get(&"i".to_string())
                    .unwrap_or(&DEFAULTS.builtin_constants_i),
            ),
        }
    }
}

optional_overwrite! {
    /// A custom constant
    pub struct CustomConstant {
        /// The name of the constant
        name: String,
        /// The value of the constant
        value: JMEString,
        /// The tex code use to display the constant
        tex: String
    }
}

impl ToNumbas<numbas::exam::ExamQuestionConstant> for CustomConstant {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionConstant {
        numbas::exam::ExamQuestionConstant {
            name: self.name.to_numbas(locale),
            value: self.value.to_numbas(locale),
            tex: self.tex.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomConstant> for numbas::exam::ExamQuestionConstant {
    fn to_rumbas(&self) -> CustomConstant {
        CustomConstant {
            name: Value::Normal(self.name.clone()),
            value: Value::Normal(self.value.clone()),
            tex: Value::Normal(self.tex.clone()),
        }
    }
}
