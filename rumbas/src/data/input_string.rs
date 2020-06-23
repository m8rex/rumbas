use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "String")]
pub struct InputString(pub String);

impl std::convert::From<String> for InputString {
    fn from(s: String) -> Self {
        //TODO: remove hardcoded µ and § -> env variables?
        let re_var = Regex::new(r"µ\{(?P<v>.*?)\}").unwrap();
        let after_var = re_var.replace_all(&s, r"\var{$v}");
        let re_simplify = Regex::new(r"§\{(?P<v>.*?)\}").unwrap();
        let after_simplify = re_simplify.replace_all(&after_var, r"\simplify{$v}");
        InputString(after_simplify.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn replace_var() {
        let s = "Test µ{a*x} something".to_string();
        assert_eq!(
            InputString::from(s).0,
            r"Test \var{a*x} something".to_string()
        );
    }

    #[test]
    fn replace_multiple_var() {
        let s = "Test µ{a*x} something and µ{exp^2} something else".to_string();
        assert_eq!(
            InputString::from(s).0,
            r"Test \var{a*x} something and \var{exp^2} something else".to_string()
        );
    }

    #[test]
    fn replace_simplify() {
        let s = "Test §{a*x} something".to_string();
        assert_eq!(
            InputString::from(s).0,
            r"Test \simplify{a*x} something".to_string()
        );
    }

    #[test]
    fn replace_multiple_simplify() {
        let s = "Test §{a*x} something and §{exp^2} something else".to_string();
        assert_eq!(
            InputString::from(s).0,
            r"Test \simplify{a*x} something and \simplify{exp^2} something else".to_string()
        );
    }
}
