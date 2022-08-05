pub mod diagnostic;
pub mod feedback;
pub mod navigation;
pub mod question_group;
pub mod timing;

use crate::question::custom_part_type::CustomPartType;
use crate::question::function::Function;
use crate::question::resource::Resource;
use crate::question::variable::Variable;
use crate::support::serde_functions::from_str_optional;
use diagnostic::Diagnostic;
use feedback::Feedback;
use navigation::Navigation;
use question_group::QuestionGroup;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use timing::Timing;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Exam {
    #[serde(flatten)]
    pub basic_settings: BasicExamSettings,
    #[serde(default)]
    pub resources: Vec<Resource>,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub custom_part_types: Vec<CustomPartType>,

    pub navigation: Navigation,
    pub timing: Timing,
    pub feedback: Feedback,

    //rulesets: HashMap<String, String>, //TODO + Type
    #[serde(default)]
    pub functions: HashMap<String, Function>,
    #[serde(default)]
    pub variables: HashMap<String, Variable>,
    #[serde(default)]
    pub question_groups: Vec<QuestionGroup>,
    //contributors TODO
    //metadata TODO
    pub diagnostic: Option<Diagnostic>,
}

pub fn hacky_fix_exam(s: &str) -> String {
    s.replace("\"checkingtype\":", "\"checkingType\":") // Can't use alias because it uses tag
}

pub enum WriteResult {
    Ok,
    IOError(std::io::Error),
    JSONError(serde_json::Error),
}

impl Exam {
    pub fn from_exam_str(s: &str) -> serde_json::Result<Exam> {
        let json = if s.starts_with("// Numbas version: exam_results_page_options") {
            s.splitn(2, '\n').collect::<Vec<_>>()[1]
        } else {
            s
        };
        let json = hacky_fix_exam(json);
        serde_json::from_str(json.as_str())
    }

    pub fn write(&self, file_name: &str) -> WriteResult {
        match serde_json::to_string(self) {
            Ok(s) => match std::fs::write(
                file_name,
                format!(
                    r#"// Numbas version: exam_results_page_options
{}"#,
                    s
                ),
            ) {
                Ok(_) => WriteResult::Ok,
                Err(e) => WriteResult::IOError(e),
            },
            Err(e) => WriteResult::JSONError(e),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct BasicExamSettings {
    pub name: String,
    #[serde(rename = "duration")]
    #[serde(default)]
    pub duration_in_seconds: usize,
    #[serde(rename = "percentPass", deserialize_with = "from_str_optional")]
    pub percentage_needed_to_pass: Option<f64>,
    #[serde(rename = "showQuestionGroupNames")]
    #[serde(default)]
    pub show_question_group_names: bool,
    #[serde(rename = "showstudentname")]
    #[serde(default = "crate::util::bool_true")]
    pub show_student_name: bool,
    #[serde(rename = "allowPrinting")]
    #[serde(default = "crate::util::bool_true")]
    /// Whether students are ammpwed to print an exam transcript
    pub allow_printing: bool,
}
