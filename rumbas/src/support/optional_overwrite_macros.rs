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
        paste::paste!{
            #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
            $(
                #[$outer]
            )*
            pub struct [<$struct Input>] {
                $(
                    $(#[$inner])*
                    pub $field: Value<[<$type Input>]>
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
            impl OptionalCheck for [<$struct Input>] {
                fn find_missing(&self) -> OptionalCheckResult {
                    let mut result = OptionalCheckResult::empty();
                    $(
                        {
                        let mut previous_result = self.$field.find_missing();
                        previous_result.extend_path(stringify!($field).to_string());
                        result.union(&previous_result);
                        }
                    )*
                    result
                }
            }
            impl OptionalOverwrite<[<$struct Input>]> for [<$struct Input>] {
                fn overwrite(&mut self, other: &[<$struct Input>]) {
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
            #[derive(Debug, Clone)]
            pub struct $struct {
                $(
                    pub $field: $type
                ),*
            }
            impl Input for [<$struct Input>] {
                type Normal = $struct;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    Self::Normal {
                        $(
                        $field: self.$field.to_normal()
                        ),*
                    }
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    Self {
                        $(
                        $field: Value::Normal([<$type Input>]::from_normal(normal.$field))
                        ),*
                    }
                }
            }

        }
    }
}

macro_rules! optional_overwrite_newtype {
    (
        $(#[$outer:meta])*
        pub struct $struct: ident($type: ty)
    ) => {
        paste::paste!{
            #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
            $(
                #[$outer]
            )*
            pub struct [<$struct Input>](pub Value<[<$type Input>]>);
            impl RumbasCheck for $struct {
                fn check(&self, locale: &str) -> RumbasCheckResult {
                    self.0.check(locale)
                }
            }
            impl OptionalCheck for [<$struct Input>] {
                fn find_missing(&self) -> OptionalCheckResult {
                    self.0.find_missing()
                }
            }
            impl OptionalOverwrite<[<$struct Input>]> for [<$struct Input>] {
                fn overwrite(&mut self, other: &[<$struct Input>]) {
                    self.0.overwrite(&other.0);
                }
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                    self.0.insert_template_value(&key, &val);
                }
            }
            #[derive(Debug, Clone)]
            pub struct $struct(pub $type);
            impl Input for [<$struct Input>] {
                type Normal = $struct;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    $struct(self.0.to_normal())
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    Self(Value::Normal([<$type Input>]::from_normal(normal.0)))
                }
            }

        }
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
                $variant: ident($type: ty)
            ),+
        }
    ) => {
        paste::paste!{
            #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
            $(
                #[$outer]
            )*
            pub enum [<$enum Input>] {
                $(
                    $(
                        #[$inner]
                    )*
                    $variant([<$type Input>])
                ),*
            }
            impl RumbasCheck for $enum {
                fn check(&self, locale: &str) -> RumbasCheckResult {
                    match self {
                        $(
                           $enum::$variant(val) => val.check(locale),
                        )*
                    }
                }
            }
            impl OptionalCheck for [<$enum Input>] {
                fn find_missing(&self) -> OptionalCheckResult {
                    match self {
                        $(
                           [<$enum Input>]::$variant(val) => val.find_missing(),
                        )*
                    }
                }
            }
            impl OptionalOverwrite<[<$enum Input>]> for [<$enum Input>] {
                fn overwrite(&mut self, other: &[<$enum Input>]) {
                    match (self, other) {
                    $(
                        (&mut [<$enum Input>]::$variant(ref mut val), &[<$enum Input>]::$variant(ref valo)) => val.overwrite(&valo)
                    ),*
                        , _ => ()
                    };
                }
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                    match self {
                    $(
                        &mut [<$enum Input>]::$variant(ref mut enum_val) => enum_val.insert_template_value(&key, &val)
                    ),*
                    };
                }
            }
            #[derive(Debug, Clone)]
            pub enum $enum {
                $(
                    $variant($type)
                ),*
            }
            impl Input for [<$enum Input>] {
                type Normal = $enum;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    match self {
                        $(
                        Self::$variant(a) => $enum::$variant(a.to_normal())
                        ),*
                    }
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    match normal {
                        $(
                        $enum::$variant(a) => [<$enum Input>]::$variant([<$type Input>]::from_normal(a))
                        ),*

                    }
                }
            }
        }
    }
}

/// Implement the RumbasCheck and OptionalOverwrite traits with blanket implementations
macro_rules! impl_optional_overwrite {
    ($($type: ident), *) => {
        $(
        impl RumbasCheck for $type {
            fn check(&self, _locale: &str) -> RumbasCheckResult {
                RumbasCheckResult::empty()
            }
        }
        impl OptionalCheck for $type {
            fn find_missing(&self) -> OptionalCheckResult {
                OptionalCheckResult::empty()
            }
        }
        impl OptionalOverwrite<$type> for $type {
            fn overwrite(&mut self, _other: &$type) {}
            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
        }
        crate::support::rumbas_types::create_input_alias!($type, $type);
        paste::paste! {
            impl Input for [<$type Input>] {
                type Normal = $type;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    self.to_owned()
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    normal
                }
            }
        }
        )*
    };
}

pub(crate) use impl_optional_overwrite;
pub(crate) use optional_overwrite;
pub(crate) use optional_overwrite_enum;
pub(crate) use optional_overwrite_newtype;
