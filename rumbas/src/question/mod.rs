//! Contains all the data types

pub mod constants;
pub mod custom_part_type;
pub mod extension;
pub mod feedback;
pub mod function;
pub mod navigation;
pub mod part;
pub mod preamble;
pub mod resource;
pub mod variable;
pub mod variable_test;

use crate::support::optional_overwrite::*;
use crate::support::template::TemplateData;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::{YamlError, YamlResult};
use constants::BuiltinConstants;
use constants::CustomConstant;
use custom_part_type::CustomPartTypeDefinitionPath;
use extension::Extensions;
use function::Function;
use navigation::QuestionNavigation;
use part::question_part::QuestionPart;
use preamble::Preamble;
use resource::ResourcePath;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use variable::VariableRepresentation;
use variable::UNGROUPED_GROUP;
use variable_test::VariablesTest;

optional_overwrite! {
    pub struct Question {
        /// The statement is a content area which appears at the top of the question, before any input boxes. Use the statement to set up the question and provide any information the student needs to answer it.
        statement: ContentAreaTranslatableString,
        /// Advice is a content area which is shown when the student presses the Reveal button to reveal the question’s answers, or at the end of the exam.
        /// The advice area is normally used to present a worked solution to the question.
        advice: ContentAreaTranslatableString,
        parts: Vec<Value<QuestionPart>>,
        builtin_constants: BuiltinConstants,
        custom_constants: Vec<CustomConstant>,
        variables: HashMap<String, Value<VariableRepresentation>>,
        variables_test: VariablesTest,
        functions: HashMap<String, Value<Function>>,
        preamble: Preamble,
        navigation: QuestionNavigation,
        extensions: Extensions,
        /// The names of the topics used in diagnostic exams that this question belongs to
        diagnostic_topic_names: Vec<TranslatableString>, // TODO: validate? / warnings?
        resources: Vec<Value<ResourcePath>>,
        /// The custom part types used in this exam
        custom_part_types: Vec<CustomPartTypeDefinitionPath>
        //TODO a lot of options
    }
}

impl ToNumbas<numbas::exam::ExamQuestion> for Question {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamQuestion {
        //TODO?
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
    //TODO: add to_numbas on Option's to reduce burden?
    fn to_numbas_with_name(&self, locale: &str, name: String) -> numbas::exam::ExamQuestion {
        if self.variables.unwrap().contains_key("e") {
            panic!("e is not allowed as a variable name"); //TODO is this still the case?
        }
        numbas::exam::ExamQuestion {
            name,
            statement: self.statement.to_numbas(locale),
            advice: self.advice.to_numbas(locale),
            parts: self.parts.to_numbas(locale),
            builtin_constants: numbas::exam::BuiltinConstants(
                self.builtin_constants.clone().unwrap().to_numbas(locale),
            ),
            constants: self.custom_constants.to_numbas(locale),
            variables: self
                .variables
                .clone()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.clone(), v.to_numbas_with_name(locale, k)))
                .collect(),
            variables_test: self.variables_test.clone().unwrap().to_numbas(locale),
            functions: self.functions.to_numbas(locale),
            ungrouped_variables: self
                .variables
                .clone()
                .unwrap()
                .into_iter()
                .filter(|(_k, v)| &v.unwrap().to_variable().group.unwrap()[..] == UNGROUPED_GROUP)
                .map(|(k, _)| k)
                .collect(),
            variable_groups: Vec::new(), // Don't add variable groups
            rulesets: HashMap::new(),    //TODO: add to Question type ?
            preamble: self.preamble.clone().unwrap().to_numbas(locale),
            navigation: self.navigation.clone().unwrap().to_numbas(locale),
            extensions: self.extensions.clone().unwrap().to_numbas(locale),
            tags: self
                .diagnostic_topic_names
                .clone()
                .unwrap()
                .into_iter()
                .map(|t| format!("skill: {}", t.to_string(locale).unwrap()))
                .collect(),
            resources: self.resources.to_numbas(locale),
            custom_part_types: self
                .custom_part_types
                .clone()
                .unwrap()
                .into_iter()
                .map(|c| {
                    c.custom_part_type_data
                        .to_numbas_with_name(locale, c.custom_part_type_name)
                })
                .collect(),
        }
    }
}

impl ToRumbas<Question> for numbas::exam::ExamQuestion {
    fn to_rumbas(&self) -> Question {
        Question {
            statement: self.statement.to_rumbas(),
            advice: self.advice.to_rumbas(),
            parts: self.parts.to_rumbas(),
            builtin_constants: self.builtin_constants.to_rumbas(),
            custom_constants: self.constants.to_rumbas(),
            variables: self.variables.to_rumbas(),
            variables_test: self.variables_test.to_rumbas(),
            functions: self.functions.to_rumbas(),
            preamble: self.preamble.to_rumbas(),
            navigation: self.navigation.to_rumbas(),
            extensions: self.extensions.to_rumbas(),
            diagnostic_topic_names: Value::Normal(
                self.tags
                    .iter()
                    .filter(|t| t.starts_with("skill: "))
                    .map(|t| t.splitn(2, ": ").collect::<Vec<_>>()[1].to_string().into())
                    .collect(),
            ),
            resources: self.resources.to_rumbas(),
            custom_part_types: self.custom_part_types.to_rumbas(),
        }
    }
}

impl Question {
    pub fn from_name(name: &str) -> YamlResult<Question> {
        use QuestionFileType::*;
        let file = Path::new(crate::QUESTIONS_FOLDER).join(format!("{}.yaml", name));
        let yaml = fs::read_to_string(&file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        let input: std::result::Result<QuestionFileType, serde_yaml::Error> =
            serde_yaml::from_str(&yaml);
        input
            .map(|e| match e {
                Normal(e) => Ok(*e),
                Template(t) => {
                    let template_file = Path::new(crate::QUESTION_TEMPLATES_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path));
                    let template_yaml = fs::read_to_string(&template_file).expect(
                        &format!(
                            "Failed to read {}",
                            template_file.to_str().map_or("invalid filename", |s| s)
                        )[..],
                    );
                    let mut question: Question = serde_yaml::from_str(&template_yaml).unwrap();
                    t.data.iter().for_each(|(k, v)| {
                        question.insert_template_value(k, &v.0);
                    });
                    Ok(question)
                }
            })
            .and_then(std::convert::identity) //flatten result is currently only possible in nightly
            .map_err(|e| YamlError::from(e, file.to_path_buf()))
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum QuestionFileType {
    Template(TemplateData),
    Normal(Box<Question>),
}

impl QuestionFileType {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}
