use crate::input::*;
use comparable::Comparable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const TEMPLATE_PREFIX: &str = "template";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValueType<T> {
    Template(TemplateString),
    Normal(T),
    Invalid(serde_yaml::Value),
}

#[derive(PartialEq, Debug)]
pub enum ValueTypeDesc<T: Comparable + PartialEq + std::fmt::Debug> {
    Template(<TemplateString as Comparable>::Desc),
    Normal(<T as Comparable>::Desc),
    Invalid,
}

#[derive(PartialEq, Debug)]
pub enum ValueTypeChange<T: Comparable + PartialEq + std::fmt::Debug> {
    BothTemplate(<TemplateString as comparable::Comparable>::Change),
    BothNormal(<T as comparable::Comparable>::Change),
    BothInvalid,
    Different(
        <ValueType<T> as comparable::Comparable>::Desc,
        <ValueType<T> as comparable::Comparable>::Desc,
    ),
}

impl<T: Comparable + PartialEq + std::fmt::Debug> comparable::Comparable for ValueType<T> {
    type Desc = ValueTypeDesc<T>;
    fn describe(&self) -> Self::Desc {
        match self {
            ValueType::Template(var0) => ValueTypeDesc::Template(var0.describe()),
            ValueType::Normal(var0) => ValueTypeDesc::Normal(var0.describe()),
            ValueType::Invalid(_var0) => ValueTypeDesc::Invalid,
        }
    }
    type Change = ValueTypeChange<T>;
    fn comparison(&self, other: &Self) -> comparable::Changed<Self::Change> {
        match (self, other) {
            (ValueType::Template(self_var0), ValueType::Template(other_var0)) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(ValueTypeChange::BothTemplate)
            }
            (ValueType::Normal(self_var0), ValueType::Normal(other_var0)) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(ValueTypeChange::BothNormal)
            }
            (ValueType::Invalid(self_var0), ValueType::Invalid(other_var0)) => {
                if self_var0 == other_var0 {
                    comparable::Changed::Unchanged
                } else {
                    comparable::Changed::Changed(ValueTypeChange::BothInvalid)
                }
            }
            (_, _) => comparable::Changed::Changed(ValueTypeChange::Different(
                self.describe(),
                other.describe(),
            )),
        }
    }
}

impl<T: Input> InputInverse for ValueType<T> {
    type Input = Self;
    type EnumInput = Self::Input;
}

impl<T: Input> Input for ValueType<T>
where
    T: serde::de::DeserializeOwned,
{
    type Normal = <T as Input>::Normal;
    fn to_normal(&self) -> <Self as Input>::Normal {
        self.clone().unwrap().to_normal()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        ValueType::Normal(<T as Input>::from_normal(normal))
    }
    fn find_missing(&self) -> InputCheckResult {
        match &self {
            ValueType::Normal(val) => val.find_missing(),
            ValueType::Template(ts) => InputCheckResult::from_missing(Some(ts.yaml())),
            ValueType::Invalid(v) => {
                let parsing: Result<T, _> = serde_yaml::from_value(v.clone());
                InputCheckResult::from_invalid(v, parsing.err())
            }
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        if let ValueType::Template(ts) = &self {
            if ts.key == Some(key.to_string()) {
                if let Ok(v) = serde_yaml::from_value(val.clone()) {
                    *self = ValueType::Normal(v);
                } else {
                    *self = ValueType::Invalid(val.clone());
                }
            }
        } else if let ValueType::Normal(ref mut v) = self {
            v.insert_template_value(key, val);
        }
    }

    fn files_to_load(&self) -> Vec<FileToLoad> {
        match &self {
            ValueType::Normal(val) => val.files_to_load(),
            ValueType::Template(ts) => ts.files_to_load(),
            ValueType::Invalid(_) => vec![],
        }
    }

    fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
        match self {
            ValueType::Normal(ref mut val) => val.insert_loaded_files(files),
            ValueType::Template(ref mut ts) => ts.insert_loaded_files(files),
            ValueType::Invalid(_v) => (),
        }
    }

    fn dependencies(&self) -> std::collections::HashSet<std::path::PathBuf> {
        match &self {
            ValueType::Normal(val) => val.dependencies(),
            ValueType::Template(ts) => ts.dependencies(),
            ValueType::Invalid(_) => std::collections::HashSet::new(),
        }
    }
}

impl<T: std::clone::Clone> ValueType<T> {
    #[inline]
    pub fn unwrap(&self) -> T {
        match self {
            ValueType::Normal(val) => val.to_owned(),
            ValueType::Template(ts) => {
                panic!("missing value for template key {}", ts.clone().key.unwrap())
            }
            ValueType::Invalid(v) => match serde_yaml::to_string(v) {
                Ok(s) => panic!("invalid yaml in part {}", s),
                _ => panic!("invalid yaml"),
            },
        }
    }
}

impl<T: std::clone::Clone> ValueType<T> {
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        match self {
            ValueType::Normal(val) => Some(f(val)),
            ValueType::Template(ts) => panic!("missing value for template key {}", ts.key.unwrap()),
            ValueType::Invalid(v) => match serde_yaml::to_string(&v) {
                Ok(s) => panic!("invalid yaml in part {}", s),
                _ => panic!("invalid yaml"),
            },
        }
    }
}

impl<T: std::clone::Clone> ValueType<T> {
    #[inline]
    pub fn real_map<U, F: FnOnce(T) -> U>(self, f: F) -> ValueType<U> {
        match self {
            ValueType::Normal(val) => ValueType::Normal(f(val)),
            ValueType::Template(ts) => ValueType::Template(ts),
            ValueType::Invalid(v) => ValueType::Invalid(v),
        }
    }
}

mod value_type_schema {
    use super::{TemplateString, ValueType};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
    #[serde(untagged)]
    enum ValidValueType<T> {
        Template(TemplateString),
        Normal(T),
    }

    impl<T: JsonSchema> JsonSchema for ValueType<T> {
        fn schema_name() -> String {
            format!("ValueType_{}", T::schema_name())
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            gen.subschema_for::<ValidValueType<T>>()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct Value<T>(pub Option<ValueType<T>>);

#[derive(Debug, PartialEq)]
pub struct ValueDesc<T: Comparable + PartialEq + std::fmt::Debug>(
    pub <Option<ValueType<T>> as comparable::Comparable>::Desc,
);
#[derive(Debug, PartialEq)]
pub struct ValueChange<T: Comparable + PartialEq + std::fmt::Debug>(
    pub <Option<ValueType<T>> as comparable::Comparable>::Change,
);
impl<T: Comparable + PartialEq + std::fmt::Debug> comparable::Comparable for Value<T> {
    type Desc = ValueDesc<T>;
    fn describe(&self) -> Self::Desc {
        ValueDesc(self.0.describe())
    }
    type Change = ValueChange<T>;
    fn comparison(&self, other: &Self) -> comparable::Changed<Self::Change> {
        self.0.comparison(&other.0).map(ValueChange)
    }
}

impl<T: Input> InputInverse for Value<T> {
    type Input = Self;
    type EnumInput = Self::Input;
}
impl<T: Input> Input for Value<T>
where
    T: serde::de::DeserializeOwned,
{
    type Normal = <T as Input>::Normal;
    fn to_normal(&self) -> Self::Normal {
        self.clone().unwrap().to_normal()
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Value::Normal(<T as Input>::from_normal(normal))
    }
    fn find_missing(&self) -> InputCheckResult {
        match &self.0 {
            Some(v) => v.find_missing(),
            None => InputCheckResult::from_missing(None),
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        if let Some(ref mut v) = self.0 {
            v.insert_template_value(key, val);
        }
    }
    fn files_to_load(&self) -> Vec<FileToLoad> {
        match &self.0 {
            Some(v) => v.files_to_load(),
            None => vec![],
        }
    }
    fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
        if let Some(ref mut v) = self.0 {
            v.insert_loaded_files(files);
        }
    }

    fn dependencies(&self) -> std::collections::HashSet<std::path::PathBuf> {
        match &self.0 {
            Some(v) => v.dependencies(),
            None => std::collections::HashSet::new(),
        }
    }
}

impl<T: JsonSchema> JsonSchema for Value<T> {
    fn schema_name() -> String {
        format!("Value_{}", T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<ValueType<T>>() // Didn't add the option
    }
}

impl<T> Value<T> {
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

impl<T: std::clone::Clone> Value<T> {
    #[inline]
    pub fn unwrap_or(&self, default: T) -> T {
        self.clone()
            .0
            .unwrap_or(ValueType::Normal(default))
            .unwrap()
    }
}

impl<T: std::clone::Clone> Value<T> {
    #[inline]
    pub fn unwrap(&self) -> T {
        self.clone().0.unwrap().unwrap()
    }
}

impl<T: std::clone::Clone> Value<T> {
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        self.0.unwrap().map(f)
    }
}

impl<T: std::clone::Clone> Value<T> {
    #[inline]
    pub fn real_map<U, F: FnOnce(T) -> U>(self, f: F) -> Value<U> {
        Value(self.0.map(|v| v.real_map(f)))
    }
}

impl<T> Value<T> {
    #[inline]
    #[allow(non_snake_case)]
    pub fn Normal(val: T) -> Value<T> {
        Value(Some(ValueType::Normal(val)))
    }
    #[inline]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn Template(ts: TemplateString) -> Value<T> {
        Value(Some(ValueType::Template(ts)))
    }
    #[inline]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn None() -> Value<T> {
        Value(None)
    }
}

impl<T> Default for Value<T> {
    fn default() -> Self {
        Value(None)
    }
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct TemplateString {
    pub key: Option<String>,
    pub error_message: Option<String>,
}

impl InputInverse for TemplateString {
    type Input = Self;
    type EnumInput = Self::Input;
}
impl Input for TemplateString {
    type Normal = TemplateString;
    fn to_normal(&self) -> Self::Normal {
        self.to_owned()
    }
    fn from_normal(normal: Self::Normal) -> Self {
        normal
    }
    fn find_missing(&self) -> InputCheckResult {
        if let Some(e) = &self.error_message {
            InputCheckResult::from_missing(Some(e.clone())) // TODO: seperate missing files? (also see FileString)
        } else {
            InputCheckResult::empty()
        }
    }
    fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}

    fn files_to_load(&self) -> Vec<FileToLoad> {
        Vec::new()
    }

    fn insert_loaded_files(&mut self, _files: &std::collections::HashMap<FileToLoad, LoadedFile>) {}

    fn dependencies(&self) -> std::collections::HashSet<std::path::PathBuf> {
        std::collections::HashSet::new()
    }
}

impl JsonSchema for TemplateString {
    fn schema_name() -> String {
        "TemplateString".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            string: Some(Box::new(schemars::schema::StringValidation {
                min_length: Some(1 + (TEMPLATE_PREFIX.len() as u32)),
                max_length: None,
                pattern: Some(format!("^{}:.*$", TEMPLATE_PREFIX)),
            })),
            ..Default::default()
        }
        .into()
    }
}

impl TemplateString {
    pub fn yaml(&self) -> String {
        format!("{}:{}", TEMPLATE_PREFIX, self.key.clone().unwrap())
    }
}

//TODO: error message is not shown if no file found
impl std::convert::TryFrom<String> for TemplateString {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut prefix = TEMPLATE_PREFIX.to_owned();
        prefix.push(':');
        if s.starts_with(&prefix) {
            if s == prefix {
                Ok(TemplateString {
                    key: Some("".to_string()),
                    error_message: Some("Missing template key".to_string()),
                })
            } else {
                let key = s.split(&prefix).collect::<Vec<&str>>()[1];
                Ok(TemplateString {
                    key: Some(key.to_string()),
                    error_message: None,
                })
            }
        } else {
            Err(format!("String does not start with {}", prefix))
        }
    }
}

impl std::convert::From<TemplateString> for String {
    fn from(ts: TemplateString) -> Self {
        ts.yaml()
    }
}
