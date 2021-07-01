use crate::data::extension::Extensions;
use crate::data::feedback::Feedback;
use crate::data::locale::Locale;
use crate::data::navigation::DiagnosticNavigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_group::QuestionGroup;
use crate::data::template::{ExamFileType, TemplateData, Value, ValueType, TEMPLATE_EXAMS_FOLDER};
use crate::data::timing::Timing;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use crate::data::yaml::{YamlError, YamlResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

optional_overwrite! {
    /// An Exam
    pub struct DiagnosticExam {
        /// All locales for which the exam should be generated
        locales: Vec<Value<Locale>>,
        /// The name of the exam
        name: TranslatableString,
        /// The navigation settings for this exam
        navigation: DiagnosticNavigation,
        /// The timing settings for this exam
        timing: Timing,
        /// The feedback settings for this exam
        feedback: Feedback,
        /// The questions groups for this exam
        question_groups: Vec<Value<QuestionGroup>>, //TODO: remove?
        /// The settings to set for numbas
        numbas_settings: NumbasSettings
    }
}

impl ToNumbas for DiagnosticExam {
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
                        .shared_data
                        .clone()
                        .unwrap()
                        .show_names_of_question_groups
                        .unwrap(),
                ),
                Some(self.feedback.clone().unwrap().show_name_of_student.unwrap()),
                Some(
                    self.numbas_settings
                        .clone()
                        .unwrap()
                        .allow_printing
                        .unwrap(),
                ),
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
