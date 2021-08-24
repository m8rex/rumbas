use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::question::part::question_part::{QuestionPartInput, VariableReplacementStrategyInput};
use crate::question::QuestionParts;
use crate::question::QuestionPartsInput;
use crate::support::file_reference::{FileString, JMEFileString};
use crate::support::file_reference::{FileStringInput, JMEFileStringInput};
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::rumbas_types::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::EmbracedJMETranslatableStringInput;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
use crate::support::translatable::TranslatableStrings;
use crate::support::translatable::TranslatableStringsInput;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    pub struct QuestionPartJME {
        answer: EmbracedJMETranslatableString, //TODO: should this be translatable?
        answer_simplification: JMEAnswerSimplification,
        show_preview: RumbasBool,
        answer_check: CheckingType,
        failure_rate:RumbasFloat,
        vset_range: RumbasFloats2, // TODO: seperate (flattened) struct for vset items & checking items etc?
        vset_range_points: RumbasNatural,
        check_variable_names: RumbasBool,
        single_letter_variables: RumbasBool,
        allow_unknown_functions: RumbasBool,
        implicit_function_composition: RumbasBool,
        max_length: NoneableJMELengthRestriction,
        min_length: NoneableJMELengthRestriction,
        must_have: NoneableJMEStringRestriction,
        may_not_have: NoneableJMEStringRestriction,
        must_match_pattern: NoneableJMEPatternRestriction,
        value_generators: NoneableJMEValueGenerators
    }
}

pub type NoneableJMELengthRestriction = Noneable<JMELengthRestriction>;
pub type NoneableJMELengthRestrictionInput = Noneable<JMELengthRestrictionInput>;
pub type NoneableJMEStringRestriction = Noneable<JMEStringRestriction>;
pub type NoneableJMEStringRestrictionInput = Noneable<JMEStringRestrictionInput>;
pub type NoneableJMEPatternRestriction = Noneable<JMEPatternRestriction>;
pub type NoneableJMEPatternRestrictionInput = Noneable<JMEPatternRestrictionInput>;

pub type NoneableJMEValueGeneratorsInput = Noneable<JMEValueGeneratorsInput>;
pub type NoneableJMEValueGenerators = Noneable<JMEValueGenerators>;

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

// See https://numbas-editor.readthedocs.io/en/latest/simplification.html#term-expandbrackets
//TODO: rename etc
optional_overwrite! {
    pub struct JMEAnswerSimplification {
        simplify_basic: RumbasBool,
        simplify_unit_factor: RumbasBool,
        simplify_unit_power: RumbasBool,
        simplify_unit_denominator: RumbasBool,
        simplify_zero_factor: RumbasBool,
        simplify_zero_term: RumbasBool,
        simplify_zero_power: RumbasBool,
        simplify_zero_base: RumbasBool,
        collect_numbers: RumbasBool,
        constants_first: RumbasBool,
        simplify_sqrt_products: RumbasBool,
        simplify_sqrt_division: RumbasBool,
        simplify_sqrt_square: RumbasBool,
        simplify_other_numbers: RumbasBool,
        simplify_no_leading_minus: RumbasBool,
        simplify_fractions: RumbasBool,
        simplify_trigonometric: RumbasBool,
        cancel_terms: RumbasBool,
        cancel_factors: RumbasBool,
        collect_like_fractions: RumbasBool,
        order_canonical: RumbasBool,
        use_times_dot: RumbasBool, // Use \cdot instead of \times
        expand_brackets: RumbasBool
    }
}

impl ToNumbas<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
    for JMEAnswerSimplification
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> Vec<numbas::question::answer_simplification::AnswerSimplificationType> {
        let mut v = Vec::new();
        if self.simplify_basic.unwrap() {
            v.push(numbas::question::answer_simplification::AnswerSimplificationType::Basic(true));
        }
        if self.simplify_unit_factor.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::UnitFactor(true),
            );
        }
        if self.simplify_unit_power.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::UnitPower(true),
            );
        }
        if self.simplify_unit_denominator.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::UnitDenominator(
                    true,
                ),
            );
        }
        if self.simplify_zero_factor.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::ZeroFactor(true),
            );
        }
        if self.simplify_zero_term.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::ZeroTerm(true),
            );
        }
        if self.simplify_zero_power.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::ZeroPower(true),
            );
        }
        if self.simplify_zero_base.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::ZeroBase(true),
            );
        }
        if self.collect_numbers.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::CollectNumbers(
                    true,
                ),
            );
        }
        if self.constants_first.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::ConstantsFirst(
                    true,
                ),
            );
        }
        if self.simplify_sqrt_products.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::SqrtProduct(
                    true,
                ),
            );
        }
        if self.simplify_sqrt_division.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::SqrtDivision(
                    true,
                ),
            );
        }
        if self.simplify_sqrt_square.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::SqrtSquare(true),
            );
        }
        if self.simplify_other_numbers.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::OtherNumbers(
                    true,
                ),
            );
        }
        if self.simplify_no_leading_minus.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::NoLeadingMinus(
                    true,
                ),
            );
        }
        if self.simplify_fractions.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::Fractions(true),
            );
        }
        if self.simplify_trigonometric.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::Trigonometric(
                    true,
                ),
            );
        }
        if self.cancel_terms.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::CancelTerms(
                    true,
                ),
            );
        }
        if self.cancel_factors.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::CancelFactors(
                    true,
                ),
            );
        }
        if self.collect_like_fractions.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::CollectLikeFractions(
                    true,
                ),
            );
        }
        if self.order_canonical.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::CanonicalOrder(
                    true,
                ),
            );
        }
        if self.use_times_dot.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::TimesDot(true),
            );
        }
        if self.expand_brackets.unwrap() {
            v.push(
                numbas::question::answer_simplification::AnswerSimplificationType::ExpandBrackets(
                    true,
                ),
            );
        }
        v
    }
}

impl ToRumbas<JMEAnswerSimplification>
    for Option<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
{
    fn to_rumbas(&self) -> JMEAnswerSimplification {
        let mut result = JMEAnswerSimplification {
            simplify_basic: Value::Normal(DEFAULTS.jme_simplification_simplify_basic),
            simplify_unit_factor: Value::Normal(DEFAULTS.jme_simplification_simplify_unit_factor),
            simplify_unit_power: Value::Normal(DEFAULTS.jme_simplification_simplify_unit_power),
            simplify_unit_denominator: Value::Normal(
                DEFAULTS.jme_simplification_simplify_unit_denominator,
            ),
            simplify_zero_factor: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_factor),
            simplify_zero_term: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_term),
            simplify_zero_power: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_power),
            simplify_zero_base: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_base),
            collect_numbers: Value::Normal(DEFAULTS.jme_simplification_collect_numbers),
            constants_first: Value::Normal(DEFAULTS.jme_simplification_constants_first),
            simplify_sqrt_products: Value::Normal(
                DEFAULTS.jme_simplification_simplify_sqrt_products,
            ),
            simplify_sqrt_division: Value::Normal(
                DEFAULTS.jme_simplification_simplify_sqrt_division,
            ),
            simplify_sqrt_square: Value::Normal(DEFAULTS.jme_simplification_simplify_sqrt_square),
            simplify_other_numbers: Value::Normal(
                DEFAULTS.jme_simplification_simplify_other_numbers,
            ),
            simplify_no_leading_minus: Value::Normal(
                DEFAULTS.jme_simplification_simplify_no_leading_minus,
            ),
            simplify_fractions: Value::Normal(DEFAULTS.jme_simplification_simplify_fractions),
            simplify_trigonometric: Value::Normal(
                DEFAULTS.jme_simplification_simplify_trigonometric,
            ),
            cancel_terms: Value::Normal(DEFAULTS.jme_simplification_cancel_terms),
            cancel_factors: Value::Normal(DEFAULTS.jme_simplification_cancel_factors),
            collect_like_fractions: Value::Normal(
                DEFAULTS.jme_simplification_collect_like_fractions,
            ),
            order_canonical: Value::Normal(DEFAULTS.jme_simplification_order_canonical),
            use_times_dot: Value::Normal(DEFAULTS.jme_simplification_use_times_dot),
            expand_brackets: Value::Normal(DEFAULTS.jme_simplification_expand_brackets),
        }; // Numbas default
        if let Some(v) = self {
            for a in v.iter() {
                match a {
                    numbas::question::answer_simplification::AnswerSimplificationType::All(b) => {
                        result.simplify_basic = Value::Normal(*b);
                        result.simplify_unit_factor = Value::Normal(*b);
                        result.simplify_unit_power = Value::Normal(*b);
                        result.simplify_unit_denominator = Value::Normal(*b);
                        result.simplify_zero_factor = Value::Normal(*b);
                        result.simplify_zero_term = Value::Normal(*b);
                        result.simplify_zero_power = Value::Normal(*b);
                        result.simplify_zero_base = Value::Normal(*b);
                        result.collect_numbers = Value::Normal(*b);
                        result.constants_first = Value::Normal(*b);
                        result.simplify_sqrt_products = Value::Normal(*b);
                        result.simplify_sqrt_division = Value::Normal(*b);
                        result.simplify_sqrt_square = Value::Normal(*b);
                        result.simplify_other_numbers = Value::Normal(*b);
                        result.simplify_no_leading_minus = Value::Normal(*b);
                        result.simplify_fractions = Value::Normal(*b);
                        result.simplify_trigonometric = Value::Normal(*b);
                        result.cancel_terms = Value::Normal(*b);
                        result.cancel_factors = Value::Normal(*b);
                        result.collect_like_fractions = Value::Normal(*b);
                        result.use_times_dot = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::Basic(b) => {
                        result.simplify_basic = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::UnitFactor(b) => {
                        result.simplify_unit_factor = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::UnitPower(b) => {
                        result.simplify_unit_power = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::UnitDenominator(b) => {
                        result.simplify_unit_denominator = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::ZeroFactor(b) => {
                        result.simplify_zero_factor = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::ZeroTerm(b) => {
                        result.simplify_zero_term = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::ZeroPower(b) => {
                        result.simplify_zero_power = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::CollectNumbers(b) => {
                        result.collect_numbers = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::ZeroBase(b) => {
                        result.simplify_zero_base = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::ConstantsFirst(b) => {
                        result.constants_first = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::SqrtProduct(b) => {
                        result.simplify_sqrt_products = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::SqrtDivision(b) => {
                        result.simplify_sqrt_division = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::SqrtSquare(b) => {
                        result.simplify_sqrt_square = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::OtherNumbers(b) => {
                        result.simplify_other_numbers = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::NoLeadingMinus(b) => {
                        result.simplify_no_leading_minus = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::Fractions(b) => {
                        result.simplify_fractions = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::Trigonometric(b) => {
                        result.simplify_trigonometric = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::CancelTerms(b) => {
                        result.cancel_terms = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::CancelFactors(b) => {
                        result.cancel_factors = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::CollectLikeFractions(b) => {
                        result.collect_like_fractions = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::TimesDot(b) => {
                        result.use_times_dot = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::ExpandBrackets(b) => {
                        result.expand_brackets = Value::Normal(*b);
                    }
                    numbas::question::answer_simplification::AnswerSimplificationType::CanonicalOrder(b) => {
                        result.order_canonical = Value::Normal(*b);
                    }
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

optional_overwrite! {
    pub struct CheckingTypeDataFloat {
        max_difference: RumbasFloat
    }
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
            checking_accuracy: self.max_difference.unwrap().into(),
        }
    }
}

optional_overwrite! {
    pub struct CheckingTypeDataNatural {
        amount: RumbasNatural
    }
}

impl ToNumbas<numbas::question::part::jme::JMECheckingTypeData<usize>> for CheckingTypeDataNatural {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::jme::JMECheckingTypeData<usize> {
        numbas::question::part::jme::JMECheckingTypeData {
            checking_accuracy: self.amount.unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum CheckingType {
    RelativeDifference(CheckingTypeDataFloat),
    AbsoluteDifference(CheckingTypeDataFloat),
    DecimalPlaces(CheckingTypeDataNatural),
    SignificantFigures(CheckingTypeDataNatural),
}
impl_optional_overwrite!(CheckingType);

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

optional_overwrite! {
    pub struct JMERestriction {
        // name: TranslatableString,
        partial_credit: RumbasFloat, //TODO, is number, so maybe usize?
        message: TranslatableString
    }
}

impl ToNumbas<numbas::question::part::jme::JMERestriction> for JMERestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMERestriction {
        numbas::question::part::jme::JMERestriction {
            // name: self.name.clone().unwrap().to_string(locale).unwrap(),
            partial_credit: self.partial_credit.clone().unwrap().to_numbas(locale),
            message: self.message.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMERestriction> for numbas::question::part::jme::JMERestriction {
    fn to_rumbas(&self) -> JMERestriction {
        JMERestriction {
            //name: Value::Normal(TranslatableString::s(&self.name)),
            partial_credit: self.partial_credit.0.to_rumbas(),
            message: self.message.to_rumbas(),
        }
    }
}

optional_overwrite! {
    pub struct JMELengthRestriction {
        #[serde(flatten)]
        restriction: JMERestriction,
        length: RumbasNatural
    }
}

impl ToNumbas<numbas::question::part::jme::JMELengthRestriction> for JMELengthRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMELengthRestriction {
        numbas::question::part::jme::JMELengthRestriction {
            restriction: self.restriction.clone().unwrap().to_numbas(locale),
            length: Some(self.length.clone().unwrap().to_numbas(locale)),
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

optional_overwrite! {
    pub struct JMEStringRestriction {
        #[serde(flatten)]
        restriction: JMERestriction,
        show_strings: RumbasBool,
        strings: TranslatableStrings
    }
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

optional_overwrite! {
    pub struct JMEPatternRestriction {
        partial_credit: RumbasFloat, //TODO, is number, so maybe usize?
        message: TranslatableString,
        pattern: RumbasString, //TODO type? If string -> InputString?
        name_to_compare: RumbasString //TODO, translateable?
    }
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

optional_overwrite! {
    pub struct JMEValueGenerator {
        name: FileString,
        value: JMEFileString
    }
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
        let s: RumbasString = self.value.clone().into();
        JMEValueGenerator {
            name: self.name.to_rumbas(),
            value: s.to_rumbas(),
        }
    }
}

pub type JMEValueGeneratorsInput = Vec<Value<JMEValueGeneratorInput>>;
pub type JMEValueGenerators = Vec<JMEValueGenerator>;
