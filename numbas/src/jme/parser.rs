use crate::jme::ast;
use pest::error::Error;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::{Parser, Span};
use std::iter::Peekable;

mod jme {
    #[derive(Parser)]
    #[grammar = "jme/jme.pest"]
    pub struct JMEParser;
}
use jme::Rule;

mod html {
    #[derive(Parser)]
    #[grammar = "jme/html.pest"]
    pub struct HTMLParser;
}
use html::Rule as HTMLRule;

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
    Range(ast::RangeOperator, Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    Arithmetic(
        ast::ArithmeticOperator,
        Box<ParserNode<'i>>,
        Box<ParserNode<'i>>,
    ),
    AnnotatedConstant(String),
    AnnotatedIdent(String),
    Relation(String, Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    Logic(String, Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    List(Vec<ParserNode<'i>>),
    Dictionary(Vec<(ParserNode<'i>, ParserNode<'i>)>),
    FunctionApplication(String, Vec<ParserNode<'i>>),
    Prefix(String, Box<ParserNode<'i>>),
    Faculty(Box<ParserNode<'i>>),
    Indexation(Box<ParserNode<'i>>),
    Cast(Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    Sequence(Box<ParserNode<'i>>, Box<ParserNode<'i>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptParserNode<'i> {
    pub expr: ScriptParserExpr<'i>,
    pub span: Span<'i>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScriptParserExpr<'i> {
    Note(String, Option<String>, ParserNode<'i>, String),
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
            ParserExpr::Range(o, n1, n2) => {
                ast::Expr::Range(o, Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::Arithmetic(a, n1, n2) => {
                ast::Expr::Arithmetic(a, Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::AnnotatedIdent(s) => ast::Expr::Ident(s.into()),
            ParserExpr::AnnotatedConstant(s) => ast::Expr::Constant(s.into()),
            ParserExpr::Relation(s, n1, n2) => {
                ast::Expr::Relation(s.into(), Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::Logic(s, n1, n2) => {
                ast::Expr::Logic(s.into(), Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::List(n1) => ast::Expr::List(n1.into_iter().map(|n| n.into()).collect()),
            ParserExpr::Dictionary(n1) => {
                ast::Expr::Dictionary(n1.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
            ParserExpr::FunctionApplication(s, n1) => {
                ast::Expr::FunctionApplication(s.into(), n1.into_iter().map(|n| n.into()).collect())
            }
            ParserExpr::Prefix(s, n) => ast::Expr::Prefix(s.into(), Box::new((*n).into())),
            ParserExpr::Faculty(n) => ast::Expr::Faculty(Box::new((*n).into())),
            ParserExpr::Indexation(n) => ast::Expr::Indexation(Box::new((*n).into())),
            ParserExpr::Cast(n1, n2) => {
                ast::Expr::Cast(Box::new((*n1).into()), Box::new((*n2).into()))
            }
            ParserExpr::Sequence(n1, n2) => {
                ast::Expr::Sequence(Box::new((*n1).into()), Box::new((*n2).into()))
            }
        }
    }
}

impl<'i> std::convert::From<ScriptParserNode<'i>> for ast::Note {
    fn from(node: ScriptParserNode<'i>) -> ast::Note {
        node.expr.into()
    }
}
impl<'i> std::convert::From<ScriptParserExpr<'i>> for ast::Note {
    fn from(expr: ScriptParserExpr<'i>) -> ast::Note {
        match expr {
            ScriptParserExpr::Note(name, description, expression, expression_string) => {
                ast::Note::create(
                    name.into(),
                    description,
                    expression.into(),
                    expression_string,
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsumeError {
    JMEParseError(Vec<Error<Rule>>),
    HTMLParseError(Vec<Error<HTMLRule>>),
    UnknownParseError,
}

impl std::fmt::Display for ConsumeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConsumeError::JMEParseError(errors) => {
                write!(
                    f,
                    "Error while parsing JME:\n{}",
                    errors
                        .iter()
                        .map(|e| format!("{:?}", e))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            ConsumeError::HTMLParseError(errors) => {
                write!(
                    f,
                    "Error while parsing HTML:\n{}",
                    errors
                        .iter()
                        .map(|e| format!("{:?}", e))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            ConsumeError::UnknownParseError => write!(f, "Unknown parse error, please report."),
        }
    }
}

pub fn consume_content_area_expressions(
    pairs: Pairs<HTMLRule>,
) -> Result<Vec<ast::Expr>, ConsumeError> {
    let pairs = pairs.clone().next().unwrap().into_inner();
    let mut asts = vec![];
    for expression in pairs.filter(|p| p.as_rule() == HTMLRule::expression) {
        let parsed_jme =
            parse_as_jme(expression.as_str()).map_err(|e| ConsumeError::JMEParseError(vec![e]))?;
        let ast = consume_one_expression(parsed_jme)?;
        asts.push(ast);
    }
    Ok(asts)
}

pub fn consume_notes(pairs: Pairs<Rule>) -> Result<Vec<ast::Note>, ConsumeError> {
    let pairs = pairs.clone().next().unwrap().into_inner();
    let res_res = std::panic::catch_unwind(|| {
        let expression = consume_note_with_spans(pairs)?;
        //let errors = validator::validate_ast(&rules);
        //if errors.is_empty() {
        Ok(expression.into_iter().map(|e| e.into()).collect())
        /*} else {
            Err(errors)
        }*/
    });
    match res_res {
        Ok(res) => res.map_err(ConsumeError::JMEParseError),
        Err(_err) => Err(ConsumeError::UnknownParseError),
    }
}

pub fn consume_expressions(pairs: Pairs<Rule>) -> Result<Vec<ast::Expr>, ConsumeError> {
    let pairs = pairs.clone().next().unwrap().into_inner();
    let res_res = std::panic::catch_unwind(|| {
        let expression = consume_expression_with_spans(pairs)?;
        //let errors = validator::validate_ast(&rules);
        //if errors.is_empty() {
        Ok(expression.into_iter().map(|e| e.into()).collect())
        /*} else {
            Err(errors)
        }*/
    });
    match res_res {
        Ok(res) => res.map_err(ConsumeError::JMEParseError),
        Err(_err) => Err(ConsumeError::UnknownParseError),
    }
}

pub fn consume_one_expression(pairs: Pairs<Rule>) -> Result<ast::Expr, ConsumeError> {
    consume_expressions(pairs).map(|v| v.into_iter().next().unwrap())
}

fn consume_note_with_spans(pairs: Pairs<Rule>) -> Result<Vec<ScriptParserNode>, Vec<Error<Rule>>> {
    let mut results = Vec::new();
    for note in pairs.filter(|pair| pair.as_rule() == Rule::note) {
        results.push(consume_note(note.into_inner().peekable())?);
    }
    Ok(results)
}

fn consume_note(mut pairs: Peekable<Pairs<Rule>>) -> Result<ScriptParserNode, Vec<Error<Rule>>> {
    let first = pairs.next().unwrap();
    //let start = first.as_span().start();
    let s = first.as_str().to_string();
    let mut pair = pairs.next().unwrap();
    let description = if pair.as_rule() == Rule::description {
        let res = Some(pair.as_str().to_string());
        pair = pairs.next().unwrap();
        res
    } else {
        None
    };
    //let end = pair.as_span().end();
    let expression_string = pair.as_str().to_string();
    let expression = consume_expression(pair)?;

    Ok(ScriptParserNode {
        expr: ScriptParserExpr::Note(s, description, expression, expression_string),
        span: first.as_span(), //Span::new("", start, end).unwrap(), // TODO: input string?
    })
}

fn consume_expression_with_spans(pairs: Pairs<Rule>) -> Result<Vec<ParserNode>, Vec<Error<Rule>>> {
    let mut results = Vec::new();
    for expression in pairs.filter(|pair| pair.as_rule() == Rule::expression) {
        results.push(consume_expression(expression)?);
    }
    Ok(results)
}

fn consume_expression(expression: Pair<Rule>) -> Result<ParserNode, Vec<Error<Rule>>> {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::sequence_operator, Assoc::Left),
        Operator::new(Rule::logic_binary_operator, Assoc::Left),
        Operator::new(Rule::relational_operator, Assoc::Left),
        Operator::new(Rule::cast_operator, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left)
            | Operator::new(Rule::subtract, Assoc::Left)
            | Operator::new(Rule::except, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left)
            | Operator::new(Rule::implicit_multiplication_operator, Assoc::Left)
            | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::range_step_separator, Assoc::Left), // TODO...
        Operator::new(Rule::range_separator, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    consume_expression_internal(expression.into_inner().peekable(), &climber)
}

fn consume_expression_internal<'i>(
    pairs: Peekable<Pairs<'i, Rule>>,
    climber: &PrecClimber<Rule>,
) -> Result<ParserNode<'i>, Vec<Error<Rule>>> {
    fn unaries<'i>(
        mut pairs: Peekable<Pairs<'i, Rule>>,
        climber: &PrecClimber<Rule>,
    ) -> Result<ParserNode<'i>, Vec<Error<Rule>>> {
        let pair = pairs.next().unwrap();

        let node = match pair.as_rule() {
            Rule::prefix_operator => {
                let node = unaries(pairs, climber)?;
                let end = node.span.end_pos();

                ParserNode {
                    expr: ParserExpr::Prefix(pair.as_str().trim().to_owned(), Box::new(node)),
                    span: pair.as_span().start_pos().span(&end),
                }
            }
            other_rule => {
                let node = match other_rule {
                    Rule::expression => {
                        consume_expression_internal(pair.into_inner().peekable(), climber)?
                    }
                    Rule::annotated_ident => ParserNode {
                        expr: ParserExpr::AnnotatedIdent(pair.as_str().trim().to_owned()),
                        span: pair.clone().as_span(),
                    },
                    Rule::constant => ParserNode {
                        expr: ParserExpr::AnnotatedConstant(pair.as_str().trim().to_owned()),
                        span: pair.clone().as_span(),
                    },
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
                    Rule::list => {
                        let span = pair.as_span();
                        let pairs = pair.into_inner();
                        let mut elements = Vec::new();
                        for p in pairs.filter(|p| p.as_rule() == Rule::expression) {
                            elements.push(consume_expression_internal(
                                p.into_inner().peekable(),
                                climber,
                            )?);
                        }
                        ParserNode {
                            expr: ParserExpr::List(elements),
                            span,
                        }
                    }
                    Rule::dictionary => {
                        let span = pair.as_span();
                        let pairs = pair.into_inner();
                        let mut elements = Vec::new();
                        for p in pairs.filter(|p| p.as_rule() == Rule::expression) {
                            let mut item = p.into_inner();
                            let key_pair = item.next().unwrap();
                            let value_pair = item.next().unwrap();
                            elements.push((
                                consume_expression_internal(
                                    key_pair.into_inner().peekable(),
                                    climber,
                                )?,
                                consume_expression_internal(
                                    value_pair.into_inner().peekable(),
                                    climber,
                                )?,
                            ));
                        }
                        ParserNode {
                            expr: ParserExpr::Dictionary(elements),
                            span,
                        }
                    }
                    Rule::function_application => {
                        let mut pairs = pair.into_inner();
                        let pair = pairs.next().unwrap();
                        let ident = pair.as_str();
                        let start_pos = pair.clone().as_span().start_pos();
                        //pairs.next().unwrap(); // (
                        let pair = pairs.next().unwrap();
                        let end_pos = pair.as_span().end_pos();
                        let inner_pairs = pair.into_inner();
                        let mut arguments = Vec::new();
                        for p in inner_pairs.filter(|p| p.as_rule() == Rule::expression) {
                            arguments.push(consume_expression_internal(
                                p.into_inner().peekable(),
                                climber,
                            )?);
                        }
                        ParserNode {
                            expr: ParserExpr::FunctionApplication(ident.to_string(), arguments),
                            span: start_pos.span(&end_pos),
                        }
                    }
                    _ => unreachable!(),
                };

                pairs.fold(
                    Ok(node),
                    |node: Result<ParserNode<'i>, Vec<Error<Rule>>>, pair| {
                        let node = node?;
                        let node = match pair.as_rule() {
                            Rule::faculty_operator => {
                                let start = node.span.start_pos();
                                ParserNode {
                                    expr: ParserExpr::Faculty(Box::new(node)),
                                    span: start.span(&pair.as_span().end_pos()),
                                }
                            }
                            Rule::index_operator => {
                                let start = node.span.start_pos();
                                ParserNode {
                                    expr: ParserExpr::Indexation(Box::new(node)),
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
    let term = |pair: Pair<'i, Rule>| unaries(pair.into_inner().peekable(), climber);
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
        Rule::range_separator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Range(ast::RangeOperator::Create, Box::new(lhs), Box::new(rhs)),
                span: start.span(&end),
            })
        }
        Rule::range_step_separator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Range(ast::RangeOperator::Step, Box::new(lhs), Box::new(rhs)),
                span: start.span(&end),
            })
        }
        Rule::cast_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Cast(Box::new(lhs), Box::new(rhs)),
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
        Rule::sequence_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Sequence(Box::new(lhs), Box::new(rhs)),
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
                '\\' => result.push('\\'),
                _ => return None,
            },
            Some(c) => result.push(c),
            None => return Some(result),
        };
    }
}

pub fn parse_as_jme(s: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    log::debug!("Parsing as JME: {}", s);
    jme::JMEParser::parse(Rule::jme, s)
}

pub fn parse_as_embraced_jme(s: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    log::debug!("Parsing as embraced JME: {}", s);
    jme::JMEParser::parse(Rule::embraced_jme, s)
}

pub fn parse_as_content_area(s: &str) -> Result<Pairs<'_, HTMLRule>, pest::error::Error<HTMLRule>> {
    log::debug!("Parsing as content area: {}", s);
    html::HTMLParser::parse(HTMLRule::content_area, s)
}

pub fn parse_as_jme_script(s: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    log::debug!("Parsing as JME script: {}", s);
    jme::JMEParser::parse(Rule::script, s)
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_NAMES: [&str; 6] = ["x", "x_1", "time_between_trials", "var1", "row1val2", "y''"];
    const VALID_ANNOTATIONS: [&str; 11] = [
        "verb", "op", "v", "vector", "unit", "dot", "m", "matrix", "diff", "degrees", "vec",
    ];
    const VALID_LITERALS: [&str; 7] = [
        "true",
        "false",
        "1",
        "4.3",
        "\"Numbas\"",
        "'Numbas'",
        "\"String with \\\\ which is no comment\"",
    ];
    const BUILTIN_CONSTANTS: [&str; 8] = ["pi", "π", "e", "i", "infinity", "infty", "∞", "nan"];

    #[test]
    fn variable_names() {
        for valid_name in VALID_NAMES {
            assert!(parse_as_jme(valid_name).is_ok());
        }
    }

    #[test]
    fn annotated_variables() {
        for valid_name in VALID_NAMES {
            for valid_annotation in VALID_ANNOTATIONS {
                let annotated = format!("{}:{}", valid_annotation, valid_name);
                assert!(parse_as_jme(&annotated[..]).is_ok());
            }
        }
        assert!(parse_as_jme("v:dot:x").is_ok()); // multiple annotations
    }

    #[test]
    fn literals() {
        for valid_literal in VALID_LITERALS {
            assert!(parse_as_jme(valid_literal).is_ok());
        }
    }

    #[test]
    fn builtin_constants() {
        for builtin_constant in BUILTIN_CONSTANTS {
            assert!(parse_as_jme(builtin_constant).is_ok());
        }
    }

    #[test]
    fn grouped_terms_simple() {
        for valid_name in VALID_NAMES {
            let grouped = format!("({})", valid_name);
            assert!(parse_as_jme(&grouped[..]).is_ok());
        }
    }

    #[test]
    fn function_application() {
        assert!(parse_as_jme("f(a)").is_ok());
        assert!(parse_as_jme("g(a,b)").is_ok());
    }

    #[test]
    fn collections() {
        assert!(parse_as_jme("[a: 1, \"first name\": \"Owen\"]").is_ok());
        assert!(parse_as_jme("[1, 2, 3]").is_ok());
        assert!(parse_as_jme("[a]").is_ok());
        assert!(parse_as_jme("[]").is_ok());
    }

    #[test]
    fn indices() {
        assert!(parse_as_jme("[1, 2, 3][0]").is_ok());
        assert!(parse_as_jme("x[3..7]").is_ok());
        assert!(parse_as_jme("id(4)[1]").is_ok());
        assert!(parse_as_jme("info[\"name\"]").is_ok());
        assert!(parse_as_jme("\"Numbas\"[0]").is_ok());
    }

    #[test]
    fn implicit_multiplication() {
        // TODO: see warning in docs about settings https://numbas-editor.readthedocs.io/en/latest/jme-reference.html#implicit-multiplication
        assert!(parse_as_jme("(a+2)(a+1)").is_ok());
        assert!(parse_as_jme("(a+1)2").is_ok());
        assert!(parse_as_jme("(x+y)z").is_ok());
        assert!(parse_as_jme("2x").is_ok());
        assert!(parse_as_jme("x y").is_ok());
    }

    #[test]
    fn embraced_expression() {
        assert!(parse_as_embraced_jme("hallo {x+5} test {x*y+7} \\{xxxtest\\}").is_ok());
    }
}
