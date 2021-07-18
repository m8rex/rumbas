use crate::data::diagnostic_exam::DiagnosticExam;
use crate::data::locale::Locale;
use crate::data::normal_exam::NormalExam;
use crate::data::optional_overwrite::*;
use crate::data::template::{ExamFileType, TemplateData, Value, ValueType, TEMPLATE_EXAMS_FOLDER};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::yaml::{YamlError, YamlResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Exam {
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}
impl_optional_overwrite!(Exam); // TODO?

impl ToNumbas for Exam {
    type NumbasType = numbas::exam::Exam;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::Exam> {
        match self {
            Exam::Normal(n) => n.to_numbas(locale),
            Exam::Diagnostic(n) => n.to_numbas(locale),
        }
    }
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

    pub fn from_file(file: &Path) -> YamlResult<Exam> {
        use ExamFileType::*;
        let input: std::result::Result<ExamFileType, serde_yaml::Error> =
            if file.starts_with("exams") {
                //TODO: better solutions? Use static values for "exams"
                let yaml = fs::read_to_string(file).expect(
                    &format!(
                        "Failed to read {}",
                        file.to_str().map_or("invalid filename", |s| s)
                    )[..],
                );
                serde_yaml::from_str(&yaml)
            } else if file.starts_with("questions") {
                //TODO: improve this part
                let mut data = HashMap::new();
                data.insert(
                    "question".to_string(),
                    serde_yaml::Value::String(
                        file.with_extension("")
                            .strip_prefix("questions")
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    ),
                );
                let t = TemplateData {
                    relative_template_path: "question_preview".to_string(), //TODO
                    data,
                };
                Ok(Template(t))
            } else {
                panic!("{} should start with questions/ or exams/", file.display());
            };
        input
            .map(|e| match e {
                Normal(e) => Ok(Exam::Normal(e)),
                Diagnostic(e) => Ok(Exam::Diagnostic(e)),
                Template(t) => {
                    let template_file = Path::new(TEMPLATE_EXAMS_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path));
                    let template_yaml = fs::read_to_string(&template_file).expect(
                        &format!(
                            "Failed to read {}",
                            template_file.to_str().map_or("invalid filename", |s| s)
                        )[..],
                    );

                    let mut exam: Exam = serde_yaml::from_str(&template_yaml).unwrap();
                    t.data.iter().for_each(|(k, v)| {
                        exam.insert_template_value(k, v);
                    });
                    Ok(exam)
                }
            })
            .and_then(std::convert::identity) //flatten result is currently only possible in nightly
            .map_err(|e| YamlError::from(e, file.to_path_buf()))
    }
}
