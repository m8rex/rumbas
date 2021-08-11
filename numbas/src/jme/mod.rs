use serde::{Deserialize, Serialize};

pub mod ast;
pub mod builtin_functions;
pub mod parser;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct JMEString {
    s: String,
    ast: Option<ast::Expr>,
}

impl std::convert::TryFrom<String> for JMEString {
    type Error = String; // TODO
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let ast = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_jme(&trimmed).map_err(|e| format!("{:?}", e))?;
            let ast = parser::consume_one_expression(pairs).map_err(|e| format!("{:?}", e))?;
            Some(ast)
        };
        Ok(Self {
            s: trimmed.to_owned(),
            ast,
        })
    }
}

impl std::convert::From<JMEString> for String {
    fn from(jme: JMEString) -> Self {
        jme.s
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct EmbracedJMEString {
    s: String,
    asts: Option<Vec<ast::Expr>>,
}

impl std::convert::TryFrom<String> for EmbracedJMEString {
    type Error = String; // TODO
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let asts = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_embraced_jme(&trimmed).map_err(|e| format!("{:?}", e))?;
            let asts = parser::consume_expressions(pairs).map_err(|e| format!("{:?}", e))?;
            Some(asts)
        };
        Ok(Self {
            s: trimmed.to_owned(),
            asts,
        })
    }
}

impl std::convert::From<EmbracedJMEString> for String {
    fn from(jme: EmbracedJMEString) -> Self {
        jme.s
    }
}
