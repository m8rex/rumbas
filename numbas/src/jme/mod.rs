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
            let pairs = parser::parse(&trimmed).map_err(|e| format!("{:?}", e))?;
            let ast = parser::consume_outer_expression(pairs).map_err(|e| format!("{:?}", e))?;
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
