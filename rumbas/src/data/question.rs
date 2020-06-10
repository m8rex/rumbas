use crate::data::extension::Extensions;
use crate::data::function::Function;
use crate::data::json::{JsonError, JsonResult};
use crate::data::navigation::QuestionNavigation;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::preamble::Preamble;
use crate::data::question_part::QuestionPart;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use crate::data::variable::Variable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

optional_overwrite! {
    Question,
    statement: TranslatableString,
    advice: TranslatableString,
    parts: Vec<QuestionPart>,
    variables: HashMap<String, Variable>,
    variables_test: VariablesTest,
    functions: HashMap<String, Function>,
    preamble: Preamble,
    navigation: QuestionNavigation,
    extensions: Extensions,
    ungrouped_variables: Vec<String>
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
                    .map(|(k, v)| (k.clone(), v.to_numbas_with_name(&locale, k).unwrap()))
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
                self.ungrouped_variables.clone().unwrap(),
                Vec::new(),     //TODO: calculate from variables
                HashMap::new(), //TODO: add to Question type
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
    pub fn from_name(name: &String) -> JsonResult<Question> {
        let file = Path::new("questions").join(format!("{}.json", name));
        let json = fs::read_to_string(&file).expect(
            &format!(
                "Failed to read {}",
                file.to_str().map_or("invalid filename", |s| s)
            )[..],
        );
        serde_json::from_str(&json).map_err(|e| JsonError::from(e, file.to_path_buf()))
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
