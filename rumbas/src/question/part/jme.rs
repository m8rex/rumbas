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
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartJMEInput")]
    #[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartJME {
        answer: EmbracedJMETranslatableString, //TODO: should this be translatable?
        answer_simplification: JMEAnswerSimplification,
        answer_display: JMEAnswerDisplay,
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
                answer_display: self.answer_simplification.to_rumbas(),
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
    ($struct: ident: $input: literal: $variant: ident: $variant_struct: ident,
        $(
            $(#[$inner:meta])*
            $name: ident: $numbas_name: ident: $default: ident: $partofall: expr
        ),*) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples)]
        #[input(name = $input)]
        #[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
        pub struct $struct {
            $(
                $(#[$inner])*
                pub $name: bool
            ),*
        }
        impl ToNumbas<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
            for $struct
        {
            fn to_numbas(
                &self,
                _locale: &str,
            ) -> Vec<numbas::question::answer_simplification::AnswerSimplificationType> {
                let mut v = Vec::new();
                $(if self.$name {
                    v.push(
                        numbas::question::answer_simplification::AnswerSimplificationType::$variant(
                            numbas::question::answer_simplification::$variant_struct::$numbas_name(true)
                        )
                    );
                })*
                v
            }
        }

        impl ToRumbas<$struct>
            for Option<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
        {
            fn to_rumbas(&self) -> $struct {
                let mut result = $struct {
                    $(
                        $name: DEFAULTS.$default
                    ),*
                }; // Numbas default
                if let Some(v) = self {
                    for a in v.iter() {
                        match a {
                            numbas::question::answer_simplification::AnswerSimplificationType::$variant(r) =>
                                match r {
                                    numbas::question::answer_simplification::$variant_struct::All(b) => {
                                        $(if $partofall {
                                            result.$name = *b;
                                        })*
                                    }
                                    $(numbas::question::answer_simplification::$variant_struct::$numbas_name(b) => {
                                        result.$name = *b;
                                    })*
                                }
                            _ => ()
                        }
                    }
                }

                result
            }
        }
    }
}

// See https://numbas-editor.readthedocs.io/en/latest/simplification.html#term-expandbrackets
create_answer_simplification! {
    JMEAnswerSimplification: "JMEAnswerSimplificationInput": Rule: AnswerSimplificationRule,
    simplify_basic: Basic: jme_simplification_simplify_basic: true,
    #[serde(alias = "simplify_unit_factor")]
    cancel_unit_factors: CancelUnitFactors: jme_simplification_simplify_unit_factor: true,
    #[serde(alias = "simplify_unit_power")]
    cancel_unit_powers: CancelUnitPowers: jme_simplification_simplify_unit_power: true,
    #[serde(alias = "simplify_unit_denominator")]
    cancel_unit_denominators: CancelUnitDenominators: jme_simplification_simplify_unit_denominator: true,
    #[serde(alias = "simplify_zero_factor")]
    cancel_zero_factors: CancelZeroFactors: jme_simplification_simplify_zero_factor: true,
    #[serde(alias = "simplify_zero_term")]
    omit_zero_terms: OmitZeroTerms: jme_simplification_simplify_zero_term: true,
    #[serde(alias = "simplify_zero_power")]
    cancel_zero_powers: CancelZeroPowers: jme_simplification_simplify_zero_power: true,
    #[serde(alias = "simplify_zero_base")]
    cancel_powers_with_base_zero: CancelPowersWithBaseZero: jme_simplification_simplify_zero_base: true,
    collect_numbers: CollectNumbers: jme_simplification_collect_numbers: true,
    constants_first: ConstantsFirst: jme_simplification_constants_first: true,
    #[serde(alias = "simplify_sqrt_products")]
    collect_sqrt_products: CollectSqrtProducts: jme_simplification_simplify_sqrt_products: true,
    #[serde(alias = "simplify_sqrt_division")]
    collect_sqrt_divisions: CollectSqrtDivisions: jme_simplification_simplify_sqrt_division: true,
    #[serde(alias = "simplify_sqrt_square")]
    cancel_sqrt_square: CancelSqrtSquares: jme_simplification_simplify_sqrt_square: true,
    #[serde(alias = "simplify_other_numbers")]
    evaluate_powers_of_numbers: EvaluatePowersOfNumbers: jme_simplification_simplify_other_numbers: true,
    #[serde(alias = "simplify_no_leading_minus")]
    rewrite_to_no_leading_minus: NoLeadingMinus: jme_simplification_simplify_no_leading_minus: true,
    simplify_fractions: Fractions: jme_simplification_simplify_fractions: true,
    simplify_trigonometric: Trigonometric: jme_simplification_simplify_trigonometric: true,
    #[serde(alias = "cancel_terms")]
    collect_terms: CollectTerms: jme_simplification_cancel_terms: true,
    #[serde(alias = "cancel_factors")]
    collect_powers_of_common_factors: CollectPowersOfCommonFactors: jme_simplification_cancel_factors: true,
    collect_like_fractions: CollectLikeFractions: jme_simplification_collect_like_fractions: true,
    order_canonical: CanonicalOrder: jme_simplification_order_canonical: false,
    expand_brackets: ExpandBrackets: jme_simplification_expand_brackets: false
}

macro_rules! create_answer_display_type {
    ($struct: ident: $input: literal: $variant: ident: $variant_struct: ident,
        $(
            $(#[$inner:meta])*
            $name: ident: $numbas_name: ident: $default: ident
        ),*) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples)]
        #[input(name = $input)]
        #[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
        pub struct $struct {
            $(
                $(#[$inner])*
                pub $name: bool
            ),*
        }
        impl ToNumbas<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
            for $struct
        {
            fn to_numbas(
                &self,
                _locale: &str,
            ) -> Vec<numbas::question::answer_simplification::AnswerSimplificationType> {
                let mut v = Vec::new();
                $(if self.$name {
                    v.push(
                        numbas::question::answer_simplification::AnswerSimplificationType::$variant(
                            numbas::question::answer_simplification::$variant_struct::$numbas_name(true)
                        )
                    );
                })*
                v
            }
        }

        impl ToRumbas<$struct>
            for Option<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
        {
            fn to_rumbas(&self) -> $struct {
                let mut result = $struct {
                    $(
                        $name: DEFAULTS.$default
                    ),*
                }; // Numbas default
                if let Some(v) = self {
                    for a in v.iter() {
                        match a {
                            numbas::question::answer_simplification::AnswerSimplificationType::$variant(r) =>
                                match r {
                                    $(numbas::question::answer_simplification::$variant_struct::$numbas_name(b) => {
                                        result.$name = *b;
                                    })*
                                }
                            _ => ()
                        }
                    }
                }

                result
            }
        }
    }
}

create_answer_display_type! {
    JMEAnswerDisplay: "JMEAnswerDisplayInput": DisplayOption: AnswerSimplificationDisplayOption,
    broken_as_fractions: Fractions: jme_display_fraction_numbers,
    mixed_fractions: MixedFractions: jme_display_mixed_fractions,
    flat_fractions: FlatFractions: jme_display_flat_fractions,
    vector_as_row: RowVector: jme_display_row_vector,
    always_show_multiplication_sign: AlwaysShowMultiplicationSign : jme_display_always_times,
    use_dot_as_multiplication_sign: DotAsMultiplicationSign: jme_display_use_times_dot, // Use \cdot instead of \times
    matrices_without_parentheses: MatricesWithoutParentheses: jme_display_bare_matrices
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CheckingTypeDataFloatInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CheckingTypeDataNaturalInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "CheckingTypeInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "JMERestrictionInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "JMELengthRestrictionInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "JMEStringRestrictionInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "JMEPatternRestrictionInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "JMEValueGeneratorInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
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
