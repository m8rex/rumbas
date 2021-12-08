use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionPart;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use numbas::defaults::DEFAULTS;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartNumberEntryInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct QuestionPartNumberEntry {
        answer: NumberEntryAnswer,
        display_correct_as_fraction: bool,
        allow_fractions: bool,
        allowed_notation_styles: Vec<AnswerStyle>,

        display_correct_in_style: AnswerStyle,
        fractions_must_be_reduced: bool,
        partial_credit_if_fraction_not_reduced: numbas::support::primitive::Number,

        hint_fraction: bool

        //TODO: precision, show_precision_hint
    }

}

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
                    .to_numbas(locale)
                    .into(),
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NumberEntryAnswerInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum NumberEntryAnswer {
    Normal(JMEString),
    Range(NumberEntryAnswerRange),
}

// TODO, better (add toRumbas etc)
#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NumberEntryAnswerRangeInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct NumberEntryAnswerRange {
    pub from: JMEString,
    pub to: JMEString,
}

impl ToNumbas<numbas::question::part::number_entry::NumberEntryAnswerType> for NumberEntryAnswer {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::number_entry::NumberEntryAnswerType {
        match self {
            NumberEntryAnswer::Normal(f) => {
                numbas::question::part::number_entry::NumberEntryAnswerType::Answer {
                    answer: f.to_numbas(locale),
                }
            }
            NumberEntryAnswer::Range(range) => {
                numbas::question::part::number_entry::NumberEntryAnswerType::MinMax {
                    min_value: range.from.to_numbas(locale),
                    max_value: range.to.to_numbas(locale),
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
                from: min_value.to_rumbas(),
                to: max_value.to_rumbas(),
            }),
            numbas::question::part::number_entry::NumberEntryAnswerType::Answer { answer } => {
                NumberEntryAnswer::Normal(answer.to_rumbas())
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "AnswerStyleInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
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
