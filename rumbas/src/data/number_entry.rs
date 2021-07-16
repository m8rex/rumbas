use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartNumberEntry {
        answer: NumberEntryAnswer,
        display_correct_as_fraction: bool,
        allow_fractions: bool,
        allowed_notation_styles: Vec<AnswerStyle>,

        display_correct_in_style: AnswerStyle,
        fractions_must_be_reduced: bool,
        partial_credit_if_fraction_not_reduced: numbas::exam::Primitive,

        hint_fraction: bool

        //TODO: precision, show_precision_hint
    }

}
impl_optional_overwrite!(numbas::exam::AnswerStyle);

impl ToNumbas for QuestionPartNumberEntry {
    type NumbasType = numbas::exam::ExamQuestionPartNumberEntry;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(&locale),
                correct_answer_fraction: self.display_correct_as_fraction.clone().unwrap(),
                correct_answer_style: Some(
                    self.display_correct_in_style
                        .clone()
                        .map(|a| a.to_numbas(&locale).unwrap())
                        .unwrap(),
                ),
                allow_fractions: self.allow_fractions.unwrap(),
                notation_styles: Some(
                    self.allowed_notation_styles
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.to_numbas(&locale).unwrap())
                        .collect(),
                ),
                fractions_must_be_reduced: Some(self.fractions_must_be_reduced.clone().unwrap()),
                partial_credit_if_fraction_not_reduced: Some(
                    self.partial_credit_if_fraction_not_reduced.clone().unwrap(),
                ),
                precision: None,           //TODO
                show_precision_hint: None, //TODO
                show_fraction_hint: Some(self.hint_fraction.clone().unwrap()),
                answer: self.answer.to_numbas(locale).unwrap(),
                // checking_type: Some(numbas::exam::CheckingType::Range), //TODO
            })
        } else {
            Err(check)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NumberEntryAnswer {
    Normal(FileString), //TODO: filestrings?
    Range { from: FileString, to: FileString },
}
impl_optional_overwrite!(NumberEntryAnswer);

impl ToNumbas for NumberEntryAnswer {
    type NumbasType = numbas::exam::NumberEntryAnswerType;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            NumberEntryAnswer::Normal(f) => numbas::exam::NumberEntryAnswerType::Answer {
                answer: numbas::exam::Primitive::String(f.get_content(&locale)),
            },
            NumberEntryAnswer::Range { from, to } => numbas::exam::NumberEntryAnswerType::MinMax {
                min_value: numbas::exam::Primitive::String(from.get_content(&locale)),
                max_value: numbas::exam::Primitive::String(to.get_content(&locale)),
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AnswerStyle {
    /// English style - commas separate thousands, dot for decimal point
    #[serde(rename = "english")]
    English,
    /// Plain English style - no thousands separator, dot for decimal point
    #[serde(rename = "english-plain")]
    EnglishPlain,
    /// English SI style - spaces separate thousands, dot for decimal point
    #[serde(rename = "english-si")]
    EnglishSI,
    /// Continental European style - dots separate thousands, comma for decimal poin
    #[serde(rename = "european")]
    European,
    /// Plain French style - no thousands separator, comma for decimal point
    #[serde(rename = "european-plain")]
    EuropeanPlain,
    /// French SI style - spaces separate thousands, comma for decimal point
    #[serde(rename = "french-si")]
    FrenchSI,
    /// Indian style - commas separate groups, dot for decimal point. The rightmost group is three digits, other groups are two digits.
    #[serde(rename = "indian")]
    Indian,
    /// Significand-exponent ("scientific") style
    #[serde(rename = "scientific")]
    Scientific,
    /// Swiss style - apostrophes separate thousands, dot for decimal point
    #[serde(rename = "swiss")]
    Swiss,
}
impl_optional_overwrite!(AnswerStyle);

impl ToNumbas for AnswerStyle {
    type NumbasType = numbas::exam::AnswerStyle;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            AnswerStyle::English => numbas::exam::AnswerStyle::English,
            AnswerStyle::EnglishPlain => numbas::exam::AnswerStyle::EnglishPlain,
            AnswerStyle::EnglishSI => numbas::exam::AnswerStyle::EnglishSI,
            AnswerStyle::European => numbas::exam::AnswerStyle::European,
            AnswerStyle::EuropeanPlain => numbas::exam::AnswerStyle::EuropeanPlain,
            AnswerStyle::FrenchSI => numbas::exam::AnswerStyle::FrenchSI,
            AnswerStyle::Indian => numbas::exam::AnswerStyle::Indian,
            AnswerStyle::Scientific => numbas::exam::AnswerStyle::Scientific,
            AnswerStyle::Swiss => numbas::exam::AnswerStyle::Swiss,
        })
    }
}

impl ToRumbas for numbas::exam::AnswerStyle {
    type RumbasType = AnswerStyle;
    fn to_rumbas(&self) -> Self::RumbasType {
        match self {
            numbas::exam::AnswerStyle::English => AnswerStyle::English,
            numbas::exam::AnswerStyle::EnglishPlain => AnswerStyle::EnglishPlain,
            numbas::exam::AnswerStyle::EnglishSI => AnswerStyle::EnglishSI,
            numbas::exam::AnswerStyle::European => AnswerStyle::European,
            numbas::exam::AnswerStyle::EuropeanPlain => AnswerStyle::EuropeanPlain,
            numbas::exam::AnswerStyle::FrenchSI => AnswerStyle::FrenchSI,
            numbas::exam::AnswerStyle::Indian => AnswerStyle::Indian,
            numbas::exam::AnswerStyle::Scientific => AnswerStyle::Scientific,
            numbas::exam::AnswerStyle::Swiss => AnswerStyle::Swiss,
        }
    }
}
