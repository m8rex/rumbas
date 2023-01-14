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

use crate::exam::{FileReadError, ParseError, RecursiveTemplatesError};
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::question::part::jme::JMERulesetItem;
use crate::question::part::question_part::QuestionPart;
use crate::support::file_manager::CACHE;
use crate::support::noneable::Noneable;
use crate::support::template::TemplateFile;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use comparable::Comparable;
use constants::BuiltinConstants;
use constants::CustomConstant;
use extension::Extensions;
use function::Function;
use navigation::QuestionNavigation;
use preamble::Preamble;
use resource::ResourcePath;
use rumbas_support::path::RumbasPath;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use structdoc::StructDoc;
use variable::VariableRepresentation;
use variable::UNGROUPED_GROUP;
use variable_test::VariablesTest;

use self::variable::VariableToNumbasHelper;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct Question {
    /// The statement is a content area which appears at the top of the question, before any input boxes. Use the statement to set up the question and provide any information the student needs to answer it.
    pub statement: ContentAreaTranslatableString,
    /// Advice is a content area which is shown when the student presses the Reveal button to reveal the question’s answers, or at the end of the exam.
    /// The advice area is normally used to present a worked solution to the question.
    pub advice: ContentAreaTranslatableString,
    /// A question consists of one or more parts. Each part can have a different type to create
    /// elaborate questions.
    pub parts: Vec<QuestionPart>,
    /// Specifies which constants are enabled. You might want to disable the constant e so it can
    /// be used as a variable in the questions.
    pub builtin_constants: BuiltinConstants,
    /// Custom constants that are used in your question.
    pub custom_constants: Vec<CustomConstant>,
    /// The (ungrouped) variables that are used in this question.
    pub variables: BTreeMap<String, VariableRepresentation>,
    /// The (grouped) variables that are used in this question. This is a map from a group name to a map of variables. Should mainly be used to make it easier to template multiple variables at once.
    pub grouped_variables: Noneable<BTreeMap<String, BTreeMap<String, VariableRepresentation>>>,
    /// The test to which your variables should comply. Sometimes it’s hard to define randomised question variables so they’re guaranteed to produce a usable set of values. In these cases, it’s easier to state the condition you want the variables to satisfy, Variable values are generated until this condition passes.
    ///
    /// While this tool allows you to pick sets of variables that would be hard to generate constructively, it’s a random process so you must be aware that there’s a chance no suitable set of values will ever be found.
    pub variables_test: VariablesTest,
    /// The functions that are used in this question
    pub functions: BTreeMap<String, Function>,
    /// Specify custom javascript and css code that should be loaded.
    pub preamble: Preamble,
    /// Specify some navigation options for the question.
    // TODO: does this do anything?
    pub navigation: QuestionNavigation,
    /// Use this to enable the extensions that are used in the question
    pub extensions: Extensions,
    /// The names of the topics used in diagnostic exams that this question belongs to
    pub diagnostic_topic_names: Vec<TranslatableString>, // TODO: validate? / warnings?
    /// The paths to the resources
    pub resources: Vec<ResourcePath>,
    /// The custom part types used in this exam
    #[input(skip)]
    pub custom_part_types: Vec<CustomPartTypeDefinitionPath>, //TODO a lot of options
    /// The rulesets defined in this question. A “ruleset” defines a list of named simplification rules used to manipulate mathematical expressions. https://numbas-editor.readthedocs.io/en/latest/question/reference.html#rulesets
    pub rulesets: BTreeMap<String, JMERulesetItem>,
}

impl ToNumbas<numbas::question::Question> for Question {
    type ToNumbasHelper = String;
    fn to_numbas(&self, locale: &str, name: &String) -> numbas::question::Question {
        if self.variables.contains_key("e") {
            panic!("e is not allowed as a variable name"); //TODO is this still the case? // TODO: check this in validation?
        }
        numbas::question::Question {
            name: name.clone(),
            statement: self.statement.to_numbas(locale, &()),
            advice: self.advice.to_numbas(locale, &()),
            parts: self.parts.to_numbas(locale, &()),
            builtin_constants: numbas::question::constants::BuiltinConstants(
                self.builtin_constants.clone().to_numbas(locale, &()),
            ),
            constants: self.custom_constants.to_numbas(locale, &()),
            variables: self
                .variables
                .clone()
                .into_iter()
                .map(|(k, v)| (k.clone(), v.to_numbas(locale, &VariableToNumbasHelper::ungrouped(k))))
                .chain(
                    self.grouped_variables.clone().unwrap_or_default().into_iter().flat_map(|(group, variable_map)| 
                        variable_map.into_iter()
                        .map(move |(k, v)| (k.clone(), v.to_numbas(locale, &VariableToNumbasHelper::with_group(k, group.clone()))))                       
                    )
                )
                .collect(),
            variables_test: self.variables_test.clone().to_numbas(locale, &()),
            functions: self.functions.to_numbas(locale, &()),
            ungrouped_variables: self
                .variables
                .clone()
                .into_iter()
                .map(|(k, _)| k)
                .collect(),
            variable_groups: self.grouped_variables.clone().unwrap_or_default().into_iter().map(|(group, variable_map)| 
                numbas::question::variable::VariableGroup {
                    name: group,
                    variables: variable_map.into_iter()
                    .map(|(k, _)| k).collect()

                }
                                  
        ).collect(), // TODO Don't add variable groups
            preamble: self.preamble.to_numbas(locale, &()),
            rulesets: self.rulesets.to_numbas(locale, &()),
            navigation: self.navigation.to_numbas(locale, &()),
            extensions: self.extensions.to_numbas(locale, &()),
            tags: self
                .diagnostic_topic_names
                .iter()
                .map(|t| format!("skill: {}", t.to_string(locale).unwrap()))
                .collect(),
            resources: self.resources.to_numbas(locale, &()),
            custom_part_types: self
                .custom_part_types
                .iter()
                .map(|c| c.data.to_numbas(locale, &c.file_name))
                .collect(),
        }
    }
}

impl ToRumbas<Question> for numbas::question::Question {
    fn to_rumbas(&self) -> Question {

        // Split variables to different groups
        let mut variables_by_groups: BTreeMap<String, BTreeMap<String, numbas::question::variable::Variable>> = BTreeMap::new();
        self.variables.clone().into_iter().map(|(_,v)| {
            let group = if v.group.is_empty() || &v.group[..] == UNGROUPED_GROUP {
                UNGROUPED_GROUP.to_string()
            } else {
                v.group.replace(" ", "_").to_lowercase()
            };
            (group, v)
        }).for_each(|(group, v)| {
            if let Some(l) = variables_by_groups.get_mut(&group) {
                l.insert(v.name.clone(), v);
            } else {
                variables_by_groups.insert(group, vec![(v.name.clone(), v)].into_iter().collect());
            }
        });
        let variables = variables_by_groups.get(UNGROUPED_GROUP).map(|k| k.clone()).unwrap_or_default();
        let grouped_variables : BTreeMap<_,_> = variables_by_groups.into_iter().filter(|(k,_)| k != &UNGROUPED_GROUP.to_string()).collect();

        let grouped_variables = if grouped_variables.is_empty() { 
            Noneable::None } else { Noneable::NotNone(grouped_variables.to_rumbas())};

        Question {
            statement: self.statement.to_rumbas(),
            advice: self.advice.to_rumbas(),
            parts: self.parts.to_rumbas(),
            builtin_constants: self.builtin_constants.to_rumbas(),
            custom_constants: self.constants.to_rumbas(),
            variables: variables.to_rumbas(),
            grouped_variables,
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
            rulesets: self.rulesets.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionFileTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum QuestionFileType {
    /// A question that uses a template
    Template(TemplateFile),
    /// A normal question
    Normal(Box<Question>),
}

impl QuestionFileType {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl QuestionFileTypeInput {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}
