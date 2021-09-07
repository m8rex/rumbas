use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::VariableReplacementStrategyInput;
use crate::question::QuestionPartInput;
use crate::question::QuestionParts;
use crate::question::QuestionPartsInput;
use crate::support::file_reference::FileString;
use crate::support::file_reference::FileStringInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use numbas::defaults::DEFAULTS;
use numbas::support::answer_style::AnswerStyle as NumbasAnswerStyle;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartNumberEntry {
        answer: NumberEntryAnswer,
        display_correct_as_fraction: RumbasBool,
        allow_fractions: RumbasBool,
        allowed_notation_styles: AnswerStyles,

        display_correct_in_style: AnswerStyle,
        fractions_must_be_reduced: RumbasBool,
        partial_credit_if_fraction_not_reduced: Primitive,

        hint_fraction: RumbasBool

        //TODO: precision, show_precision_hint
    }

}
impl_optional_overwrite!(NumbasAnswerStyle);

impl ToNumbas<numbas::question::part::number_entry::QuestionPartNumberEntry>
    for QuestionPartNumberEntry
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::number_entry::QuestionPartNumberEntry {
        numbas::question::part::number_entry::QuestionPartNumberEntry {
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

impl ToRumbas<QuestionPartNumberEntry>
    for numbas::question::part::number_entry::QuestionPartNumberEntry
{
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

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum NumberEntryAnswer {
        Normal(FileString), //TODO: filestrings?
        Range(NumberEntryAnswerRange)
    }
}
optional_overwrite! { // TODO, better (add toRumbas etc)
    pub struct NumberEntryAnswerRange {
        from: FileString,
        to: FileString
    }
}

impl ToNumbas<numbas::question::part::number_entry::NumberEntryAnswerType> for NumberEntryAnswer {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::number_entry::NumberEntryAnswerType {
        match self {
            NumberEntryAnswer::Normal(f) => {
                numbas::question::part::number_entry::NumberEntryAnswerType::Answer {
                    answer: numbas::support::primitive::Primitive::String(f.to_numbas(locale)),
                }
            }
            NumberEntryAnswer::Range(range) => {
                numbas::question::part::number_entry::NumberEntryAnswerType::MinMax {
                    min_value: numbas::support::primitive::Primitive::String(
                        range.from.to_numbas(locale),
                    ),
                    max_value: numbas::support::primitive::Primitive::String(
                        range.to.to_numbas(locale),
                    ),
                }
            }
        }
    }
}

impl ToRumbas<NumberEntryAnswer> for numbas::question::part::number_entry::NumberEntryAnswerType {
    fn to_rumbas(&self) -> NumberEntryAnswer {
        match self {
            numbas::question::part::number_entry::NumberEntryAnswerType::MinMax {
                min_value,
                max_value,
            } => NumberEntryAnswer::Range(NumberEntryAnswerRange {
                from: min_value.to_string().to_rumbas(),
                to: max_value.to_string().to_rumbas(),
            }),
            numbas::question::part::number_entry::NumberEntryAnswerType::Answer { answer } => {
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

impl ToNumbas<numbas::support::answer_style::AnswerStyle> for AnswerStyle {
    fn to_numbas(&self, _locale: &str) -> numbas::support::answer_style::AnswerStyle {
        match self {
            AnswerStyle::English => numbas::support::answer_style::AnswerStyle::English,
            AnswerStyle::EnglishPlain => numbas::support::answer_style::AnswerStyle::EnglishPlain,
            AnswerStyle::EnglishSI => numbas::support::answer_style::AnswerStyle::EnglishSI,
            AnswerStyle::European => numbas::support::answer_style::AnswerStyle::European,
            AnswerStyle::EuropeanPlain => numbas::support::answer_style::AnswerStyle::EuropeanPlain,
            AnswerStyle::FrenchSI => numbas::support::answer_style::AnswerStyle::FrenchSI,
            AnswerStyle::Indian => numbas::support::answer_style::AnswerStyle::Indian,
            AnswerStyle::Scientific => numbas::support::answer_style::AnswerStyle::Scientific,
            AnswerStyle::Swiss => numbas::support::answer_style::AnswerStyle::Swiss,
        }
    }
}

impl ToRumbas<AnswerStyle> for numbas::support::answer_style::AnswerStyle {
    fn to_rumbas(&self) -> AnswerStyle {
        match self {
            numbas::support::answer_style::AnswerStyle::English => AnswerStyle::English,
            numbas::support::answer_style::AnswerStyle::EnglishPlain => AnswerStyle::EnglishPlain,
            numbas::support::answer_style::AnswerStyle::EnglishSI => AnswerStyle::EnglishSI,
            numbas::support::answer_style::AnswerStyle::European => AnswerStyle::European,
            numbas::support::answer_style::AnswerStyle::EuropeanPlain => AnswerStyle::EuropeanPlain,
            numbas::support::answer_style::AnswerStyle::FrenchSI => AnswerStyle::FrenchSI,
            numbas::support::answer_style::AnswerStyle::Indian => AnswerStyle::Indian,
            numbas::support::answer_style::AnswerStyle::Scientific => AnswerStyle::Scientific,
            numbas::support::answer_style::AnswerStyle::Swiss => AnswerStyle::Swiss,
        }
    }
}

pub type AnswerStylesInput = Vec<Value<AnswerStyleInput>>;
pub type AnswerStyles = Vec<AnswerStyle>;
