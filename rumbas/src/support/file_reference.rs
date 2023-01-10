use crate::support::file_manager::{FileToRead, TextFileToRead};
use crate::support::input_string::InputString;
use crate::support::to_numbas::ToNumbas;
use comparable::Comparable;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Into;
use std::convert::TryInto;
use structdoc::StructDoc;

/// The prefix used to specify a file reference
pub const FILE_PREFIX: &str = "file";

#[derive(Serialize, Deserialize, Comparable, StructDoc)]
#[serde(untagged)]
pub enum AnyString {
    Str(InputString),
    Isize(isize),
    Float(f64),
}

impl std::convert::From<AnyString> for String {
    fn from(a: AnyString) -> Self {
        match a {
            AnyString::Str(s) => s.into(),
            AnyString::Isize(v) => v.to_string(),
            AnyString::Float(v) => v.to_string(),
        }
    }
}

impl std::convert::From<&'static str> for AnyString {
    fn from(s: &'static str) -> Self {
        Self::Str(s.to_string().into())
    }
}

macro_rules! file_type {
    (
        $(#[$outer:meta])*
        type $type: ident,
        subtype $subtype: ty,
        rumbas_check $check_expr: expr
    ) => {
        paste::paste! {
            #[derive(Serialize, Deserialize, Comparable, Debug, Clone, PartialEq, Eq)]
            #[serde(from = "AnyString")]
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
            #[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Comparable, Eq)]
            #[serde(into = "String")]
            $(
                #[$outer]
            )*
            pub struct $type {
                file_name: Option<String>,
                content: Option<String>,
                translated_content: HashMap<String, String>,
            }
            /// Enum used for the documentation
            #[derive(StructDoc)]
            #[structdoc(untagged)]
            enum [<$type StructDocHelper>] {
                /// A string of the form `file:<filepath>` where `filepath` is the relative path (within the `exams` or `questions` folder) to a file containing content.
                /// This content can be localized by placing it in locale folders.
                /// e.g. `file:examples/basic-explanation.html` will search for files in folders
                /// with following form: `questions/examples/locale-<localename>/basic-explanation.html`
                /// If a file isn't found for a specific locale,
                /// `questions/examples/basic-explanation.html` will be used
                FileReference(String),
                /// A literal string.
                String(String),
            }
            impl StructDoc for $type {
                fn document() -> structdoc::Documentation {
                    [<$type StructDocHelper>]::document().rename(stringify!($type).to_string())
                }
            }
            impl Examples for [<$type Input>] {
                fn examples() -> Vec<Self> {
                    vec![Self { file_name: Some("path/to/file".to_string()), content: None, translated_content: Default::default(), error_message: None }] // TODO file: string
                }
            }
            impl Examples for $type {
                fn examples() -> Vec<Self> {
                    <[<$type Input>]>::examples().into_iter().map(|a| $type {
                        file_name: a.file_name,
                        content: a.content,
                        translated_content: a.translated_content
                    }).collect()
                }
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

            impl std::convert::From<AnyString> for [<$type Input>] {
                fn from(a: AnyString) -> Self {
                    let s : String = a.into();
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
                                Self {
                                    file_name: Some(relative_file_name.to_string()),
                                    content: None,
                                    translated_content: HashMap::new(),
                                    error_message: None
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
                    if let Some(file_name) = fs.file_name {
                        format!("{}:{}", FILE_PREFIX, file_name)
                    } else if let Some(content) = fs.content {
                        content.into()
                    } else {
                        panic!("Deserializing FileRef only supported when plain String or filename")
                    }
                }
            }

            // Currently only implemented for conversion from numbas to rumbas: so not with translations or
            // file references
            impl std::convert::From<[<$type>]> for String {
                fn from(fs: [<$type>]) -> Self {
                    if let Some(file_name) = fs.file_name {
                        format!("{}:{}", FILE_PREFIX, file_name)
                    } else if let Some(content) = fs.content {
                        content.into()
                    } else {
                        panic!("Deserializing FileRef only supported when plain String or filename")
                    }
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
                pub fn file_to_read(&self, main_file_path: &RumbasPath) -> Option<FileToRead> {
                    self.file_name.as_ref().map(|file_name| {
                        TextFileToRead::with_file_name(file_name.clone(), main_file_path).into()
                    })
                }
                pub fn is_loaded(&self) -> bool {
                    self.content.is_some() || !self.translated_content.is_empty() || self.error_message.is_some()
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

                fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
                    if self.is_loaded() {
                        Vec::new()
                    } else {
                        let file = self.file_to_read(main_file_path);
                            if let Some(f) = file {
                                vec![f.into()]
                            }
                            else { vec![] }
                    }
                }

                fn insert_loaded_files(&mut self, main_file_path: &RumbasPath, files: &HashMap<FileToLoad, LoadedFile>) {
                    if self.is_loaded() {
                        return
                    }
                    let file = self.file_to_read(main_file_path);
                    if let Some(f) = file {
                        let file : FileToLoad = f.into();
                        let found_file = files.get(&file);
                        match found_file {
                            Some(LoadedFile::Normal(n)) => {
                                self.content = Some(n.content.clone());
                            }
                            Some(LoadedFile::Localized(l)) => {
                                self.content = l.content.clone();
                                self.translated_content = l.localized_content.clone();
                            }
                            None => self.error_message = Some(format!("File not found: {}", file.file_path.display()))
                        }
                    }
                }

                fn dependencies(&self, main_file_path: &RumbasPath) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
                    self.file_to_read(main_file_path).map(|a| {
                        let p : rumbas_support::path::RumbasPath = a.into();
                        vec![p].into_iter().collect()
                    }).unwrap_or_else(std::collections::HashSet::new)
                }
            }
            impl InputInverse for $type {
                type Input = [<$type Input>];
                type EnumInput = [<$type Input>];
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
