use crate::support::file_reference::AnyString;
use crate::support::file_reference::FileString;
use crate::support::file_reference::FileStringInput;
use crate::support::to_numbas::ToNumbas;
use comparable::Comparable;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMENotesString, JMEString};
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::From;
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
    use super::*;
    use crate::support::file_reference::FileString;

    #[test]
    fn no_translation() {
        let val = "some string".to_string();
        let t: Translation = FileString::s(&val).into();
        assert_eq!(t.to_string("any locale"), Some(val));
    }

    #[test]
    fn simple_translation() {
        let val_nl = "een string".to_string();
        let val_en = "some string".to_string();
        let mut m = HashMap::new();
        m.insert("nl".to_string(), FileString::s(&val_nl));
        m.insert("en".to_string(), FileString::s(&val_en));
        let t = Translation {
            content: TranslationContent::Locales(m),
            placeholders: HashMap::new(),
        };
        assert_eq!(t.to_string("nl"), Some(val_nl));
        assert_eq!(t.to_string("en"), Some(val_en));
    }

    #[test]
    fn substitution_translation() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        let mut placeholders = HashMap::new();
        m.insert("nl".to_string(), FileString::s(&val_nl));
        m.insert("en".to_string(), FileString::s(&val_en));
        let val1 = "x^2";
        let val2 = "e^x";
        placeholders.insert(
            "0".to_string(),
            Translation {
                content: TranslationContent::Content(FileString::s(val1)),
                placeholders: HashMap::new(),
            },
        );
        placeholders.insert(
            "func".to_string(),
            Translation {
                content: TranslationContent::Content(FileString::s(val2)),
                placeholders: HashMap::new(),
            },
        );
        let t = Translation {
            content: TranslationContent::Locales(m),
            placeholders,
        };
        assert_eq!(
            t.to_string("nl"),
            Some(format!("een string met functie {} en {}", val2, val1))
        );
        assert_eq!(
            t.to_string("en"),
            Some(format!("some string with function {} and {}", val2, val1))
        );
    }

    #[test]
    fn substitution_translation_recusive() {
        let val_nl = "een string met functie {func} en {0}".to_string();
        let val_en = "some string with function {func} and {0}".to_string();
        let mut m = HashMap::new();
        let mut placeholders = HashMap::new();
        m.insert("nl".to_string(), FileString::s(&val_nl));
        m.insert("en".to_string(), FileString::s(&val_en));
        let val1 = "x^2";
        let val2 = "e^x ({cond})";
        placeholders.insert(
            "0".to_string(),
            Translation {
                content: TranslationContent::Content(FileString::s(val1)),
                placeholders: HashMap::new(),
            },
        );

        let mut placeholders2 = HashMap::new();

        let mut m3 = HashMap::new();
        m3.insert("nl".to_string(), FileString::s("met x groter dan 0"));
        m3.insert("en".to_string(), FileString::s("with x larger than 0"));
        placeholders2.insert(
            "cond".to_string(),
            Translation {
                content: TranslationContent::Locales(m3),
                placeholders: HashMap::new(),
            },
        );

        placeholders.insert(
            "func".to_string(),
            Translation {
                content: TranslationContent::Content(FileString::s(val2)),
                placeholders: placeholders2,
            },
        );
        let t = Translation {
            content: TranslationContent::Locales(m),
            placeholders,
        };
        assert_eq!(
            t.to_string("nl"),
            Some(format!(
                "een string met functie e^x (met x groter dan 0) en {}",
                val1
            ))
        );
        assert_eq!(
            t.to_string("en"),
            Some(format!(
                "some string with function e^x (with x larger than 0) and {}",
                val1
            ))
        );
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "TranslationContentInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum TranslationContent {
    Locales(HashMap<String, FileString>),
    Content(FileString),
}

impl From<FileStringInput> for TranslationContentInput {
    fn from(f: FileStringInput) -> Self {
        Self::Content(f)
    }
}

impl From<FileString> for TranslationContent {
    fn from(f: FileString) -> Self {
        Self::Content(f)
    }
}

impl TranslationContent {
    pub fn get(&self, locale: &str) -> Option<&FileString> {
        match self {
            Self::Content(c) => Some(c),
            Self::Locales(m) => m.get(locale),
        }
    }
}

mod helpers {
    use super::{TranslationContent, TranslationContentInput};
    use crate::support::file_reference::FileString;
    use rumbas_support::preamble::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::convert::From;

    #[derive(Input)]
    #[input(name = "TranslationInput")]
    #[derive(Clone, Serialize, Deserialize)]
    #[serde(untagged)] // TODO: custom visitor?
    pub enum Translation {
        Normal(TranslationStruct),
        Short(FileString),
    }

    #[derive(Input)]
    #[input(name = "TranslationStructInput")]
    #[derive(Clone, Serialize, Deserialize)]
    pub struct TranslationStruct {
        content: TranslationContent,
        placeholders: HashMap<String, Translation>,
    }

    impl From<TranslationStructInput> for super::TranslationInput {
        fn from(t: TranslationStructInput) -> Self {
            Self {
                content: t.content,
                placeholders: t.placeholders.real_map(|p| {
                    p.into_iter()
                        .map(|(k, v)| (k, v.real_map(|v| v.into())))
                        .collect()
                }),
            }
        }
    }

    impl From<TranslationInput> for super::TranslationInput {
        fn from(t: TranslationInput) -> Self {
            match t {
                TranslationInput::Normal(c) => c.0.into(),
                TranslationInput::Short(c) => c.into(),
            }
        }
    }

    impl From<super::TranslationInput> for TranslationInput {
        fn from(s: super::TranslationInput) -> Self {
            if s.placeholders.clone().map(|p| p.len()) == Some(0) {
                if let Some(ValueType::Normal(TranslationContentInput::Content(f))) = s.content.0 {
                    return Self::Short(f);
                }
            }
            Self::Normal(TranslationStructInputEnum(TranslationStructInput {
                content: s.content,
                placeholders: s.placeholders.real_map(|p| {
                    p.into_iter()
                        .map(|(k, v)| (k, v.real_map(|v| v.into())))
                        .collect()
                }),
            }))
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "TranslationInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[input(from = "helpers::TranslationInput")]
#[input(into = "helpers::TranslationInput")]
pub struct Translation {
    content: TranslationContent,
    placeholders: HashMap<String, Translation>,
}

impl From<FileStringInput> for TranslationInput {
    fn from(f: FileStringInput) -> Self {
        Self {
            content: Value::Normal(f.into()),
            placeholders: Value::Normal(HashMap::new()),
        }
    }
}

impl From<FileStringInput> for TranslationInputEnum {
    fn from(f: FileStringInput) -> Self {
        Self(f.into())
    }
}

impl From<FileString> for Translation {
    fn from(f: FileString) -> Self {
        Self {
            content: f.into(),
            placeholders: HashMap::new(),
        }
    }
}

impl Examples for TranslationInputEnum {
    fn examples() -> Vec<Self> {
        TranslationInput::examples()
            .into_iter()
            .map(TranslationInputEnum)
            .collect()
    }
}

impl Examples for TranslationInput {
    fn examples() -> Vec<Self> {
        let contents = TranslationContentInput::examples();
        let placeholder_keys = vec!["placeholder1".to_string(), "placeholder2".to_string()];
        let placeholder_values = vec![
            ValueType::Normal(TranslationInput {
                content: Value::Normal(TranslationContentInput::Locales(
                    vec![
                        (
                            "nl".to_string(),
                            ValueType::Normal(FileStringInput::from(AnyString::from(
                                "nl value of placeholder1",
                            ))),
                        ),
                        (
                            "en".to_string(),
                            ValueType::Normal(FileStringInput::from(AnyString::from(
                                "en value of placeholder1",
                            ))),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                )),
                placeholders: Value::Normal(HashMap::new()),
            }),
            ValueType::Normal(TranslationInput {
                content: Value::Normal(TranslationContentInput::Locales(
                    vec![
                        (
                            "nl".to_string(),
                            ValueType::Normal(FileStringInput::from(AnyString::from(
                                "nl version of placeholder2",
                            ))),
                        ),
                        (
                            "en".to_string(),
                            ValueType::Normal(FileStringInput::from(AnyString::from(
                                "en version of placeholder2",
                            ))),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                )),
                placeholders: Value::Normal(HashMap::new()),
            }),
        ];
        let placeholders: HashMap<_, _> = placeholder_keys
            .into_iter()
            .zip(placeholder_values.into_iter())
            .collect();
        contents
            .clone()
            .into_iter()
            .map(|c| TranslationInput {
                content: Value::Normal(c),
                placeholders: Value::Normal(placeholders.clone()),
            })
            .chain(contents.into_iter().filter_map(|c| match c {
                TranslationContentInput::Content(_) => Some(TranslationInput {
                    content: Value::Normal(c),
                    placeholders: Value::Normal(Default::default()),
                }),
                _ => None,
            }))
            .collect()
    }
}

impl Translation {
    pub fn to_string(&self, locale: &str) -> Option<String> {
        //TODO: check for infinite loops / recursion? -> don't substitute something that is already
        //substituted
        fn substitute(
            pattern: &Option<String>,
            locale: &str,
            translation: &Translation,
        ) -> Option<String> {
            pattern.as_ref().and_then(|pattern| {
                let mut result = pattern.to_string();
                let mut substituted = false;
                for (placeholder, val) in translation.placeholders.iter() {
                    let before = result.clone();
                    if let Some(v) = val.to_string(locale) {
                        let key = format!("{{{}}}", placeholder);
                        result = result.replace(&key[..], &v);
                        substituted = substituted || before != result;
                    } else {
                        return None;
                    }
                }
                if substituted {
                    substitute(&Some(result), locale, translation)
                } else {
                    Some(result)
                }
            })
        }
        self.content
            .get(locale)
            .and_then(|s| substitute(&s.get_content(locale), locale, self))
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

            #[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq, Eq)]
            pub struct [<$type Input>](TranslationInputEnum);

            impl std::convert::From<$subtype> for [<$type Input>] {
                fn from(sub: $subtype) -> Self {
                    let s: String = sub.into();
                    [<$type Input>](FileStringInput::s(&s).into())
                }
            }

            impl std::convert::From<$subtype> for $type {
                fn from(sub: $subtype) -> Self {
                    let s: String = sub.into();
                    $type(FileString::s(&s).into())
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

            #[derive(Debug, Clone, PartialEq, JsonSchema, Serialize, Deserialize, Comparable, Eq)]
            pub struct $type(Translation);

            impl Input for [<$type Input>] {
                type Normal = $type;
                fn to_normal(&self) -> <Self as Input>::Normal {
                    $type(self.0.to_normal())
                }
                fn from_normal(normal: <Self as Input>::Normal) -> Self {
                       [<$type Input>](TranslationInputEnum::from_normal(normal.0))

                }
                fn find_missing(&self) -> InputCheckResult {
                    self.0.find_missing()
                }
                fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
                    self.0.insert_template_value(key, val)
                }

                fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
                    self.0.files_to_load(main_file_path)
                }

                fn insert_loaded_files(&mut self, main_file_path: &RumbasPath, files: &HashMap<FileToLoad, LoadedFile>) {
                    self.0.insert_loaded_files(main_file_path, files)
                }

                fn dependencies(&self, main_file_path: &RumbasPath) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
                    self.0.dependencies(main_file_path)
                }
            }

            impl InputInverse for $type {
                type Input = [<$type Input>];
                type EnumInput = [<$type Input>];
            }

            impl Overwrite<[<$type Input>]> for [<$type Input>] {
                fn overwrite(&mut self, _other: &[<$type Input>]) {
                    //TODO: Maybe add languages of other that are missing in self?
                    // These default values should be read before language is interpreted
                }
            }

            impl $type {
                pub fn to_string(&self, locale: &str) -> Option<String> {
                    self.0.to_string(locale)
                }
            }

            impl Examples for [<$type Input>] {
                fn examples() ->  Vec<Self> {
                    let translations = TranslationInputEnum::examples();
                    translations.into_iter().map(Self).collect()
                }
            }
        }
    };
}

use translatable_type;
