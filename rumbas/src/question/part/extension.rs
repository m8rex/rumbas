use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::{AdaptiveMarking, CustomMarking};
use crate::question::QuestionPart;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
    #[input(name = "QuestionPartExtensionInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartExtension {}
}

impl ToNumbas<numbas::question::part::extension::QuestionPartExtension> for QuestionPartExtension {
    type ToNumbasHelper = ();
    fn to_numbas(&self, locale: &str, _: &Self::ToNumbasHelper) -> numbas::question::part::extension::QuestionPartExtension {
        numbas::question::part::extension::QuestionPartExtension {
            part_data: self.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<QuestionPartExtension> for numbas::question::part::extension::QuestionPartExtension {
    fn to_rumbas(&self) -> QuestionPartExtension {
        create_question_part! {QuestionPartExtension with &self.part_data => {}}
    }
}
