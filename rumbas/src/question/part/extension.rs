use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::{QuestionPartInput, VariableReplacementStrategyInput};
use crate::question::QuestionParts;
use crate::question::QuestionPartsInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
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
