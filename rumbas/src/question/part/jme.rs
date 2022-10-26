use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::{AdaptiveMarking, CustomMarking};
use crate::question::QuestionPart;
use crate::support::file_reference::{FileString, JMEFileString};
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::TranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
    #[input(name = "QuestionPartJMEInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartJME {
        /// The expected answer to the part.
        answer: EmbracedJMETranslatableString, //TODO: should this be translatable?
        /// Simplification rules to apply to the correct answer, if it is displayed to the student (for example, after clicking the Reveal answers button). This shouldn’t affect marking.
        /// The simplification rules to apply to the answer
        answer_simplification: JMEAnswerSimplification,
        /// The display rules to apply to the answer
        answer_display: JMEAnswerDisplay,
        /// If ticked, a rendering of the student’s answer in mathematical notation is displayed beside the input box.
        show_preview: bool,

        /// Defines the range of points over which the student’s answer will be compared with the
        /// correct answer, and the method used to compare them
        accuracy: JMEAccuracy,

        /// If this is ticked, all variable names used in the student’s are checked against the variable names used in the correct answer. The first variable name which is not used in the correct answer will trigger a warning. You can use this option to prevent students incorrectly entering answers such as xy, which is interpreted as a single variable, when they mean x*y, the product of two variables.
        check_variable_names: bool,
        /// If this is ticked, long variable names will be interpreted as implicit multiplication of variables with single-letter names. For example, xyz will be interpreted as x * y * z. Digits, primes and single-letter underscores are still valid in variable names: a'x12y_z will be interpreted as a' * x12 * y_z. Greek letters are considered to be a single letter: pix will be interpreted as pi * x.
        single_letter_variables: bool,
        /// If this is not ticked, the application of a function that is not defined in JME will be reinterpreted. If the function name can be split into several shorter names, each of which is defined in JME, it will be: for example, lnabs(x) will be interpreted as ln(abs(x)). Function names are recognised from right to left. Any remaining characters are interpreted as implicit multiplication by a variable. For example, xsin(x) will be interpreted as x * sin(x).
        allow_unknown_functions: bool,
        /// If this is ticked, the multiplication symbol (or implicit multiplication) will be interpreted as function composition when the right-hand side is a function application with one argument, and the left-hand side is the name of a function defined in JME. For example, ln * abs(x) and ln abs(x) will be interpreted as ln(abs(x)).
        implicit_function_composition: bool,

        /// The student’s answer must match the given pattern. If it does not, then a penalty is applied.
        must_match_pattern: Noneable<JMEPatternRestriction>,
        /// Variable value generators override the default method used to pick values for variables when comparing the correct answer with the student’s answer.
        /// The expression for each variable can be written in terms of the other variables, as long as there are no circular dependencies. The values will be evaluated in order, like question variables.
        /// Each variable specified in the expected answer can be overriden
        /// The variable vRange represents the checking range defined for this part: a continuous interval between the checking range start and checking range end.
        value_generators: Noneable<Vec<JMEValueGenerator>>,

        /// *DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.
        ///
        /// Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.
        ///
        /// If the student’s answer contains more than this many characters, the penalty is applied. A value of zero means no restriction is applied. The student’s answer is tidied up slightly so that things like extra or missing space characters don’t affect the calculated length. All spaces are removed, and then spaces are inserted between binary operations. For example, the answer 1+x (three characters) is marked as 1 + x (five characters).
        max_length: Noneable<JMELengthRestriction>,
        /// *DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.
        ///
        /// Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.
        ///
        /// If the student’s answer contains fewer than this many characters, the penalty is applied. A value of zero means no restriction is applied.
        min_length: Noneable<JMELengthRestriction>,
        /// *DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.
        ///
        /// Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.
        ///
        /// If the student’s answer doesn’t contain all of these strings, the penalty is applied.
        must_have: Noneable<JMEStringRestriction>,
        /// *DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.
        ///
        /// Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.
        ///
        /// If the student’s answer contains any of these strings, the penalty is applied.
        may_not_have: Noneable<JMEStringRestriction>

        // TODO: case sensitive
    }
}

impl ToNumbas<numbas::question::part::jme::QuestionPartJME> for QuestionPartJME {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::QuestionPartJME {
        numbas::question::part::jme::QuestionPartJME {
            part_data: self.to_numbas(locale),
            answer: self.answer.to_numbas(locale),
            answer_simplification: self.answer_simplification.to_numbas(locale),
            show_preview: self.show_preview.to_numbas(locale),
            accuracy: self.accuracy.to_numbas(locale),
            check_variable_names: self.check_variable_names.to_numbas(locale),
            single_letter_variables: self.single_letter_variables.to_numbas(locale),
            allow_unknown_functions: self.allow_unknown_functions.to_numbas(locale),
            implicit_function_composition: self.implicit_function_composition.to_numbas(locale),
            max_length: self.max_length.to_numbas(locale),
            min_length: self.min_length.to_numbas(locale),

            must_have: self.must_have.to_numbas(locale),
            may_not_have: self.may_not_have.to_numbas(locale),
            must_match_pattern: self.must_match_pattern.to_numbas(locale),
            value_generators: self.value_generators.to_numbas(locale).unwrap_or_default(),
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
                accuracy: self.accuracy.to_rumbas(),
                check_variable_names: self.check_variable_names.to_rumbas(),
                single_letter_variables:
                    self.single_letter_variables
                        .to_rumbas(),
                allow_unknown_functions:
                    self.allow_unknown_functions
                        .to_rumbas(),
                implicit_function_composition:
                    self.implicit_function_composition
                        .to_rumbas(),
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
                    if self.value_generators.is_empty() {
                        Noneable::None
                    } else {
                        Noneable::NotNone(self.value_generators.to_rumbas())
                    }
            }
        }
    }
}

macro_rules! create_answer_simplification {
    ($struct: ident: $input: literal: $variant: ident: $variant_struct: ident,
        $(
            $(#[$inner:meta])*
            $name: ident: $numbas_name: ident: $partofall: expr
        ),*) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
        #[input(name = $input)]
        #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
        pub struct $struct {
            $(
                $(#[$inner])*
                pub $name: bool
            ),*
        }

        impl std::default::Default for $struct {
            fn default() -> Self {
                // load empty json object
                let default_answer_simplification = numbas::question::part::jme::default_answer_simplification();
                let mut result = $struct {
                    $(
                        $name: false
                    ),*
                };
                    for a in default_answer_simplification.into_iter() {
                        match a {
                            numbas::question::answer_simplification::AnswerSimplificationType::$variant(r) =>
                                match r {
                                    numbas::question::answer_simplification::$variant_struct::All(b) => {
                                        $(if $partofall {
                                            result.$name = b;
                                        })*
                                    }
                                    $(numbas::question::answer_simplification::$variant_struct::$numbas_name(b) => {
                                        result.$name = b;
                                    })*
                                }
                            _ => ()
                        }
                    }
                    result
            }
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
            for Vec<numbas::question::answer_simplification::AnswerSimplificationType>
        {
            fn to_rumbas(&self) -> $struct {
                let mut result = $struct::default();
                    for a in self.iter() {
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

                result
            }
        }
    }
}

// See https://numbas-editor.readthedocs.io/en/latest/simplification.html#term-expandbrackets
create_answer_simplification! {
    JMEAnswerSimplification: "JMEAnswerSimplificationInput": Rule: AnswerSimplificationRule,
    /// Some basic rules: https://numbas-editor.readthedocs.io/en/latest/simplification.html?highlight=simplification#term-basic
    simplify_basic: Basic: true,
    #[serde(alias = "simplify_unit_factor")]
    /// Cancel products of 1
    cancel_unit_factors: CancelUnitFactors: true,
    #[serde(alias = "simplify_unit_power")]
    /// Cancel exponents of 1
    cancel_unit_powers: CancelUnitPowers: true,
    #[serde(alias = "simplify_unit_denominator")]
    /// Cancel fractions with denominator 1
    cancel_unit_denominators: CancelUnitDenominators: true,
    #[serde(alias = "simplify_zero_factor")]
    /// Cancel products of zero to zero
    cancel_zero_factors: CancelZeroFactors: true,
    #[serde(alias = "simplify_zero_term")]
    /// Omit zero terms
    omit_zero_terms: OmitZeroTerms: true,
    #[serde(alias = "simplify_zero_power")]
    /// Cancel exponents of 0
    cancel_zero_powers: CancelZeroPowers: true,
    #[serde(alias = "simplify_zero_base")]
    /// Cancel any power of zero
    cancel_powers_with_base_zero: CancelPowersWithBaseZero: true,
    /// Collect together numerical (as opposed to variable) products and sums.
    collect_numbers: CollectNumbers: true,
    /// Numbers go to the left of multiplications
    constants_first: ConstantsFirst: true,
    #[serde(alias = "simplify_sqrt_products")]
    /// Collect products of square roots
    collect_sqrt_products: CollectSqrtProducts: true,
    #[serde(alias = "simplify_sqrt_division")]
    /// Collect fractions of square roots
    collect_sqrt_divisions: CollectSqrtDivisions: true,
    #[serde(alias = "simplify_sqrt_square")]
    /// Cancel square roots of squares, and squares of square roots
    cancel_sqrt_square: CancelSqrtSquares: true,
    #[serde(alias = "simplify_other_numbers")]
    /// Evaluate powers of numbers.
    evaluate_powers_of_numbers: EvaluatePowersOfNumbers: true,
    #[serde(alias = "simplify_no_leading_minus")]
    /// Rearrange expressions so they don’t start with a unary minus
    rewrite_to_no_leading_minus: NoLeadingMinus: true,
    /// Cancel fractions to lowest form
    simplify_fractions: Fractions: true,
    /// Simplify some trigonometric identities
    simplify_trigonometric: Trigonometric: true,
    #[serde(alias = "cancel_terms")]
    /// Collect together and cancel terms. Like collectNumbers, but for any kind of term.
    collect_terms: CollectTerms: true,
    #[serde(alias = "cancel_factors")]
    /// Collect together powers of common factors.
    collect_powers_of_common_factors: CollectPowersOfCommonFactors: true,
    /// Collect together fractions over the same denominator.
    collect_like_fractions: CollectLikeFractions: true,
    /// Rearrange the expression into a “canonical” order, using canonical_compare.
    ///
    /// Note: This rule can not be used at the same time as rewrite_to_no_leading_minus - it can lead to an infinite loop.
    order_canonical: CanonicalOrder: false,
    /// Expand out products of sums.
    expand_brackets: ExpandBrackets: false
}

macro_rules! create_answer_display_type {
    ($struct: ident: $input: literal: $variant: ident: $variant_struct: ident,
        $(
            $(#[$inner:meta])*
            $name: ident: $numbas_name: ident
        ),*) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
        #[input(name = $input)]
        #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
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
            for Vec<numbas::question::answer_simplification::AnswerSimplificationType>
        {
            fn to_rumbas(&self) -> $struct {
                let mut result = $struct {
                    $(
                        $name: false
                    ),*
                };
                    for a in self.iter() {
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

                result
            }
        }
    }
}

create_answer_display_type! {
    JMEAnswerDisplay: "JMEAnswerDisplayInput": DisplayOption: AnswerSimplificationDisplayOption,
    /// This rule doesn’t rewrite expressions, but tells the maths renderer that you’d like non-integer numbers to be displayed as fractions instead of decimals.
    broken_as_fractions: Fractions,
    /// Improper fractions (with numerator larger than the denominator) are displayed in mixed form, as an integer next to a proper fraction.
    mixed_fractions: MixedFractions,
    /// Fractions are displayed on a single line, with a slash between the numerator and denominator.
    flat_fractions: FlatFractions,
    /// This rule doesn’t rewrite expressions, but tells the maths renderer that you’d like vectors to be rendered as rows instead of columns.
    vector_as_row: RowVector,
    /// The multiplication symbol is always included between multiplicands.
    always_show_multiplication_sign: AlwaysShowMultiplicationSign,
    /// Use a dot for the multiplication symbol instead of a cross.
    use_dot_as_multiplication_sign: DotAsMultiplicationSign, // Use \cdot instead of \times
    /// Matrices are rendered without parentheses.
    matrices_without_parentheses: MatricesWithoutParentheses
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "CheckingTypeDataFloatInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct CheckingTypeDataFloat {
    /// Maximum relative or absolute difference
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "CheckingTypeDataNaturalInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct CheckingTypeDataNatural {
    /// Amount of decimal places or significant figures
    pub amount: usize,
}

impl ToNumbas<numbas::question::part::jme::JMECheckingTypeData<usize>> for CheckingTypeDataNatural {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::jme::JMECheckingTypeData<usize> {
        numbas::question::part::jme::JMECheckingTypeData {
            checking_accuracy: self.amount,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "CheckingTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum CheckingType {
    /// Fail if studentanswer / expectedanswer - 1 > amount
    RelativeDifference(CheckingTypeDataFloat),
    /// Fail if abs(studentanswer - expectedanswer) > amount
    AbsoluteDifference(CheckingTypeDataFloat),
    /// x and y are rounded to a certain amount of decimal places, and the test fails if the
    /// rounded values are unequal
    DecimalPlaces(CheckingTypeDataNatural),
    /// x and y are rounded to significant figures, and the test fails if the rounded values are unequal.
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMERestrictionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct JMERestriction {
    // name: TranslatableString,
    /// The partial credit (percentage) attributed when failing the restriction
    pub partial_credit: f64, //TODO, is number, so maybe usize?
    /// Warning message
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMELengthRestrictionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct JMELengthRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    /// The minimum or maximum length
    pub length: usize,
}

impl ToNumbas<numbas::question::part::jme::JMELengthRestriction> for JMELengthRestriction {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMELengthRestriction {
        numbas::question::part::jme::JMELengthRestriction {
            restriction: self.restriction.to_numbas(locale),
            length: self.length.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMELengthRestriction> for numbas::question::part::jme::JMELengthRestriction {
    fn to_rumbas(&self) -> JMELengthRestriction {
        JMELengthRestriction {
            restriction: self.restriction.to_rumbas(),
            length: self.length.0.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMEStringRestrictionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct JMEStringRestriction {
    #[serde(flatten)]
    pub restriction: JMERestriction,
    /// The strings that are required or forbidden
    pub strings: Vec<TranslatableString>,
    /// Whether to show the strings
    pub show_strings: bool,
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMEPatternRestrictionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct JMEPatternRestriction {
    /// If the student’s answer does not match the given pattern, their score is multiplied by this percentage.
    pub partial_credit: f64, //TODO, is number, so maybe usize?
    /// Warning message
    pub message: TranslatableString,
    /// See https://numbas-editor.readthedocs.io/en/latest/pattern-matching/examples.html#pattern-matching-examples for example patterns
    pub pattern: String, //TODO type? If string -> InputString?
    /// The part of the expression to mark
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMEValueGeneratorInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct JMEValueGenerator {
    /// The name of the variable
    pub name: FileString,
    /// The expression to generate the value
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMERulesetItemInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", content = "rules")]
pub enum JMERulesetItem {
    Simplification(JMEAnswerSimplification),
    Display(JMEAnswerDisplay),
}

impl ToNumbas<Vec<numbas::question::answer_simplification::AnswerSimplificationType>>
    for JMERulesetItem
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> Vec<numbas::question::answer_simplification::AnswerSimplificationType> {
        match self {
            Self::Simplification(s) => s.to_numbas(locale),
            Self::Display(d) => d.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMERulesetItem>
    for Vec<numbas::question::answer_simplification::AnswerSimplificationType>
{
    fn to_rumbas(&self) -> JMERulesetItem {
        let has_different_types = !self.is_empty()
            && (0..self.len() - 1).any(|i| {
                !matches!((&self[i], &self[i + 1]),
                (
                                     &numbas::question::answer_simplification::AnswerSimplificationType::Rule(_),
                                     &numbas::question::answer_simplification::AnswerSimplificationType::Rule(_),
                                 ) | (
                                     &numbas::question::answer_simplification::AnswerSimplificationType::DisplayOption(
                                         _,
                                     ),
                                     &numbas::question::answer_simplification::AnswerSimplificationType::DisplayOption(
                                         _,
                                     )
                ))
            });
        if has_different_types {
            // It would be needed to create two different ruleset's for and they should have a different name.
            // Their usage should be changed in the jme
            // TODO
            log::error!("Importing rulesets with both Simplification rules and DisplayOptions, is currently not supported. Only adding simplification rules.");
        }

        if has_different_types {
            JMERulesetItem::Simplification(self.to_rumbas())
        } else if let Some(
            numbas::question::answer_simplification::AnswerSimplificationType::Rule(_),
        ) = self.get(0)
        {
            JMERulesetItem::Simplification(self.to_rumbas())
        } else {
            JMERulesetItem::Display(self.to_rumbas())
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMEAccuracyInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct JMEAccuracy {
    /// The rule to use to compare the student’s answer with the correct answer.
    checking_type: CheckingType,
    /// The minimum and maximum value sample points can take [minimum, maximum]
    checking_range: [f64; 2], // TODO: seperate (flattened) struct for vset items & checking items etc?
    /// The number of comparisons to make between the student’s answer and the correct answer.
    points_to_check: usize,
    /// If the comparison fails this many times or more, the student’s answer is marked as wrong.
    max_failures: f64,
}

impl ToNumbas<numbas::question::part::jme::JMEAccuracy> for JMEAccuracy {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::jme::JMEAccuracy {
        numbas::question::part::jme::JMEAccuracy {
            checking_type: self.checking_type.to_numbas(locale),
            failure_rate: self.max_failures.to_numbas(locale),
            vset_range: self.checking_range.to_numbas(locale),
            vset_range_points: self.points_to_check.to_numbas(locale),
        }
    }
}

impl ToRumbas<JMEAccuracy> for numbas::question::part::jme::JMEAccuracy {
    fn to_rumbas(&self) -> JMEAccuracy {
        JMEAccuracy {
            checking_type: self.checking_type.to_rumbas(),
            max_failures: self.failure_rate.to_rumbas(),
            checking_range: [self.vset_range[0].0, self.vset_range[1].0].to_rumbas(),
            points_to_check: self.vset_range_points.0.to_rumbas(),
        }
    }
}
