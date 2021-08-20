use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::question::extension::Extensions;
use crate::question::feedback::Feedback;
use crate::exam::locale::Locale;
use crate::exam::locale::SupportedLocale;
use crate::exam::navigation::DiagnosticNavigation;
use crate::exam::numbas_settings::NumbasSettings;
use crate::exam::question_group::QuestionGroup;
use crate::exam::question_group::QuestionPath;
use crate::support::template::{Value, ValueType};
use crate::exam::timing::Timing;
use crate::support::translatable::JMENotesTranslatableString;
use crate::support::translatable::TranslatableString;
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use schemars::JsonSchema;
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
        question_groups: Vec<Value<QuestionGroup>>,
        /// The settings to set for numbas
        numbas_settings: NumbasSettings,
        /// The diagnostic data
        diagnostic: Diagnostic
    }
}

impl ToNumbas<numbas::exam::BasicExamSettings> for DiagnosticExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::BasicExamSettings {
        numbas::exam::BasicExamSettings {
            name: self.name.to_numbas(locale), //TODO: might fail, not checked
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
                    .shared_data
                    .unwrap()
                    .show_names_of_question_groups
                    .unwrap(),
            ),
            show_student_name: Some(self.feedback.clone().unwrap().show_name_of_student.unwrap()),
            allow_printing: Some(
                self.navigation
                    .clone()
                    .unwrap()
                    .shared_data
                    .unwrap()
                    .allow_printing
                    .unwrap(),
            ),
        }
    }
}

impl ToNumbas<numbas::exam::Exam> for DiagnosticExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        let basic_settings = self.to_numbas(locale);

        let navigation = self.navigation.to_numbas(locale);

        let timing = self.timing.to_numbas(locale);

        let feedback = self.feedback.to_numbas(locale);

        //TODO
        let functions = Some(HashMap::new());

        //TODO
        let variables = Some(HashMap::new());

        let question_groups: Vec<numbas::exam::ExamQuestionGroup> =
            self.question_groups.to_numbas(locale);

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
                    .flat_map(|q| q.unwrap().question_data.custom_part_types.unwrap())
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale);

        let diagnostic = Some(self.diagnostic.to_numbas(locale));

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
            diagnostic,
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

impl ToNumbas<numbas::exam::ExamDiagnostic> for Diagnostic {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamDiagnostic {
        let knowledge_graph = numbas::exam::ExamDiagnosticKnowledgeGraph {
            topics: self.topics.to_numbas(locale),
            learning_objectives: self.objectives.to_numbas(locale),
        };

        numbas::exam::ExamDiagnostic {
            knowledge_graph,
            script: self.script.to_numbas(locale),
            custom_script: self.script.to_numbas(locale),
        }
    }
}

impl ToRumbas<Diagnostic> for numbas::exam::ExamDiagnostic {
    fn to_rumbas(&self) -> Diagnostic {
        Diagnostic {
            script: Value::Normal(self.to_rumbas()),
            objectives: Value::Normal(self.knowledge_graph.clone().learning_objectives.to_rumbas()),
            topics: Value::Normal(self.knowledge_graph.topics.to_rumbas()),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticScript {
    Mastery,
    Diagnosys,
    Custom(JMENotesTranslatableString),
}
impl_optional_overwrite!(DiagnosticScript);
impl ToNumbas<numbas::exam::ExamDiagnosticScript> for DiagnosticScript {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamDiagnosticScript {
        match self {
            DiagnosticScript::Mastery => numbas::exam::ExamDiagnosticScript::Mastery,
            DiagnosticScript::Custom(_) => numbas::exam::ExamDiagnosticScript::Custom,
            DiagnosticScript::Diagnosys => numbas::exam::ExamDiagnosticScript::Diagnosys,
        }
    }
}

impl ToRumbas<DiagnosticScript> for numbas::exam::ExamDiagnostic {
    fn to_rumbas(&self) -> DiagnosticScript {
        match self.script {
            numbas::exam::ExamDiagnosticScript::Mastery => DiagnosticScript::Mastery,
            numbas::exam::ExamDiagnosticScript::Diagnosys => DiagnosticScript::Diagnosys,
            numbas::exam::ExamDiagnosticScript::Custom => {
                DiagnosticScript::Custom(self.custom_script.clone().into())
            }
        }
    }
}

impl ToNumbas<numbas::jme::JMENotesString> for DiagnosticScript {
    fn to_numbas(&self, locale: &str) -> numbas::jme::JMENotesString {
        match self {
            DiagnosticScript::Custom(s) => s.to_numbas(locale),
            DiagnosticScript::Diagnosys => Default::default(),
            DiagnosticScript::Mastery => Default::default(),
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

impl ToNumbas<numbas::exam::ExamDiagnosticKnowledgeGraphLearningObjective> for LearningObjective {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::exam::ExamDiagnosticKnowledgeGraphLearningObjective {
        numbas::exam::ExamDiagnosticKnowledgeGraphLearningObjective {
            name: self.name.to_numbas(locale),
            description: self.description.to_numbas(locale),
        }
    }
}

impl ToRumbas<LearningObjective> for numbas::exam::ExamDiagnosticKnowledgeGraphLearningObjective {
    fn to_rumbas(&self) -> LearningObjective {
        LearningObjective {
            name: Value::Normal(self.name.clone().into()),
            description: Value::Normal(self.description.clone().into()),
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

impl ToNumbas<numbas::exam::ExamDiagnosticKnowledgeGraphTopic> for LearningTopic {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamDiagnosticKnowledgeGraphTopic {
        numbas::exam::ExamDiagnosticKnowledgeGraphTopic {
            name: self.name.to_numbas(locale),
            description: self.description.to_numbas(locale),
            learning_objectives: self.objectives.to_numbas(locale),
            depends_on: self.depends_on.to_numbas(locale),
        }
    }
}

impl ToRumbas<LearningTopic> for numbas::exam::ExamDiagnosticKnowledgeGraphTopic {
    fn to_rumbas(&self) -> LearningTopic {
        LearningTopic {
            name: Value::Normal(self.name.clone().into()),
            description: Value::Normal(self.description.clone().into()),
            objectives: Value::Normal(
                self.learning_objectives
                    .clone()
                    .into_iter()
                    .map(|o| o.into())
                    .collect(),
            ),
            depends_on: Value::Normal(
                self.depends_on
                    .clone()
                    .into_iter()
                    .map(|o| o.into())
                    .collect(),
            ),
        }
    }
}

/// Converts a diagnostic numbas exam to a NormalExam
pub fn convert_diagnostic_numbas_exam(
    exam: numbas::exam::Exam,
) -> (
    DiagnosticExam,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let question_groups = exam
        .question_groups
        .to_rumbas()
        .into_iter()
        .map(Value::Normal)
        .collect::<Vec<_>>();
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        DiagnosticExam {
            locales: Value::Normal(vec![Value::Normal(Locale {
                name: Value::Normal("en".to_string()),
                numbas_locale: Value::Normal(SupportedLocale::EnGB),
            })]), // todo: argument?
            name: Value::Normal(exam.basic_settings.name.clone().into()),
            navigation: Value::Normal(exam.to_rumbas()),
            timing: Value::Normal(exam.to_rumbas()),
            feedback: Value::Normal(exam.to_rumbas()),
            question_groups: Value::Normal(question_groups.clone()),
            numbas_settings: Value::Normal(NumbasSettings {
                locale: Value::Normal(SupportedLocale::EnGB),
                theme: Value::Normal("default".to_string()),
            }), // todo: argument?
            diagnostic: Value::Normal(exam.diagnostic.unwrap().to_rumbas()),
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
