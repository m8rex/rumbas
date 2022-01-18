use crate::exam::feedback::Feedback;
use crate::exam::locale::Locale;
use crate::exam::locale::SupportedLocale;
use crate::exam::navigation::NormalNavigation;
use crate::exam::numbas_settings::NumbasSettings;
use crate::exam::question_group::QuestionGroup;
use crate::exam::question_group::QuestionPath;
use crate::exam::timing::Timing;
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::question::extension::Extensions;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};
use std::collections::HashMap;

// TODO: remove duplication of NormalExam & Diagnostic Exam?
#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NormalExamInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
/// An Exam
pub struct NormalExam {
    /// All locales for which the exam should be generated
    pub locales: Vec<Locale>,
    /// The name of the exam
    pub name: TranslatableString,
    /// The navigation settings for this exam
    pub navigation: NormalNavigation,
    /// The timing settings for this exam
    pub timing: Timing,
    /// The feedback settings for this exam
    pub feedback: Feedback,
    /// The questions groups for this exam
    pub question_groups: Vec<QuestionGroup>,
    /// The settings to set for numbas
    pub numbas_settings: NumbasSettings,
}

impl ToNumbas<numbas::exam::Exam> for NormalExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
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
            .iter()
            .flat_map(|qg| {
                qg.clone()
                    .questions
                    .into_iter()
                    .flat_map(|q| q.data.resources)
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale);

        let extensions: Vec<String> = self
            .question_groups
            .iter()
            .flat_map(|qg| qg.clone().questions.into_iter().map(|q| q.data.extensions))
            .fold(Extensions::default(), Extensions::combine)
            .to_paths();

        let custom_part_types: Vec<numbas::question::custom_part_type::CustomPartType> = self
            .question_groups
            .clone()
            .iter()
            .flat_map(|qg| {
                qg.clone()
                    .questions
                    .into_iter()
                    .flat_map(|q| q.data.custom_part_types)
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale);

        numbas::exam::Exam {
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

impl ToNumbas<numbas::exam::BasicExamSettings> for NormalExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::BasicExamSettings {
        numbas::exam::BasicExamSettings {
            name: self.name.to_numbas(locale),
            duration_in_seconds: self.timing.duration_in_seconds.to_numbas(locale),
            percentage_needed_to_pass: self.feedback.percentage_needed_to_pass.to_numbas(locale),
            show_question_group_names: Some(
                self.navigation
                    .to_shared_data()
                    .show_names_of_question_groups,
            ),
            show_student_name: Some(self.feedback.show_name_of_student),
            allow_printing: Some(self.navigation.to_shared_data().allow_printing),
        }
    }
}

/// Converts a normal numbas exam to a NormalExam
pub fn convert_normal_numbas_exam(
    exam: numbas::exam::Exam,
) -> (
    NormalExam,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let question_groups: Vec<_> = exam.question_groups.to_rumbas();
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        NormalExam {
            locales: vec![Locale {
                name: "en".to_string(),
                numbas_locale: SupportedLocale::EnGB,
            }], // todo: argument?
            name: exam.basic_settings.name.to_rumbas(),
            navigation: exam.to_rumbas(),
            timing: exam.to_rumbas(),
            feedback: exam.to_rumbas(),
            question_groups: question_groups.clone(),
            numbas_settings: NumbasSettings {
                theme: "default".to_string(),
            }, // todo: argument?
        },
        question_groups
            .into_iter()
            .flat_map(|qg| qg.questions.into_iter())
            .collect(),
        custom_part_types,
    )
}
