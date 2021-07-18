use crate::data::optional_overwrite::*;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartInformation {}
}

impl ToNumbas for QuestionPartInformation {
    type NumbasType = numbas::exam::ExamQuestionPartInformation;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(locale),
            })
        } else {
            Err(check)
        }
    }
}
