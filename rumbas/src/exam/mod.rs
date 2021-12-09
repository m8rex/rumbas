//! Contains all the exam types

pub mod diagnostic;
pub mod feedback;
pub mod locale;
pub mod navigation;
pub mod normal;
pub mod numbas_settings;
pub mod question_group;
pub mod timing;

use crate::exam::diagnostic::convert_diagnostic_numbas_exam;
use crate::exam::diagnostic::DiagnosticExam;
use crate::exam::locale::Locale;
use crate::exam::normal::convert_normal_numbas_exam;
use crate::exam::normal::NormalExam;
use crate::exam::question_group::QuestionPath;
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::support::template::{TemplateFile, TemplateFileInputEnum};
use crate::support::to_numbas::ToNumbas;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "ExamInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Exam {
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}

impl ToNumbas<numbas::exam::Exam> for Exam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        match self {
            Exam::Normal(n) => n.to_numbas(locale),
            Exam::Diagnostic(n) => n.to_numbas(locale),
        }
    }
}

impl Exam {
    pub fn locales(&self) -> Vec<Locale> {
        match self {
            Exam::Normal(n) => n.locales.clone(),
            Exam::Diagnostic(n) => n.locales.clone(),
        }
    }

    pub fn numbas_settings(&self) -> crate::exam::numbas_settings::NumbasSettings {
        match self {
            Exam::Normal(n) => n.numbas_settings.clone(),
            Exam::Diagnostic(n) => n.numbas_settings.clone(),
        }
    }
}
impl ExamInput {
    pub fn from_file(file: &Path) -> Result<ExamInput, ParseError> {
        use ExamFileTypeInput::*;
        let input: std::result::Result<ExamFileTypeInput, serde_yaml::Error> =
            if file.starts_with(crate::EXAMS_FOLDER) {
                let yaml = fs::read_to_string(file).map_err(ParseError::IOError)?;
                serde_yaml::from_str(&yaml)
            } else if file.starts_with(crate::QUESTIONS_FOLDER) {
                let mut data = HashMap::new();
                data.insert(
                    "question".to_string(),
                    serde_yaml::Value::String(
                        file.with_extension("")
                            .strip_prefix(crate::QUESTIONS_FOLDER)
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    )
                    .into(),
                );
                let t = TemplateFile {
                    relative_template_path: crate::QUESTION_PREVIEW_TEMPLATE_NAME.to_string(),
                    data,
                };
                Ok(Template(TemplateFileInputEnum::from_normal(t)))
            } else {
                panic!(
                    "{} should start with {}/ or {}/",
                    file.display(),
                    crate::EXAMS_FOLDER,
                    crate::QUESTIONS_FOLDER
                );
            };
        input
            .map(|e| match e {
                Normal(e) => Ok(ExamInput::Normal(e)),
                Diagnostic(e) => Ok(ExamInput::Diagnostic(e)),
                Template(t_val) => {
                    let t = t_val.to_normal();
                    let template_file = Path::new(crate::EXAM_TEMPLATES_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path)); // TODO: check for missing fields.....
                    let template_yaml = fs::read_to_string(&template_file).expect(
                        &format!(
                            "Failed to read {}",
                            template_file.to_str().map_or("invalid filename", |s| s)
                        )[..],
                    );

                    let mut exam: ExamInput = serde_yaml::from_str(&template_yaml).unwrap();
                    t.data.iter().for_each(|(k, v)| {
                        exam.insert_template_value(k, &v.0);
                    });
                    Ok(exam)
                }
            })
            .and_then(std::convert::identity) //flatten result is currently only possible in nightly
            .map_err(|e| ParseError::YamlError(YamlError::from(e, file.to_path_buf())))
    }
}

#[derive(Debug, Display)]
pub enum ParseError {
    YamlError(YamlError),
    IOError(std::io::Error),
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "ExamFileTypeInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ExamFileType {
    Template(TemplateFile),
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}

impl ExamFileTypeInput {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl ExamFileType {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        ExamFileTypeInput::from_normal(self.to_owned()).to_yaml()
    }
}

/// Convert a numbas exam to rumbas data
/// Returns the name of the exam, the resulting exam (as ExamFileType)
/// and vectors of questions and custom part type definitions
pub fn convert_numbas_exam(
    exam: numbas::exam::Exam,
) -> (
    String,
    ExamFileType,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let (name, exam, qgs, cpts) = match exam.navigation.navigation_mode {
        numbas::exam::navigation::NavigationMode::Diagnostic(ref _d) => {
            let (exam, qgs, cpts) = convert_diagnostic_numbas_exam(exam);
            (exam.name.clone(), ExamFileType::Diagnostic(exam), qgs, cpts)
        }
        _ => {
            let (exam, qgs, cpts) = convert_normal_numbas_exam(exam);
            (exam.name.clone(), ExamFileType::Normal(exam), qgs, cpts)
        }
    };
    (
        {
            if let TranslatableString::NotTranslated(n) = name {
                n.get_content("").unwrap()
            } else {
                panic!("Should not happen");
            }
        },
        exam,
        qgs,
        cpts,
    )
}
