use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TranslatableString {
    //TODO: custom reader that checks for missing values etc?
    Translated(HashMap<String, FileString>), // Maps locales on formattable strings and parts like "{func}" (between {}) to values
    NotTranslated(FileString),
}

impl OptionalOverwrite for TranslatableString {
    type Item = TranslatableString;
    fn empty_fields(&self) -> Vec<String> {
        let empty = Vec::new();
        empty
    }
    fn overwrite(&mut self, _other: &Self::Item) {
        //TODO: Maybe add languages of other that are missing in self?
        // These default values should be read before language is interpreted
    }
}
impl_optional_overwrite_option!(TranslatableString);

impl TranslatableString {
    pub fn to_string(&self, locale: &String) -> Option<String> {
        match self {
            TranslatableString::NotTranslated(s) => Some(s.get_content(&locale)),
            TranslatableString::Translated(m) => m
                .get(locale)
                .or(m.get("content"))
                .map(|s| substitute(&s.get_content(&locale), &locale, &m)), //TODO content to static string //TODO: check for missing translations
        }
    }
}

//TODO: check for infinite loops / recursion? -> don't substitute something that is already
//substituted
fn substitute(pattern: &String, locale: &String, map: &HashMap<String, FileString>) -> String {
    let mut result = pattern.clone();
    let mut substituted = false;
    for (key, val) in map.iter() {
        if key.starts_with("{") && key.ends_with("}") {
            let before = result.clone();
            result = result.replace(key, &val.get_content(&locale));
            substituted = substituted || before != result;
        }
    }
    if substituted {
        return substitute(&result, &locale, &map);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::file_reference::FileString;

    #[test]
    fn no_translation() {
        let val = "some string".to_string();
        let t = TranslatableString::NotTranslated(FileString::s(&val));
        assert_eq!(t.to_string(&"any locale".to_string()), Some(val));
    }

    #[test]
    fn simple_translation() {
        let val_nl = "een string".to_string();
        let val_en = "some string".to_string();
        let mut m = HashMap::new();
        m.insert("nl".to_string(), FileString::s(&val_nl));
        m.insert("en".to_string(), FileString::s(&val_en));
        let t = TranslatableString::Translated(m);
        assert_eq!(t.to_string(&"nl".to_string()), Some(val_nl));
        assert_eq!(t.to_string(&"en".to_string()), Some(val_en));
    }

    #[test]
    fn substitution_translation() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        m.insert("nl".to_string(), FileString::s(&val_nl));
        m.insert("en".to_string(), FileString::s(&val_en));
        m.insert("{0}".to_string(), FileString::s(&"x^2".to_string()));
        m.insert("{func}".to_string(), FileString::s(&"e^x".to_string()));
        let t = TranslatableString::Translated(m.clone());
        assert_eq!(
            t.to_string(&"nl".to_string()),
            Some(format!(
                "een string met functie {} en {}",
                m["{func}"].get_content(),
                m["{0}"].get_content()
            ))
        );
        assert_eq!(
            t.to_string(&"en".to_string()),
            Some(format!(
                "some string with function {} and {}",
                m["{func}"].get_content(),
                m["{0}"].get_content()
            ))
        );
    }
}
