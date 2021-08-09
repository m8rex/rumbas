use crate::jme::ast;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::{Parser, Span};
use std::iter::Peekable;

#[derive(Parser)]
#[grammar = "jme/jme.pest"]
struct JMEParser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParserNode<'i> {
    pub expr: ParserExpr<'i>,
    pub span: Span<'i>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParserExpr<'i> {
    Str(String),
    Int(isize),
    Float(isize, String),
    Bool(bool),
    Range(isize, isize),
    Arithmetic(
        ast::ArithmeticOperator,
        Box<ParserNode<'i>>,
        Box<ParserNode<'i>>,
    ),
    AnnotatedIdent(String),
    Relation(String, Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    Logic(String, Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    FunctionApplication(String, Box<Vec<ParserNode<'i>>>),
    Not(Box<ParserNode<'i>>),
    Faculty(Box<ParserNode<'i>>),
}

impl<'i> std::convert::From<ParserNode<'i>> for ast::Expr {
    fn from(node: ParserNode<'i>) -> ast::Expr {
        node.expr.into()
    }
}
impl<'i> std::convert::From<ParserExpr<'i>> for ast::Expr {
    fn from(expr: ParserExpr<'i>) -> ast::Expr {
        match expr {
            ParserExpr::Str(s) => ast::Expr::Str(s),
            ParserExpr::Int(i) => ast::Expr::Int(i),
            ParserExpr::Float(i, s) => ast::Expr::Float(i, s),
            ParserExpr::Bool(b) => ast::Expr::Bool(b),
            ParserExpr::Range(f, t) => ast::Expr::Range(f, t),
            ParserExpr::Arithmetic(a, n1, n2) => {
                ast::Expr::Arithmetic(a, Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::AnnotatedIdent(s) => ast::Expr::Ident(s.into()),
            ParserExpr::Relation(s, n1, n2) => {
                ast::Expr::Relation(s.into(), Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::Logic(s, n1, n2) => {
                ast::Expr::Logic(s.into(), Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::FunctionApplication(s, n1) => ast::Expr::FunctionApplication(
                s.into(),
                Box::new(n1.into_iter().map(|n| n.into()).collect()),
            ),
            ParserExpr::Not(n) => ast::Expr::Not(Box::new((*n).into())),
            ParserExpr::Faculty(n) => ast::Expr::Faculty(Box::new((*n).into())),
        }
    }
}

pub fn consume_outer_expression(pairs: Pairs<Rule>) -> Result<ast::Expr, Vec<Error<Rule>>> {
    let expression = consume_expression_with_spans(pairs)?;
    //let errors = validator::validate_ast(&rules);
    //if errors.is_empty() {
    Ok(expression.into())
    /*} else {
        Err(errors)
    }*/
}

fn consume_expression_with_spans(pairs: Pairs<Rule>) -> Result<ParserNode, Vec<Error<Rule>>> {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::logic_binary_operator, Assoc::Left),
        Operator::new(Rule::relational_operator, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left)
            | Operator::new(Rule::subtract, Assoc::Left)
            | Operator::new(Rule::except, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left)
            | Operator::new(Rule::implicit_multiplication_operator, Assoc::Left)
            | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    let expression = pairs
        .clone()
        .next()
        .unwrap()
        .into_inner()
        .filter(|pair| pair.as_rule() == Rule::expression)
        .next()
        .unwrap();
    consume_expression(expression.into_inner().peekable(), &climber)
}

fn consume_expression<'i>(
    pairs: Peekable<Pairs<'i, Rule>>,
    climber: &PrecClimber<Rule>,
) -> Result<ParserNode<'i>, Vec<Error<Rule>>> {
    println!("outer {:#?}", pairs);
    fn unaries<'i>(
        mut pairs: Peekable<Pairs<'i, Rule>>,
        climber: &PrecClimber<Rule>,
    ) -> Result<ParserNode<'i>, Vec<Error<Rule>>> {
        println!("unaries {:?}", pairs);
        let pair = pairs.next().unwrap();

        let node = match pair.as_rule() {
            Rule::not_operator => {
                let node = unaries(pairs, climber)?;
                let end = node.span.end_pos();

                ParserNode {
                    expr: ParserExpr::Not(Box::new(node)),
                    span: pair.as_span().start_pos().span(&end),
                }
            }
            other_rule => {
                println!("other {:#?}", other_rule);
                println!("other {:#?}", pair);
                let node = match other_rule {
                    Rule::expression => consume_expression(pair.into_inner().peekable(), climber)?,
                    Rule::ident => {
                        println!("ident {:?}", pair);
                        ParserNode {
                            expr: ParserExpr::AnnotatedIdent(pair.as_str().trim().to_owned()),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::string => {
                        let string =
                            unescape(pair.as_str().trim()).expect("incorrect string literal");
                        ParserNode {
                            expr: ParserExpr::Str(string[1..string.len() - 1].to_owned()),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::boolean => {
                        let b: bool = pair
                            .as_str()
                            .trim()
                            .parse()
                            .expect("incorrect bool literal");
                        ParserNode {
                            expr: ParserExpr::Bool(b),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::integer => {
                        let integer: isize = pair
                            .as_str()
                            .trim()
                            .parse()
                            .expect("incorrect integer literal");
                        ParserNode {
                            expr: ParserExpr::Int(integer),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::range => {
                        let mut pairs = pair.into_inner();
                        let pair = pairs.next().unwrap();
                        let start: isize = pair
                            .as_str()
                            .trim()
                            .parse()
                            .expect("incorrect integer start point of range");
                        //pairs.next().unwrap(); // ..
                        let pair = pairs.next().unwrap();
                        let end: isize = pair
                            .as_str()
                            .trim()
                            .parse()
                            .expect("incorrect integer end point of range");
                        ParserNode {
                            expr: ParserExpr::Range(start, end),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::broken_number => {
                        let mut pairs = pair.into_inner();
                        let pair = pairs.next().unwrap();
                        let integer: isize = pair
                            .as_str()
                            .trim()
                            .parse()
                            .expect("incorrect integer part of float literal");
                        let pair = pairs.next().unwrap();
                        let broken_part: String = pair.as_str().trim().to_owned();
                        ParserNode {
                            expr: ParserExpr::Float(integer, broken_part),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::function_application => {
                        let mut pairs = pair.into_inner();
                        println!("pairs {:#?}", pairs);
                        let pair = pairs.next().unwrap();
                        let ident = pair.as_str();
                        println!("ident {:?}", ident);
                        let start_pos = pair.clone().as_span().start_pos();
                        //pairs.next().unwrap(); // (
                        let pair = pairs.next().unwrap();
                        let end_pos = pair.as_span().end_pos();
                        let inner_pairs = pair.into_inner();
                        let mut arguments = Vec::new();
                        for p in inner_pairs.filter(|p| p.as_rule() == Rule::expression) {
                            arguments.push(consume_expression(p.into_inner().peekable(), climber)?);
                        }
                        println!("args {:?}", arguments);

                        ParserNode {
                            expr: ParserExpr::FunctionApplication(
                                ident.to_string(),
                                Box::new(arguments),
                            ),
                            span: start_pos.span(&end_pos),
                        }
                    }
                    /* Rule::implicit_multiplication_grouped => {
                        // TODO: this gives wrong precedence for (a)(b)^2
                        let span = pair.as_span();
                        let mut pairs = pair.into_inner();
                        let pair = pairs.next().unwrap();
                        let exp1 = consume_expression(pair.into_inner().peekable(), climber)?;
                        let pair = pairs.next().unwrap();
                        let exp2 = consume_expression(pair.into_inner().peekable(), climber)?;
                        ParserNode {
                            expr: ParserExpr::Product(Box::new(exp1), Box::new(exp2)),
                            span,
                        }
                    }
                    Rule::implicit_multiplication_ident => {
                        // TODO: this gives wrong precedence for (a)(b)^2
                        let span = pair.as_span();
                        let mut pairs = pair.into_inner();
                        let pair = pairs.next().unwrap();
                        let exp1 = consume_expression(pair.into_inner().peekable(), climber)?;
                        let pair = pairs.next().unwrap();
                        let exp2 = consume_expression(pair.into_inner().peekable(), climber)?;
                        ParserNode {
                            expr: ParserExpr::Product(Box::new(exp1), Box::new(exp2)),
                            span, //start_pos.span(&end_pos),
                        }
                    } */
                    _ => unreachable!(),
                };

                pairs.fold(
                    Ok(node),
                    |node: Result<ParserNode<'i>, Vec<Error<Rule>>>, pair| {
                        let node = node?;
                        println!("folding {:#?}", pair);
                        let node = match pair.as_rule() {
                            Rule::faculty_operator => {
                                let start = node.span.start_pos();
                                ParserNode {
                                    expr: ParserExpr::Faculty(Box::new(node)),
                                    span: start.span(&pair.as_span().end_pos()),
                                }
                            }
                            _ => unreachable!(),
                        };

                        Ok(node)
                    },
                )?
            }
        };

        Ok(node)
    }
    let term = |pair: Pair<'i, Rule>| {
        println!("term {:?}", pair);
        unaries(pair.into_inner().peekable(), climber)
    };
    let infix = |lhs: Result<ParserNode<'i>, Vec<Error<Rule>>>,
                 op: Pair<'i, Rule>,
                 rhs: Result<ParserNode<'i>, Vec<Error<Rule>>>| match op.as_rule() {
        Rule::add => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Arithmetic(
                    ast::ArithmeticOperator::Add,
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                span: start.span(&end),
            })
        }
        Rule::subtract => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Arithmetic(
                    ast::ArithmeticOperator::Subtract,
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                span: start.span(&end),
            })
        }
        Rule::except => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Arithmetic(
                    ast::ArithmeticOperator::Except,
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                span: start.span(&end),
            })
        }
        Rule::multiply | Rule::implicit_multiplication_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Arithmetic(
                    ast::ArithmeticOperator::Multiply,
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                span: start.span(&end),
            })
        }
        Rule::divide => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Arithmetic(
                    ast::ArithmeticOperator::Divide,
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                span: start.span(&end),
            })
        }
        Rule::power => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Arithmetic(
                    ast::ArithmeticOperator::Power,
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                span: start.span(&end),
            })
        }
        Rule::relational_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Relation(op.as_str().to_string(), Box::new(lhs), Box::new(rhs)),
                span: start.span(&end),
            })
        }
        Rule::logic_binary_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Logic(op.as_str().to_string(), Box::new(lhs), Box::new(rhs)),
                span: start.span(&end),
            })
        }
        _ => unreachable!(),
    };

    climber.climb(pairs, term, infix)
}

fn unescape(string: &str) -> Option<String> {
    let mut result = String::new();
    let mut chars = string.chars();

    loop {
        match chars.next() {
            Some('\\') => match chars.next()? {
                '"' => result.push('"'),
                '\'' => result.push('\''),
                'n' => result.push('\n'),
                '{' => {
                    result.push('\\');
                    result.push('{')
                }
                '}' => {
                    result.push('\\');
                    result.push('}')
                }
                //'\\' => result.push('\\'),
                //'r' => result.push('\r'),
                //'t' => result.push('\t'),
                //  '0' => result.push('\0'),
                /*    'x' => {
                    let string: String = chars.clone().take(2).collect();

                    if string.len() != 2 {
                        return None;
                    }

                    for _ in 0..string.len() {
                        chars.next()?;
                    }

                    let value = u8::from_str_radix(&string, 16).ok()?;

                    result.push(char::from(value));
                }
                'u' => {
                    if chars.next()? != '{' {
                        return None;
                    }

                    let string: String = chars.clone().take_while(|c| *c != '}').collect();

                    if string.len() < 2 || 6 < string.len() {
                        return None;
                    }

                    for _ in 0..string.len() + 1 {
                        chars.next()?;
                    }

                    let value = u32::from_str_radix(&string, 16).ok()?;

                    result.push(char::from_u32(value)?);
                } */
                _ => return None,
            },
            Some(c) => result.push(c),
            None => return Some(result),
        };
    }
}

pub fn parse(s: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    JMEParser::parse(Rule::jme, s)
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_NAMES: [&str; 6] = ["x", "x_1", "time_between_trials", "var1", "row1val2", "y''"];
    const VALID_ANNOTATIONS: [&str; 11] = [
        "verb", "op", "v", "vector", "unit", "dot", "m", "matrix", "diff", "degrees", "vec",
    ];
    const VALID_LITERALS: [&str; 6] = ["true", "false", "1", "4.3", "\"Numbas\"", "'Numbas'"]; // unicode pi and infinity
    const BUILTIN_CONSTANTS: [&str; 6] = ["pi", "e", "i", "infinity", "infty", "nan"]; // unicode pi and infinity

    #[test]
    fn variable_names() {
        for valid_name in VALID_NAMES {
            assert!(parse(valid_name).is_ok());
        }
    }

    #[test]
    fn annotated_variables() {
        for valid_name in VALID_NAMES {
            for valid_annotation in VALID_ANNOTATIONS {
                let annotated = format!("{}:{}", valid_annotation, valid_name);
                println!("{}", annotated);
                assert!(parse(&annotated[..]).is_ok());
            }
        }
        assert!(parse("v:dot:x").is_ok()); // multiple annotations
    }

    #[test]
    fn literals() {
        for valid_literal in VALID_LITERALS {
            assert!(parse(valid_literal).is_ok());
        }
    }

    #[test]
    fn builtin_constants() {
        for builtin_constant in BUILTIN_CONSTANTS {
            assert!(parse(builtin_constant).is_ok());
        }
    }

    #[test]
    fn grouped_terms_simple() {
        for valid_name in VALID_NAMES {
            let grouped = format!("({})", valid_name);
            assert!(parse(&grouped[..]).is_ok());
        }
    }

    #[test]
    fn function_application() {
        assert!(parse("f(a)").is_ok());
        assert!(parse("g(a,b)").is_ok());
    }

    // TODO test operators

    #[test]
    fn collections() {
        assert!(parse("[a: 1, \"first name\": \"Owen\"]").is_ok());
        assert!(parse("[1, 2, 3]").is_ok());
        assert!(parse("[a]").is_ok());
        assert!(parse("[]").is_ok());
    }

    #[test]
    fn indices() {
        assert!(parse("[1, 2, 3][0]").is_ok());
        assert!(parse("x[3..7]").is_ok());
        assert!(parse("id(4)[1]").is_ok());
        assert!(parse("info[\"name\"]").is_ok());
        assert!(parse("\"Numbas\"[0]").is_ok());
    }

    #[test]
    fn implicit_multiplication() {
        // TODO: see warning in docs about settings https://numbas-editor.readthedocs.io/en/latest/jme-reference.html#implicit-multiplication
        assert!(parse("(a+2)(a+1)").is_ok());
        assert!(parse("(a+1)2").is_ok());
        assert!(parse("(x+y)z").is_ok());
        assert!(parse("2x").is_ok());
        assert!(parse("x y").is_ok());
    }
}
