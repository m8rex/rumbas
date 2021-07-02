use crate::data::diagnostic_exam::DiagnosticExam;
use crate::data::normal_exam::NormalExam;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question::Question;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const TEMPLATE_EXAMS_FOLDER: &'static str = "template_exams";
pub const TEMPLATE_QUESTIONS_FOLDER: &'static str = "template_questions";
pub const TEMPLATE_PREFIX: &'static str = "template";

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateData {
    #[serde(rename = "template")]
    pub relative_template_path: String,
    #[serde(flatten)]
    pub data: HashMap<String, serde_yaml::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ExamFileType {
    Template(TemplateData),
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum QuestionFileType {
    Template(TemplateData),
    Normal(Question),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct TemplateString {
    pub key: Option<String>,
    pub error_message: Option<String>,
}
impl OptionalOverwrite for TemplateString {
    type Item = TemplateString;
    fn empty_fields(&self) -> Vec<String> {
        if let Some(e) = &self.error_message {
            vec![e.clone()]
        } else {
            Vec::new()
        }
    }
    fn overwrite(&mut self, _other: &Self::Item) {}
    fn insert_template_value(&mut self, _key: &String, _val: &serde_yaml::Value) {}
}
impl_optional_overwrite_value!(TemplateString);

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
        prefix.push_str(":");
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct Value<T>(pub Option<ValueType<T>>);

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

impl<T: std::clone::Clone> Value<T> {
    #[inline]
    pub fn unwrap(&self) -> T {
        self.clone().0.unwrap().unwrap()
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
        }
    }
}

impl<T: std::clone::Clone> Value<T> {
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        self.0.unwrap().map(f)
    }
}

impl<T: std::clone::Clone> ValueType<T> {
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        match self {
            ValueType::Normal(val) => Some(f(val)),
            ValueType::Template(ts) => panic!("missing value for template key {}", ts.key.unwrap()),
        }
    }
}
