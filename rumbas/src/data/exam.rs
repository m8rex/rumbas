use crate::data::diagnostic_exam::DiagnosticExam;
use crate::data::locale::Locale;
use crate::data::normal_exam::NormalExam;
use crate::data::optional_overwrite::*;
use crate::data::template::{ExamFileType, TemplateData, Value, ValueType};
use crate::data::to_numbas::ToNumbas;
use crate::data::yaml::YamlError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;

optional_overwrite_enum! {
    #[serde(rename_all = "snake_case")]
    #[serde(tag = "type")]
    pub enum Exam {
        Normal(NormalExam),
        Diagnostic(DiagnosticExam)
    }
}

impl ToNumbas<numbas::exam::Exam> for Exam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        match self {
            Exam::Normal(n) => n.to_numbas(locale),
            Exam::Diagnostic(n) => n.to_numbas(locale),
        }
    }
}

#[derive(Debug, Display)]
pub enum ParseError {
    YamlError(YamlError),
    IOError(std::io::Error),
}

impl Exam {
    pub fn locales(&self) -> Value<Vec<Value<Locale>>> {
        match self {
            Exam::Normal(n) => n.locales.clone(),
            Exam::Diagnostic(n) => n.locales.clone(),
        }
    }

    pub fn numbas_settings(&self) -> Value<super::numbas_settings::NumbasSettings> {
        match self {
            Exam::Normal(n) => n.numbas_settings.clone(),
            Exam::Diagnostic(n) => n.numbas_settings.clone(),
        }
    }

    pub fn from_file(file: &Path) -> Result<Exam, ParseError> {
        use ExamFileType::*;
        let input: std::result::Result<ExamFileType, serde_yaml::Error> =
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
                let t = TemplateData {
                    relative_template_path: crate::QUESTION_PREVIEW_TEMPLATE_NAME.to_string(),
                    data,
                };
                Ok(Template(t))
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
                Normal(e) => Ok(Exam::Normal(e)),
                Diagnostic(e) => Ok(Exam::Diagnostic(e)),
                Template(t) => {
                    let template_file = Path::new(crate::EXAM_TEMPLATES_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path));
                    let template_yaml = fs::read_to_string(&template_file).expect(
                        &format!(
                            "Failed to read {}",
                            template_file.to_str().map_or("invalid filename", |s| s)
                        )[..],
                    );

                    let mut exam: Exam = serde_yaml::from_str(&template_yaml).unwrap();
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
