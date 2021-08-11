use crate::data::input_string::InputString;
use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;

/// The prefix used to specify a file reference
const FILE_PREFIX: &str = "file:";

macro_rules! file_type {
    (
        $(#[$outer:meta])*
        type $type: ident,
        subtype $subtype: ty
    ) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(from = "String")]
        #[serde(into = "String")]
        $(
            #[$outer]
        )*
        pub struct $type {
            file_name: Option<String>,
            content: Option<$subtype>,
            translated_content: HashMap<String, $subtype>,
            error_message: Option<String>,
        }
        impl RumbasCheck for $type {
            fn check(&self) -> RumbasCheckResult {
                if let Some(e) = &self.error_message {
                    RumbasCheckResult::from_missing(Some(e.clone()))
                } else {
                    RumbasCheckResult::empty()
                }
            }
        }
        impl OptionalOverwrite<$type> for $type {
            fn overwrite(&mut self, _other: &$type) {}
            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
        }
        impl_optional_overwrite_value!($type);

        //TODO: error message is not shown if no file found
        impl std::convert::From<String> for $type {
            fn from(s: String) -> Self {
                if s.starts_with(FILE_PREFIX) {
                    if s == FILE_PREFIX {
                        Self {
                            file_name: Some("".to_string()),
                            content: None,
                            translated_content: HashMap::new(),
                            error_message: Some("Missing filename".to_string()),
                        }
                    } else {
                        let relative_file_name = s.split(FILE_PREFIX).collect::<Vec<&str>>()[1];
                        let file_path = Path::new(crate::QUESTIONS_FOLDER).join(relative_file_name);
                        let file_name = file_path.file_name().unwrap().to_str().unwrap(); //TODO
                        if let Some(file_dir) = file_path.parent() {
                            //Look for translation dirs
                            let mut translated_content = HashMap::new();
                            for entry in file_dir.read_dir().expect("read_dir call failed").flatten()
                            // We only care about the ones that are 'Ok'
                            {
                                if let Ok(entry_name) = entry.file_name().into_string() {
                                    //println!("{}", entry_name);
                                    if entry_name.starts_with("locale-") {
                                        let locale = entry_name
                                            .splitn(2, "locale-")
                                            .collect::<Vec<_>>()
                                            .get(1)
                                            .unwrap()
                                            .to_string();
                                        let locale_file_path = file_dir.join(entry_name).join(file_name);
                                        //println!("{}", locale_file_path.display());
                                        if locale_file_path.exists() {
                                            if let Ok(s) = std::fs::read_to_string(&locale_file_path) {
                                                //println!("{}", s);
                                                if let Ok(s) = s.clone().try_into() {
                                                    translated_content.insert(
                                                        locale,
                                                        s,
                                                    );
                                                }
                                                else {
                                                    log::warn!("Failed converting content in {}", locale_file_path.display());
                                                }
                                            }
                                            else {
                                                log::warn!("Failed reading {}", locale_file_path.display());
                                            }
                                        }
                                    }
                                }
                            }

                            let content = std::fs::read_to_string(&file_path)
                                .ok();
                            if content.is_none() && translated_content.is_empty() {
                                Self {
                                    file_name: Some(relative_file_name.to_string()),
                                    content: None,
                                    translated_content: HashMap::new(),
                                    error_message: Some(relative_file_name.to_string()),
                                }
                            } else {
                                if let Some(content) = content {
                                    let content = content.try_into().ok();
                                    if content.is_some() {
                                        Self {
                                            file_name: Some(relative_file_name.to_string()),
                                            content,
                                            translated_content,
                                            error_message: None,
                                        }
                                    }
                                    else {
                                        Self {
                                            file_name: Some(relative_file_name.to_string()),
                                            content: None,
                                            translated_content,
                                            error_message: Some(format!("Failed converting content in {}", file_path.display())),
                                        }
                                    }
                                } else {
                                    Self {
                                        file_name: Some(relative_file_name.to_string()),
                                        content: None,
                                        translated_content,
                                        error_message: None,
                                    }
                                }
                            }
                        } else {
                            Self {
                                file_name: Some(relative_file_name.to_string()),
                                content: None,
                                translated_content: HashMap::new(),
                                error_message: Some(relative_file_name.to_string()),
                            }
                        }
                    }
                } else {
                    Self::s(&s)
                }
            }
        }

        // Currently only implemented for conversion from numbas to rumbas: so not with translations or
        // file references
        impl std::convert::From<$type> for String {
            fn from(fs: $type) -> Self {
                if fs.file_name.is_some() || !fs.translated_content.is_empty() || fs.content.is_none() {
                    panic!("Deserializing FileRef only supported when plain String")
                }
                fs.content.unwrap().into()
            }
        }

        impl $type {
            pub fn get_content(&self, locale: &str) -> String {
                if let Some(c) = self.translated_content.get(locale) {
                    return c.clone().into();
                }
                if let Some(c) = &self.content {
                    return c.clone().into();
                }
                panic!("Missing translation for locale {}", locale); //TODO
            }
            pub fn s(content: &str) -> $type {
                let content = content.to_string().try_into();
                let error_message = if let Err(e) = content {
                    Some(format!("Invalid file content: {}", e))
                } else { None };
                $type {
                    file_name: None,
                    content: content.ok(),
                    translated_content: HashMap::new(),
                    error_message,
                }
            }
        }
    }
}

file_type! {
    /// A string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type FileString,
    subtype InputString
}

file_type! {
    /// A JME string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type JMEFileString,
    subtype String
}

file_type! {
    /// An embraced JME string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type EmbracedJMEFileString,
    subtype String
}
