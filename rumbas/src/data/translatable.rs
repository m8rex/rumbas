use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TranslatableString {
    //TODO: custom reader that checks for missing values etc?
    Translated(HashMap<String, String>), // Maps locales on formattable strings and parts like "{func}" (between {}) to values
    NotTranslated(String),
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
            TranslatableString::NotTranslated(s) => Some(s.clone()),
            TranslatableString::Translated(m) => m.get(locale).map(|s| substitute(s, &m)),
        }
    }
}

fn substitute(pattern: &String, map: &HashMap<String, String>) -> String {
    let mut result = pattern.clone();
    for (key, val) in map.iter() {
        if key.starts_with("{") && key.ends_with("}") {
            result = result.replace(key, val);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_translation() {
        let val = "some string".to_string();
        let t = TranslatableString::NotTranslated(val.clone());
        assert_eq!(t.to_string(&"any locale".to_string()), Some(val));
    }

    #[test]
    fn simple_translation() {
        let val_nl = "een string".to_string();
        let val_en = "some string".to_string();
        let mut m = HashMap::new();
        m.insert("nl".to_string(), val_nl.clone());
        m.insert("en".to_string(), val_en.clone());
        let t = TranslatableString::Translated(m);
        assert_eq!(t.to_string(&"nl".to_string()), Some(val_nl));
        assert_eq!(t.to_string(&"en".to_string()), Some(val_en));
    }

    #[test]
    fn substitution_translation() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        m.insert("nl".to_string(), val_nl.clone());
        m.insert("en".to_string(), val_en.clone());
        m.insert("{0}".to_string(), "x^2".to_string());
        m.insert("{func}".to_string(), "e^x".to_string());
        let t = TranslatableString::Translated(m.clone());
        assert_eq!(
            t.to_string(&"nl".to_string()),
            Some(format!(
                "een string met functie {} en {}",
                m["{func}"], m["{0}"]
            ))
        );
        assert_eq!(
            t.to_string(&"en".to_string()),
            Some(format!(
                "some string with function {} and {}",
                m["{func}"], m["{0}"]
            ))
        );
    }
}
