use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionPart;
use crate::support::file_reference::{FileString, JMEFileString};
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck)]
    #[input(name = "QuestionPartJMEInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct QuestionPartJME {
        answer: EmbracedJMETranslatableString, //TODO: should this be translatable?
        answer_simplification: JMEAnswerSimplification,
        show_preview: bool,
        answer_check: CheckingType,
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
        must_match_pattern: Noneable<JMEPatternRestriction>,
        value_generators: Noneable<Vec<JMEValueGenerator>>
    }
}

impl ToNumbas<numbas::question::part::jme::QuestionPartJME> for QuestionPartJME {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::QuestionPartJME {
        numbas::question::part::jme::QuestionPartJME {
            part_data: self.to_numbas(locale),
            answer: self.answer.to_numbas(locale),
            answer_simplification: Some(self.answer_simplification.to_numbas(locale)),
            show_preview: self.show_preview.to_numbas(locale),
            checking_type: self.answer_check.to_numbas(locale),
            failure_rate: Some(self.failure_rate.to_numbas(locale)),
            vset_range: self.vset_range.to_numbas(locale),
            vset_range_points: self.vset_range_points.to_numbas(locale),
            check_variable_names: self.check_variable_names.to_numbas(locale),
            single_letter_variables: Some(self.single_letter_variables.to_numbas(locale)),
            allow_unknown_functions: Some(self.allow_unknown_functions.to_numbas(locale)),
            implicit_function_composition: Some(
                self.implicit_function_composition.to_numbas(locale),
            ),
            max_length: self.max_length.to_numbas(locale),
            min_length: self.min_length.to_numbas(locale),

            must_have: self.must_have.to_numbas(locale),
            may_not_have: self.may_not_have.to_numbas(locale),
            must_match_pattern: self.must_match_pattern.to_numbas(locale),
            value_generators: self.value_generators.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartJME> for numbas::question::part::jme::QuestionPartJME {
    fn to_rumbas(&self) -> QuestionPartJME {
        create_question_part! {
            QuestionPartJME with &self.part_data => {
                answer: self.answer.to_rumbas(),
                answer_simplification: self.answer_simplification.to_rumbas(),
                show_preview: self.show_preview.to_rumbas(),
                answer_check: self.checking_type.to_rumbas(),
                failure_rate: self.failure_rate.unwrap_or(DEFAULTS.jme_failure_rate).to_rumbas(),
                vset_range: [self.vset_range[0].0, self.vset_range[1].0].to_rumbas(),
                vset_range_points: self.vset_range_points.0.to_rumbas(),
                check_variable_names: self.check_variable_names.to_rumbas(),
                single_letter_variables:
                    self.single_letter_variables
                        .unwrap_or(DEFAULTS.jme_single_letter_variables).to_rumbas(),
                allow_unknown_functions:
                    self.allow_unknown_functions
                        .unwrap_or(DEFAULTS.jme_allow_unknown_functions).to_rumbas(),
                implicit_function_composition:
                    self.implicit_function_composition
                        .unwrap_or(DEFAULTS.jme_implicit_function_composition).to_rumbas(),
                max_length:
                    self.max_length.to_rumbas(),
                min_length:
                    self.min_length.to_rumbas(),
                must_have:
                    self.must_have
                        .to_rumbas(),
                may_not_have:
                    self.may_not_have
                        .to_rumbas(),
                must_match_pattern:
                    self.must_match_pattern
                        .to_rumbas(),
                value_generators:
                    self.value_generators
                        .to_rumbas()
            }
        }
    }
}

macro_rules! create_answer_simplification {
    ($($name: ident: $numbas_name: ident),*) => {
        #[derive(Input, Overwrite, RumbasCheck)]
        #[input(name = "JMEAnswerSimplificationInput")]
        #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
        pub struct JMEAnswerSimplification {
            $(pub $name: bool),*
        }
        impl ToNumbas<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
            for JMEAnswerSimplification
        {
            fn to_numbas(
                &self,
                _locale: &str,
            ) -> Vec<numbas::question::answer_simplification::AnswerSimplificationType> {
                let mut v = Vec::new();
                $(if self.$name {
                    v.push(numbas::question::answer_simplification::AnswerSimplificationType::$numbas_name(true));
                })*
                v
            }
        }

        impl ToRumbas<JMEAnswerSimplification>
            for Option<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
        {
            fn to_rumbas(&self) -> JMEAnswerSimplification {
                let mut result = JMEAnswerSimplification {
                    simplify_basic: DEFAULTS.jme_simplification_simplify_basic,
                    simplify_unit_factor: DEFAULTS.jme_simplification_simplify_unit_factor,
                    simplify_unit_power: DEFAULTS.jme_simplification_simplify_unit_power,
                    simplify_unit_denominator: DEFAULTS.jme_simplification_simplify_unit_denominator,
                    simplify_zero_factor: DEFAULTS.jme_simplification_simplify_zero_factor,
                    simplify_zero_term: DEFAULTS.jme_simplification_simplify_zero_term,
                    simplify_zero_power: DEFAULTS.jme_simplification_simplify_zero_power,
                    simplify_zero_base: DEFAULTS.jme_simplification_simplify_zero_base,
                    collect_numbers: DEFAULTS.jme_simplification_collect_numbers,
                    constants_first: DEFAULTS.jme_simplification_constants_first,
                    simplify_sqrt_products: DEFAULTS.jme_simplification_simplify_sqrt_products,
                    simplify_sqrt_division: DEFAULTS.jme_simplification_simplify_sqrt_division,
                    simplify_sqrt_square: DEFAULTS.jme_simplification_simplify_sqrt_square,
                    simplify_other_numbers: DEFAULTS.jme_simplification_simplify_other_numbers,
                    simplify_no_leading_minus: DEFAULTS.jme_simplification_simplify_no_leading_minus,
                    simplify_fractions: DEFAULTS.jme_simplification_simplify_fractions,
                    simplify_trigonometric: DEFAULTS.jme_simplification_simplify_trigonometric,
                    cancel_terms: DEFAULTS.jme_simplification_cancel_terms,
                    cancel_factors: DEFAULTS.jme_simplification_cancel_factors,
                    collect_like_fractions: DEFAULTS.jme_simplification_collect_like_fractions,
                    order_canonical: DEFAULTS.jme_simplification_order_canonical,
                    use_times_dot: DEFAULTS.jme_simplification_use_times_dot,
                    expand_brackets: DEFAULTS.jme_simplification_expand_brackets,
                }; // Numbas default
                if let Some(v) = self {
                    for a in v.iter() {
                        match a {
                            numbas::question::answer_simplification::AnswerSimplificationType::All(b) => {
                                $(result.$name = *b);*
                            }
                            $(numbas::question::answer_simplification::AnswerSimplificationType::$numbas_name(b) => {
                                result.$name = *b;
                            })*
                            numbas::question::answer_simplification::AnswerSimplificationType::Unknown((name, val)) => {
                                // TODO: remove, add display options
                                log::info!(
                                    "Found unknown answer simplification type {}{}",
                                    if *val { "!" } else { "" },
                                    name
                                )
                            }
                        }
                    }
                }

                result
            }
        }
    }
}

// See https://numbas-editor.readthedocs.io/en/latest/simplification.html#term-expandbrackets
//TODO: rename etc
create_answer_simplification! {
    simplify_basic: Basic,
    simplify_unit_factor: UnitFactor,
    simplify_unit_power: UnitPower,
    simplify_unit_denominator: UnitDenominator,
    simplify_zero_factor: ZeroFactor,
    simplify_zero_term: ZeroTerm,
    simplify_zero_power: ZeroPower,
    simplify_zero_base: ZeroBase,
    collect_numbers: CollectNumbers,
    constants_first: ConstantsFirst,
    simplify_sqrt_products: SqrtProduct,
    simplify_sqrt_division: SqrtDivision,
    simplify_sqrt_square: SqrtSquare,
    simplify_other_numbers: OtherNumbers,
    simplify_no_leading_minus: NoLeadingMinus,
    simplify_fractions: Fractions,
    simplify_trigonometric: Trigonometric,
    cancel_terms: CancelTerms,
    cancel_factors: CancelFactors,
    collect_like_fractions: CollectLikeFractions,
    order_canonical: CanonicalOrder,
    use_times_dot: TimesDot, // Use \cdot instead of \times
    expand_brackets: ExpandBrackets
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "CheckingTypeDataFloatInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckingTypeDataFloat {
    pub max_difference: f64,
}

impl
    ToNumbas<
        numbas::question::part::jme::JMECheckingTypeData<numbas::support::primitive::SafeFloat>,
    > for CheckingTypeDataFloat
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::jme::JMECheckingTypeData<numbas::support::primitive::SafeFloat>
    {
        numbas::question::part::jme::JMECheckingTypeData {
            checking_accuracy: self.max_difference.into(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "CheckingTypeDataNaturalInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckingTypeDataNatural {
    pub amount: usize,
}

impl ToNumbas<numbas::question::part::jme::JMECheckingTypeData<usize>> for CheckingTypeDataNatural {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::jme::JMECheckingTypeData<usize> {
        numbas::question::part::jme::JMECheckingTypeData {
            checking_accuracy: self.amount,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "CheckingTypeInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum CheckingType {
    RelativeDifference(CheckingTypeDataFloat),
    AbsoluteDifference(CheckingTypeDataFloat),
    DecimalPlaces(CheckingTypeDataNatural),
    SignificantFigures(CheckingTypeDataNatural),
}

impl ToNumbas<numbas::question::part::jme::JMECheckingType> for CheckingType {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMECheckingType {
        match self {
            CheckingType::RelativeDifference(f) => {
                numbas::question::part::jme::JMECheckingType::RelativeDifference(
                    f.to_numbas(locale),
                )
            }
            CheckingType::AbsoluteDifference(f) => {
                numbas::question::part::jme::JMECheckingType::AbsoluteDifference(
                    f.to_numbas(locale),
                )
            }
            CheckingType::DecimalPlaces(f) => {
                numbas::question::part::jme::JMECheckingType::DecimalPlaces(f.to_numbas(locale))
            }
            CheckingType::SignificantFigures(f) => {
                numbas::question::part::jme::JMECheckingType::SignificantFigures(
                    f.to_numbas(locale),
                )
            }
        }
    }
}

impl ToRumbas<CheckingType> for numbas::question::part::jme::JMECheckingType {
    fn to_rumbas(&self) -> CheckingType {
        match self {
            numbas::question::part::jme::JMECheckingType::RelativeDifference(v) => {
                CheckingType::RelativeDifference(CheckingTypeDataFloat {
                    max_difference: v.checking_accuracy.0.to_rumbas(),
                })
            }
            numbas::question::part::jme::JMECheckingType::AbsoluteDifference(v) => {
                CheckingType::AbsoluteDifference(CheckingTypeDataFloat {
                    max_difference: v.checking_accuracy.0.to_rumbas(),
                })
            }
            numbas::question::part::jme::JMECheckingType::DecimalPlaces(v) => {
                CheckingType::DecimalPlaces(CheckingTypeDataNatural {
                    amount: v.checking_accuracy.to_rumbas(),
                })
            }
            numbas::question::part::jme::JMECheckingType::SignificantFigures(v) => {
                CheckingType::SignificantFigures(CheckingTypeDataNatural {
                    amount: v.checking_accuracy.to_rumbas(),
                })
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "JMERestrictionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct JMERestriction {
    // name: TranslatableString,
    pub partial_credit: f64, //TODO, is number, so maybe usize?
    pub message: TranslatableString,
}

impl ToNumbas<numbas::question::part::jme::JMERestriction> for JMERestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMERestriction {
        numbas::question::part::jme::JMERestriction {
            // name: self.name.clone().to_string(locale),
            partial_credit: self.partial_credit.clone().to_numbas(locale),
            message: self.message.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMERestriction> for numbas::question::part::jme::JMERestriction {
    fn to_rumbas(&self) -> JMERestriction {
        JMERestriction {
            //name: TranslatableString::s(&self.name)),
            partial_credit: self.partial_credit.0.to_rumbas(),
            message: self.message.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "JMELengthRestrictionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct JMELengthRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    pub length: usize,
}

impl ToNumbas<numbas::question::part::jme::JMELengthRestriction> for JMELengthRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMELengthRestriction {
        numbas::question::part::jme::JMELengthRestriction {
            restriction: self.restriction.to_numbas(locale),
            length: Some(self.length.to_numbas(locale)),
        }
    }
}

impl ToRumbas<JMELengthRestriction> for numbas::question::part::jme::JMELengthRestriction {
    fn to_rumbas(&self) -> JMELengthRestriction {
        JMELengthRestriction {
            restriction: self.restriction.to_rumbas(),
            length: self
                .length
                .map(|v| v.0)
                .unwrap_or(DEFAULTS.length_restriction_length)
                .to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "JMEStringRestrictionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct JMEStringRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    pub show_strings: bool,
    pub strings: Vec<TranslatableString>,
}

impl ToNumbas<numbas::question::part::jme::JMEStringRestriction> for JMEStringRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMEStringRestriction {
        numbas::question::part::jme::JMEStringRestriction {
            restriction: self.restriction.to_numbas(locale),
            show_strings: self.show_strings.to_numbas(locale),
            strings: self.strings.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMEStringRestriction> for numbas::question::part::jme::JMEStringRestriction {
    fn to_rumbas(&self) -> JMEStringRestriction {
        JMEStringRestriction {
            restriction: self.restriction.to_rumbas(),
            show_strings: self.show_strings.to_rumbas(),
            strings: self.strings.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "JMEPatternRestrictionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct JMEPatternRestriction {
    pub partial_credit: f64, //TODO, is number, so maybe usize?
    pub message: TranslatableString,
    pub pattern: String,         //TODO type? If string -> InputString?
    pub name_to_compare: String, //TODO, translateable?
}

impl ToNumbas<numbas::question::part::jme::JMEPatternRestriction> for JMEPatternRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMEPatternRestriction {
        numbas::question::part::jme::JMEPatternRestriction {
            partial_credit: self.partial_credit.to_numbas(locale),
            message: self.message.to_numbas(locale),
            pattern: self.pattern.to_numbas(locale),
            name_to_compare: self.name_to_compare.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMEPatternRestriction> for numbas::question::part::jme::JMEPatternRestriction {
    fn to_rumbas(&self) -> JMEPatternRestriction {
        JMEPatternRestriction {
            partial_credit: self.partial_credit.0.to_rumbas(),
            message: self.message.to_rumbas(),
            pattern: self.pattern.to_rumbas(),
            name_to_compare: self.name_to_compare.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "JMEValueGeneratorInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct JMEValueGenerator {
    pub name: FileString,
    pub value: JMEFileString,
}

impl ToNumbas<numbas::question::part::jme::JMEValueGenerator> for JMEValueGenerator {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMEValueGenerator {
        numbas::question::part::jme::JMEValueGenerator {
            name: self.name.to_numbas(locale),
            value: self.value.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMEValueGenerator> for numbas::question::part::jme::JMEValueGenerator {
    fn to_rumbas(&self) -> JMEValueGenerator {
        let s: String = self.value.clone().into();
        JMEValueGenerator {
            name: self.name.to_rumbas(),
            value: s.to_rumbas(),
        }
    }
}
