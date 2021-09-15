use crate::support::file_reference::FileString;
use crate::support::file_reference::FileStringInput;
use crate::support::to_numbas::ToNumbas;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMENotesString, JMEString};
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryInto;

translatable_type! {
    /// A translatable string
    ///
    /// In yaml it should be specified as either
    /// - a simple string: "this is a string"
    /// - a file string: file:<path>
    /// - A map that maps locales on formattables strings and parts like "{func}" (between {}) to values.
    type TranslatableString,
    subtype String,
    rumbas_check |_e| RumbasCheckResult::empty() // never happens

}

translatable_type! {
    /// A translatable JME string
    ///
    /// In yaml it should be specified as either
    /// - a simple string: "this is a string"
    /// - a file string: file:<path>
    /// - A map that maps locales on formattables strings and parts like "{func}" (between {}) to values.
    type JMETranslatableString,
    subtype JMEString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)
}

translatable_type! {
    /// A translatable embraced JME string
    ///
    /// In yaml it should be specified as either
    /// - a simple string: "this is a string"
    /// - a file string: file:<path>
    /// - A map that maps locales on formattables strings and parts like "{func}" (between {}) to values.
    type EmbracedJMETranslatableString,
    subtype EmbracedJMEString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)

}

translatable_type! {
    /// A translatable JME Notes string
    ///
    /// In yaml it should be specified as either
    /// - a simple string: "this is a string"
    /// - a file string: file:<path>
    /// - A map that maps locales on formattables strings and parts like "{func}" (between {}) to values.
    type JMENotesTranslatableString,
    subtype JMENotesString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)

}
translatable_type! {
    /// A translatable ContentArea string
    ///
    /// In yaml it should be specified as either
    /// - a simple string: "<p>The value is {value}</p>"
    /// - a file string: file:<path>
    /// - A map that maps locales on formattables strings and parts like "{func}" (between {}) to values.
    type ContentAreaTranslatableString,
    subtype ContentAreaString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)

}

#[cfg(test)]
mod test {
    use super::TranslatableString::*;
    use super::*;
    use crate::support::file_reference::FileString;

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
        m.insert("nl".to_string(), NotTranslated(FileString::s(&val_nl)));
        m.insert("en".to_string(), NotTranslated(FileString::s(&val_en)));
        let t = Translated(m);
        assert_eq!(t.to_string(&"nl".to_string()), Some(val_nl));
        assert_eq!(t.to_string(&"en".to_string()), Some(val_en));
    }

    #[test]
    fn substitution_translation() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        m.insert("nl".to_string(), NotTranslated(FileString::s(&val_nl)));
        m.insert("en".to_string(), NotTranslated(FileString::s(&val_en)));
        let val1 = "x^2";
        let val2 = "e^x";
        m.insert(
            "{0}".to_string(),
            NotTranslated(FileString::s(&val1.to_string())),
        );
        m.insert(
            "{func}".to_string(),
            NotTranslated(FileString::s(&val2.to_string())),
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
        m.insert("nl".to_string(), NotTranslated(FileString::s(&val_nl)));
        m.insert("en".to_string(), NotTranslated(FileString::s(&val_en)));
        let val1 = "x^2";
        let val2 = "e^x ({cond})";
        m.insert(
            "{0}".to_string(),
            NotTranslated(FileString::s(&val1.to_string())),
        );
        let mut m2 = HashMap::new();
        m2.insert(
            "content".to_string(),
            NotTranslated(FileString::s(&val2.to_string())),
        );

        let mut m3 = HashMap::new();
        m3.insert(
            "nl".to_string(),
            NotTranslated(FileString::s(&"met x groter dan 0".to_string())),
        );
        m3.insert(
            "en".to_string(),
            NotTranslated(FileString::s(&"with x larger than 0".to_string())),
        );
        m2.insert("{cond}".to_string(), Translated(m3));

        m.insert("{func}".to_string(), Translated(m2));
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

macro_rules! translatable_type {
    (
        $(#[$outer:meta])*
        type $type: ident,
        subtype $subtype: ty,
        rumbas_check $check_expr: expr
    ) => {
        paste::paste! {
            #[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
            #[serde(untagged)]
            pub enum [<$type Input>] {
                //TODO: custom reader that checks for missing values etc?
                /// Maps locales on formattable strings and parts like "{func}" (between {}) to values
                Translated(HashMap<String, Value<[<$type Input>]>>),
                /// A file reference or string
                NotTranslated(FileStringInput),
            }

            impl std::convert::From<$subtype> for [<$type Input>] {
                fn from(sub: $subtype) -> Self {
                    let s: String = sub.into();
                    [<$type Input>]::NotTranslated(FileStringInput::s(&s))
                }
            }

            impl std::convert::From<$subtype> for $type {
                fn from(sub: $subtype) -> Self {
                    let s: String = sub.into();
                    $type::NotTranslated(FileString::s(&s))
                }
            }

            impl RumbasCheck for $type {
                fn check(&self, locale: &str) -> RumbasCheckResult {
                    let content = self.to_string(locale);
                    match content {
                        Some(c) => {
                            let conversion_res: Result<$subtype, _> = c.try_into();
                            match conversion_res {
                                Ok(_) => RumbasCheckResult::empty(),
                                Err(e) => $check_expr(e),
                            }
                        }
                        None => RumbasCheckResult::from_missing_translation(Some(locale.to_owned())),
                    }
                }
            }

            impl ToNumbas<$subtype> for $type {
                fn to_numbas(&self, locale: &str) -> $subtype {
                    self.to_string(locale).unwrap().try_into().unwrap()
                }
            }

            #[derive(Debug, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
            pub enum $type {
                //TODO: custom reader that checks for missing values etc?
                /// Maps locales on formattable strings and parts like "{func}" (between {}) to values
                Translated(HashMap<String, $type>),
                /// A file reference or string
                NotTranslated(FileString),
            }

            impl Input for [<$type Input>] {
                type Normal = $type;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    match self {
                        [<$type Input>]::Translated(t) => $type::Translated(t.to_normal()),
                        [<$type Input>]::NotTranslated(f) => $type::NotTranslated(f.to_normal()),
                    }
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                    match normal {
                        $type::Translated(t) => [<$type Input>]::Translated(HashMap::from_normal(t)),
                        $type::NotTranslated(f) => [<$type Input>]::NotTranslated(FileStringInput::from_normal(f)),
                    }
                }
                fn find_missing(&self) -> InputCheckResult {
                    match self {
                        Self::Translated(s) => s.find_missing(),
                        Self::NotTranslated(s) => s.find_missing()
                    }
                }
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
                    match self {
                        [<$type Input>]::Translated(m) => m.insert_template_value(key, val),
                        [<$type Input>]::NotTranslated(f) => f.insert_template_value(key, val),
                    }
                }
            }

            impl InputInverse for $type {
                type Input = [<$type Input>];
            }

            impl Overwrite<[<$type Input>]> for [<$type Input>] {
                fn overwrite(&mut self, _other: &[<$type Input>]) {
                    //TODO: Maybe add languages of other that are missing in self?
                    // These default values should be read before language is interpreted
                }
            }

            impl $type {
                pub fn to_string(&self, locale: &str) -> Option<String> {
                    //TODO: check for infinite loops / recursion? -> don't substitute something that is already
                    //substituted
                    fn substitute(
                        pattern: &Option<String>,
                        locale: &str,
                        map: &HashMap<String, $type>,
                    ) -> Option<String> {
                        pattern
                            .as_ref()
                            .map(|pattern| {
                                let mut result = pattern.to_string();
                                let mut substituted = false;
                                for (key, val) in map.iter() {
                                    if key.starts_with('{') && key.ends_with('}') {
                                        let before = result.clone();
                                        if let Some(v) = val.to_string(locale) {
                                            result = result.replace(key, &v);
                                            substituted = substituted || before != result;
                                        } else {
                                            return None;
                                        }
                                    }
                                }
                                if substituted {
                                    substitute(&Some(result), locale, map)
                                } else {
                                    Some(result)
                                }
                            })
                            .flatten()
                    }
                    match self {
                        //TODO: just use unwrap on values?
                        $type::NotTranslated(s) => s.get_content(locale),
                        $type::Translated(m_value) => {
                            let m = m_value.clone();
                            m.get(locale)
                                .or_else(|| m.get("content")) //TODO
                                .map(|t| {
                                    match t {
                                        $type::NotTranslated(s) => {
                                            substitute(&s.get_content(locale), locale, &m)
                                        }
                                        _ => t.to_string(locale),
                                    }
                                })
                                .flatten()
                        } //TODO content to static string
                    }
                }
            }
        }
    };
}

use translatable_type;
