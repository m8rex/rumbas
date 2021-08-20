use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::file_reference::{FileString, JMEFileString};
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
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

impl ToNumbas<numbas::exam::ExamQuestionPartJME> for QuestionPartJME {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartJME {
        numbas::exam::ExamQuestionPartJME {
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

impl ToRumbas<QuestionPartJME> for numbas::exam::ExamQuestionPartJME {
    fn to_rumbas(&self) -> QuestionPartJME {
        create_question_part! {
            QuestionPartJME with &self.part_data => {
                answer: Value::Normal(self.answer.to_rumbas()),
                answer_simplification: Value::Normal(self.answer_simplification.to_rumbas()),
                show_preview: Value::Normal(self.show_preview),
                answer_check: Value::Normal(self.checking_type.to_rumbas()),
                failure_rate: Value::Normal(self.failure_rate.unwrap_or(DEFAULTS.jme_failure_rate)),
                vset_range: Value::Normal([self.vset_range[0].0, self.vset_range[1].0]),
                vset_range_points: Value::Normal(self.vset_range_points.0),
                check_variable_names: Value::Normal(self.check_variable_names),
                single_letter_variables: Value::Normal(
                    self.single_letter_variables
                        .unwrap_or(DEFAULTS.jme_single_letter_variables),
                ),
                allow_unknown_functions: Value::Normal(
                    self.allow_unknown_functions
                        .unwrap_or(DEFAULTS.jme_allow_unknown_functions),
                ),
                implicit_function_composition: Value::Normal(
                    self.implicit_function_composition
                        .unwrap_or(DEFAULTS.jme_implicit_function_composition),
                ),

                max_length: Value::Normal(
                    self.max_length
                        .clone()
                        .map(|r| Noneable::NotNone(r.to_rumbas()))
                        .unwrap_or_else(Noneable::nn),
                ),
                min_length: Value::Normal(
                    self.min_length
                        .clone()
                        .map(|r| Noneable::NotNone(r.to_rumbas()))
                        .unwrap_or_else(Noneable::nn),
                ),
                must_have: Value::Normal(
                    self.must_have
                        .clone()
                        .map(|r| Noneable::NotNone(r.to_rumbas()))
                        .unwrap_or_else(Noneable::nn),
                ),
                may_not_have: Value::Normal(
                    self.may_not_have
                        .clone()
                        .map(|r| Noneable::NotNone(r.to_rumbas()))
                        .unwrap_or_else(Noneable::nn),
                ),
                must_match_pattern: Value::Normal(
                    self.must_match_pattern
                        .clone()
                        .map(|r| Noneable::NotNone(r.to_rumbas()))
                        .unwrap_or_else(Noneable::nn),
                ),
                value_generators: Value::Normal(
                    self.value_generators
                        .clone()
                        .map(|v| Noneable::NotNone(v.iter().map(|g| g.to_rumbas()).collect()))
                        .unwrap_or_else(Noneable::nn),
                )
            }
        }
    }
}

// See https://numbas-editor.readthedocs.io/en/latest/simplification.html#term-expandbrackets
//TODO: rename etc
optional_overwrite! {
    pub struct JMEAnswerSimplification {
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
        simplify_other_numbers: bool,
        simplify_no_leading_minus: bool,
        simplify_fractions: bool,
        simplify_trigonometric: bool,
        cancel_terms: bool,
        cancel_factors: bool,
        collect_like_fractions: bool,
        order_canonical: bool,
        use_times_dot: bool, // Use \cdot instead of \times
        expand_brackets: bool
    }
}

impl ToNumbas<Vec<numbas::exam::AnswerSimplificationType>> for JMEAnswerSimplification {
    fn to_numbas(&self, _locale: &str) -> Vec<numbas::exam::AnswerSimplificationType> {
        let mut v = Vec::new();
        if self.simplify_basic.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::Basic(true));
        }
        if self.simplify_unit_factor.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::UnitFactor(true));
        }
        if self.simplify_unit_power.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::UnitPower(true));
        }
        if self.simplify_unit_denominator.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::UnitDenominator(
                true,
            ));
        }
        if self.simplify_zero_factor.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::ZeroFactor(true));
        }
        if self.simplify_zero_term.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::ZeroTerm(true));
        }
        if self.simplify_zero_power.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::ZeroPower(true));
        }
        if self.simplify_zero_base.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::ZeroBase(true));
        }
        if self.collect_numbers.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::CollectNumbers(true));
        }
        if self.constants_first.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::ConstantsFirst(true));
        }
        if self.simplify_sqrt_products.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::SqrtProduct(true));
        }
        if self.simplify_sqrt_division.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::SqrtDivision(true));
        }
        if self.simplify_sqrt_square.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::SqrtSquare(true));
        }
        if self.simplify_other_numbers.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::OtherNumbers(true));
        }
        if self.simplify_no_leading_minus.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::NoLeadingMinus(true));
        }
        if self.simplify_fractions.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::Fractions(true));
        }
        if self.simplify_trigonometric.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::Trigonometric(true));
        }
        if self.cancel_terms.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::CancelTerms(true));
        }
        if self.cancel_factors.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::CancelFactors(true));
        }
        if self.collect_like_fractions.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::CollectLikeFractions(true));
        }
        if self.order_canonical.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::CanonicalOrder(true));
        }
        if self.use_times_dot.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::TimesDot(true));
        }
        if self.expand_brackets.unwrap() {
            v.push(numbas::exam::AnswerSimplificationType::ExpandBrackets(true));
        }
        v
    }
}

impl ToRumbas<JMEAnswerSimplification> for Option<Vec<numbas::exam::AnswerSimplificationType>> {
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
                    numbas::exam::AnswerSimplificationType::All(b) => {
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
                    numbas::exam::AnswerSimplificationType::Basic(b) => {
                        result.simplify_basic = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::UnitFactor(b) => {
                        result.simplify_unit_factor = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::UnitPower(b) => {
                        result.simplify_unit_power = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::UnitDenominator(b) => {
                        result.simplify_unit_denominator = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::ZeroFactor(b) => {
                        result.simplify_zero_factor = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::ZeroTerm(b) => {
                        result.simplify_zero_term = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::ZeroPower(b) => {
                        result.simplify_zero_power = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::CollectNumbers(b) => {
                        result.collect_numbers = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::ZeroBase(b) => {
                        result.simplify_zero_base = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::ConstantsFirst(b) => {
                        result.constants_first = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::SqrtProduct(b) => {
                        result.simplify_sqrt_products = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::SqrtDivision(b) => {
                        result.simplify_sqrt_division = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::SqrtSquare(b) => {
                        result.simplify_sqrt_square = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::OtherNumbers(b) => {
                        result.simplify_other_numbers = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::NoLeadingMinus(b) => {
                        result.simplify_no_leading_minus = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::Fractions(b) => {
                        result.simplify_fractions = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::Trigonometric(b) => {
                        result.simplify_trigonometric = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::CancelTerms(b) => {
                        result.cancel_terms = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::CancelFactors(b) => {
                        result.cancel_factors = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::CollectLikeFractions(b) => {
                        result.collect_like_fractions = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::TimesDot(b) => {
                        result.use_times_dot = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::ExpandBrackets(b) => {
                        result.expand_brackets = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::CanonicalOrder(b) => {
                        result.order_canonical = Value::Normal(*b);
                    }
                    numbas::exam::AnswerSimplificationType::Unknown((name, val)) => {
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
        max_difference: f64
    }
}

impl ToNumbas<numbas::exam::JMECheckingTypeData<numbas::exam::SafeFloat>>
    for CheckingTypeDataFloat
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::exam::JMECheckingTypeData<numbas::exam::SafeFloat> {
        numbas::exam::JMECheckingTypeData {
            checking_accuracy: self.max_difference.unwrap().into(),
        }
    }
}

optional_overwrite! {
    pub struct CheckingTypeDataNatural {
        amount: usize
    }
}

impl ToNumbas<numbas::exam::JMECheckingTypeData<usize>> for CheckingTypeDataNatural {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::JMECheckingTypeData<usize> {
        numbas::exam::JMECheckingTypeData {
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

impl ToNumbas<numbas::exam::JMECheckingType> for CheckingType {
    fn to_numbas(&self, locale: &str) -> numbas::exam::JMECheckingType {
        match self {
            CheckingType::RelativeDifference(f) => {
                numbas::exam::JMECheckingType::RelativeDifference(f.to_numbas(locale))
            }
            CheckingType::AbsoluteDifference(f) => {
                numbas::exam::JMECheckingType::AbsoluteDifference(f.to_numbas(locale))
            }
            CheckingType::DecimalPlaces(f) => {
                numbas::exam::JMECheckingType::DecimalPlaces(f.to_numbas(locale))
            }
            CheckingType::SignificantFigures(f) => {
                numbas::exam::JMECheckingType::SignificantFigures(f.to_numbas(locale))
            }
        }
    }
}

impl ToRumbas<CheckingType> for numbas::exam::JMECheckingType {
    fn to_rumbas(&self) -> CheckingType {
        match self {
            numbas::exam::JMECheckingType::RelativeDifference(v) => {
                CheckingType::RelativeDifference(CheckingTypeDataFloat {
                    max_difference: Value::Normal(v.checking_accuracy.0),
                })
            }
            numbas::exam::JMECheckingType::AbsoluteDifference(v) => {
                CheckingType::AbsoluteDifference(CheckingTypeDataFloat {
                    max_difference: Value::Normal(v.checking_accuracy.0),
                })
            }
            numbas::exam::JMECheckingType::DecimalPlaces(v) => {
                CheckingType::DecimalPlaces(CheckingTypeDataNatural {
                    amount: Value::Normal(v.checking_accuracy),
                })
            }
            numbas::exam::JMECheckingType::SignificantFigures(v) => {
                CheckingType::SignificantFigures(CheckingTypeDataNatural {
                    amount: Value::Normal(v.checking_accuracy),
                })
            }
        }
    }
}

optional_overwrite! {
    pub struct JMERestriction {
        // name: TranslatableString,
        partial_credit: f64, //TODO, is number, so maybe usize?
        message: TranslatableString
    }
}

impl ToNumbas<numbas::exam::JMERestriction> for JMERestriction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::JMERestriction {
        numbas::exam::JMERestriction {
            // name: self.name.clone().unwrap().to_string(locale).unwrap(),
            partial_credit: self.partial_credit.clone().unwrap().into(),
            message: self.message.clone().unwrap().to_string(locale).unwrap(),
        }
    }
}

impl ToRumbas<JMERestriction> for numbas::exam::JMERestriction {
    fn to_rumbas(&self) -> JMERestriction {
        JMERestriction {
            //name: Value::Normal(TranslatableString::s(&self.name)),
            partial_credit: Value::Normal(self.partial_credit.0),
            message: Value::Normal(self.message.clone().into()),
        }
    }
}

optional_overwrite! {
    pub struct JMELengthRestriction {
        #[serde(flatten)]
        restriction: JMERestriction,
        length: usize
    }
}

impl ToNumbas<numbas::exam::JMELengthRestriction> for JMELengthRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::JMELengthRestriction {
        numbas::exam::JMELengthRestriction {
            restriction: self.restriction.clone().unwrap().to_numbas(locale),
            length: Some(self.length.clone().unwrap().into()),
        }
    }
}

impl ToRumbas<JMELengthRestriction> for numbas::exam::JMELengthRestriction {
    fn to_rumbas(&self) -> JMELengthRestriction {
        JMELengthRestriction {
            restriction: Value::Normal(self.restriction.to_rumbas()),
            length: Value::Normal(
                self.length
                    .map(|v| v.0)
                    .unwrap_or(DEFAULTS.length_restriction_length),
            ),
        }
    }
}

optional_overwrite! {
    pub struct JMEStringRestriction {
        #[serde(flatten)]
        restriction: JMERestriction,
        show_strings: bool,
        strings: Vec<TranslatableString>
    }
}

impl ToNumbas<numbas::exam::JMEStringRestriction> for JMEStringRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::JMEStringRestriction {
        numbas::exam::JMEStringRestriction {
            restriction: self.restriction.clone().unwrap().to_numbas(locale),
            show_strings: self.show_strings.clone().unwrap(),
            strings: self
                .strings
                .clone()
                .unwrap()
                .into_iter()
                .map(|s| s.to_string(locale).unwrap())
                .collect(),
        }
    }
}

impl ToRumbas<JMEStringRestriction> for numbas::exam::JMEStringRestriction {
    fn to_rumbas(&self) -> JMEStringRestriction {
        JMEStringRestriction {
            restriction: Value::Normal(self.restriction.to_rumbas()),
            show_strings: Value::Normal(self.show_strings),
            strings: Value::Normal(self.strings.clone().into_iter().map(|s| s.into()).collect()),
        }
    }
}

optional_overwrite! {
    pub struct JMEPatternRestriction {
        partial_credit: f64, //TODO, is number, so maybe usize?
        message: TranslatableString,
        pattern: String, //TODO type? If string -> InputString?
        name_to_compare: String //TODO, translateable?
    }
}

impl ToNumbas<numbas::exam::JMEPatternRestriction> for JMEPatternRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::JMEPatternRestriction {
        numbas::exam::JMEPatternRestriction {
            partial_credit: self.partial_credit.clone().unwrap().into(),
            message: self.message.clone().unwrap().to_string(locale).unwrap(),
            pattern: self.pattern.clone().unwrap(),
            name_to_compare: self.name_to_compare.clone().unwrap(),
        }
    }
}

impl ToRumbas<JMEPatternRestriction> for numbas::exam::JMEPatternRestriction {
    fn to_rumbas(&self) -> JMEPatternRestriction {
        JMEPatternRestriction {
            partial_credit: Value::Normal(self.partial_credit.0),
            message: Value::Normal(self.message.clone().into()),
            pattern: Value::Normal(self.pattern.clone()),
            name_to_compare: Value::Normal(self.name_to_compare.clone()),
        }
    }
}

optional_overwrite! {
    pub struct JMEValueGenerator {
        name: FileString,
        value: JMEFileString
    }
}

impl ToNumbas<numbas::exam::JMEValueGenerator> for JMEValueGenerator {
    fn to_numbas(&self, locale: &str) -> numbas::exam::JMEValueGenerator {
        numbas::exam::JMEValueGenerator {
            name: self.name.clone().unwrap().to_numbas(locale),
            value: self.value.clone().unwrap().to_numbas(locale),
        }
    }
}

impl ToRumbas<JMEValueGenerator> for numbas::exam::JMEValueGenerator {
    fn to_rumbas(&self) -> JMEValueGenerator {
        let s: String = self.value.clone().into();
        JMEValueGenerator {
            name: Value::Normal(self.name.clone().into()),
            value: Value::Normal(JMEFileString::s(&s[..])),
        }
    }
}
