use crate::data::extension::Extensions;
use crate::data::function::Function;
use crate::data::navigation::QuestionNavigation;
use crate::data::optional_overwrite::*;
use crate::data::preamble::Preamble;
use crate::data::question_part::QuestionPart;
use crate::data::resource::ResourcePath;
use crate::data::template::{QuestionFileType, TEMPLATE_QUESTIONS_FOLDER};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use crate::data::variable::VariableRepresentation;
use crate::data::yaml::{YamlError, YamlResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const UNGROUPED_GROUP: &str = "Ungrouped variables";

optional_overwrite! {
    pub struct Question {
        statement: TranslatableString,
        advice: TranslatableString,
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
        resources: Vec<Value<ResourcePath>>
        //TODO a lot of options
    }
}

impl ToNumbas for Question {
    type NumbasType = numbas::exam::ExamQuestion;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        //TODO?
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
    //TODO: add to_numbas on Option's to reduce burden?
    fn to_numbas_with_name(
        &self,
        locale: &String,
        name: String,
    ) -> NumbasResult<numbas::exam::ExamQuestion> {
        let check = self.check();
        if check.is_empty() {
            if self.variables.unwrap().contains_key("e") {
                panic!("e is not allowed as a variable name"); //TODO
            }
            Ok(numbas::exam::ExamQuestion {
                name,
                statement: self.statement.clone().unwrap().to_string(&locale).unwrap(),
                advice: self.advice.clone().unwrap().to_string(&locale).unwrap(),
                parts: self
                    .parts
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| p.to_numbas(&locale).unwrap())
                    .collect(),
                builtin_constants: numbas::exam::BuiltinConstants(
                    self.builtin_constants
                        .clone()
                        .unwrap()
                        .to_numbas(&locale)
                        .unwrap(),
                ),
                constants: self
                    .custom_constants
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| p.to_numbas(&locale).unwrap())
                    .collect(),
                variables: self
                    .variables
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            v.unwrap()
                                .to_variable()
                                .to_numbas_with_name(&locale, k)
                                .unwrap(),
                        )
                    })
                    .collect(),
                variables_test: self
                    .variables_test
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                functions: self
                    .functions
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_numbas(&locale).unwrap()))
                    .collect(),
                ungrouped_variables: self
                    .variables
                    .clone()
                    .unwrap()
                    .into_iter()
                    .filter(|(_k, v)| {
                        &v.unwrap().to_variable().group.unwrap()[..] == UNGROUPED_GROUP
                    })
                    .map(|(k, _)| k)
                    .collect(),
                variable_groups: Vec::new(), // Don't add variable groups
                rulesets: HashMap::new(),    //TODO: add to Question type ?
                preamble: self.preamble.clone().unwrap().to_numbas(&locale).unwrap(),
                navigation: self.navigation.clone().unwrap().to_numbas(&locale).unwrap(),
                extensions: self.extensions.clone().unwrap().to_numbas(&locale).unwrap(),
                tags: self
                    .diagnostic_topic_names
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|t| format!("skill: {}", t.to_string(&locale).unwrap()))
                    .collect(),
                resources: self.resources.clone().unwrap().to_numbas(&locale).unwrap(),
            })
        } else {
            Err(check)
        }
    }
}

impl Question {
    pub fn from_name(name: &String) -> YamlResult<Question> {
        use QuestionFileType::*;
        let file = Path::new("questions").join(format!("{}.yaml", name));
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
                Normal(e) => Ok(e),
                Template(t) => {
                    let template_file = Path::new(TEMPLATE_QUESTIONS_FOLDER)
                        .join(format!("{}.yaml", t.relative_template_path));
                    let template_yaml = fs::read_to_string(&template_file).expect(
                        &format!(
                            "Failed to read {}",
                            template_file.to_str().map_or("invalid filename", |s| s)
                        )[..],
                    );
                    let mut question: Question = serde_yaml::from_str(&template_yaml).unwrap();
                    t.data.iter().for_each(|(k, v)| {
                        question.insert_template_value(k, v);
                    });
                    Ok(question)
                }
            })
            .and_then(std::convert::identity) //flatten result is currently only possible in nightly
            .map_err(|e| YamlError::from(e, file.to_path_buf()))
    }
}

optional_overwrite! {
    pub struct VariablesTest {
        condition: String,
        max_runs: usize
    }
}

impl ToNumbas for VariablesTest {
    type NumbasType = numbas::exam::ExamQuestionVariablesTest;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::ExamQuestionVariablesTest> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamQuestionVariablesTest::new(
                self.condition.clone().unwrap(),
                self.max_runs.clone().unwrap(),
            ))
        } else {
            Err(check)
        }
    }
}

optional_overwrite! {
    /// Specify which builtin constants should be enabled
    pub struct BuiltinConstants {
        /// Whether the constant e is enabled
        e: bool,
        /// Whether the constant pi is enabled
        pi: bool,
        /// Whether the constant i is enabled-
        i: bool
    }
}

impl ToNumbas for BuiltinConstants {
    type NumbasType = std::collections::HashMap<String, bool>;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            let mut builtin = std::collections::HashMap::new();
            // TODO: use macro to make sure that this list always remains up to date
            builtin.insert("e".to_string(), self.e.unwrap());
            builtin.insert("pi,\u{03c0}".to_string(), self.pi.unwrap());
            builtin.insert("i".to_string(), self.i.unwrap());
            Ok(builtin)
        } else {
            Err(check)
        }
    }
}

optional_overwrite! {
    /// A custom constant
    pub struct CustomConstant {
        /// The name of the constant
        name: String,
        /// The value of the constant
        value: String,
        /// The tex code use to display the constant
        tex: String
    }
}

impl ToNumbas for CustomConstant {
    type NumbasType = numbas::exam::ExamQuestionConstant;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(Self::NumbasType {
                name: self.name.clone().unwrap(),
                value: self.value.clone().unwrap(),
                tex: self.tex.clone().unwrap(),
            })
        } else {
            Err(check)
        }
    }
}
