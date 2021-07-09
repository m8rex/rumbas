use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Function {
        parameters: Vec<(String, numbas::exam::ExamFunctionType)>,
        output_type: numbas::exam::ExamFunctionType,
        definition: FileString,
        language: numbas::exam::ExamFunctionLanguage
    }
}
impl_optional_overwrite! {(String, numbas::exam::ExamFunctionType)}

impl ToNumbas for Function {
    type NumbasType = numbas::exam::ExamFunction;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamFunction> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamFunction::new(
                self.parameters
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|(a, b)| (a, b))
                    .collect(),
                self.output_type.clone().unwrap(),
                self.definition.clone().unwrap().get_content(&locale),
                self.language.clone().unwrap(),
            ))
        } else {
            Err(check)
        }
    }
}

impl_optional_overwrite!(
    numbas::exam::ExamFunctionType,
    numbas::exam::ExamFunctionLanguage
);
