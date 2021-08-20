use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    // The Gap fill question part type
    pub struct QuestionPartGapFill {
        /// Whether the answers should be sorted
        sort_answers: bool,
        /// The gaps
        gaps: Vec<QuestionPart>
    }
}

impl ToNumbas<numbas::exam::ExamQuestionPartGapFill> for QuestionPartGapFill {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartGapFill {
        numbas::exam::ExamQuestionPartGapFill {
            part_data: self.to_numbas(locale),
            sort_answers: Some(self.sort_answers.to_numbas(locale)),
            gaps: self.gaps.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartGapFill> for numbas::exam::ExamQuestionPartGapFill {
    fn to_rumbas(&self) -> QuestionPartGapFill {
        create_question_part! {
            QuestionPartGapFill with &self.part_data => {

                sort_answers: Value::Normal(self.sort_answers.unwrap_or(DEFAULTS.gapfill_sort_answers)),

                gaps: Value::Normal(self.gaps.to_rumbas())
            }
        }
    }
}
