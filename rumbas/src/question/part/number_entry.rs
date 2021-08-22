use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::file_reference::FileString;
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
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

impl ToNumbas<numbas::exam::ExamQuestionPartNumberEntry> for QuestionPartNumberEntry {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartNumberEntry {
        numbas::exam::ExamQuestionPartNumberEntry {
            part_data: self.to_numbas(locale),
            correct_answer_fraction: self.display_correct_as_fraction.to_numbas(locale),
            correct_answer_style: Some(self.display_correct_in_style.to_numbas(locale)),
            allow_fractions: self.allow_fractions.to_numbas(locale),
            notation_styles: Some(self.allowed_notation_styles.to_numbas(locale)),
            fractions_must_be_reduced: Some(self.fractions_must_be_reduced.to_numbas(locale)),
            partial_credit_if_fraction_not_reduced: Some(
                self.partial_credit_if_fraction_not_reduced
                    .to_numbas(locale),
            ),
            precision: None,           //TODO
            show_precision_hint: None, //TODO
            show_fraction_hint: Some(self.hint_fraction.to_numbas(locale)),
            answer: self.answer.to_numbas(locale),
            // checking_type: Some(numbas::exam::CheckingType::Range), //TODO
        }
    }
}

impl ToRumbas<QuestionPartNumberEntry> for numbas::exam::ExamQuestionPartNumberEntry {
    fn to_rumbas(&self) -> QuestionPartNumberEntry {
        create_question_part! {
            QuestionPartNumberEntry with &self.part_data => {
                answer: self.answer.to_rumbas(),
                display_correct_as_fraction: self.correct_answer_fraction.to_rumbas(),
                allow_fractions: self.allow_fractions.to_rumbas(),
                allowed_notation_styles:
                    self.notation_styles.clone().unwrap_or_default().to_rumbas(),

                display_correct_in_style:
                    self.correct_answer_style
                        .clone()
                        .unwrap_or(DEFAULTS.number_entry_correct_answer_style)
                        .to_rumbas(),

                fractions_must_be_reduced:
                    self.fractions_must_be_reduced
                        .unwrap_or(DEFAULTS.number_entry_fractions_must_be_reduced).to_rumbas(),
                partial_credit_if_fraction_not_reduced:
                    self.partial_credit_if_fraction_not_reduced
                        .clone()
                        .unwrap_or(DEFAULTS.number_entry_partial_credit_if_fraction_not_reduced).to_rumbas(),
                hint_fraction:
                    self.show_fraction_hint
                        .unwrap_or(DEFAULTS.number_entry_hint_fraction).to_rumbas()

            }
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NumberEntryAnswer {
    Normal(FileString), //TODO: filestrings?
    Range { from: FileString, to: FileString },
}
impl_optional_overwrite!(NumberEntryAnswer);

impl ToNumbas<numbas::exam::NumberEntryAnswerType> for NumberEntryAnswer {
    fn to_numbas(&self, locale: &str) -> numbas::exam::NumberEntryAnswerType {
        match self {
            NumberEntryAnswer::Normal(f) => numbas::exam::NumberEntryAnswerType::Answer {
                answer: numbas::exam::Primitive::String(f.to_numbas(locale)),
            },
            NumberEntryAnswer::Range { from, to } => numbas::exam::NumberEntryAnswerType::MinMax {
                min_value: numbas::exam::Primitive::String(from.to_numbas(locale)),
                max_value: numbas::exam::Primitive::String(to.to_numbas(locale)),
            },
        }
    }
}

impl ToRumbas<NumberEntryAnswer> for numbas::exam::NumberEntryAnswerType {
    fn to_rumbas(&self) -> NumberEntryAnswer {
        match self {
            numbas::exam::NumberEntryAnswerType::MinMax {
                min_value,
                max_value,
            } => NumberEntryAnswer::Range {
                from: min_value.to_string().to_rumbas(),
                to: max_value.to_string().to_rumbas(),
            },
            numbas::exam::NumberEntryAnswerType::Answer { answer } => {
                NumberEntryAnswer::Normal(answer.to_string().to_rumbas())
            }
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
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

impl ToNumbas<numbas::exam::AnswerStyle> for AnswerStyle {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::AnswerStyle {
        match self {
            AnswerStyle::English => numbas::exam::AnswerStyle::English,
            AnswerStyle::EnglishPlain => numbas::exam::AnswerStyle::EnglishPlain,
            AnswerStyle::EnglishSI => numbas::exam::AnswerStyle::EnglishSI,
            AnswerStyle::European => numbas::exam::AnswerStyle::European,
            AnswerStyle::EuropeanPlain => numbas::exam::AnswerStyle::EuropeanPlain,
            AnswerStyle::FrenchSI => numbas::exam::AnswerStyle::FrenchSI,
            AnswerStyle::Indian => numbas::exam::AnswerStyle::Indian,
            AnswerStyle::Scientific => numbas::exam::AnswerStyle::Scientific,
            AnswerStyle::Swiss => numbas::exam::AnswerStyle::Swiss,
        }
    }
}

impl ToRumbas<AnswerStyle> for numbas::exam::AnswerStyle {
    fn to_rumbas(&self) -> AnswerStyle {
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
