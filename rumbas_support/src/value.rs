use crate::input::*;
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

impl<T: Input> InputInverse for ValueType<T> {
    type Input = Self;
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
            ValueType::Invalid(v) => InputCheckResult::from_invalid(v),
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        if let ValueType::Template(ts) = &self {
            if ts.key == Some(key.to_string()) {
                *self = ValueType::Normal(serde_yaml::from_value(val.clone()).unwrap());
            }
        } else if let ValueType::Normal(ref mut v) = self {
            v.insert_template_value(key, val);
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

impl<T: Input> InputInverse for Value<T> {
    type Input = Self;
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct TemplateString {
    pub key: Option<String>,
    pub error_message: Option<String>,
}

impl InputInverse for TemplateString {
    type Input = Self;
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
