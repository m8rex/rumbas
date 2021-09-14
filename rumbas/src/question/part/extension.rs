use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionPart;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use numbas::support::primitive::Primitive;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck)]
    #[input(name = "QuestionPartExtensionInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct QuestionPartExtension {}
}

impl ToNumbas<numbas::question::part::extension::QuestionPartExtension> for QuestionPartExtension {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::extension::QuestionPartExtension {
        numbas::question::part::extension::QuestionPartExtension {
            part_data: self.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartExtension> for numbas::question::part::extension::QuestionPartExtension {
    fn to_rumbas(&self) -> QuestionPartExtension {
        create_question_part! {QuestionPartExtension with &self.part_data => {}}
    }
}
