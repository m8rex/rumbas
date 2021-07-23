use crate::data::custom_part_type::CustomPartTypeDefinitionPath;
use crate::data::extension::Extensions;
use crate::data::file_reference::FileString;
use crate::data::function::Function;
use crate::data::navigation::QuestionNavigation;
use crate::data::optional_overwrite::*;
use crate::data::preamble::Preamble;
use crate::data::question_part::QuestionPart;
use crate::data::resource::ResourcePath;
use crate::data::template::QuestionFileType;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use crate::data::translatable::TranslatableString;
use crate::data::variable::VariableRepresentation;
use crate::data::yaml::{YamlError, YamlResult};
use numbas::defaults::DEFAULTS;
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
        resources: Vec<Value<ResourcePath>>,
        /// The custom part types used in this exam
        custom_part_types: Vec<CustomPartTypeDefinitionPath>
        //TODO a lot of options
    }
}

impl ToNumbas for Question {
    type NumbasType = numbas::exam::ExamQuestion;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
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
    ) -> NumbasResult<numbas::exam::ExamQuestion> {
        let check = self.check();
        if check.is_empty() {
            if self.variables.unwrap().contains_key("e") {
                panic!("e is not allowed as a variable name"); //TODO
            }
            Ok(numbas::exam::ExamQuestion {
                name,
                statement: self.statement.clone().unwrap().to_string(locale).unwrap(),
                advice: self.advice.clone().unwrap().to_string(locale).unwrap(),
                parts: self
                    .parts
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| p.to_numbas(locale).unwrap())
                    .collect(),
                builtin_constants: numbas::exam::BuiltinConstants(
                    self.builtin_constants
                        .clone()
                        .unwrap()
                        .to_numbas(locale)
                        .unwrap(),
                ),
                constants: self
                    .custom_constants
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| p.to_numbas(locale).unwrap())
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
                                .to_numbas_with_name(locale, k)
                                .unwrap(),
                        )
                    })
                    .collect(),
                variables_test: self
                    .variables_test
                    .clone()
                    .unwrap()
                    .to_numbas(locale)
                    .unwrap(),
                functions: self
                    .functions
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_numbas(locale).unwrap()))
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
                preamble: self.preamble.clone().unwrap().to_numbas(locale).unwrap(),
                navigation: self.navigation.clone().unwrap().to_numbas(locale).unwrap(),
                extensions: self.extensions.clone().unwrap().to_numbas(locale).unwrap(),
                tags: self
                    .diagnostic_topic_names
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|t| format!("skill: {}", t.to_string(locale).unwrap()))
                    .collect(),
                resources: self.resources.clone().unwrap().to_numbas(locale).unwrap(),
                custom_part_types: self
                    .custom_part_types
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|c| {
                        c.custom_part_type_data
                            .to_numbas_with_name(locale, c.custom_part_type_name)
                            .unwrap()
                    })
                    .collect(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<Question> for numbas::exam::ExamQuestion {
    fn to_rumbas(&self) -> Question {
        Question {
            statement: Value::Normal(TranslatableString::s(&self.statement)),
            advice: Value::Normal(TranslatableString::s(&self.advice)),
            parts: Value::Normal(
                self.parts
                    .iter()
                    .map(|p| Value::Normal(p.to_rumbas()))
                    .collect(),
            ),
            builtin_constants: Value::Normal(self.builtin_constants.to_rumbas()),
            custom_constants: Value::Normal(
                self.constants.iter().map(|cc| cc.to_rumbas()).collect(),
            ),
            variables: Value::Normal(
                self.variables
                    .iter()
                    .map(|(k, v)| (k.clone(), Value::Normal(v.to_rumbas())))
                    .collect::<std::collections::HashMap<_, _>>(),
            ),
            variables_test: Value::Normal(self.variables_test.to_rumbas()),
            functions: Value::Normal(
                self.functions
                    .iter()
                    .map(|(k, f)| (k.clone(), Value::Normal(f.to_rumbas())))
                    .collect::<std::collections::HashMap<_, _>>(),
            ),
            preamble: Value::Normal(Preamble {
                js: Value::Normal(FileString::s(&self.preamble.js)),
                css: Value::Normal(FileString::s(&self.preamble.css)),
            }),
            navigation: Value::Normal(self.navigation.to_rumbas()),
            extensions: Value::Normal(Extensions::from(&self.extensions)),
            diagnostic_topic_names: Value::Normal(
                self.tags
                    .iter()
                    .filter(|t| t.starts_with("skill: "))
                    .map(|t| {
                        TranslatableString::s(&t.splitn(2, ": ").collect::<Vec<_>>()[1].to_string())
                    })
                    .collect(),
            ),
            resources: Value::Normal(
                self.resources
                    .to_rumbas()
                    .into_iter()
                    .map(Value::Normal)
                    .collect(),
            ),
            custom_part_types: Value::Normal(self.custom_part_types.to_rumbas()),
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
    fn to_numbas(&self, _locale: &str) -> NumbasResult<numbas::exam::ExamQuestionVariablesTest> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamQuestionVariablesTest {
                condition: self.condition.clone().unwrap(),
                max_runs: self.max_runs.clone().unwrap().into(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<VariablesTest> for numbas::exam::ExamQuestionVariablesTest {
    fn to_rumbas(&self) -> VariablesTest {
        VariablesTest {
            condition: Value::Normal(self.condition.clone()),
            max_runs: Value::Normal(self.max_runs.0),
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
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
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

impl ToRumbas<BuiltinConstants> for numbas::exam::BuiltinConstants {
    fn to_rumbas(&self) -> BuiltinConstants {
        BuiltinConstants {
            e: Value::Normal(
                *self
                    .0
                    .get(&"e".to_string())
                    .unwrap_or(&DEFAULTS.builtin_constants_e),
            ),
            pi: Value::Normal(
                *self
                    .0
                    .get(&"pi,\u{03c0}".to_string())
                    .unwrap_or(&DEFAULTS.builtin_constants_pi),
            ),
            i: Value::Normal(
                *self
                    .0
                    .get(&"i".to_string())
                    .unwrap_or(&DEFAULTS.builtin_constants_i),
            ),
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
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
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

impl ToRumbas<CustomConstant> for numbas::exam::ExamQuestionConstant {
    fn to_rumbas(&self) -> CustomConstant {
        CustomConstant {
            name: Value::Normal(self.name.clone()),
            value: Value::Normal(self.value.clone()),
            tex: Value::Normal(self.tex.clone()),
        }
    }
}
