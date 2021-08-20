use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartExtension {}
}

impl ToNumbas<numbas::exam::ExamQuestionPartExtension> for QuestionPartExtension {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartExtension {
        numbas::exam::ExamQuestionPartExtension {
            part_data: self.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartExtension> for numbas::exam::ExamQuestionPartExtension {
    fn to_rumbas(&self) -> QuestionPartExtension {
        create_question_part! {QuestionPartExtension with &self.part_data => {}}
    }
}
