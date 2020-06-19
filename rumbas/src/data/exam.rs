use crate::data::feedback::Feedback;
use crate::data::json::{JsonError, JsonResult};
use crate::data::locale::Locale;
use crate::data::navigation::Navigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_group::QuestionGroup;
use crate::data::timing::Timing;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

optional_overwrite! {
    Exam,
    locales: Vec<Locale>,
    name: TranslatableString,
    navigation: Navigation,
    timing: Timing,
    feedback: Feedback,
    question_groups: Vec<QuestionGroup>, //TODO: remove?
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
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.feedback
                    .clone()
                    .unwrap()
                    .percentage_needed_to_pass
                    .to_numbas(&locale)
                    .unwrap(),
                self.navigation
                    .clone()
                    .unwrap()
                    .show_names_of_question_groups,
                self.feedback.clone().unwrap().show_name_of_student,
            );

            //TODO
            let navigation = self.navigation.clone().unwrap().to_numbas(&locale).unwrap();

            //TODO
            let timing = self.timing.clone().unwrap().to_numbas(&locale).unwrap();

            //TODO
            let feedback = self.feedback.clone().unwrap().to_numbas(&locale).unwrap();

            //TODO
            let functions = Some(HashMap::new());

            //TODO
            let variables = Some(HashMap::new());

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
            //TODO from obj of bools
            let extensions: Vec<String> = Vec::new();
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
                functions,
                variables,
                question_groups,
            ))
        } else {
            Err(empty_fields)
        }
    }
}

impl Exam {
    pub fn from_file(file: &Path) -> JsonResult<Exam> {
        let json = fs::read_to_string(file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_json::from_str(&json).map_err(|e| JsonError::from(e, file.to_path_buf()))
    }
}
