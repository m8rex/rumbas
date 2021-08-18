use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::question_part::JMENotes;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::ToNumbas;
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
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
            part_data: self.to_numbas_shared_data(locale),
            correct_answer_fraction: self.display_correct_as_fraction.clone().unwrap(),
            correct_answer_style: Some(
                self.display_correct_in_style
                    .clone()
                    .map(|a| a.to_numbas(locale))
                    .unwrap(),
            ),
            allow_fractions: self.allow_fractions.unwrap(),
            notation_styles: Some(self.allowed_notation_styles.to_numbas(locale)),
            fractions_must_be_reduced: Some(self.fractions_must_be_reduced.clone().unwrap()),
            partial_credit_if_fraction_not_reduced: Some(
                self.partial_credit_if_fraction_not_reduced.clone().unwrap(),
            ),
            precision: None,           //TODO
            show_precision_hint: None, //TODO
            show_fraction_hint: Some(self.hint_fraction.clone().unwrap()),
            answer: self.answer.to_numbas(locale),
            // checking_type: Some(numbas::exam::CheckingType::Range), //TODO
        }
    }
}

impl ToRumbas<QuestionPartNumberEntry> for numbas::exam::ExamQuestionPartNumberEntry {
    fn to_rumbas(&self) -> QuestionPartNumberEntry {
        QuestionPartNumberEntry {
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(extract_part_common_prompt(&self.part_data)),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&self.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&self.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&self.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &self.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&self.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &self.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &self.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                self.part_data.variable_replacement_strategy.to_rumbas(),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &self.part_data,
            )),
            custom_marking_algorithm_notes: Value::Normal(
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
            ),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),

            answer: Value::Normal(self.answer.to_rumbas()),
            display_correct_as_fraction: Value::Normal(self.correct_answer_fraction),
            allow_fractions: Value::Normal(self.allow_fractions),
            allowed_notation_styles: Value::Normal(
                self.notation_styles.clone().unwrap_or_default().to_rumbas(),
            ),
            display_correct_in_style: Value::Normal(
                self.correct_answer_style
                    .clone()
                    .unwrap_or(DEFAULTS.number_entry_correct_answer_style)
                    .to_rumbas(),
            ),

            fractions_must_be_reduced: Value::Normal(
                self.fractions_must_be_reduced
                    .unwrap_or(DEFAULTS.number_entry_fractions_must_be_reduced),
            ),
            partial_credit_if_fraction_not_reduced: Value::Normal(
                self.partial_credit_if_fraction_not_reduced
                    .clone()
                    .unwrap_or(DEFAULTS.number_entry_partial_credit_if_fraction_not_reduced),
            ),
            hint_fraction: Value::Normal(
                self.show_fraction_hint
                    .unwrap_or(DEFAULTS.number_entry_hint_fraction),
            ),
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
                from: FileString::s(&min_value.to_string()),
                to: FileString::s(&max_value.to_string()),
            },
            numbas::exam::NumberEntryAnswerType::Answer { answer } => {
                NumberEntryAnswer::Normal(FileString::s(&answer.to_string()))
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
