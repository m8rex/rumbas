pub use crate::support::noneable::Noneable;
pub use crate::support::rumbas_check::{RumbasCheck, RumbasCheckResult};

/// This macro creates a struct with all optional fields (see [Value] struct)
/// It also implements toe RumbasCheck and OptionalOverwrite traits
/// on both the created struct as Value<struct>
macro_rules! optional_overwrite {
    (
        $(#[$outer:meta])*
        pub struct $struct: ident {
            $(
                $(#[$inner:meta])*
                $field: ident: $type: ty
            ),+
        }
    ) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
        $(
            #[$outer]
        )*
        pub struct $struct {
            $(
                $(#[$inner])*
                pub $field: Value<$type>
            ),*
        }
        impl RumbasCheck for $struct {
            fn check(&self, locale: &str) -> RumbasCheckResult {
                let mut result = RumbasCheckResult::empty();
                $(
                    {
                    let mut previous_result = self.$field.check(locale);
                    previous_result.extend_path(stringify!($field).to_string());
                    result.union(&previous_result);
                    }
                )*
                result
            }
        }
        impl OptionalOverwrite<$struct> for $struct {
            fn overwrite(&mut self, other: &$struct) {
                $(
                    self.$field.overwrite(&other.$field);
                )*
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                $(
                    self.$field.insert_template_value(&key, &val);
                )*
            }
        }

        impl std::convert::From<$struct> for Value<$struct> {
            fn from(val: $struct) -> Self {
                Value::Normal(val)
            }
        }
        impl_optional_overwrite_value!($struct);
    }
}

/// This macro creates an enum and implements toe RumbasCheck and OptionalOverwrite traits
/// on both the created enum as Value<enum>
macro_rules! optional_overwrite_enum {
    (

        $(#[$outer:meta])*
        pub enum $enum: ident {
            $(
                $(#[$inner:meta])*
                $field: ident($type: ty)
            ),+
        }
    ) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
        $(
            #[$outer]
        )*
        pub enum $enum {
            $(
                $(
                    #[$inner]
                )*
                $field($type)
            ),*
        }
        impl RumbasCheck for $enum {
            fn check(&self, locale: &str) -> RumbasCheckResult {
                match self {
                $(
                    $enum::$field(val) => val.check(locale)
                ),*
                }
            }
        }
        impl OptionalOverwrite<$enum> for $enum {
            fn overwrite(&mut self, other: &$enum) {
                match (self, other) {
                $(
                    (&mut $enum::$field(ref mut val), &$enum::$field(ref valo)) => val.overwrite(&valo)
                ),*
                    , _ => ()
                };
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                match self {
                $(
                    &mut $enum::$field(ref mut enum_val) => enum_val.insert_template_value(&key, &val)
                ),*
                };
            }
        }
        impl_optional_overwrite_value!($enum);
    }
}

macro_rules! impl_optional_overwrite_value_only {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl$(< $($gen: OptionalOverwrite<$gen> + DeserializeOwned),* >)? OptionalOverwrite<Value<$type>> for Value<$type> {
            fn overwrite(&mut self, other: &Value<$type>) {
                if let Some(ValueType::Normal(ref mut val)) = self.0 {
                    if let Some(ValueType::Normal(other_val)) = &other.0 {
                        val.overwrite(&other_val);
                    }
                } else if self.0.is_none() {
                    *self = other.clone();
                }
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                if let Some(ValueType::Template(ts)) = &self.0 {
                    if ts.key == Some(key.to_string()) {
                        *self=Value::Normal(serde_yaml::from_value(val.clone()).unwrap());
                    }
                } else if let Some(ValueType::Normal(ref mut v)) = &mut self.0 {
                    v.insert_template_value(key, val);
                }
            }
        }

        )*
    };
}

macro_rules! impl_optional_overwrite_value {
    ($($type: ty$([$($gen: tt), *])?), *) => {
        $(
        impl_optional_overwrite_value_only!($type$([ $($gen),* ])?);
        impl$(< $($gen: OptionalOverwrite<$gen>),* >)? OptionalOverwrite<Noneable<$type>> for Noneable<$type> {
            fn overwrite(&mut self, other: &Noneable<$type>) {
                if let Noneable::NotNone(ref mut val) = self {
                    if let Noneable::NotNone(other_val) = &other {
                        val.overwrite(&other_val);
                    }
                } else {
                    // Do nothing, none is a valid value
                }
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                if let Noneable::NotNone(item) = self {
                    item.insert_template_value(&key, &val);
                }
            }
        }
        impl_optional_overwrite_value_only!(Noneable<$type>$([ $($gen),* ])?);
        )*
    };
}

/// Implement the RumbasCheck and OptionalOverwrite traits with blanket implementations
macro_rules! impl_optional_overwrite {
    ($($type: ty), *) => {
        $(
        impl RumbasCheck for $type {
            fn check(&self, _locale: &str) -> RumbasCheckResult {
                RumbasCheckResult::empty()
            }
        }
        impl OptionalOverwrite<$type> for $type {
            fn overwrite(&mut self, _other: &$type) {}
            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
        }
        impl_optional_overwrite_value!($type);
        )*
    };
}

pub(crate) use impl_optional_overwrite;
pub(crate) use impl_optional_overwrite_value;
pub(crate) use impl_optional_overwrite_value_only;
pub(crate) use optional_overwrite;
pub(crate) use optional_overwrite_enum;
