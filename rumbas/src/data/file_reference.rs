use crate::data::input_string::InputString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

const FILE_PREFIX: &'static str = "file:";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "String")]
pub struct FileString {
    file_name: Option<String>,
    content: Option<InputString>,
    translated_content: HashMap<String, InputString>,
    error_message: Option<String>,
}
impl OptionalOverwrite for FileString {
    type Item = FileString;
    fn empty_fields(&self) -> Vec<String> {
        if let Some(e) = &self.error_message {
            vec![e.clone()]
        } else {
            Vec::new()
        }
    }
    fn overwrite(&mut self, _other: &Self::Item) {}
}
impl_optional_overwrite_option!(FileString);

//TODO: error message is not shown if no file found
impl std::convert::From<String> for FileString {
    fn from(s: String) -> Self {
        if s.starts_with(FILE_PREFIX) {
            if s == FILE_PREFIX {
                FileString {
                    file_name: Some("".to_string()),
                    content: None,
                    translated_content: HashMap::new(),
                    error_message: Some("Missing filename".to_string()),
                }
            } else {
                let relative_file_name = s.split(FILE_PREFIX).collect::<Vec<&str>>()[1];
                let file_path = Path::new("questions").join(relative_file_name);
                let file_name = file_path.file_name().unwrap().to_str().unwrap(); //TODO
                if let Some(file_dir) = file_path.parent() {
                    //Look for translation dirs
                    let mut translated_content = HashMap::new();
                    for entry in file_dir.read_dir().expect("read_dir call failed") {
                        if let Ok(entry) = entry {
                            if let Ok(entry_name) = entry.file_name().into_string() {
                                //println!("{}", entry_name);
                                if entry_name.starts_with("locale-") {
                                    let locale = entry_name
                                        .splitn(2, "locale-")
                                        .collect::<Vec<_>>()
                                        .get(1)
                                        .unwrap()
                                        .to_string();
                                    let locale_file_path =
                                        file_dir.join(entry_name).join(file_name);
                                    //println!("{}", locale_file_path.display());
                                    if locale_file_path.exists() {
                                        if let Ok(s) = std::fs::read_to_string(&locale_file_path) {
                                            //println!("{}", s);
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
                        FileString {
                            file_name: Some(relative_file_name.to_string()),
                            content: None,
                            translated_content: HashMap::new(),
                            error_message: Some(relative_file_name.to_string()),
                        }
                    } else {
                        FileString {
                            file_name: Some(relative_file_name.to_string()),
                            content,
                            translated_content,
                            error_message: None,
                        }
                    }
                } else {
                    FileString {
                        file_name: Some(relative_file_name.to_string()),
                        content: None,
                        translated_content: HashMap::new(),
                        error_message: Some(relative_file_name.to_string()),
                    }
                }
            }
        } else {
            FileString::s(&s)
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
            error_message: None,
        }
    }
}
