//! Contains all the data types

pub mod constants;
pub mod custom_part_type;
pub mod extension;
pub mod function;
pub mod navigation;
pub mod part;
pub mod preamble;
pub mod resource;
pub mod variable;
pub mod variable_test;

use crate::question::custom_part_type::CustomPartTypeDefinitionPaths;
use crate::question::custom_part_type::CustomPartTypeDefinitionPathsInput;
use crate::question::variable_test::VariablesTestInput;
use crate::support::optional_overwrite::*;
use crate::support::template::Value;
use crate::support::template::{TemplateFile, TemplateFileInput};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use crate::support::translatable::TranslatableStrings;
use crate::support::translatable::TranslatableStringsInput;
use crate::support::yaml::{YamlError, YamlResult};
use constants::BuiltinConstants;
use constants::BuiltinConstantsInput;
use constants::CustomConstants;
use constants::CustomConstantsInput;
use extension::Extensions;
use extension::ExtensionsInput;
use function::Function;
use function::FunctionInput;
use navigation::QuestionNavigation;
use navigation::QuestionNavigationInput;
use part::question_part::QuestionPartInput;
use part::question_part::QuestionParts;
use part::question_part::QuestionPartsInput;
use preamble::Preamble;
use preamble::PreambleInput;
use resource::ResourcePaths;
use resource::ResourcePathsInput;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use variable::VariableRepresentation;
use variable::VariableRepresentationInput;
use variable::UNGROUPED_GROUP;
use variable_test::VariablesTest;

optional_overwrite! {
    pub struct Question {
        /// The statement is a content area which appears at the top of the question, before any input boxes. Use the statement to set up the question and provide any information the student needs to answer it.
        statement: ContentAreaTranslatableString,
        /// Advice is a content area which is shown when the student presses the Reveal button to reveal the question’s answers, or at the end of the exam.
        /// The advice area is normally used to present a worked solution to the question.
        advice: ContentAreaTranslatableString,
        parts: QuestionParts,
        builtin_constants: BuiltinConstants,
        custom_constants: CustomConstants,
        variables: StringToVariableRepresentation,
        variables_test: VariablesTest,
        functions: StringToFunction,
        preamble: Preamble,
        navigation: QuestionNavigation,
        extensions: Extensions,
        /// The names of the topics used in diagnostic exams that this question belongs to
        diagnostic_topic_names: TranslatableStrings, // TODO: validate? / warnings?
        resources: ResourcePaths,
        /// The custom part types used in this exam
        custom_part_types: CustomPartTypeDefinitionPaths
        //TODO a lot of options
    }
}

type StringToVariableRepresentationInput = HashMap<String, Value<VariableRepresentationInput>>;
type StringToVariableRepresentation = HashMap<String, VariableRepresentation>;

type StringToFunctionInput = HashMap<String, Value<FunctionInput>>;
type StringToFunction = HashMap<String, Function>;

impl ToNumbas<numbas::question::question::Question> for Question {
    fn to_numbas(&self, _locale: &str) -> numbas::question::question::Question {
        //TODO?
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
    //TODO: add to_numbas on Option's to reduce burden?
    fn to_numbas_with_name(
        &self,
        locale: &str,
        name: String,
    ) -> numbas::question::question::Question {
        if self.variables.contains_key("e") {
            panic!("e is not allowed as a variable name"); //TODO is this still the case?
        }
        numbas::question::question::Question {
            name,
            statement: self.statement.to_numbas(locale),
            advice: self.advice.to_numbas(locale),
            parts: self.parts.to_numbas(locale),
            builtin_constants: numbas::question::constants::BuiltinConstants(
                self.builtin_constants.clone().to_numbas(locale),
            ),
            constants: self.custom_constants.to_numbas(locale),
            variables: self
                .variables
                .clone()
                .into_iter()
                .map(|(k, v)| (k.clone(), v.to_numbas_with_name(locale, k)))
                .collect(),
            variables_test: self.variables_test.clone().to_numbas(locale),
            functions: self.functions.to_numbas(locale),
            ungrouped_variables: self
                .variables
                .clone()
                .into_iter()
                .filter(|(_k, v)| &v.to_variable().group[..] == UNGROUPED_GROUP)
                .map(|(k, _)| k)
                .collect(),
            variable_groups: Vec::new(), // Don't add variable groups
            rulesets: HashMap::new(),    //TODO: add to Question type ?
            preamble: self.preamble.to_numbas(locale),
            navigation: self.navigation.to_numbas(locale),
            extensions: self.extensions.to_numbas(locale),
            tags: self
                .diagnostic_topic_names
                .iter()
                .map(|t| format!("skill: {}", t.to_string(locale).unwrap()))
                .collect(),
            resources: self.resources.to_numbas(locale),
            custom_part_types: self
                .custom_part_types
                .iter()
                .map(|c| {
                    c.custom_part_type_data
                        .to_numbas_with_name(locale, c.custom_part_type_name.to_owned())
                })
                .collect(),
        }
    }
}

impl ToRumbas<Question> for numbas::question::question::Question {
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
            diagnostic_topic_names: self
                .tags
                .iter()
                .filter(|t| t.starts_with("skill: "))
                .map(|t| t.splitn(2, ": ").collect::<Vec<_>>()[1].to_string().into())
                .collect(),
            resources: self.resources.to_rumbas(),
            custom_part_types: self.custom_part_types.to_rumbas(),
        }
    }
}

impl QuestionInput {
    pub fn from_name(name: &str) -> YamlResult<QuestionInput> {
        use QuestionFileTypeInput::*;
        let file = Path::new(crate::QUESTIONS_FOLDER).join(format!("{}.yaml", name));
        let yaml = fs::read_to_string(&file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        let input: std::result::Result<QuestionFileTypeInput, serde_yaml::Error> =
            serde_yaml::from_str(&yaml);
        input
            .map(|e| match e {
                Normal(e) => Ok(*e),
                Template(t_res) => {
                    let t = t_res.to_normal(); // TODO?
                    let template_file = Path::new(crate::QUESTION_TEMPLATES_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path));
                    let template_yaml = fs::read_to_string(&template_file).expect(
                        &format!(
                            "Failed to read {}",
                            template_file.to_str().map_or("invalid filename", |s| s)
                        )[..],
                    );
                    let mut question: QuestionInput = serde_yaml::from_str(&template_yaml).unwrap();
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

optional_overwrite_enum! {
    #[serde(rename_all = "snake_case")]
    #[serde(tag = "type")]
    pub enum QuestionFileType {
        Template(TemplateFile),
        Normal(BoxQuestion)
    }
}

type BoxQuestion = Box<Question>;
type BoxQuestionInput = Box<QuestionInput>;

impl QuestionFileType {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        QuestionFileTypeInput::from_normal(self.to_owned()).to_yaml()
    }
}

impl QuestionFileTypeInput {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}
