use crate::jme::JMENotesString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub knowledge_graph: DiagnosticKnowledgeGraph,
    pub script: DiagnosticScript,
    #[serde(rename = "customScript")]
    pub custom_script: JMENotesString,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticKnowledgeGraph {
    pub topics: Vec<DiagnosticKnowledgeGraphTopic>,
    pub learning_objectives: Vec<DiagnosticKnowledgeGraphLearningObjective>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticKnowledgeGraphTopic {
    pub name: String,
    pub description: String,
    pub learning_objectives: Vec<String>,
    pub depends_on: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticKnowledgeGraphLearningObjective {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticScript {
    Mastery,
    Diagnosys,
    Custom,
}
