use crate::data::extension::Extensions;
use crate::data::feedback::Feedback;
use crate::data::locale::Locale;
use crate::data::navigation::Navigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_group::QuestionGroup;
use crate::data::template::{ExamFileType, TemplateData, TEMPLATE_EXAMS_FOLDER, TEMPLATE_PREFIX};
use crate::data::template::{Value, ValueType};
use crate::data::timing::Timing;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use crate::data::yaml::{YamlError, YamlResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

optional_overwrite! {
    Exam,
    locales: Vec<Value<Locale>>,
    name: TranslatableString,
    navigation: Navigation,
    timing: Timing,
    feedback: Feedback,
    question_groups: Vec<Value<QuestionGroup>>, //TODO: remove?
    numbas_settings: NumbasSettings
}

impl ToNumbas for Exam {
    type NumbasType = numbas::exam::Exam;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::Exam> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let basic_settings = numbas::exam::BasicExamSettings::new(
                self.name.clone().unwrap().to_string(locale).unwrap(), //TODO: might fail, not checked
                self.timing
                    .clone()
                    .unwrap()
                    .duration_in_seconds
                    .to_numbas(&locale)
                    .unwrap(),
                self.feedback
                    .clone()
                    .unwrap()
                    .percentage_needed_to_pass
                    .to_numbas(&locale)
                    .unwrap(),
                Some(
                    self.navigation
                        .clone()
                        .unwrap()
                        .show_names_of_question_groups
                        .unwrap(),
                ),
                Some(self.feedback.clone().unwrap().show_name_of_student.unwrap()),
            );

            //TODO
            let navigation = self.navigation.clone().unwrap().to_numbas(&locale).unwrap();

            //TODO
            let timing = self.timing.clone().unwrap().to_numbas(&locale).unwrap();

            //TODO
            let feedback = self.feedback.clone().unwrap().to_numbas(&locale).unwrap();

            //TODO
            let functions = Value::Normal(HashMap::new());

            //TODO
            let variables = Value::Normal(HashMap::new());

            //TODO
            let question_groups: Vec<numbas::exam::ExamQuestionGroup> = self
                .question_groups
                .clone()
                .unwrap()
                .iter()
                .map(|qg| qg.clone().to_numbas(&locale).unwrap())
                .collect();

            // Below from questions
            //TODO
            let resources: Vec<[String; 2]> = Vec::new();

            let extensions: Vec<String> = self
                .question_groups
                .clone()
                .unwrap()
                .iter()
                .flat_map(|qg| {
                    qg.clone()
                        .unwrap()
                        .questions
                        .unwrap()
                        .into_iter()
                        .map(|q| q.unwrap().question_data.unwrap().extensions.unwrap())
                })
                .fold(Extensions::new(), |a, b| Extensions::combine(a, b))
                .to_paths();

            //TODO
            let custom_part_types: Vec<numbas::exam::CustomPartType> = Vec::new();

            Ok(numbas::exam::Exam::new(
                basic_settings,
                resources,
                extensions,
                custom_part_types,
                navigation,
                timing,
                feedback,
                Some(functions.unwrap()),
                Some(variables.unwrap()),
                question_groups,
            ))
        } else {
            Err(empty_fields)
        }
    }
}

impl Exam {
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
                Normal(e) => Ok(e),
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
