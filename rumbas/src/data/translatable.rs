use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_rumbas::ToRumbas;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// A translatable string
///
/// In yaml it should be specified as either
/// - a simple string: "this is a string"
/// - a file string: file:<path>
/// - A map that maps locales on formattables strings and parts like "{func}" (between {}) to values.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TranslatableString {
    //TODO: custom reader that checks for missing values etc?
    /// Maps locales on formattable strings and parts like "{func}" (between {}) to values
    Translated(HashMap<String, Value<TranslatableString>>),
    /// A file reference or string
    NotTranslated(FileString),
}

impl_to_rumbas!(TranslatableString);

impl TranslatableString {
    pub fn s(s: &str) -> Self {
        TranslatableString::NotTranslated(FileString::s(s))
    }
}

impl RumbasCheck for TranslatableString {
    fn check(&self) -> RumbasCheckResult {
        match self {
            TranslatableString::Translated(m) => {
                let mut empty = RumbasCheckResult::empty();
                for (_, v) in m.iter() {
                    empty.union(&v.check());
                }
                empty
            }
            TranslatableString::NotTranslated(f) => f.check(),
        }
    }
}
impl OptionalOverwrite<TranslatableString> for TranslatableString {
    fn overwrite(&mut self, _other: &TranslatableString) {
        //TODO: Maybe add languages of other that are missing in self?
        // These default values should be read before language is interpreted
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match self {
            TranslatableString::Translated(m) => m.insert_template_value(key, val),
            TranslatableString::NotTranslated(f) => f.insert_template_value(key, val),
        }
    }
}
impl_optional_overwrite_value!(TranslatableString);

impl TranslatableString {
    pub fn to_string(&self, locale: &str) -> Option<String> {
        match self {
            //TODO: just use unwrap on values?
            TranslatableString::NotTranslated(s) => Some(s.get_content(locale)),
            TranslatableString::Translated(m_value) => {
                let m = m_value.clone();
                m.get(locale)
                    .or_else(|| m.get("content")) //TODO
                    .map(|t_value| {
                        let t = t_value.unwrap();
                        match t {
                            TranslatableString::NotTranslated(s) => {
                                substitute(&s.get_content(locale), locale, &m)
                            }
                            _ => t.to_string(locale),
                        }
                    })
                    .flatten()
            } //TODO content to static string //TODO: check for missing translations
        }
    }
}

//TODO: check for infinite loops / recursion? -> don't substitute something that is already
//substituted
fn substitute(
    pattern: &str,
    locale: &str,
    map: &HashMap<String, Value<TranslatableString>>,
) -> Option<String> {
    let mut result = pattern.to_string();
    let mut substituted = false;
    for (key, val) in map.iter() {
        if key.starts_with('{') && key.ends_with('}') {
            let before = result.clone();
            if let Some(v) = val.unwrap().to_string(locale) {
                result = result.replace(key, &v);
                substituted = substituted || before != result;
            } else {
                return None;
            }
        }
    }
    if substituted {
        return substitute(&result, locale, map);
    }
    Some(result)
}

#[cfg(test)]
mod test {
    use super::TranslatableString::*;
    use super::*;
    use crate::data::file_reference::FileString;

    #[test]
    fn no_translation() {
        let val = "some string".to_string();
        let t = NotTranslated(FileString::s(&val));
        assert_eq!(t.to_string(&"any locale".to_string()), Some(val));
    }

    #[test]
    fn simple_translation() {
        let val_nl = "een string".to_string();
        let val_en = "some string".to_string();
        let mut m = HashMap::new();
        m.insert(
            "nl".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val_nl))),
        );
        m.insert(
            "en".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val_en))),
        );
        let t = Translated(m);
        assert_eq!(t.to_string(&"nl".to_string()), Some(val_nl));
        assert_eq!(t.to_string(&"en".to_string()), Some(val_en));
    }

    #[test]
    fn substitution_translation() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        m.insert(
            "nl".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val_nl))),
        );
        m.insert(
            "en".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val_en))),
        );
        let val1 = "x^2";
        let val2 = "e^x";
        m.insert(
            "{0}".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val1.to_string()))),
        );
        m.insert(
            "{func}".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val2.to_string()))),
        );
        let t = Translated(m);
        assert_eq!(
            t.to_string(&"nl".to_string()),
            Some(format!("een string met functie {} en {}", val2, val1))
        );
        assert_eq!(
            t.to_string(&"en".to_string()),
            Some(format!("some string with function {} and {}", val2, val1))
        );
    }

    #[test]
    fn substitution_translation_recusive() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        m.insert(
            "nl".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val_nl))),
        );
        m.insert(
            "en".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val_en))),
        );
        let val1 = "x^2";
        let val2 = "e^x ({cond})";
        m.insert(
            "{0}".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val1.to_string()))),
        );
        let mut m2 = HashMap::new();
        m2.insert(
            "content".to_string(),
            Value::Normal(NotTranslated(FileString::s(&val2.to_string()))),
        );

        let mut m3 = HashMap::new();
        m3.insert(
            "nl".to_string(),
            Value::Normal(NotTranslated(FileString::s(
                &"met x groter dan 0".to_string(),
            ))),
        );
        m3.insert(
            "en".to_string(),
            Value::Normal(NotTranslated(FileString::s(
                &"with x larger than 0".to_string(),
            ))),
        );
        m2.insert("{cond}".to_string(), Value::Normal(Translated(m3)));

        m.insert("{func}".to_string(), Value::Normal(Translated(m2)));
        let t = Translated(m);
        assert_eq!(
            t.to_string(&"nl".to_string()),
            Some(format!(
                "een string met functie e^x (met x groter dan 0) en {}",
                val1
            ))
        );
        assert_eq!(
            t.to_string(&"en".to_string()),
            Some(format!(
                "some string with function e^x (with x larger than 0) and {}",
                val1
            ))
        );
    }
}
