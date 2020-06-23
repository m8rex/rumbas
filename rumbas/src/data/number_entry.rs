use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

question_part_type! {
    QuestionPartNumberEntry,
    answer: NumberEntryAnswer,
    display_correct_as_fraction: bool,
    allow_fractions: bool,
    allowed_notation_styles: Vec<numbas::exam::AnswerStyle>,

    display_correct_in_style: numbas::exam::AnswerStyle,
    fractions_must_be_reduced: bool,
    partial_credit_if_fraction_not_reduced: f64,

    hint_fraction: bool

    //TODO: precision, show_precision_hint

}
impl_optional_overwrite!(numbas::exam::AnswerStyle);

impl ToNumbas for QuestionPartNumberEntry {
    type NumbasType = numbas::exam::ExamQuestionPartNumberEntry;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(&locale),
                correct_answer_fraction: self.display_correct_as_fraction.clone().unwrap(),
                correct_answer_style: self.display_correct_in_style.clone(),
                allow_fractions: self.allow_fractions.unwrap(),
                notation_styles: self.allowed_notation_styles.clone(),
                fractions_must_be_reduced: self.fractions_must_be_reduced,
                partial_credit_if_fraction_not_reduced: self.partial_credit_if_fraction_not_reduced,
                precision: None,           //TODO
                show_precision_hint: None, //TODO
                show_fraction_hint: self.hint_fraction,
                answer: self.answer.to_numbas(locale).unwrap(),

                checking_type: Some(numbas::exam::CheckingType::Range), //TODO
            })
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NumberEntryAnswer {
    Normal(FileString),
    Range { from: FileString, to: FileString },
}
impl_optional_overwrite!(NumberEntryAnswer);

impl ToNumbas for NumberEntryAnswer {
    type NumbasType = numbas::exam::NumberEntryAnswerType;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            NumberEntryAnswer::Normal(f) => numbas::exam::NumberEntryAnswerType::Answer {
                answer: numbas::exam::Primitive::String(f.get_content()),
            },
            NumberEntryAnswer::Range { from, to } => numbas::exam::NumberEntryAnswerType::MinMax {
                min_value: numbas::exam::Primitive::String(from.get_content()),
                max_value: numbas::exam::Primitive::String(to.get_content()),
            },
        })
    }
}