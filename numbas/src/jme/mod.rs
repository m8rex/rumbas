use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use std::convert::Into;
use std::convert::TryInto;

pub mod ast;
pub mod builtin_functions;
pub mod parser;

macro_rules! impl_string_json_schema {
    ($t: ty, $e: expr) => {
        impl JsonSchema for $t {
            fn schema_name() -> String {
                $e.to_owned()
            }

            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                gen.subschema_for::<String>()
            }
        }
    };
}

#[derive(Deserialize)]
#[serde(untagged)]
/// Helper type to parse numbers as strings
enum StringOrNumber {
    String(String),
    Number(i64),
    Float(f64),
}

impl std::convert::From<StringOrNumber> for String {
    fn from(son: StringOrNumber) -> Self {
        match son {
            StringOrNumber::String(s) => s,
            StringOrNumber::Number(i) => i.to_string(),
            StringOrNumber::Float(f) => f.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, SerdeDiff)]
#[serde(try_from = "StringOrNumber")]
#[serde(into = "String")]
pub struct JMEString {
    s: String,
    #[serde_diff(skip)]
    ast: Option<ast::Expr>,
}
impl_string_json_schema!(JMEString, "JMEString");

impl std::convert::TryFrom<StringOrNumber> for JMEString {
    type Error = parser::ConsumeError;
    fn try_from(son: StringOrNumber) -> Result<Self, Self::Error> {
        let s: String = son.into();
        s.try_into()
    }
}

impl std::convert::TryFrom<String> for JMEString {
    type Error = parser::ConsumeError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let ast = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_jme(trimmed)
                .map_err(|e| parser::ConsumeError::JMEParseError(vec![e]))?;
            let ast = parser::consume_one_expression(pairs)?;
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

impl std::fmt::Display for JMEString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Default for JMEString {
    fn default() -> Self {
        JMEString {
            s: String::new(),
            ast: None,
        }
    }
}

impl JMEString {
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, SerdeDiff)]
#[serde(try_from = "StringOrNumber")]
#[serde(into = "String")]
pub struct EmbracedJMEString {
    s: String,
    #[serde_diff(skip)]
    asts: Option<Vec<ast::Expr>>,
}
impl_string_json_schema!(EmbracedJMEString, "EmbracedJMEString"); // maybe add pattern?

impl std::convert::TryFrom<StringOrNumber> for EmbracedJMEString {
    type Error = parser::ConsumeError;
    fn try_from(son: StringOrNumber) -> Result<Self, Self::Error> {
        let s: String = son.into();
        s.try_into()
    }
}

impl std::convert::TryFrom<String> for EmbracedJMEString {
    type Error = parser::ConsumeError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let asts = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_embraced_jme(trimmed)
                .map_err(|e| parser::ConsumeError::JMEParseError(vec![e]))?;
            let asts = parser::consume_expressions(pairs)?;
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

impl std::fmt::Display for EmbracedJMEString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Default for EmbracedJMEString {
    fn default() -> Self {
        EmbracedJMEString {
            s: String::new(),
            asts: None,
        }
    }
}

impl EmbracedJMEString {
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
    pub fn new() -> Self {
        Self {
            s: String::new(),
            asts: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, SerdeDiff)]
#[serde(try_from = "String")]
#[serde(into = "String")]
/// Each portion of text displayed to the student (for example, the statement, advice, and part prompts) is a content area. A content area can include text, images, or more dynamic content such as videos and interactive diagrams.
pub struct ContentAreaString {
    s: String,
    #[serde_diff(skip)]
    asts: Option<Vec<ast::Expr>>,
}
impl_string_json_schema!(ContentAreaString, "ContentAreaString");

impl std::convert::TryFrom<String> for ContentAreaString {
    type Error = parser::ConsumeError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let asts = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_content_area(trimmed)
                .map_err(|e| parser::ConsumeError::HTMLParseError(vec![e]))?;
            let asts = parser::consume_content_area_expressions(pairs)?;
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

impl std::fmt::Display for ContentAreaString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Default for ContentAreaString {
    fn default() -> Self {
        ContentAreaString {
            s: String::new(),
            asts: None,
        }
    }
}

impl ContentAreaString {
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, SerdeDiff)]
#[serde(try_from = "String")]
#[serde(into = "String")]
/// Each portion of text displayed to the student (for example, the statement, advice, and part prompts) is a content area. A content area can include text, images, or more dynamic content such as videos and interactive diagrams.
pub struct JMENotesString {
    pub s: String,
    #[serde_diff(skip)]
    pub notes: Option<Vec<ast::Note>>,
}
impl_string_json_schema!(JMENotesString, "JMENotesString");

impl std::convert::TryFrom<String> for JMENotesString {
    type Error = parser::ConsumeError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let trimmed = s.trim();
        let notes = if trimmed.is_empty() {
            None
        } else {
            let pairs = parser::parse_as_jme_script(trimmed)
                .map_err(|e| parser::ConsumeError::JMEParseError(vec![e]))?;
            let notes = parser::consume_notes(pairs)?;
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

impl std::fmt::Display for JMENotesString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Default for JMENotesString {
    fn default() -> Self {
        JMENotesString {
            s: String::new(),
            notes: None,
        }
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

    #[test]
    fn marking_notes_with_space_between_newlines() {
        let s = include_str!("test_assets/example.jme");
        let res = JMENotesString::try_from(s.to_string());
        assert!(res.is_ok());
        assert!(res.as_ref().unwrap().notes.is_some());
        assert_eq!(res.unwrap().notes.unwrap().len(), 8);
    }

    #[test]
    fn embraced_without_braces() {
        let res = EmbracedJMEString::try_from("Answer 1".to_string());
        assert!(res.is_ok());
    }
}
