use crate::data::optional_overwrite::{EmptyFields, Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
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

impl ToNumbas for QuestionPartGapFill {
    type NumbasType = numbas::exam::ExamQuestionPartGapFill;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamQuestionPartGapFill> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionPartGapFill::new(
                self.to_numbas_shared_data(&locale),
                Some(self.sort_answers.clone().unwrap()),
                self.gaps
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|g| g.to_numbas(&locale).unwrap())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
