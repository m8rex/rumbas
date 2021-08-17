use crate::data::extension::Extensions;
use crate::data::feedback::Feedback;
use crate::data::locale::Locale;
use crate::data::navigation::DiagnosticNavigation;
use crate::data::numbas_settings::NumbasSettings;
use crate::data::optional_overwrite::*;
use crate::data::question_group::QuestionGroup;
use crate::data::template::{Value, ValueType};
use crate::data::timing::Timing;
use crate::data::to_numbas::ToNumbas;
use crate::data::to_rumbas::ToRumbas;
use crate::data::translatable::TranslatableString;
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

impl ToNumbas<numbas::exam::Exam> for DiagnosticExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        let basic_settings = numbas::exam::BasicExamSettings {
            name: self.name.clone().unwrap().to_string(locale).unwrap(), //TODO: might fail, not checked
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
        };

        let navigation = self.navigation.clone().unwrap().to_numbas(locale);

        let timing = self.timing.clone().unwrap().to_numbas(locale);

        let feedback = self.feedback.clone().unwrap().to_numbas(locale);

        //TODO
        let functions = Value::Normal(HashMap::new());

        //TODO
        let variables = Value::Normal(HashMap::new());

        let question_groups: Vec<numbas::exam::ExamQuestionGroup> = self
            .question_groups
            .clone()
            .unwrap()
            .iter()
            .map(|qg| qg.clone().to_numbas(locale))
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

        let diagnostic = Some(self.diagnostic.clone().unwrap().to_numbas(locale));

        numbas::exam::Exam {
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
            topics: self
                .topics
                .clone()
                .unwrap()
                .into_iter()
                .map(|t| t.to_numbas(locale))
                .collect(),
            learning_objectives: self
                .objectives
                .clone()
                .unwrap()
                .into_iter()
                .map(|t| t.to_numbas(locale))
                .collect(),
        };

        numbas::exam::ExamDiagnostic {
            knowledge_graph,
            script: self.script.to_numbas(locale),
            custom_script: self.script.clone().unwrap().to_custom_script(locale), // TODO TONUMBAS
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
    Custom(TranslatableString),
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
                DiagnosticScript::Custom(TranslatableString::s(&self.custom_script))
            }
        }
    }
}

impl DiagnosticScript {
    pub fn to_custom_script(&self, locale: &str) -> String {
        match self {
            DiagnosticScript::Custom(s) => s.clone().to_string(locale).unwrap(),
            DiagnosticScript::Diagnosys => String::new(),
            DiagnosticScript::Mastery => String::new(),
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
            name: self.name.clone().unwrap().to_string(locale).unwrap(),
            description: self.description.clone().unwrap().to_string(locale).unwrap(),
        }
    }
}

impl ToRumbas<LearningObjective> for numbas::exam::ExamDiagnosticKnowledgeGraphLearningObjective {
    fn to_rumbas(&self) -> LearningObjective {
        LearningObjective {
            name: Value::Normal(TranslatableString::s(&self.name)),
            description: Value::Normal(TranslatableString::s(&self.description)),
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
            name: self.name.clone().unwrap().to_string(locale).unwrap(),
            description: self.description.clone().unwrap().to_string(locale).unwrap(),
            learning_objectives: self
                .objectives
                .clone()
                .unwrap()
                .into_iter()
                .map(|s| s.to_string(locale).unwrap())
                .collect(),
            depends_on: self
                .depends_on
                .clone()
                .unwrap()
                .into_iter()
                .map(|s| s.to_string(locale).unwrap())
                .collect(),
        }
    }
}

impl ToRumbas<LearningTopic> for numbas::exam::ExamDiagnosticKnowledgeGraphTopic {
    fn to_rumbas(&self) -> LearningTopic {
        LearningTopic {
            name: Value::Normal(TranslatableString::s(&self.name)),
            description: Value::Normal(TranslatableString::s(&self.description)),
            objectives: Value::Normal(
                self.learning_objectives
                    .clone()
                    .into_iter()
                    .map(|o| TranslatableString::s(&o))
                    .collect(),
            ),
            depends_on: Value::Normal(
                self.depends_on
                    .clone()
                    .into_iter()
                    .map(|o| TranslatableString::s(&o))
                    .collect(),
            ),
        }
    }
}
