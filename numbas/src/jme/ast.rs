use crate::jme::builtin_functions::BuiltinFunctions;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Except,
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
    IsA,
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
            "isa" => RelationalOperator::IsA,
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
        match &s[..] {
            "and" | "&&" | "&" => LogicalOperator::And,
            "or" => LogicalOperator::Or,
            "xor" => LogicalOperator::Xor,
            "implies" => LogicalOperator::Implies,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RangeOperator {
    Create,
    Step,
}

impl std::convert::From<String> for RangeOperator {
    fn from(s: String) -> Self {
        match &s[..] {
            ".." => RangeOperator::Create,
            "#" => RangeOperator::Step,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefixOperator {
    Not,
    Minus,
}

impl std::convert::From<String> for PrefixOperator {
    fn from(s: String) -> Self {
        match &s[..] {
            "!" | "not" => PrefixOperator::Not,
            "-" => PrefixOperator::Minus,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident {
    name: String,
    annotations: Vec<String>, // TODO: enum value
}

impl Ident {
    pub fn is_builtin_funtion(&self) -> bool {
        BuiltinFunctions::get(&self.name[..]).is_some()
    }
}

impl std::convert::From<String> for Ident {
    fn from(s: String) -> Self {
        let items = s.split(':');
        let mut annotations = items.map(|s| s.to_owned()).collect::<Vec<_>>();
        let name = annotations.pop().expect("impossible");
        Ident { name, annotations }
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
    /// Matches a range operation-
    Range(RangeOperator, Box<Expr>, Box<Expr>),
    /// Matches an arithmetic operation of two expressions`
    Arithmetic(ArithmeticOperator, Box<Expr>, Box<Expr>),
    /// Matches an identifier
    Ident(Ident),
    /// Matches a constant
    Constant(Ident),
    /// Matches a relationship between two expressions`
    Relation(RelationalOperator, Box<Expr>, Box<Expr>),
    /// Matches a logical operation between two expressions`
    Logic(LogicalOperator, Box<Expr>, Box<Expr>),
    /// Matches a list
    List(Vec<Expr>),
    /// Matches a dictionary
    Dictionary(Vec<(Expr, Expr)>),
    /// Matches a function application
    FunctionApplication(Ident, Vec<Expr>),
    /// Matches a prefixed expression
    Prefix(PrefixOperator, Box<Expr>),
    /// Matches a faculty expression
    Faculty(Box<Expr>),
    /// Matches an indexation expression
    Indexation(Box<Expr>),
    /// Matches a cast expression
    Cast(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExprValidationError {
    UnknownFunction(Ident),
    UnknownVariable(Ident),
}

impl Expr {
    pub fn validate(&self) -> Vec<ExprValidationError> {
        match self {
            Expr::Str(_) => vec![],
            Expr::Int(_) => vec![],
            Expr::Float(_, _) => vec![],
            Expr::Bool(_) => vec![],
            Expr::Range(_, e1, e2) => e1
                .validate()
                .into_iter()
                .chain(e2.validate().into_iter())
                .collect(),
            Expr::Arithmetic(_, e1, e2) => e1
                .validate()
                .into_iter()
                .chain(e2.validate().into_iter())
                .collect(),
            Expr::Ident(_) => vec![], // TODO: check if part of variable list
            Expr::Constant(_) => vec![],
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
            Expr::List(es) => es.iter().flat_map(|e| e.validate()).collect::<Vec<_>>(),
            Expr::Dictionary(es) => es
                .iter()
                .flat_map(|(k, v)| vec![k, v])
                .flat_map(|e| e.validate())
                .collect::<Vec<_>>(),
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
            Expr::Prefix(_, e1) => e1.validate(),
            Expr::Faculty(e1) => e1.validate(),
            Expr::Indexation(e1) => e1.validate(),
            Expr::Cast(e1, e2) => e1
                .validate()
                .into_iter()
                .chain(e2.validate().into_iter())
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArithmeticOperator::*;
    use super::Expr::*;
    use super::ExprValidationError::*;
    use super::Ident;
    use super::LogicalOperator::*;
    use super::RangeOperator::*;
    use super::RelationalOperator::*;
    use crate::jme::parser::consume_one_expression;
    use crate::jme::parser::parse_as_jme;
    use serde::{Deserialize, Serialize};
    use std::fmt::Write;

    #[test]
    fn ast() {
        let input = "a * 7 > 5 and true or 9^10 + 8 * 5 < 6 / 10 && false";

        let pairs = parse_as_jme(input).unwrap();
        let ast = consume_one_expression(pairs).unwrap();

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

        let pairs = parse_as_jme(input).unwrap();
        let ast = consume_one_expression(pairs).unwrap();

        assert_eq!(
            ast,
            FunctionApplication(
                Ident {
                    name: "repeat".to_string(),
                    annotations: vec![]
                },
                vec![
                    FunctionApplication(
                        Ident {
                            name: "random".to_string(),
                            annotations: vec![]
                        },
                        vec![Range(Create, Box::new(Int(1)), Box::new(Int(4)))]
                    ),
                    Int(5)
                ]
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
            let pairs = parse_as_jme(implicit).unwrap();
            let ast = consume_one_expression(pairs).unwrap();
            let explicit_ast = consume_one_expression(parse_as_jme(explicit).unwrap()).unwrap();

            assert_eq!(ast, explicit_ast);
            assert_eq!(ast.validate(), vec![]);
        }
    }

    #[test]
    fn ast_implicit_multiplication_precedence() {
        let explicit_ast = consume_one_expression(parse_as_jme("(b+2)*(a+1)^2").unwrap()).unwrap();
        let input = "(b+2)(a+1)^2";
        let pairs = parse_as_jme(input).unwrap();
        let ast = consume_one_expression(pairs).unwrap();

        assert_eq!(ast, explicit_ast);
        assert_eq!(ast.validate(), vec![]);
    }

    #[test]
    fn ast_list() {
        let input = "[1,2,3]+4";
        let pairs = parse_as_jme(input).unwrap();
        let ast = consume_one_expression(pairs).unwrap();

        assert_eq!(
            ast,
            Arithmetic(
                Add,
                Box::new(List(vec![Int(1), Int(2), Int(3)])),
                Box::new(Int(4))
            )
        );
        assert_eq!(ast.validate(), vec![]);
    }

    #[test]
    fn ast_non_ascii_ident() {
        let input = "vec:ε";
        let pairs = parse_as_jme(input).unwrap();
        let ast = consume_one_expression(pairs).unwrap();

        assert_eq!(
            ast,
            Ident(Ident {
                name: "ε".to_owned(),
                annotations: vec!["vec".to_owned()]
            })
        );
        assert_eq!(ast.validate(), vec![]);
    }

    #[derive(Serialize, Deserialize)]
    struct DocTest {
        name: String,
        fns: Vec<DocTestFn>,
    }

    #[derive(Serialize, Deserialize)]
    struct DocTestFn {
        name: String,
        examples: Vec<DocTestFnExample>,
    }

    #[derive(Serialize, Deserialize)]
    struct DocTestFnExample {
        r#in: String,
        out: String,
    }

    #[test]
    fn numbas_doc_tests() {
        let mut total_tests = 0;
        let mut passed_parse_as_jme_tests = 0;
        let mut passed_validate_tests = 0;
        let mut output = String::new();
        let doc_tests_json = include_str!("numbas-jme-doc-tests.json");
        let doc_tests: Vec<DocTest> =
            serde_json::from_str(doc_tests_json).expect("it to parse_as_jme the docstest json");
        for test in doc_tests.into_iter() {
            for r#fn in test.fns {
                for example in r#fn.examples {
                    total_tests += 1;
                    let use_hacky_f_g_fix = vec![
                        "canonical_compare(f(y),g(x))",
                        "canonical_compare(f(x),g(x))",
                    ]
                    .contains(&&example.r#in[..]);

                    let pairs_res = parse_as_jme(&example.r#in[..]);
                    let mut failed_parsing = true;
                    let mut failed_validating = true;
                    if let Ok(pairs) = pairs_res.clone() {
                        let ast_res = consume_one_expression(pairs);
                        if let Ok(ast) = ast_res {
                            failed_parsing = false;
                            let validation_errors = ast.validate();
                            if validation_errors.is_empty()
                                || (use_hacky_f_g_fix
                                    && validation_errors
                                        == vec![
                                            UnknownFunction(Ident {
                                                name: "f".to_owned(),
                                                annotations: vec![],
                                            }),
                                            UnknownFunction(Ident {
                                                name: "g".to_owned(),
                                                annotations: vec![],
                                            }),
                                        ])
                            {
                                failed_validating = false;
                            } else {
                                writeln!(
                                    output,
                                    "VALIDATE: {}.{}.{}: {:?}",
                                    test.name,
                                    r#fn.name,
                                    example.r#in,
                                    validation_errors
                                        .iter()
                                        .map(|v| format!("{:?}", v))
                                        .collect::<Vec<_>>()
                                        .join(" & ")
                                )
                                .expect("Error occured while trying to write to string");
                            }
                        }
                    }
                    if failed_parsing {
                        writeln!(
                            output,
                            "parse_as_jme: {}.{}.{}:", //" {:?}",
                            test.name,
                            r#fn.name,
                            example.r#in //, pairs_res
                        )
                        .expect("Error occured while trying to write to string");
                    } else {
                        passed_parse_as_jme_tests += 1;
                    }
                    if !failed_validating && !failed_parsing {
                        passed_validate_tests += 1;
                    }
                }
            }
        }
        println!("{}", output);
        assert_eq!(total_tests, passed_parse_as_jme_tests);
        assert_eq!(total_tests, passed_validate_tests);
    }
}
