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
    pub struct QuestionPartInformation {}
}

impl ToNumbas<numbas::question::part::information::QuestionPartInformation> for QuestionPartInformation {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::information::QuestionPartInformation {
        numbas::question::part::information::QuestionPartInformation {
            part_data: self.to_numbas(locale), // TODO: to numbas?
        }
    }
}

impl ToRumbas<QuestionPartInformation> for numbas::question::part::information::QuestionPartInformation {
    fn to_rumbas(&self) -> QuestionPartInformation {
        create_question_part!(QuestionPartInformation with &self.part_data => {})
    }
}
