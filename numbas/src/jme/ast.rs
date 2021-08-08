use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
    In,
    Divides,
}

impl std::convert::From<String> for RelationalOperator {
    fn from(s: String) -> Self {
        match &s[..] {
            "<" => RelationalOperator::LessThan,
            "<=" => RelationalOperator::LessThanOrEqual,
            ">" => RelationalOperator::GreaterThan,
            ">=" => RelationalOperator::GreaterThanOrEqual,
            "=" => RelationalOperator::Equals,
            "<>" => RelationalOperator::NotEquals,
            "in" => RelationalOperator::In,
            "|" => RelationalOperator::Divides,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
    Xor,
    Implies,
}

impl std::convert::From<String> for LogicalOperator {
    fn from(s: String) -> Self {
        println!("{}", s);
        match &s[..] {
            "and" | "&&" | "&" => LogicalOperator::And,
            "or" => LogicalOperator::Or,
            "xor" => LogicalOperator::Xor,
            "implies" => LogicalOperator::Implies,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident {
    name: String,
    annotations: Vec<String>, // TODO: enu value
}

impl Ident {
    pub fn is_builtin_funtion(&self) -> bool {
        BuiltinFunctions::get(&self.name[..]).is_some()
    }
}

impl std::convert::From<String> for Ident {
    fn from(s: String) -> Self {
        let mut items = s.split(":");
        let name = items.next().unwrap().to_owned();
        let annotations = items.map(|s| s.to_owned()).collect::<Vec<_>>();
        Ident { name, annotations }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuiltinFunctions {
    /// Synonym of e^x
    Exp,

    /// Construct a decimal value. Any string accepted by Decimal.js is accepted.
    Decimal,
    /// Convert n to a rational nubmer, taking an approximation when necessary.
    Rational,
    /// Convert n to an integer, rounding to the nearest integer.
    Int,
    #[serde(alias = "len")]
    #[serde(alias = "length")]
    /// Absolute value, or modulus.
    Abs, // TODO: maybe split these?
    /// Argument of a complex number.
    Arg,
    /// Real part of a complex number.
    Re,
    /// Imaginary part of a complex number.
    Im,
    /// Complex conjugate.
    Conj,
    /// Returns true if x is an integer - that is, it is real and has no fractional part.
    IsInt,
    /// Returns true when n is exactly 0.
    IsZero,
    #[serde(alias = "sqr")]
    /// Square root of a number.
    Sqrt,
    /// nth root of x
    Root,
    /// Natural Logarithm
    Ln,
    /// Logarithm with base b, or base 10 if b is not given.
    Log,
    /// Convert radians to degrees.
    Degrees,
    /// Convert degrees to radians.
    Radians,
    /// Sign of a number, 1 if positive, -1 if negative and 0 when 0.
    #[serde(alias = "sgn")]
    Sign,
    /// Greatest of the given numbers.
    Max,
    /// Least of the given numbers.
    Min,
    /// Return the point nearest to x in the interval [a,b].
    /// Equivalent to max(a,min(x,b)).
    Clamp,
    /// Round n to d decimal places. On matrices and vectors, this rounds each element independently.
    PrecRound,
    /// Round n to f significant figures. On matrices and vectors, this rounds each element independently.
    SigRound,
    /// Returns true if b−t<=a<=b+t
    WithIntolerance,
    /// Round n to d decimal places and return a string, padding with zeros if necessary.
    /// If style is given, the number is rendered using the given notation style. See the page on Number notation for more on notation styles.
    DPFormat,
    /// Assuming n is a string representing a number, return the number of decimal places used. The string is passed through cleannumber() first.
    CountDP,
    /// Round n to d significant figures and return a string, padding with zeros if necessary.
    SigFormat,
    /// Assuming n is a string representing a number, return the number of significant figures. The string is passed through cleannumber() first.
    CountSigFids,
    /// Returns true if str is a string representing a number given to the desired number of decimal places or significant figures.
    /// precisionType is either "dp", for decimal places, or "sigfig", for significant figures.
    /// If strict is true, then trailing zeroes must be included.
    ToGivenPrecision,
    /// Round a to the nearest multiple of b.
    ToNearest,
    /// Render the number n using the given number notation style.
    /// See the page on Number notation for more on notation styles.
    FormatNumber,
    /// Return a LaTeX string representing the given number in scientific notation, a×10^b.
    /// This function exists because scientific notation may use superscripts, which aren’t easily typeset in plain text.
    ScientificNumberLaTeX,
    /// Return an HTML element representing the given number in scientific notation, a×10^b.
    /// This function exists because scientific notation may use superscripts, which aren’t easily typeset in plain text.
    ScientificNumberHTML,
    /// Clean a string potentially representing a number. Remove space, and then try to identify a notation style, and rewrite to the plain-en style.
    /// styles is a list of notation styles. If styles is given, str will be tested against the given styles. If it matches, the string will be rewritten using the matched integer and decimal parts, with punctuation removed and the decimal point changed to a dot.
    CleanNumber,
    /// Try to match a string representing a number in any of the given styles at the start of the given string, and return both the matched text and a corresponding number value.
    MatchNumber,
    /// Parse a string representing a number written in the given style.
    /// If a list of styles is given, the first that accepts the given string is used.
    /// See the page on Number notation for more on notation styles.
    ParseNumber,
    #[serde(rename = "parsenumber_or_fraction")]
    /// Works the same as parsenumber(), but also accepts strings of the form number/number, which it interprets as fractions.
    ParseNumberOrFraction,
    /// Parse a string representing a number written in the given style, and return a decimal value.
    /// If a list of styles is given, the first that accepts the given string is used.
    /// See the page on Number notation for more on notation styles.
    ParseDecimal,
    #[serde(rename = "parsedecimal_or_fraction")]
    /// Works the same as parsedecimal(), but also accepts strings of the form number/number, which it interprets as fractions.
    ParseDecimalOrFraction,
    /// Is n the “not a number” value, NaN?
    IsNan,

    /// Sine
    Sin,
    /// Cosine
    Cos,
    /// Tangent: tan(x) = sin(x)/cos(x)
    Tan,
    /// Cosecant: = 1 / sin(x)
    Cosec,
    /// Secant: = 1 / cos(x)
    Sec,
    /// Cotangent: = 1 / tan(x)
    Cot,
    /// Inverse of sin
    ArcSin,
    /// Inverse of cos
    ArcCos,
    /// Inverse of tan
    ArcTan,
    /// The angle in radians between the positive x-axis and the line through the origin and (x,y). This is often equivalent to arctan(y/x), except when x<0, when pi is either added or subtracted from the result.
    ATan2,
    /// Hyperbolic sine
    Sinh,
    /// Hyperbolic cosine
    Cosh,
    /// Hyperbolic tangent
    Tanh,
    /// Hyperbolic cosecant
    Cosech,
    /// Hyperbolic secant
    Sech,
    /// Hyperbolic cotangent
    Coth,
    /// Inverse of sinh
    ArcSinh,
    /// Inverse of cosh
    ArcCosh,
    /// Inverse of tanh
    ArcTanh,

    /// Factorise n. Returns the exponents of the prime factorisation of n as a list.
    Factorise,
    /// Gamma function
    Gamma,
    /// Round up to the nearest integer. When x is complex, each component is rounded separately.
    Ceil,
    /// Round down to the nearest integer. When x is complex, each component is rounded separately.
    Floor,
    /// Round to the nearest integer. 0.5 is rounded up.
    Round,
    /// If x is positive, round down to the nearest integer; if it is negative, round up to the nearest integer.
    Trunc,
    /// Fractional part of a number. Equivalent to x-trunc(x).
    Fract,
    #[serde(rename = "rational_approximation")]
    /// Compute a rational approximation to the given number by computing terms of its continued fraction, returning the numerator and denominator separately. The approximation will be within e−accuracy of the true value; the default value for accuracy is 15.
    RationalApproximation,
    /// Modulo; remainder after integral division, i.e. a mod b.
    Mod,
    /// Count permutations
    Perm,
    /// Count combinations
    Comb,
    #[serde(rename = "gcf")]
    /// Greatest common divisor of integers a and b
    GCD,
    #[serde(rename = "gcd_without_pi_or_i")]
    /// Take out factors of π or i from a and b before computing their greatest common denominator.
    GCDWithoutPIorI,
    /// Are a and b coprime? True if their gcd() is 1, or if either of a or b is not an integer.
    CoPrime,
    /// Lowest common multiple of integers a and b. Can be used with any number of arguments; it returns the lowest common multiple of all the arguments.
    LCM,

    /// Create a vector with given components. Alternately, you can create a vector from a single list of numbers.
    Vector,
    /// Create a matrix with given rows, which should be either vectors or lists of numbers. Or, you can pass in a single list of lists of numbers.
    Matrix,
    /// Identity matrix with n rows and columns.
    Id,
    /// The number of rows in the given matrix
    NumRows,
    /// The number of columns in the given matrix
    NumColumns,
    /// Create a row vector (1×n matrix) with the given components. Alternately, you can create a row vector from a single list of numbers.
    RowVector,
    /// Dot (scalar) product. Inputs can be vectors or column matrices.
    Dot,
    /// Cross product. Inputs can be vectors or column matrices.
    Cross,
    /// Angle between vectors a and b, in radians. Returns 0 if either a or b has length 0.
    Angle,
    #[serde(rename = "is_zero")]
    /// Returns true if every component of the vector x is zero.
    IsZeroVector,
    /// Determinant of a matrix. Throws an error if used on anything larger than a 3×3 matrix.
    Det, // Why not for larger matrices?
    /// Matrix transpose.
    Transpose,
    /// Calculate the sum of all the cells in a matrix.
    SumCells,

    Random,
    Repeat,
}

impl BuiltinFunctions {
    fn get(s: &str) -> Option<Self> {
        serde_plain::from_str(s).ok()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    /// Matches an exact string, e.g. `"a"`
    Str(String),
    /// Matches an integer,
    Int(isize),
    /// Matches a broken number
    Float(isize, String),
    /// Matches a boolean
    Bool(bool),
    /// Matches a range (second index is not included)-
    Range(isize, isize),
    /// Matches an arithmetic operation of two expressions`
    Arithmetic(ArithmeticOperator, Box<Expr>, Box<Expr>),
    /// Matches an identifier
    Ident(Ident),
    /// Matches a relationship between two expressions`
    Relation(RelationalOperator, Box<Expr>, Box<Expr>),
    /// Matches a logical operation between two expressions`
    Logic(LogicalOperator, Box<Expr>, Box<Expr>),
    /// Matches a function application
    FunctionApplication(Ident, Box<Vec<Expr>>),
    /// Matches a not expression
    Not(Box<Expr>),
    /// Matches a faculty expression
    Faculty(Box<Expr>),
    // TODO: collection
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExprValidationError {
    UnknownFunction(Ident),
    UnknownVariable(Ident),
}

impl Expr {
    fn validate(&self) -> Vec<ExprValidationError> {
        match self {
            Expr::Str(_) => vec![],
            Expr::Int(_) => vec![],
            Expr::Float(_, _) => vec![],
            Expr::Bool(_) => vec![],
            Expr::Range(_, _) => vec![], // TODO: if range is changed to expr, expr, to recursive call
            Expr::Arithmetic(_, e1, e2) => e1
                .validate()
                .into_iter()
                .chain(e2.validate().into_iter())
                .collect(),
            Expr::Ident(_) => vec![], // TODO: check if part of variable list
            Expr::Relation(_, e1, e2) => e1
                .validate()
                .into_iter()
                .chain(e2.validate().into_iter())
                .collect(),
            Expr::Logic(_, e1, e2) => e1
                .validate()
                .into_iter()
                .chain(e2.validate().into_iter())
                .collect(),
            Expr::FunctionApplication(ident, es) => {
                let base = es.iter().flat_map(|e| e.validate()).collect::<Vec<_>>();
                if ident.is_builtin_funtion() {
                    base
                } else {
                    base.into_iter()
                        .chain(
                            vec![ExprValidationError::UnknownFunction(ident.clone())].into_iter(),
                        )
                        .collect()
                }
            }
            Expr::Not(e1) => e1.validate(),
            Expr::Faculty(e1) => e1.validate(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArithmeticOperator::*;
    use super::Expr::*;
    use super::Ident;
    use super::LogicalOperator::*;
    use super::RelationalOperator::*;
    use crate::jme::parser::consume_outer_expression;
    use crate::jme::parser::parse;

    #[test]
    fn ast() {
        let input = "a * 7 > 5 and true or 9^10 + 8 * 5 < 6 / 10 && false";

        let pairs = parse(input).unwrap();
        let ast = consume_outer_expression(pairs).unwrap();

        assert_eq!(
            ast,
            Logic(
                And,
                Box::new(Logic(
                    Or,
                    Box::new(Logic(
                        And,
                        Box::new(Relation(
                            GreaterThan,
                            Box::new(Arithmetic(
                                Multiply,
                                Box::new(Ident(Ident {
                                    name: "a".to_string(),
                                    annotations: vec![]
                                })),
                                Box::new(Int(7))
                            )),
                            Box::new(Int(5))
                        )),
                        Box::new(Bool(true))
                    )),
                    Box::new(Relation(
                        LessThan,
                        Box::new(Arithmetic(
                            Add,
                            Box::new(Arithmetic(Power, Box::new(Int(9)), Box::new(Int(10)))),
                            Box::new(Arithmetic(Multiply, Box::new(Int(8)), Box::new(Int(5))))
                        )),
                        Box::new(Arithmetic(Divide, Box::new(Int(6)), Box::new(Int(10))))
                    ))
                )),
                Box::new(Bool(false))
            )
        );
        assert_eq!(ast.validate(), vec![]);
    }

    #[test]
    fn ast_range() {
        let input = "repeat(random(1..4),5)";

        let pairs = parse(input).unwrap();
        let ast = consume_outer_expression(pairs).unwrap();

        assert_eq!(
            ast,
            FunctionApplication(
                Ident {
                    name: "repeat".to_string(),
                    annotations: vec![]
                },
                Box::new(vec![
                    FunctionApplication(
                        Ident {
                            name: "random".to_string(),
                            annotations: vec![]
                        },
                        Box::new(vec![Range(1, 4)])
                    ),
                    Int(5)
                ])
            )
        );
        assert_eq!(ast.validate(), vec![]);
    }

    #[test]
    fn ast_implicit_multiplication() {
        for (implicit, explicit) in vec![
            ("(b+2)(a+1)", "(b+2)*(a+1)"),
            ("(a+1)2", "(a+1)*2"),
            ("(x+y)z", "(x+y)*z"),
            ("2x", "2*x"),
            ("x y", "x*y"),
        ] {
            println!("Handling {}", implicit);
            let pairs = parse(implicit).unwrap();
            let ast = consume_outer_expression(pairs).unwrap();
            let explicit_ast = consume_outer_expression(parse(explicit).unwrap()).unwrap();

            assert_eq!(ast, explicit_ast);
            assert_eq!(ast.validate(), vec![]);
        }
    }

    #[test]
    fn ast_implicit_multiplication_precedence() {
        let explicit_ast = consume_outer_expression(parse("(b+2)*(a+1)^2").unwrap()).unwrap();
        let input = "(b+2)(a+1)^2";
        let pairs = parse(input).unwrap();
        let ast = consume_outer_expression(pairs).unwrap();

        assert_eq!(ast, explicit_ast);
        assert_eq!(ast.validate(), vec![]);
    }
}
