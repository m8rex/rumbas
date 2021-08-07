#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl std::convert::From<String> for Ident {
    fn from(s: String) -> Self {
        let mut items = s.split(":");
        let name = items.next().unwrap().to_owned();
        let annotations = items.map(|s| s.to_owned()).collect::<Vec<_>>();
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
    /// Matches a range-
    Range(isize, isize),
    /// Matches a sum of two expressions, e.g. `e1 + e2`
    Sum(Box<Expr>, Box<Expr>),
    /// Matches a difference of two expressions, e.g. `e1 - e2`
    Diff(Box<Expr>, Box<Expr>),
    /// Matches a product of two expressions, e.g. `e1 * e2`
    Product(Box<Expr>, Box<Expr>),
    /// Matches a division of two expressions, e.g. `e1 / e2`
    Division(Box<Expr>, Box<Expr>),
    /// Matches a power of two expressions, e.g. `e1 ^ e2`
    Power(Box<Expr>, Box<Expr>),
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

#[cfg(test)]
mod test {
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
                            Box::new(Product(
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
                        Box::new(Sum(
                            Box::new(Power(Box::new(Int(9)), Box::new(Int(10)))),
                            Box::new(Product(Box::new(Int(8)), Box::new(Int(5))))
                        )),
                        Box::new(Division(Box::new(Int(6)), Box::new(Int(10))))
                    ))
                )),
                Box::new(Bool(false))
            )
        );
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
    }

    #[test]
    fn ast_implicit_multiplication() {
        let input = "(b+2)(a+1)";
        let pairs = parse(input).unwrap();
        let ast = consume_outer_expression(pairs).unwrap();

        assert_eq!(
            ast,
            Product(
                Box::new(Sum(
                    Box::new(Ident(Ident {
                        name: "b".to_string(),
                        annotations: vec![]
                    })),
                    Box::new(Int(2))
                )),
                Box::new(Sum(
                    Box::new(Ident(Ident {
                        name: "a".to_string(),
                        annotations: vec![]
                    })),
                    Box::new(Int(1))
                ))
            )
        );

        // TODO
        /*assert!(parse("(a+1)2").is_ok());
        assert!(parse("(x+y)z").is_ok());
        assert!(parse("2x").is_ok());
        assert!(parse("x y").is_ok());*/
    }

    #[test]
    fn ast_implicit_multiplication_precedence() {
        let input = "(b+2)(a+1)^2";
        let pairs = parse(input).unwrap();
        let ast = consume_outer_expression(pairs).unwrap();

        assert_eq!(
            ast,
            Product(
                Box::new(Sum(
                    Box::new(Ident(Ident {
                        name: "b".to_string(),
                        annotations: vec![]
                    })),
                    Box::new(Int(2))
                )),
                Box::new(Power(
                    Box::new(Sum(
                        Box::new(Ident(Ident {
                            name: "a".to_string(),
                            annotations: vec![]
                        })),
                        Box::new(Int(1))
                    )),
                    Box::new(Int(2))
                ))
            )
        );
    }
}
