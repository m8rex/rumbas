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
#[serde(rename_all = "snake_case")]
pub enum BuiltinFunctions {
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
    /// Matches a range-
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
