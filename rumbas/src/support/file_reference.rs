use crate::support::input_string::InputString;
use crate::support::to_numbas::ToNumbas;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;

/// The prefix used to specify a file reference
const FILE_PREFIX: &str = "file";

macro_rules! file_type {
    (
        $(#[$outer:meta])*
        type $type: ident,
        subtype $subtype: ty,
        rumbas_check $check_expr: expr
    ) => {
        paste::paste! {
            #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
            #[serde(from = "String")]
            #[serde(into = "String")]
            $(
                #[$outer]
            )*
            pub struct [<$type Input>] {
                file_name: Option<String>,
                content: Option<String>,
                translated_content: HashMap<String, String>,
                error_message: Option<String>,
            }
            #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
            #[serde(into = "String")]
            $(
                #[$outer]
            )*
            pub struct $type {
                file_name: Option<String>,
                content: Option<String>,
                translated_content: HashMap<String, String>,
            }
            impl RumbasCheck for $type {
                fn check(&self, locale: &str) -> RumbasCheckResult {
                    let content = self.get_content(locale);
                    match content {
                        Some(c) => {
                            let conversion_res: Result<$subtype, _> = c.try_into();
                            match conversion_res {
                                Ok(_) => RumbasCheckResult::empty(),
                                Err(e) => $check_expr(e),
                            }
                        }
                        None => RumbasCheckResult::from_missing_translation(Some(locale.to_string())),
                    }
                }
            }
            impl Overwrite< [<$type Input>]> for  [<$type Input>] {
                fn overwrite(&mut self, _other: &[<$type Input>]) {}
            }

            impl ToNumbas<String> for $type {
                fn to_numbas(&self, locale: &str)-> String {
                    self.get_content(locale).unwrap()
                }
            }

            impl ToNumbas<$subtype> for $type {
                fn to_numbas(&self, locale: &str)-> $subtype {
                    self.get_content(locale).unwrap().try_into().unwrap()
                }
            }

            impl JsonSchema for $type { // TODO: needed?
                fn schema_name() -> String {
                    stringify!($type).to_owned()
                }

                fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                    let file_schema = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::String.into()),
                        string: Some(Box::new(schemars::schema::StringValidation {
                            min_length: Some(1 + (FILE_PREFIX.len() as u32)),
                            max_length: None,
                            pattern: Some(format!("^{}:.*$", FILE_PREFIX)),
                        })),
                        ..Default::default()
                    };
                    schemars::schema::SchemaObject {
                        subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                            any_of: Some(vec![file_schema.into(), gen.subschema_for::<String>()]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }
                    .into()
                }
            }

            impl JsonSchema for [<$type Input>] {
                fn schema_name() -> String {
                    stringify!([<$type Input>]).to_owned()
                }

                fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                    let file_schema = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::String.into()),
                        string: Some(Box::new(schemars::schema::StringValidation {
                            min_length: Some(1 + (FILE_PREFIX.len() as u32)),
                            max_length: None,
                            pattern: Some(format!("^{}:.*$", FILE_PREFIX)),
                        })),
                        ..Default::default()
                    };
                    schemars::schema::SchemaObject {
                        subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                            any_of: Some(vec![file_schema.into(), gen.subschema_for::<String>()]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }
                    .into()
                }
            }

            impl std::convert::From<String> for [<$type Input>] {
                fn from(s: String) -> Self {
                    let mut prefix = FILE_PREFIX.to_owned();
                    prefix.push(':');
                    if s.starts_with(&prefix) {
                        if s == prefix {
                            Self {
                                file_name: Some("".to_string()),
                                content: None,
                                translated_content: HashMap::new(),
                                error_message: Some("Missing filename".to_string()),
                            }
                        } else {
                            let relative_file_name = s.split(&prefix).collect::<Vec<&str>>()[1];
                            let file_path = Path::new(crate::QUESTIONS_FOLDER).join(relative_file_name);
                            let file_name = file_path.file_name().unwrap().to_str().unwrap(); //TODO
                            if let Some(file_dir) = file_path.parent() {
                                //Look for translation dirs
                                let mut translated_content = HashMap::new();
                                for entry in file_dir.read_dir().expect("read_dir call failed").flatten()
                                // We only care about the ones that are 'Ok'
                                {
                                    if let Ok(entry_name) = entry.file_name().into_string() {
                                        if entry_name.starts_with("locale-") { // TODO: locale prefix?
                                            let locale = entry_name
                                                .splitn(2, "locale-")
                                                .collect::<Vec<_>>()
                                                .get(1)
                                                .unwrap()
                                                .to_string();
                                            let locale_file_path = file_dir.join(entry_name).join(file_name);
                                            if locale_file_path.exists() {
                                                if let Ok(s) = std::fs::read_to_string(&locale_file_path) {
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
                                    if let Some(_) = content {
                                            Self {
                                                file_name: Some(relative_file_name.to_string()),
                                                content,
                                                translated_content,
                                                error_message: None,
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
            impl std::convert::From<[<$type Input>]> for String {
                fn from(fs: [<$type Input>]) -> Self {
                    if fs.file_name.is_some() || !fs.translated_content.is_empty() || fs.content.is_none() {
                        panic!("Deserializing FileRef only supported when plain String")
                    }
                    fs.content.unwrap().into()
                }
            }

            // Currently only implemented for conversion from numbas to rumbas: so not with translations or
            // file references
            impl std::convert::From<[<$type>]> for String {
                fn from(fs: [<$type>]) -> Self {
                    if fs.file_name.is_some() || !fs.translated_content.is_empty() || fs.content.is_none() {
                        panic!("Deserializing FileRef only supported when plain String")
                    }
                    fs.content.unwrap().into()
                }
            }

            impl $type {
                pub fn get_content(&self, locale: &str) -> Option<String> {
                    if let Some(c) = self.translated_content.get(locale) {
                        Some(c.clone().into())
                    } else {
                        self.content.as_ref().map(|c| c.clone().into())
                    }
                }
                pub fn s(content: &str) -> Self {
                    let content = content.to_string().try_into();
                    Self {
                        file_name: None,
                        content: content.clone().ok(),
                        translated_content: HashMap::new(),
                    }
                }
            }
            impl [<$type Input>] {
                pub fn s(content: &str) -> Self {
                    let content = content.to_string();
                    Self {
                        file_name: None,
                        content: Some(content),
                        translated_content: HashMap::new(),
                        error_message: None,
                    }
                }
            }
            impl Input for [<$type Input>] {
                type Normal = $type;
                fn to_normal(&self) -> Self::Normal {
                    Self::Normal {
                        file_name: self.file_name.to_owned(),
                        content: self.content.to_owned(),
                        translated_content: self.translated_content.to_owned(),
                    }
                }
                fn from_normal(normal: Self::Normal) -> Self {
                    Self {
                        file_name: normal.file_name,
                        content: normal.content,
                        translated_content: normal.translated_content,
                        error_message: None
                    }
                }
                fn find_missing(&self) -> InputCheckResult {
                    if let Some(e) = &self.error_message {
                        InputCheckResult::from_missing(Some(e.clone()))
                    } else {
                        InputCheckResult::empty()
                    }
                }

                fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
            }
            impl InputInverse for $type {
                type Input = [<$type Input>];
            }
        }
    }
}

file_type! {
    /// A string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type FileString,
    subtype InputString,
    rumbas_check |_e| RumbasCheckResult::empty() // never happens
}

file_type! {
    /// A JME string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type JMEFileString,
    subtype JMEString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)
}

file_type! {
    /// An embraced JME string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type EmbracedJMEFileString,
    subtype EmbracedJMEString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)
}

file_type! {
    /// An ContentArea string that has to be read from a file.
    ///
    /// Specified by a string starting with [FILE_PREFIX].
    type ContentAreaFileString,
    subtype ContentAreaString,
    rumbas_check |e| RumbasCheckResult::from_invalid_jme(&e)
}
