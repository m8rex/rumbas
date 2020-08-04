use crate::data::exam::Exam;
use crate::data::question::Question;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;

pub const TEMPLATE_EXAMS_FOLDER: &'static str = "template_exams";
pub const TEMPLATE_QUESTIONS_FOLDER: &'static str = "template_questions";
pub const TEMPLATE_PREFIX: &'static str = "template";

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateData {
    #[serde(rename = "template")]
    pub relative_template_path: String,
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
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
