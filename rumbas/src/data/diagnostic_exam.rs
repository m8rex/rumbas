use crate::data::extension::Extensions;
use crate::data::feedback::Feedback;
use crate::data::locale::Locale;
use crate::data::navigation::DiagnosticNavigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_group::QuestionGroup;
use crate::data::template::{Value, ValueType};
use crate::data::timing::Timing;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

optional_overwrite! {
    /// A Diagnostic Exam
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
        numbas_settings: NumbasSettings,
        /// The diagnostic data
        diagnostic: Diagnostic
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
                    self.navigation
                        .clone()
                        .unwrap()
                        .shared_data
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

            let diagnostic = Some(self.diagnostic.clone().unwrap().to_numbas(&locale).unwrap());

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
                diagnostic,
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticScript {
    Mastery,
    Diagnosys,
    Custom(TranslatableString),
}
impl_optional_overwrite!(DiagnosticScript);
impl ToNumbas for DiagnosticScript {
    type NumbasType = numbas::exam::ExamDiagnosticScript;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(match self {
                DiagnosticScript::Mastery => Self::NumbasType::Mastery,
                DiagnosticScript::Custom(_) => Self::NumbasType::Custom,
                DiagnosticScript::Diagnosys => Self::NumbasType::Diagnosys,
            })
        } else {
            Err(empty_fields)
        }
    }
}

impl DiagnosticScript {
    pub fn to_custom_script(&self, locale: &String) -> String {
        match self {
            DiagnosticScript::Custom(s) => s.clone().to_string(&locale).unwrap(),
            DiagnosticScript::Diagnosys => String::new(),
            DiagnosticScript::Mastery => String::new(),
        }
    }
}

optional_overwrite! {
    /// Information needed for a diagnostic test
    pub struct Diagnostic {
        /// The script to use
        script: DiagnosticScript,
        /// The learning objectives,
        objectives: Vec<LearningObjective>,
        /// The learning topics
        topics: Vec<LearningTopic>
    }
}

impl ToNumbas for Diagnostic {
    type NumbasType = numbas::exam::ExamDiagnostic;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let knowledge_graph = numbas::exam::ExamDiagnosticKnowledgeGraph {
                topics: self
                    .topics
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|t| t.to_numbas(&locale).unwrap())
                    .collect(),
                learning_objectives: self
                    .objectives
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|t| t.to_numbas(&locale).unwrap())
                    .collect(),
            };

            Ok(Self::NumbasType {
                knowledge_graph,
                script: self.script.to_numbas(&locale).unwrap(),
                custom_script: self.script.clone().unwrap().to_custom_script(&locale),
            })
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    /// A Learning Objective
    pub struct LearningObjective {
        /// The name
        name: TranslatableString,
        /// A description
        description: TranslatableString
    }
}

impl ToNumbas for LearningObjective {
    type NumbasType = numbas::exam::ExamDiagnosticKnowledgeGraphLearningObjective;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(Self::NumbasType {
                name: self.name.clone().unwrap().to_string(&locale).unwrap(),
                description: self
                    .description
                    .clone()
                    .unwrap()
                    .to_string(&locale)
                    .unwrap(),
            })
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    /// A learning Topic
    pub struct  LearningTopic {
        /// The name
        name: TranslatableString,
        /// A description
        description: TranslatableString,
        /// List of names of objectives
        objectives: Vec<TranslatableString>,
        /// List of names of topic on which this topic depends
        depends_on: Vec<TranslatableString>
    }
}

impl ToNumbas for LearningTopic {
    type NumbasType = numbas::exam::ExamDiagnosticKnowledgeGraphTopic;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(Self::NumbasType {
                name: self.name.clone().unwrap().to_string(&locale).unwrap(),
                description: self
                    .description
                    .clone()
                    .unwrap()
                    .to_string(&locale)
                    .unwrap(),
                learning_objectives: self
                    .objectives
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|s| s.clone().to_string(&locale).unwrap())
                    .collect(),
                depends_on: self
                    .depends_on
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|s| s.clone().to_string(&locale).unwrap())
                    .collect(),
            })
        } else {
            Err(empty_fields)
        }
    }
}