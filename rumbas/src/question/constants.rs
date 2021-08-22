use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::defaults::DEFAULTS;
use numbas::jme::JMEString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Macro used to make sure that the ToNumbas & ToRumbas implementation remain up to data
macro_rules! builtin_constants {
    (
        $(#[$outer:meta])*
        pub struct $struct: ident {
            $(
                $(#[$inner:meta])*
                $field: ident: $type: ty: $name: literal: $default: ident
            ),+
        }
    ) => {
        optional_overwrite! {
            $(
                #[$outer]
            )*
            pub struct $struct {
                $(
                    $(#[$inner])*
                    $field: $type
                ),*
            }
        }
        impl ToNumbas<std::collections::HashMap<String, bool>> for $struct {
            fn to_numbas(&self, _locale: &str) -> std::collections::HashMap<String, bool> {
                let mut builtin = std::collections::HashMap::new();
                $(
                    builtin.insert($name.to_string(), self.$field.unwrap());
                )*
                builtin
            }
        }
        impl ToRumbas<BuiltinConstants> for numbas::exam::BuiltinConstants {
            fn to_rumbas(&self) -> BuiltinConstants {
                BuiltinConstants {
                $(
                    $field: Value::Normal(*self.0.get(&$name.to_string()).unwrap_or(&DEFAULTS.$default))
                ),*
                }
            }
        }
    }
}

builtin_constants! {
    /// Specify which builtin constants should be enabled
    pub struct BuiltinConstants {
        /// Whether the constant e is enabled
        e: bool: "e": builtin_constants_e,
        /// Whether the constant pi is enabled
        pi: bool: "pi,\u{03c0}": builtin_constants_pi,
        /// Whether the constant i is enabled-
        i: bool: "i": builtin_constants_i
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
            name: self.name.to_rumbas(),
            value: self.value.to_rumbas(),
            tex: self.tex.to_rumbas(),
        }
    }
}
