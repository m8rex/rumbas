use crate::data::extension::Extensions;
use crate::data::function::Function;
use crate::data::navigation::QuestionNavigation;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::preamble::Preamble;
use crate::data::question_part::QuestionPart;
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

pub const UNGROUPED_GROUP: &'static str = "Ungrouped variables";

optional_overwrite! {
    Question,
    statement: TranslatableString,
    advice: TranslatableString,
    parts: Vec<Value<QuestionPart>>,
    variables: HashMap<String, Value<VariableRepresentation>>,
    variables_test: VariablesTest,
    functions: HashMap<String, Value<Function>>,
    preamble: Preamble,
    navigation: QuestionNavigation,
    extensions: Extensions
    //TODO al lot of options

}

impl ToNumbas for Question {
    type NumbasType = numbas::exam::ExamQuestion;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        //TODO?
        Err(vec![
            "Should not happen, don't call this method Missing name".to_string(),
        ])
    }
    //TODO: add to_numbas on Option's to reduce burden?
    fn to_numbas_with_name(
        &self,
        locale: &String,
        name: String,
    ) -> NumbasResult<numbas::exam::ExamQuestion> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            if self.variables.unwrap().contains_key("e") {
                panic!("e is not allowed as a variable name"); //TODO
            }
            Ok(numbas::exam::ExamQuestion::new(
                name,
                self.statement.clone().unwrap().to_string(&locale).unwrap(),
                self.advice.clone().unwrap().to_string(&locale).unwrap(),
                self.parts
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| p.to_numbas(&locale).unwrap())
                    .collect(),
                self.variables
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
                self.variables_test
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.functions
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_numbas(&locale).unwrap()))
                    .collect(),
                self.variables
                    .clone()
                    .unwrap()
                    .into_iter()
                    .filter(|(_k, v)| {
                        &v.unwrap().to_variable().group.clone().unwrap()[..] == UNGROUPED_GROUP
                    })
                    .map(|(k, _)| k)
                    .collect(),
                Vec::new(),     // Don't add variable groups
                HashMap::new(), //TODO: add to Question type ?
                self.preamble.clone().unwrap().to_numbas(&locale).unwrap(),
                self.navigation.clone().unwrap().to_numbas(&locale).unwrap(),
                self.extensions.clone().unwrap().to_numbas(&locale).unwrap(),
            ))
        } else {
            Err(empty_fields)
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
    VariablesTest,
    condition: String,
    max_runs: usize
}

impl ToNumbas for VariablesTest {
    type NumbasType = numbas::exam::ExamQuestionVariablesTest;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::ExamQuestionVariablesTest> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionVariablesTest::new(
                self.condition.clone().unwrap(),
                self.max_runs.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
