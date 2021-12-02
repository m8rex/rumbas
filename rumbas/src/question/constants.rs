use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::defaults::DEFAULTS;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
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
        $(
            #[$outer]
        )*
        pub struct $struct {
            $(
                $(#[$inner])*
                pub $field: $type
            ),*
        }
        impl ToNumbas<std::collections::HashMap<String, bool>> for $struct {
            fn to_numbas(&self, _locale: &str) -> std::collections::HashMap<String, bool> {
                let mut builtin = std::collections::HashMap::new();
                $(
                    builtin.insert($name.to_string(), self.$field);
                )*
                builtin
            }
        }
        impl ToRumbas<BuiltinConstants> for numbas::question::constants::BuiltinConstants {
            fn to_rumbas(&self) -> BuiltinConstants {
                BuiltinConstants {
                $(
                    $field: *self.0.get(&$name.to_string()).unwrap_or(&DEFAULTS.$default)
                ),*
                }
            }
        }
    }
}

builtin_constants! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "BuiltinConstantsInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq,)]
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

// TODO: remove or attribute
#[cfg(test)]
mod example_test_builtin {
    use super::BuiltinConstantsInput;
    use rumbas_support::example::Examples;
    #[test]
    fn compile_examples() {
        for example in BuiltinConstantsInput::examples().into_iter() {
            println!("{:?}", example);
            let item = serde_yaml::to_string(&example);
            assert!(item.is_ok());
            let item = item.unwrap();
            insta::with_settings!({sort_maps => true}, {
                insta::assert_yaml_snapshot!(&example);
            });
            let parsed: Result<BuiltinConstantsInput, _> = serde_yaml::from_str(&item[..]);
            if let Err(ref e) = parsed {
                if "No field is set to a not-none value." == &e.to_string()[..] {
                    continue;
                }
                println!("Input {:?}", item);
                println!("Error: {:?}", e);
            }
            assert!(parsed.is_ok());
            assert_eq!(example, parsed.unwrap())
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CustomConstantInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
/// A custom constant
pub struct CustomConstant {
    /// The name of the constant
    pub name: String,
    /// The value of the constant
    pub value: JMEString,
    /// The tex code use to display the constant
    pub tex: String,
}

impl ToNumbas<numbas::question::constants::QuestionConstant> for CustomConstant {
    fn to_numbas(&self, locale: &str) -> numbas::question::constants::QuestionConstant {
        numbas::question::constants::QuestionConstant {
            name: self.name.to_numbas(locale),
            value: self.value.to_numbas(locale),
            tex: self.tex.to_numbas(locale),
        }
    }
}

impl ToRumbas<CustomConstant> for numbas::question::constants::QuestionConstant {
    fn to_rumbas(&self) -> CustomConstant {
        CustomConstant {
            name: self.name.to_rumbas(),
            value: self.value.to_rumbas(),
            tex: self.tex.to_rumbas(),
        }
    }
}

// TODO remove or from attribute
#[cfg(test)]
mod example_test_custom {
    use super::CustomConstantInput;
    use rumbas_support::example::Examples;
    #[test]
    fn compile_examples() {
        for example in CustomConstantInput::examples().into_iter() {
            println!("{:?}", example);
            let item = serde_yaml::to_string(&example);
            assert!(item.is_ok());
            let item = item.unwrap();
            insta::with_settings!({sort_maps => true}, {
                insta::assert_yaml_snapshot!(&example);
            });
            let parsed: Result<CustomConstantInput, _> = serde_yaml::from_str(&item[..]);
            if let Err(ref e) = parsed {
                if "No field is set to a not-none value." == &e.to_string()[..] {
                    continue;
                }
                println!("Input {:?}", item);
                println!("Error: {:?}", e);
            }
            assert!(parsed.is_ok());
            assert_eq!(example, parsed.unwrap())
        }
    }
}
