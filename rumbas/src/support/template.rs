use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const TEMPLATE_PREFIX: &str = "template";

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct TemplateData {
    #[serde(rename = "template")]
    pub relative_template_path: String,
    #[serde(flatten)]
    pub data: HashMap<String, MyYamlValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyYamlValue(pub serde_yaml::Value);

impl JsonSchema for MyYamlValue {
    fn schema_name() -> String {
        "YamlValue".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Bool(true)
    }
}

impl std::convert::From<serde_yaml::Value> for MyYamlValue {
    fn from(v: serde_yaml::Value) -> Self {
        Self(v)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct TemplateString {
    pub key: Option<String>,
    pub error_message: Option<String>,
}

impl RumbasCheck for TemplateString {
    fn check(&self, _locale: &str) -> RumbasCheckResult {
        if let Some(e) = &self.error_message {
            RumbasCheckResult::from_missing(Some(e.clone())) // TODO: seperate missing files? (also see FileString)
        } else {
            RumbasCheckResult::empty()
        }
    }
}

impl OptionalOverwrite<TemplateString> for TemplateString {
    fn overwrite(&mut self, _other: &TemplateString) {}
    fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
}
impl_optional_overwrite_value!(TemplateString);

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
//#[serde(try_from = "serde_yaml::Value")]
pub enum ValueType<T> {
    Template(TemplateString),
    Normal(T),
    Invalid(serde_yaml::Value),
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

impl<T: RumbasCheck> RumbasCheck for Value<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match &self.0 {
            Some(ValueType::Normal(val)) => val.check(locale),
            Some(ValueType::Template(ts)) => RumbasCheckResult::from_missing(Some(ts.yaml())),
            Some(ValueType::Invalid(v)) => RumbasCheckResult::from_invalid(v),
            None => RumbasCheckResult::from_missing(None),
        }
    }
}

impl<S, T: ToNumbas<S> + RumbasCheck> ToNumbas<S> for Value<T> {
    fn to_numbas(&self, locale: &str) -> S {
        match &self.0 {
            Some(ValueType::Normal(val)) => val.to_numbas(locale),
            Some(ValueType::Template(_ts)) => unreachable!(),
            Some(ValueType::Invalid(_v)) => unreachable!(),
            None => unreachable!(),
        }
    }
    fn to_numbas_with_name(&self, locale: &str, name: String) -> S {
        match &self.0 {
            Some(ValueType::Normal(val)) => val.to_numbas_with_name(locale, name),
            Some(ValueType::Template(_ts)) => unreachable!(),
            Some(ValueType::Invalid(_v)) => unreachable!(),
            None => unreachable!(),
        }
    }
}

impl<T, O: ToRumbas<T>> ToRumbas<Value<T>> for O {
    fn to_rumbas(&self) -> Value<T> {
        Value::Normal(self.to_rumbas())
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
