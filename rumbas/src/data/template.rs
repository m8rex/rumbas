use crate::data::exam::Exam;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap as Map;

#[derive(Serialize, Deserialize)]
pub struct TemplateData {
    #[serde(rename = "template")]
    pub relative_template_path: String,
    #[serde(flatten)]
    pub data: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExamFileType {
    Template(TemplateData),
    Normal(Exam),
}
