use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::convert::TryInto;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
pub enum AnswerSimplificationType {
    Rule(AnswerSimplificationRule),
    DisplayOption(AnswerSimplificationDisplayOption),
    Ruleset(String),
}

impl std::convert::TryFrom<&str> for AnswerSimplificationType {
    type Error = String;
    fn try_from(whole_item: &str) -> Result<Self, Self::Error> {
        let rule: Result<AnswerSimplificationRule, _> = whole_item.try_into();
        if let Ok(r) = rule {
            Ok(Self::Rule(r))
        } else {
            let display_option: Result<AnswerSimplificationDisplayOption, _> =
                whole_item.try_into();
            if let Ok(d) = display_option {
                Ok(Self::DisplayOption(d))
            } else {
                Err(whole_item.to_string())
            }
        }
    }
}

impl std::fmt::Display for AnswerSimplificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(r) => r.fmt(f),
            Self::DisplayOption(d) => d.fmt(f),
            Self::Ruleset(ruleset) => write!(f, "{}", ruleset),
        }
    }
}

impl std::convert::From<AnswerSimplificationType> for String {
    fn from(a: AnswerSimplificationType) -> String {
        format!("{}", a)
    }
}

macro_rules! create_simplification {
    (
        $(#[$outer:meta])*
        pub enum $enum: ident {
            $(
                $(#[$inner:meta])*
                $variant: ident: $name: literal: $lowercase_name: literal
            ),+
        }
    ) => {
 $(
            #[$outer]
        )*
        #[skip_serializing_none]
        #[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
        #[serde(try_from = "&str")]
        #[serde(into = "String")]
        pub enum $enum {
            $(
                $(
                    #[$inner]
                )*
                $variant(bool)
            ),*
        }

        impl std::convert::TryFrom<&str> for $enum {
            type Error = String;
            fn try_from(whole_item: &str) -> Result<Self, Self::Error> {
                $(
                    assert_eq!($lowercase_name.to_string(), $name.to_lowercase());
                )*
                let (item, value) = if whole_item.starts_with('!') {
                    let mut chars = whole_item.chars();
                    chars.next();
                    (chars.as_str(), false)
                } else {
                    (whole_item, true)
                };
                match item.to_lowercase().as_ref() {
                    $(
                        $lowercase_name => Ok(Self::$variant(value)),
                    )*
                    _ => {
                        Err(format!("Unknown answer simplification type {}", item))
                    }
                }
            }
        }

        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                    $(
                        Self::$variant(true) => $name.to_string(),
                        Self::$variant(false) => format!("!{}", $name)
                    ),*
                    }
                )
            }
        }

        impl std::convert::From<$enum> for String {
            fn from(a: $enum) -> String {
                format!("{}", a)
            }
        }


    };
}

create_simplification! {
    /// See https://docs.numbas.org.uk/en/latest/simplification.html#id1 for examples
    pub enum AnswerSimplificationRule {
        /// To turn all built-in rules on, use the name all. To turn all built-in rules off, use !all.
        /// Note: Because they can conflict with other rules, the canonicalOrder and expandBrackets rules are not included in all. You must include them separately.
        All: "all": "all",
        /// Basic simplifications like unary plus removal etc
        Basic: "basic": "basic",
        /// Cancel products of 1
        CancelUnitFactors: "unitFactor": "unitfactor",
        /// Cancel exponents of 1
        CancelUnitPowers: "unitPower": "unitpower",
        /// Cancel fractions with denominator 1
        CancelUnitDenominators: "unitDenominator": "unitdenominator",
        /// Cancel products of zero to zero
        CancelZeroFactors: "zeroFactor": "zerofactor",
        /// Omit zero terms
        OmitZeroTerms: "zeroTerm": "zeroterm",
        /// Cancel exponents of 0
        CancelZeroPowers: "zeroPower" : "zeropower",
        /// Rearrange expressions so they don’t start with a unary minus
        NoLeadingMinus: "noLeadingMinus": "noleadingminus",
        /// Collect together numerical (as opposed to variable) products and sums.
        CollectNumbers: "collectNumbers": "collectnumbers",
        /// Cancel fractions to lowest form.
        Fractions: "simplifyFractions": "simplifyfractions",
        /// Cancel any power of zero
        CancelPowersWithBaseZero: "zeroBase": "zerobase",
        /// Numbers go to the left of multiplications
        ConstantsFirst: "constantsFirst": "constantsfirst",
        /// Collect products of square roots
        CollectSqrtProducts: "sqrtProduct": "sqrtproduct",
        /// Collect fractions of square roots
        CollectSqrtDivisions: "sqrtDivision": "sqrtdivision",
        /// Cancel square roots of squares, and squares of square roots
        CancelSqrtSquares: "sqrtSquare": "sqrtsquare",
        /// Simplify some trigonometric identities
        Trigonometric: "trig": "trig",
        /// Evaluate powers of numbers.
        EvaluatePowersOfNumbers: "otherNumbers": "othernumbers",
        /// Collect together and cancel terms. Like collectNumbers, but for any kind of term.
        CollectTerms: "cancelTerms": "cancelterms",
        /// Collect together powers of common factors.
        CollectPowersOfCommonFactors: "cancelFactors": "cancelfactors",
        /// Collect together fractions over the same denominator.
        CollectLikeFractions: "collectLikeFractions": "collectlikefractions",
        /// earrange the expression into a “canonical” order, using canonical_compare().
        /// Note: This rule can not be used at the same time as noLeadingMinus - it can lead to an infinite loop.
        CanonicalOrder: "canonicalOrder": "canonicalorder",
        /// Expand out products of sums. (distributivity)
        ExpandBrackets: "expandBrackets": "expandbrackets"
    }
}

create_simplification! {
    /// Se https://docs.numbas.org.uk/en/latest/simplification.html#display-options for examples
    pub enum AnswerSimplificationDisplayOption {
        /// This rule doesn’t rewrite expressions, but tells the maths renderer that you’d like non-integer numbers to be displayed as fractions instead of decimals.
        Fractions: "fractionNumbers": "fractionnumbers",
        /// Improper fractions (with numerator larger than the denominator) are displayed in mixed form, as an integer next to a proper fraction.
        MixedFractions: "mixedFractions": "mixedfractions",
        /// Fractions are displayed on a single line, with a slash between the numerator and denominator.
        FlatFractions: "flatFractions": "flatfractions",
        /// This rule doesn’t rewrite expressions, but tells the maths renderer that you’d like vectors to be rendered as rows instead of columns.
        RowVector: "rowVector": "rowvector",
        /// The multiplication symbol is always included between multiplicands.
        AlwaysShowMultiplicationSign: "alwaysTimes": "alwaystimes",
        /// Use a dot for the multiplication symbol instead of a cross.
        DotAsMultiplicationSign: "timesDot": "timesdot",
        /// Matrices are rendered without parentheses.
        MatricesWithoutParentheses: "bareMatrices" : "barematrices"
    }
}
