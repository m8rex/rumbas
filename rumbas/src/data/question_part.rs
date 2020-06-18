use crate::data::gapfill::QuestionPartGapFill;
use crate::data::jme::QuestionPartJME;
use crate::data::multiple_choice::QuestionPartChooseOne;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};

optional_overwrite_enum! {
    QuestionPart: serde(tag = "type"),
    JME: QuestionPartJME: serde(rename = "jme"),
    GapFill: QuestionPartGapFill: serde(rename = "gapfill"),
    ChooseOne: QuestionPartChooseOne: serde(rename = "choose_one")
}

impl ToNumbas for QuestionPart {
    type NumbasType = numbas::exam::ExamQuestionPart;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamQuestionPart> {
        match self {
            QuestionPart::JME(d) => {
                let n = d.to_numbas(&locale)?;
                Ok(numbas::exam::ExamQuestionPart::JME(n))
            }
            QuestionPart::GapFill(d) => {
                let n = d.to_numbas(&locale)?;
                Ok(numbas::exam::ExamQuestionPart::GapFill(n))
            }
            QuestionPart::ChooseOne(d) => {
                let n = d.to_numbas(&locale)?;
                Ok(numbas::exam::ExamQuestionPart::ChooseOne(n))
            }
        }
    }
}

macro_rules! question_part_type {
    ($struct: ident, $($field: ident: $type: ty$(: $field_attribute: meta)?), *) => {
        optional_overwrite! {
            $struct,
            marks: usize,
            prompt: TranslatableString,
            use_custom_name: bool,
            custom_name: String, //Translatable?
            steps_penalty: usize,
            enable_minimum_marks: bool,
            minimum_marks: usize, //TODO: separate?
            show_correct_answer: bool,
            show_feedback_icon: bool,
            variable_replacement_strategy: VariableReplacementStrategy,
            adaptive_marking_penalty: usize,
            custom_marking_algorithm: String, // TODO? empty string -> none?, from file?
            extend_base_marking_algorithm: bool,
            steps: Vec<QuestionPart>,
            $(
                $field: $type $(: $field_attribute)?
            ),*
        }
        impl $struct {
            fn to_numbas_shared_data(&self, locale: &String) -> numbas::exam::ExamQuestionPartSharedData {
                numbas::exam::ExamQuestionPartSharedData::new(
            self.marks,
            self.prompt.clone().map(|s| s.to_string(&locale)).flatten(),
            self.use_custom_name,
            self.custom_name.clone(),
            self.steps_penalty,
            self.enable_minimum_marks,
            self.minimum_marks,
            self.show_correct_answer.clone().unwrap(),
            self.show_feedback_icon,
            self.variable_replacement_strategy.clone().unwrap().to_numbas(&locale).unwrap(),
            self.adaptive_marking_penalty,
            self.custom_marking_algorithm.clone(),
            self.extend_base_marking_algorithm,
            self.steps.clone().map(|v| v.iter().map(|s| s.to_numbas(&locale).unwrap()).collect()),
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    OriginalFirst,
}
impl_optional_overwrite!(VariableReplacementStrategy);

impl ToNumbas for VariableReplacementStrategy {
    type NumbasType = numbas::exam::VariableReplacementStrategy;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            VariableReplacementStrategy::OriginalFirst => {
                numbas::exam::VariableReplacementStrategy::OriginalFirst
            }
        })
    }
}
