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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
/// Each portion of text displayed to the student (for example, the statement, advice, and part prompts) is a content area. A content area can include text, images, or more dynamic content such as videos and interactive diagrams.
pub struct ContentAreaString {
    s: String,
    asts: Option<Vec<ast::Expr>>,
}

impl std::convert::TryFrom<String> for ContentAreaString {
    type Error = String; // TODO
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let asts = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_content_area(&trimmed).map_err(|e| format!("{:?}", e))?;
            let asts =
                parser::consume_content_area_expressions(pairs).map_err(|e| format!("{:?}", e))?;
            Some(asts)
        };
        Ok(Self {
            s: trimmed.to_owned(),
            asts,
        })
    }
}

impl std::convert::From<ContentAreaString> for String {
    fn from(jme: ContentAreaString) -> Self {
        jme.s
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
/// Each portion of text displayed to the student (for example, the statement, advice, and part prompts) is a content area. A content area can include text, images, or more dynamic content such as videos and interactive diagrams.
pub struct JMENotesString {
    s: String,
    notes: Option<Vec<ast::Note>>,
}

impl std::convert::TryFrom<String> for JMENotesString {
    type Error = String; // TODO
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let notes = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_jme_script(&trimmed).map_err(|e| format!("{:?}", e))?;
            let notes = parser::consume_notes(pairs).map_err(|e| format!("{:?}", e))?;
            Some(notes)
        };
        Ok(Self {
            s: trimmed.to_owned(),
            notes,
        })
    }
}

impl std::convert::From<JMENotesString> for String {
    fn from(jme: JMENotesString) -> Self {
        jme.s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;
    #[test]
    fn content_area() {
        let res = ContentAreaString::try_from(
            "<p>Bob the farmer has {num_animals} {animal_name}</p>".to_string(),
        );
        assert!(res.is_ok());
        assert!(res.as_ref().unwrap().asts.is_some());
        assert_eq!(res.unwrap().asts.unwrap().len(), 2);
    }

    #[test]
    fn content_area_with_latex() {
        let res = ContentAreaString::try_from(
            r#"<p>A mass of $\var{mass}\,\mathrm{kg}$ is resting on a plane inclined at $\var{incline}^{\circ}$ to the horizontal. The distance along the plane from the ground to the mass is $\var{distance}\mathrm{m}$.</p>"#.to_string(),
        );
        res.clone().unwrap();
        assert!(res.is_ok());
        assert!(res.as_ref().unwrap().asts.is_some());
        assert_eq!(res.unwrap().asts.unwrap().len(), 3);
    }

    #[test]
    fn diagnosys() {
        let s = include_str!("test_assets/diagnosys.jme");
        let res = JMENotesString::try_from(s.to_string());
        assert!(res.is_ok());
        assert!(res.as_ref().unwrap().notes.is_some());
        assert_eq!(res.unwrap().notes.unwrap().len(), 21);
    }
}
