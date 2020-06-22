use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

question_part_type! {
    QuestionPartPatternMatch,
    case_sensitive: bool,
    partial_credit: f64,
    pattern: FileString, //TODO: type
    display_answer: FileString,
    match_mode: numbas::exam::PatternMatchMode
}
impl_optional_overwrite!(numbas::exam::PatternMatchMode);

impl ToNumbas for QuestionPartPatternMatch {
    type NumbasType = numbas::exam::ExamQuestionPartPatternMatch;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(&locale),
                case_sensitive: self.case_sensitive.unwrap(),
                partial_credit: self.partial_credit.unwrap(),
                answer: numbas::exam::Primitive::String(
                    self.pattern.clone().unwrap().get_content(),
                ),
                display_answer: Some(numbas::exam::Primitive::String(
                    self.display_answer.clone().unwrap().get_content(),
                )),
                match_mode: self.match_mode.unwrap(),
            })
        } else {
            Err(empty_fields)
        }
    }
}
