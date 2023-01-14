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
    #[input(name = "QuestionPartInformationInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartInformation {}
}

impl ToNumbas<numbas::question::part::information::QuestionPartInformation>
    for QuestionPartInformation
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        locale: &str,
        _data: &Self::ToNumbasHelper
    ) -> numbas::question::part::information::QuestionPartInformation {
        numbas::question::part::information::QuestionPartInformation {
            part_data: self.to_numbas(locale, &()), // TODO: to numbas?
        }
    }
}

impl ToRumbas<QuestionPartInformation>
    for numbas::question::part::information::QuestionPartInformation
{
    fn to_rumbas(&self) -> QuestionPartInformation {
        create_question_part!(QuestionPartInformation with &self.part_data => {})
    }
}
