pub use crate::support::noneable::Noneable;

/// This macro creates a struct with all optional fields (see [Value] struct)
/// It also implements toe RumbasCheck and Overwrite traits
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
            impl Overwrite<[<$struct Input>]> for [<$struct Input>] {
                fn overwrite(&mut self, other: &[<$struct Input>]) {
                    $(
                        self.$field.overwrite(&other.$field);
                    )*
                }
            }
            #[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
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
                fn find_missing(&self) -> InputCheckResult {
                    let mut result = InputCheckResult::empty();
                    $(
                        {
                        let mut previous_result = self.$field.find_missing();
                        previous_result.extend_path(stringify!($field).to_string());
                        result.union(&previous_result);
                        }
                    )*
                    result
                }
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                    $(
                        self.$field.insert_template_value(&key, &val);
                    )*
                }
            }
            impl InputInverse for $struct {
                type Input = [<$struct Input>];
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
            impl Overwrite<[<$struct Input>]> for [<$struct Input>] {
                fn overwrite(&mut self, other: &[<$struct Input>]) {
                    self.0.overwrite(&other.0);
                }
            }
            #[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
            pub struct $struct(pub $type);
            impl Input for [<$struct Input>] {
                type Normal = $struct;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    $struct(self.0.to_normal())
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    Self(Value::Normal([<$type Input>]::from_normal(normal.0)))
                }
                fn find_missing(&self) -> InputCheckResult {
                    self.0.find_missing()
                }
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                    self.0.insert_template_value(&key, &val);
                }
            }
            impl InputInverse for $struct {
                type Input = [<$struct Input>];
            }
        }
    }
}

/// This macro creates an enum and implements toe RumbasCheck and Overwrite traits
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
            impl Overwrite<[<$enum Input>]> for [<$enum Input>] {
                fn overwrite(&mut self, other: &[<$enum Input>]) {
                    match (self, other) {
                    $(
                        (&mut [<$enum Input>]::$variant(ref mut val), &[<$enum Input>]::$variant(ref valo)) => val.overwrite(&valo)
                    ),*
                        , _ => ()
                    };
                }
            }
            #[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
            pub enum $enum {
                $(
                    $variant($type)
                ),*
            }
            impl Input for [<$enum Input>] {
                type Normal = $enum;
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value){
                    match self {
                    $(
                        &mut [<$enum Input>]::$variant(ref mut enum_val) => enum_val.insert_template_value(&key, &val)
                    ),*
                    };
                }
                fn find_missing(&self) -> InputCheckResult {
                    match self {
                        $(
                           [<$enum Input>]::$variant(val) => val.find_missing(),
                        )*
                    }
                }
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
            impl InputInverse for $enum {
                type Input = [<$enum Input>];
            }
        }
    }
}

/// Implement the RumbasCheck and Overwrite traits with blanket implementations
macro_rules! impl_optional_overwrite {
    ($($type: ident), *) => {
        $(
        impl RumbasCheck for $type {
            fn check(&self, _locale: &str) -> RumbasCheckResult {
                RumbasCheckResult::empty()
            }
        }
        impl Overwrite<$type> for $type {
            fn overwrite(&mut self, _other: &$type) {}
        }
        crate::support::rumbas_types::create_input_alias!($type, $type);
        paste::paste! {
            impl Input for [<$type Input>] {
                type Normal = $type;
            fn find_missing(&self) -> InputCheckResult {
                InputCheckResult::empty()
            }
            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
                fn to_normal(&self) -> <Self as Input>::Normal {
                    self.to_owned()
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    normal
                }
            }
            impl InputInverse for [<$type Input>] {
                type Input = $type;
            }
        }
        )*
    };
}

pub(crate) use impl_optional_overwrite;
pub(crate) use optional_overwrite;
pub(crate) use optional_overwrite_enum;
pub(crate) use optional_overwrite_newtype;
