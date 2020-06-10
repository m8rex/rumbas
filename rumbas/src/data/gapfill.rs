use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

question_part_type! {
    QuestionPartGapFill,
    sort_answers: bool,
    gaps: Vec<QuestionPart>
}

impl ToNumbas for QuestionPartGapFill {
    type NumbasType = numbas::exam::ExamQuestionPartGapFill;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamQuestionPartGapFill> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionPartGapFill::new(
                self.to_numbas_shared_data(&locale),
                self.sort_answers,
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
