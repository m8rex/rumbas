use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

const FILE_PREFIX: &'static str = "file:";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct FileString {
    file_name: Option<String>,
    content: String,
}
impl_optional_overwrite!(FileString);

impl std::convert::TryFrom<String> for FileString {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.starts_with(FILE_PREFIX) {
            if s == FILE_PREFIX {
                Err(format!("Missing filename after {}", FILE_PREFIX))
            } else {
                let file_name = s.split(FILE_PREFIX).collect::<Vec<&str>>()[1];
                let file_path = Path::new("questions").join(file_name);
                std::fs::read_to_string(&file_path)
                    .map(|s| FileString {
                        file_name: Some(file_name.to_string()),
                        content: s.clone(),
                    })
                    .map_err(|e| format!("Failed to read {}: {}", file_path.to_str().unwrap(), e))
                //TODO?
            }
        } else {
            Ok(FileString {
                file_name: None,
                content: s.clone(),
            })
        }
    }
}

impl FileString {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn s(content: &String) -> FileString {
        FileString {
            file_name: None,
            content: content.clone(),
        }
    }
}
