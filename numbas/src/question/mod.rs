pub mod answer_simplification;
pub mod constants;
pub mod custom_part_type;
pub mod function;
pub mod navigation;
pub mod part;
pub mod preamble;
pub mod resource;
pub mod variable;

use crate::exam::hacky_fix_exam;
use crate::jme::ContentAreaString;
use crate::jme::JMEString;
use crate::support::primitive::SafeNatural;
use answer_simplification::AnswerSimplificationType;
use constants::BuiltinConstants;
use constants::QuestionConstant;
use custom_part_type::CustomPartType;
use function::Function;
use navigation::Navigation as QuestionNavigation;
use part::QuestionPart;
use preamble::Preamble;
use resource::Resource;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;
use std::collections::HashMap;
use variable::Variable;
use variable::VariableGroup;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Question {
    //TODO
    pub name: String,
    /// The statement is a content area which appears at the top of the question, before any input boxes. Use the statement to set up the question and provide any information the student needs to answer it.
    pub statement: ContentAreaString,
    /// Advice is a content area which is shown when the student presses the Reveal button to reveal the question’s answers, or at the end of the exam.
    /// The advice area is normally used to present a worked solution to the question.
    pub advice: ContentAreaString,
    #[serde(default)]
    pub parts: Vec<QuestionPart>,
    #[serde(default)]
    pub builtin_constants: BuiltinConstants,
    #[serde(default)]
    pub constants: Vec<QuestionConstant>,
    #[serde(default)]
    pub variables: HashMap<String, Variable>,
    #[serde(rename = "variablesTest", default)]
    pub variables_test: QuestionVariablesTest,
    #[serde(default)]
    pub functions: HashMap<String, Function>,
    #[serde(default)]
    pub ungrouped_variables: Vec<String>,
    #[serde(default)]
    pub variable_groups: Vec<VariableGroup>,
    #[serde(default)]
    pub rulesets: HashMap<String, Vec<AnswerSimplificationType>>,
    #[serde(default)]
    pub preamble: Preamble,
    //contributors TODO
    pub navigation: QuestionNavigation,
    //custom part types TODO
    #[serde(default)]
    pub extensions: Vec<String>, // todo: enum
    //metadata TODO
    #[serde(default)]
    pub resources: Vec<Resource>,
    //TODO type: question?
    /// Tags starting with 'skill: ' are used in diagnostic mode to specify a topic
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub custom_part_types: Vec<CustomPartType>,
}

#[derive(Debug, Deserialize)]
struct QuestionInput<'a> {
    #[serde(borrow)]
    question_groups: [QuestionInputQuestionGroups<'a>; 1],
}
#[derive(Debug, Deserialize)]
struct QuestionInputQuestionGroups<'a> {
    #[serde(borrow)]
    questions: [BTreeMap<&'a str, serde_json::Value>; 1],
}
impl Question {
    pub fn from_question_exam_str(s: &str) -> serde_json::Result<Question> {
        let json = if s.starts_with("// Numbas version: exam_results_page_options") {
            s.splitn(2, '\n').collect::<Vec<_>>()[1]
        } else {
            s
        };
        let exam: HashMap<String, serde_json::Value> = serde_json::from_str(json)?;
        let question_input: QuestionInput = serde_json::from_str(json)?;
        let mut question = question_input.question_groups[0].questions[0].clone();
        for key in ["resources", "extensions", "custom_part_types", "navigation"] {
            if let Some(value) = exam.get(key) {
                question.insert(key, value.to_owned());
            }
        }
        let new_json = serde_json::to_string_pretty(&question).unwrap();
        let new_json = hacky_fix_exam(&new_json[..]);
        log::debug!("Question loaded as following exam");
        log::debug!(
            "{}",
            new_json
                .split('\n')
                .enumerate()
                .map(|(i, s)| format!("{} {}", i, s))
                .collect::<Vec<_>>()
                .join("\n")
        );
        serde_json::from_str(&new_json)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QuestionVariablesTest {
    pub condition: JMEString,
    #[serde(rename = "maxRuns")]
    pub max_runs: SafeNatural,
}

impl std::default::Default for QuestionVariablesTest {
    fn default() -> Self {
        Self {
            condition: JMEString::default(),
            max_runs: 10.into(),
        }
    }
}
