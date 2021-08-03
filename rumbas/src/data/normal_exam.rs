use crate::data::extension::Extensions;
use crate::data::feedback::Feedback;
use crate::data::locale::Locale;
use crate::data::navigation::NormalNavigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::optional_overwrite::*;
use crate::data::question_group::QuestionGroup;
use crate::data::template::{Value, ValueType};
use crate::data::timing::Timing;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: remove duplication of NormalExam & Diagnostic Exam?
optional_overwrite! {
    /// An Exam
    pub struct NormalExam {
        /// All locales for which the exam should be generated
        locales: Vec<Value<Locale>>,
        /// The name of the exam
        name: TranslatableString,
        /// The navigation settings for this exam
        navigation: NormalNavigation,
        /// The timing settings for this exam
        timing: Timing,
        /// The feedback settings for this exam
        feedback: Feedback,
        /// The questions groups for this exam
        question_groups: Vec<Value<QuestionGroup>>,
        /// The settings to set for numbas
        numbas_settings: NumbasSettings
    }
}

impl ToNumbas for NormalExam {
    type NumbasType = numbas::exam::Exam;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::Exam> {
        let check = self.check();
        if check.is_empty() {
            let basic_settings = numbas::exam::BasicExamSettings {
                name: self.name.clone().unwrap().to_string(locale).unwrap(), //TODO: might fail, not checked
                duration_in_seconds: self
                    .timing
                    .clone()
                    .unwrap()
                    .duration_in_seconds
                    .to_numbas(locale)
                    .unwrap(),
                percentage_needed_to_pass: self
                    .feedback
                    .clone()
                    .unwrap()
                    .percentage_needed_to_pass
                    .to_numbas(locale)
                    .unwrap(),
                show_question_group_names: Some(
                    self.navigation
                        .clone()
                        .unwrap()
                        .to_shared_data()
                        .show_names_of_question_groups
                        .unwrap(),
                ),
                show_student_name: Some(
                    self.feedback.clone().unwrap().show_name_of_student.unwrap(),
                ),
                allow_printing: Some(
                    self.navigation
                        .clone()
                        .unwrap()
                        .to_shared_data()
                        .allow_printing
                        .unwrap(),
                ),
            };

            let navigation = self.navigation.clone().unwrap().to_numbas(locale).unwrap();

            let timing = self.timing.clone().unwrap().to_numbas(locale).unwrap();

            let feedback = self.feedback.clone().unwrap().to_numbas(locale).unwrap();

            //TODO
            let functions = Value::Normal(HashMap::new());

            //TODO
            let variables = Value::Normal(HashMap::new());

            let question_groups: Vec<numbas::exam::ExamQuestionGroup> = self
                .question_groups
                .clone()
                .unwrap()
                .iter()
                .map(|qg| qg.clone().to_numbas(locale).unwrap())
                .collect();

            let resources: Vec<numbas::exam::Resource> = self
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
                        .flat_map(|q| q.unwrap().question_data.unwrap().resources.unwrap())
                })
                .map(|r| r.unwrap())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .to_numbas(locale)
                .unwrap();

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
                .fold(Extensions::default(), Extensions::combine)
                .to_paths();

            let custom_part_types: Vec<numbas::exam::CustomPartType> = self
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
                        .flat_map(|q| q.unwrap().question_data.unwrap().custom_part_types.unwrap())
                })
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .to_numbas(locale)
                .unwrap();

            Ok(numbas::exam::Exam {
                basic_settings,
                resources,
                extensions,
                custom_part_types,
                navigation,
                timing,
                feedback,
                functions: Some(functions.unwrap()),
                variables: Some(variables.unwrap()),
                question_groups,
                diagnostic: None,
            })
        } else {
            Err(check)
        }
    }
}
