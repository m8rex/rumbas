use crate::exam::feedback::Feedback;
use crate::exam::feedback::FeedbackInput;
use crate::exam::locale::SupportedLocale;
use crate::exam::locale::{Locale, Locales, LocalesInput};
use crate::exam::navigation::NormalNavigation;
use crate::exam::navigation::NormalNavigationInput;
use crate::exam::numbas_settings::NumbasSettings;
use crate::exam::numbas_settings::NumbasSettingsInput;
use crate::exam::question_group::QuestionPath;
use crate::exam::question_group::{QuestionGroup, QuestionGroups, QuestionGroupsInput};
use crate::exam::timing::Timing;
use crate::exam::timing::TimingInput;
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::question::extension::Extensions;
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: remove duplication of NormalExam & Diagnostic Exam?
optional_overwrite! {
    /// An Exam
    pub struct NormalExam {
        /// All locales for which the exam should be generated
        locales: Locales,
        /// The name of the exam
        name: TranslatableString,
        /// The navigation settings for this exam
        navigation: NormalNavigation,
        /// The timing settings for this exam
        timing: Timing,
        /// The feedback settings for this exam
        feedback: Feedback,
        /// The questions groups for this exam
        question_groups: QuestionGroups,
        /// The settings to set for numbas
        numbas_settings: NumbasSettings
    }
}

impl ToNumbas<numbas::exam::exam::Exam> for NormalExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::exam::Exam {
        let basic_settings = self.to_numbas(locale);
        let navigation = self.navigation.to_numbas(locale);

        let timing = self.timing.to_numbas(locale);

        let feedback = self.feedback.to_numbas(locale);

        //TODO
        let functions = Some(HashMap::new());

        //TODO
        let variables = Some(HashMap::new());

        let question_groups: Vec<numbas::exam::question_group::QuestionGroup> =
            self.question_groups.to_numbas(locale);

        let resources: Vec<numbas::question::resource::Resource> = self
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
                    .flat_map(|q| q.unwrap().question_data.resources.unwrap())
            })
            .map(|r| r.unwrap())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale);

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
                    .map(|q| q.unwrap().question_data.extensions.unwrap())
            })
            .fold(Extensions::default(), Extensions::combine)
            .to_paths();

        let custom_part_types: Vec<numbas::question::custom_part_type::CustomPartType> = self
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
                    .flat_map(|q| q.unwrap().question_data.custom_part_types.unwrap())
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale);

        numbas::exam::exam::Exam {
            basic_settings,
            resources,
            extensions,
            custom_part_types,
            navigation,
            timing,
            feedback,
            functions,
            variables,
            question_groups,
            diagnostic: None,
        }
    }
}

impl ToNumbas<numbas::exam::exam::BasicExamSettings> for NormalExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::exam::BasicExamSettings {
        numbas::exam::exam::BasicExamSettings {
            name: self.name.to_numbas(locale),
            duration_in_seconds: self
                .timing
                .clone()
                .unwrap()
                .duration_in_seconds
                .to_numbas(locale),
            percentage_needed_to_pass: self
                .feedback
                .clone()
                .unwrap()
                .percentage_needed_to_pass
                .to_numbas(locale),
            show_question_group_names: Some(
                self.navigation
                    .clone()
                    .unwrap()
                    .to_shared_data()
                    .show_names_of_question_groups
                    .unwrap(),
            ),
            show_student_name: Some(self.feedback.clone().unwrap().show_name_of_student.unwrap()),
            allow_printing: Some(
                self.navigation
                    .clone()
                    .unwrap()
                    .to_shared_data()
                    .allow_printing
                    .unwrap(),
            ),
        }
    }
}

/// Converts a normal numbas exam to a NormalExam
pub fn convert_normal_numbas_exam(
    exam: numbas::exam::exam::Exam,
) -> (
    NormalExam,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let question_groups: Vec<Value<_>> = exam.question_groups.to_rumbas();
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        NormalExam {
            locales: Value::Normal(vec![Value::Normal(Locale {
                name: Value::Normal("en".to_string()),
                numbas_locale: Value::Normal(SupportedLocale::EnGB),
            })]), // todo: argument?
            name: exam.basic_settings.name.to_rumbas(),
            navigation: exam.to_rumbas(),
            timing: exam.to_rumbas(),
            feedback: exam.to_rumbas(),
            question_groups: Value::Normal(question_groups.clone()),
            numbas_settings: Value::Normal(NumbasSettings {
                theme: Value::Normal("default".to_string()),
            }), // todo: argument?
        },
        question_groups
            .into_iter()
            .flat_map(|qg| {
                qg.unwrap()
                    .questions
                    .unwrap()
                    .into_iter()
                    .map(|q| q.unwrap())
            })
            .collect(),
        custom_part_types,
    )
}
