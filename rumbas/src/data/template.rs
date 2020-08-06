use crate::data::exam::Exam;
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
#[serde(untagged)]
pub enum ExamFileType {
    Template(TemplateData),
    Normal(Exam),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
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
    fn insert_template_value(&mut self, key: &String, val: &serde_yaml::Value) {}
}
impl_optional_overwrite_value!(TemplateString);

impl TemplateString {
    pub fn yaml(&self) -> String {
        format!("{}:{}", TEMPLATE_PREFIX, self.key.clone().unwrap())
    }
}

//TODO: error message is not shown if no file found
impl std::convert::From<String> for TemplateString {
    fn from(s: String) -> Self {
        let mut prefix = TEMPLATE_PREFIX.to_owned();
        prefix.push_str(":");
        if s.starts_with(&prefix) {
            if s == prefix {
                TemplateString {
                    key: Some("".to_string()),
                    error_message: Some("Missing template key".to_string()),
                }
            } else {
                let key = s.split(&prefix).collect::<Vec<&str>>()[1];
                TemplateString {
                    key: Some(key.to_string()),
                    error_message: None,
                }
            }
        } else {
            //TODO
            TemplateString {
                key: Some("".to_string()),
                error_message: Some("Temp error!".to_string()),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Value<T> {
    Template(TemplateString),
    Normal(T),
    None,
}

impl<T> std::convert::From<Value<T>> for Option<T> {
    fn from(value: Value<T>) -> Option<T> {
        match value {
            Value::Normal(val) => Some(val),
            Value::Template(ts) => panic!(format!(
                "missing value for template key {}",
                ts.key.unwrap()
            )),
            Value::None => None,
        }
    }
}

impl<T: std::clone::Clone> Value<T> {
    /*  #[inline]
    pub fn to_option(self) -> Option<T> {
        Option::from(self)
    }*/

    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        Option::from(self).map(f)
        /*match self {
            Value::Normal(val) => Some(f(val)),
            Value::Template(ts) => panic!(format!(
                "missing value for template key {}",
                ts.key.unwrap()
            )),
            Value::None => None,
        }*/ //TODO
    }

    #[inline]
    pub fn unwrap(&self) -> T {
        match self {
            Value::Normal(val) => val.clone(),
            Value::Template(ts) => panic!(format!(
                "missing value for template key {}",
                ts.key.clone().unwrap()
            )),
            Value::None => panic!("called `Value::unwrap()` on a `None` value"),
        }
    }
}
