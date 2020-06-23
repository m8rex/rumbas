use crate::data::input_string::InputString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

const FILE_PREFIX: &'static str = "file:";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct FileString {
    file_name: Option<String>,
    content: Option<InputString>,
    translated_content: HashMap<String, InputString>,
}
impl_optional_overwrite!(FileString);

//TODO: error message is not shown if no file found
impl std::convert::TryFrom<String> for FileString {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.starts_with(FILE_PREFIX) {
            if s == FILE_PREFIX {
                Err(format!("Missing filename after {}", FILE_PREFIX))
            } else {
                let relative_file_name = s.split(FILE_PREFIX).collect::<Vec<&str>>()[1];
                let file_path = Path::new("questions").join(relative_file_name);
                let file_name = file_path.file_name().unwrap().to_str().unwrap(); //TODO
                if let Some(file_dir) = file_path.parent() {
                    //Look for translation dirs
                    let mut translated_content = HashMap::new();
                    for entry in file_dir.read_dir().expect("read_dir call failed") {
                        //TODO
                        if let Ok(entry) = entry {
                            if let Ok(entry_name) = entry.file_name().into_string() {
                                println!("{}", entry_name);
                                if entry_name.starts_with("locale-") {
                                    let locale = entry_name
                                        .splitn(2, "locale-")
                                        .collect::<Vec<_>>()
                                        .get(1)
                                        .unwrap()
                                        .to_string();
                                    let locale_file_path =
                                        file_dir.join(entry_name).join(file_name);
                                    println!("{}", locale_file_path.display());
                                    if locale_file_path.exists() {
                                        if let Ok(s) = std::fs::read_to_string(&locale_file_path) {
                                            println!("{}", s);
                                            translated_content
                                                .insert(locale, InputString::from(s.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let content = std::fs::read_to_string(&file_path)
                        .map(|s| InputString::from(s.clone()))
                        .ok();
                    if content.is_none() && translated_content.len() == 0 {
                        Err(format!("Failed to read {}", file_path.to_str().unwrap()))
                    } else {
                        Ok(FileString {
                            file_name: Some(relative_file_name.to_string()),
                            content,
                            translated_content,
                        })
                    }
                } else {
                    Err(format!("Failed to read {}", file_path.to_str().unwrap()))
                }
            }
        } else {
            Ok(FileString::s(&s))
        }
    }
}

impl FileString {
    pub fn get_content(&self, locale: &String) -> String {
        if let Some(c) = self.translated_content.get(locale) {
            return c.0.clone();
        }
        if let Some(c) = &self.content {
            return c.0.clone();
        }
        panic!(format!("Missing translation for locale {}", locale)); //TODO
    }

    pub fn s(content: &String) -> FileString {
        FileString {
            file_name: None,
            content: Some(InputString::from(content.clone())),
            translated_content: HashMap::new(),
        }
    }
}
