use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

question_part_type! {
    QuestionPartJME,
    answer: TranslatableString, //TODO: should this be translatable?
    answer_simplification: JMEAnswerSimplification,
    show_preview: bool,
    checking_type: CheckingType,
    checking_accuracy: f64,
    failure_rate: f64,
    vset_range: [f64; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    vset_range_points: usize,
    check_variable_names: bool,
    single_letter_variables: bool,
    allow_unknown_functions: bool,
    implicit_function_composition: bool,
    max_length: Noneable<JMELengthRestriction>,
    min_length: Noneable<JMELengthRestriction>,
    must_have: Noneable<JMEStringRestriction>,
    may_not_have: Noneable<JMEStringRestriction>,
    must_match_pattern: Noneable<JMEPatternRestriction>
}

impl ToNumbas for QuestionPartJME {
    type NumbasType = numbas::exam::ExamQuestionPartJME;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamQuestionPartJME> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionPartJME::new(
                self.to_numbas_shared_data(&locale),
                self.answer.clone().unwrap().to_string(&locale).unwrap(),
                Some(
                    self.answer_simplification
                        .clone()
                        .unwrap()
                        .to_numbas(&locale)
                        .unwrap(),
                ),
                self.show_preview.clone().unwrap(),
                self.checking_type
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.checking_accuracy.unwrap(),
                self.failure_rate.unwrap(),
                self.vset_range.unwrap(),
                self.vset_range_points.unwrap(),
                self.check_variable_names.unwrap(),
                self.single_letter_variables,
                self.allow_unknown_functions,
                self.implicit_function_composition,
                self.max_length
                    .clone()
                    .map(|v| v.to_numbas(&locale).unwrap())
                    .flatten(),
                self.min_length
                    .clone()
                    .map(|v| v.to_numbas(&locale).unwrap())
                    .flatten(),
                self.must_have
                    .clone()
                    .map(|v| v.to_numbas(&locale).unwrap())
                    .flatten(),
                self.may_not_have
                    .clone()
                    .map(|v| v.to_numbas(&locale).unwrap())
                    .flatten(),
                self.must_match_pattern
                    .clone()
                    .map(|v| v.to_numbas(&locale).unwrap())
                    .flatten(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

//TODO: rename etc
optional_overwrite! {
    JMEAnswerSimplification,
    simplify_basic: bool,
    simplify_unit_factor: bool,
    simplify_unit_power: bool,
    simplify_unit_denominator: bool,
    simplify_zero_factor: bool,
    simplify_zero_term: bool,
    simplify_zero_power: bool,
    simplify_zero_base: bool,
    collect_numbers: bool,
    constants_first: bool,
    simplify_sqrt_products: bool,
    simplify_sqrt_division: bool,
    simplify_sqrt_square: bool,
    simplify_other_numbers: bool
}

impl ToNumbas for JMEAnswerSimplification {
    type NumbasType = Vec<numbas::exam::AnswerSimplificationType>;
    fn to_numbas(
        &self,
        _locale: &String,
    ) -> NumbasResult<Vec<numbas::exam::AnswerSimplificationType>> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let mut v = Vec::new();
            if self.simplify_basic.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::Basic);
            }
            if self.simplify_unit_factor.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::UnitFactor);
            }
            if self.simplify_unit_power.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::UnitPower);
            }
            if self.simplify_unit_denominator.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::UnitDenominator);
            }
            if self.simplify_zero_factor.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroFactor);
            }
            if self.simplify_zero_term.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroTerm);
            }
            if self.simplify_zero_power.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroPower);
            }
            if self.simplify_zero_base.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ZeroBase);
            }
            if self.collect_numbers.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::CollectNumbers);
            }
            if self.constants_first.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::ConstantsFirst);
            }
            if self.simplify_sqrt_products.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::SqrtProduct);
            }
            if self.simplify_sqrt_division.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::SqrtDivision);
            }
            if self.simplify_sqrt_square.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::SqrtSquare);
            }
            if self.simplify_other_numbers.unwrap() {
                v.push(numbas::exam::AnswerSimplificationType::OtherNumbers);
            }
            Ok(v)
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckingType {
    RelativeDifference,
    AbsoluteDifference,
    DecimalPlaces,
    SignificantFigures,
}
impl_optional_overwrite!(CheckingType);

impl ToNumbas for CheckingType {
    type NumbasType = numbas::exam::JMECheckingType;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            CheckingType::RelativeDifference => numbas::exam::JMECheckingType::RelativeDifference,
            CheckingType::AbsoluteDifference => numbas::exam::JMECheckingType::AbsoluteDifference,
            CheckingType::DecimalPlaces => numbas::exam::JMECheckingType::DecimalPlaces,
            CheckingType::SignificantFigures => numbas::exam::JMECheckingType::SignificantFigures,
        })
    }
}

optional_overwrite! {
    JMERestriction,
    name: TranslatableString,
    strings: Vec<TranslatableString>,
    partial_credit: f64, //TODO, is number, so maybe usize?
    message: TranslatableString
}

impl ToNumbas for JMERestriction {
    type NumbasType = numbas::exam::JMERestriction;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::JMERestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMERestriction::new(
                self.name.clone().unwrap().to_string(&locale).unwrap(),
                self.strings
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|s| s.to_string(&locale).unwrap())
                    .collect(),
                self.partial_credit.clone().unwrap(),
                self.message.clone().unwrap().to_string(&locale).unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    JMELengthRestriction,
    restriction: JMERestriction: serde(flatten),
    length: usize
}

impl ToNumbas for JMELengthRestriction {
    type NumbasType = numbas::exam::JMELengthRestriction;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::JMELengthRestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMELengthRestriction::new(
                self.restriction
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.length,
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    JMEStringRestriction,
    restriction: JMERestriction: serde(flatten),
    show_strings: bool
}

impl ToNumbas for JMEStringRestriction {
    type NumbasType = numbas::exam::JMEStringRestriction;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::JMEStringRestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMEStringRestriction::new(
                self.restriction
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.show_strings.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    JMEPatternRestriction,
    restriction: JMERestriction: serde(flatten),
    pattern: String, //TODO type?
    name_to_compare: String //TODO, translateable?
}

impl ToNumbas for JMEPatternRestriction {
    type NumbasType = numbas::exam::JMEPatternRestriction;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::JMEPatternRestriction> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::JMEPatternRestriction::new(
                self.restriction
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.pattern.clone().unwrap(),
                self.name_to_compare.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
