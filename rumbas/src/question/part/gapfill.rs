use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::question::part::question_part::{
    QuestionPartInput, QuestionPartsInput, VariableReplacementStrategyInput,
};
use crate::question::part::question_part::{QuestionParts, VariableReplacementStrategy};
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    // The Gap fill question part type
    pub struct QuestionPartGapFill {
        /// Whether the answers should be sorted
        sort_answers: RumbasBool,
        /// The gaps
        gaps: QuestionParts
    }
}

impl ToNumbas<numbas::question::part::gapfill::QuestionPartGapFill> for QuestionPartGapFill {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::gapfill::QuestionPartGapFill {
        numbas::question::part::gapfill::QuestionPartGapFill {
            part_data: self.to_numbas(locale),
            sort_answers: Some(self.sort_answers.to_numbas(locale)),
            gaps: self.gaps.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartGapFill> for numbas::question::part::gapfill::QuestionPartGapFill {
    fn to_rumbas(&self) -> QuestionPartGapFill {
        create_question_part! {
            QuestionPartGapFill with &self.part_data => {

                sort_answers: self.sort_answers.unwrap_or(DEFAULTS.gapfill_sort_answers).to_rumbas(),

                gaps: self.gaps.to_rumbas()
            }
        }
    }
}
