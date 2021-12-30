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
use crate::support::default::combine_exam_with_default_files;
use crate::support::file_manager::CACHE;
use crate::support::template::{TemplateFile, TemplateFileInputEnum};
use crate::support::to_numbas::ToNumbas;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "ExamInput")]
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
        let input: std::result::Result<ExamFileTypeInput, _> =
            if file.starts_with(crate::EXAMS_FOLDER) {
                let yaml = CACHE
                    .read_file(FileToLoad {
                        file_path: file.to_path_buf(),
                        locale_dependant: false,
                    })
                    .map(|lf| match lf {
                        LoadedFile::Normal(n) => Some(n.content.clone()),
                        LoadedFile::Localized(_) => None,
                    })
                    .flatten()
                    .ok_or(ParseError::FileReadError(FileReadError(file.to_path_buf())))?;

                serde_yaml::from_str(&yaml)
                    .map_err(|e| ParseError::YamlError(YamlError::from(e, file.to_path_buf())))
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
                Err(ParseError::InvalidPath(InvalidExamPathError(
                    file.to_path_buf(),
                )))
            };
        input
            .map(|e| match e {
                Normal(e) => Ok(ExamInput::Normal(e)),
                Diagnostic(e) => Ok(ExamInput::Diagnostic(e)),
                Template(t_val) => {
                    let t = t_val.to_normal();
                    let template_file = Path::new(crate::EXAM_TEMPLATES_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path)); // TODO: check for missing fields.....

                    let template_yaml = CACHE
                        .read_file(FileToLoad {
                            file_path: template_file.to_path_buf(),
                            locale_dependant: false,
                        })
                        .map(|lf| match lf {
                            LoadedFile::Normal(n) => Some(n.content.clone()),
                            LoadedFile::Localized(_) => None,
                        })
                        .flatten()
                        .ok_or(ParseError::FileReadError(FileReadError(
                            template_file.to_path_buf(),
                        )))?;

                    let mut exam: ExamInput =
                        serde_yaml::from_str(&template_yaml).map_err(|e| {
                            ParseError::YamlError(YamlError::from(e, file.to_path_buf()))
                        })?;
                    t.data.iter().for_each(|(k, v)| {
                        exam.insert_template_value(k, &v.0);
                    });
                    Ok(exam)
                }
            })
            .and_then(std::convert::identity) //flatten result is currently only possible in nightly
    }

    pub fn combine_with_defaults(&mut self, path: &Path) {
        combine_exam_with_default_files(path, self);

        let files_to_load = self.files_to_load();
        let loaded_files = CACHE.read(files_to_load);
        self.insert_loaded_files(&loaded_files);
    }
}

#[derive(Debug, Display)]
pub enum ParseError {
    YamlError(YamlError),
    FileReadError(FileReadError),
    InvalidPath(InvalidExamPathError),
}

#[derive(Debug)]
pub struct InvalidExamPathError(std::path::PathBuf);

impl Display for InvalidExamPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid compilation path: {} should start with {}/ or {}/",
            self.0.display(),
            crate::EXAMS_FOLDER,
            crate::QUESTIONS_FOLDER
        )
    }
}

#[derive(Debug)]
pub struct FileReadError(pub std::path::PathBuf);

impl Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed reading file: {}", self.0.display(),)
    }
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
