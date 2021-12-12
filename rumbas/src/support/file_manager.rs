use crate::exam::ExamInput;
use crate::question::custom_part_type::CustomPartTypeDefinitionInput;
use crate::question::QuestionInput;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

#[derive(Debug)]
pub struct FileManager {
    cache: RwLock<HashMap<FileToRead, Mutex<ReadFile>>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum FileToRead {
    Text(TextFileToRead),
    CustomPartType(CustomPartTypeFileToRead),
    Question(QuestionFileToRead),
    Exam(ExamFileToRead),
}

impl std::convert::From<FileToRead> for rumbas_support::input::FileToLoad {
    fn from(s: FileToRead) -> Self {
        match s {
            FileToRead::Text(t) => t.into(),
            FileToRead::CustomPartType(t) => t.into(),
            FileToRead::Question(t) => t.into(),
            FileToRead::Exam(t) => t.into(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TextFileToRead {
    file_path: PathBuf,
}

impl TextFileToRead {
    pub fn with_file_name(file_name: String) -> Self {
        let file_path = std::path::Path::new(crate::QUESTIONS_FOLDER).join(file_name);
        Self { file_path }
    }
}

impl std::convert::From<TextFileToRead> for FileToRead {
    fn from(s: TextFileToRead) -> Self {
        FileToRead::Text(s)
    }
}

impl std::convert::From<TextFileToRead> for rumbas_support::input::FileToLoad {
    fn from(s: TextFileToRead) -> Self {
        Self {
            file_path: s.file_path,
            locale_dependant: true,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct CustomPartTypeFileToRead {
    file_path: PathBuf,
}

impl CustomPartTypeFileToRead {
    pub fn with_file_name(file_name: String) -> Self {
        let file_path = std::path::Path::new(crate::CUSTOM_PART_TYPES_FOLDER).join(file_name);
        Self { file_path }
    }
}

impl std::convert::From<CustomPartTypeFileToRead> for FileToRead {
    fn from(s: CustomPartTypeFileToRead) -> Self {
        FileToRead::CustomPartType(s)
    }
}

impl std::convert::From<CustomPartTypeFileToRead> for rumbas_support::input::FileToLoad {
    fn from(s: CustomPartTypeFileToRead) -> Self {
        Self {
            file_path: s.file_path,
            locale_dependant: false,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct QuestionFileToRead {
    file_path: PathBuf,
}

impl QuestionFileToRead {
    pub fn with_file_name(file_name: String) -> Self {
        let file_path = std::path::Path::new(crate::QUESTIONS_FOLDER).join(file_name);
        Self { file_path }
    }
}

impl std::convert::From<QuestionFileToRead> for FileToRead {
    fn from(s: QuestionFileToRead) -> Self {
        FileToRead::Question(s)
    }
}

impl std::convert::From<QuestionFileToRead> for rumbas_support::input::FileToLoad {
    fn from(s: QuestionFileToRead) -> Self {
        Self {
            file_path: s.file_path,
            locale_dependant: false,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ExamFileToRead {
    file_path: PathBuf,
}

impl std::convert::From<ExamFileToRead> for FileToRead {
    fn from(s: ExamFileToRead) -> Self {
        FileToRead::Exam(s)
    }
}

impl std::convert::From<ExamFileToRead> for rumbas_support::input::FileToLoad {
    fn from(s: ExamFileToRead) -> Self {
        Self {
            file_path: s.file_path,
            locale_dependant: false,
        }
    }
}

#[derive(Debug)]
pub enum ReadFile {
    Text(ReadTextFile),
    CustomPartType(ReadCustomPartTypeFile),
    Question(ReadQuestionFile),
    Exam(ReadExamFile),
}

#[derive(Debug)]
pub struct ReadTextFile {
    file_path: String,
    text: String,
}

#[derive(Debug)]
pub struct ReadCustomPartTypeFile {
    file_path: String,
    custom_part_type: CustomPartTypeDefinitionInput,
}

#[derive(Debug)]
pub struct ReadQuestionFile {
    file_path: String,
    question: QuestionInput,
}

#[derive(Debug)]
pub struct ReadExamFile {
    file_path: String,
    exam: ExamInput,
}

macro_rules! create_from_string_type {
    ($t: ident, $ti: ident, $data: ty, $datai: ty, $read_type: ty, $n_type: ty, $schema: literal) => {
        // TODO: remove this JsonSchema
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
        pub struct $t {
            pub file_name: String,
            pub data: $data,
        }
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(from = "String")]
        #[serde(into = "String")]
        pub struct $ti {
            pub file_name: String,
            pub data: Option<$datai>,
            pub error_message: Option<String>,
        }

        impl InputInverse for $t {
            type Input = $ti;
            type EnumInput = $ti;
        }

        impl Examples for $ti {
            fn examples() -> Vec<Self> {
                vec![Self {
                    file_name: "path".to_string(),
                    data: None,
                    error_message: None,
                }]
            }
        }
        impl $ti {
            pub fn file_to_read(&self) -> Option<FileToRead> {
                if let Some(q) = self.data {
                    None
                } else {
                    Some(<$read_type>::with_file_name(self.file_name).into())
                }
            }
        }

        impl Input for $ti {
            type Normal = $t;
            fn to_normal(&self) -> Self::Normal {
                Self::Normal {
                    file_name: self.file_name.to_owned(),
                    data: self.data.unwrap().to_normal(),
                }
            }
            fn from_normal(normal: Self::Normal) -> Self {
                Self {
                    file_name: normal.file_name,
                    data: Some(Input::from_normal(normal.data)),
                    error_message: None,
                }
            }
            fn find_missing(&self) -> InputCheckResult {
                if let Some(q) = self.data {
                    q.find_missing()
                } else {
                    InputCheckResult::from_missing(Some(self.file_name.clone()))
                }
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
                if let Some(ref mut q) = self.data {
                    q.insert_template_value(key, val);
                }
            }
            fn files_to_load(&self) -> Vec<FileToLoad> {
                if let Some(file) = self.file_to_read() {
                    vec![file.into()]
                } else if let Some(q) = self.data {
                    q.files_to_load()
                } else {
                    unreachable!();
                }
            }

            fn insert_loaded_files(
                &mut self,
                files: &std::collections::HashMap<FileToLoad, LoadedFile>,
            ) {
                if let Some(q) = self.data {
                    q.insert_loaded_files(files);
                } else {
                    let file = self.file_to_read();
                    if let Some(f) = file {
                        let file_to_load: FileToLoad = f.into();
                        let file = files.get(&file_to_load);
                        match file {
                            Some(LoadedFile::Normal(n)) => {
                                let data_res = <$datai>::from_str(
                                    &n.content[..],
                                    file_to_load.file_path.clone(),
                                );
                                match data_res {
                                    Ok(q) => self.data = Some(q.clone()),
                                    Err(e) => self.error_message = Some(e.to_string()),
                                }
                            }
                            Some(LoadedFile::Localized(l)) => {
                                unreachable!()
                            }
                            None => {
                                self.error_message =
                                    Some(format!("Missing file: {}", self.file_name))
                            }
                        }
                    }
                }
            }
        }

        impl RumbasCheck for $t {
            fn check(&self, locale: &str) -> RumbasCheckResult {
                self.data.check(locale)
            }
        }

        impl Overwrite<$ti> for $ti {
            fn overwrite(&mut self, _other: &Self) {}
        }

        impl ToNumbas<$n_type> for $t {
            fn to_numbas(&self, locale: &str) -> $n_type {
                self.data
                    .clone()
                    .to_numbas_with_name(locale, self.file_name.clone())
            }
        }

        impl ToRumbas<$t> for $n_type {
            fn to_rumbas(&self) -> $t {
                $t {
                    file_name: sanitize(&self.name),
                    data: self.to_rumbas(),
                }
            }
        }

        impl JsonSchema for $ti {
            fn schema_name() -> String {
                $schema.to_owned()
            }

            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                gen.subschema_for::<String>()
            }
        }

        impl std::convert::From<String> for $ti {
            fn from(s: String) -> Self {
                //let question_data = QuestionInput::from_name(&s).map_err(|e| e)?;
                Self {
                    file_name: s,
                    data: None,
                    error_message: None,
                }
            }
        }

        impl std::convert::From<$ti> for String {
            fn from(q: $ti) -> Self {
                /*let q_yaml = crate::question::QuestionFileTypeInput::Normal(Box::new(q.question_data))
                    .to_yaml()
                    .unwrap();
                let file = format!("{}/{}.yaml", crate::QUESTIONS_FOLDER, q.question_name);
                log::info!("Writing to {}", file);
                std::fs::write(file, q_yaml).unwrap(); //fix handle result (try_from)
                */
                q.file_name
            }
        }

        impl std::hash::Hash for $t {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.file_name.hash(state);
            }
        }
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.file_name == other.file_name
            }
        }
        impl Eq for $t {}

        impl PartialEq for $ti {
            fn eq(&self, other: &Self) -> bool {
                self.file_name == other.file_name
            }
        }
        impl Eq for $ti {}
    };
}
pub(crate) use create_from_string_type;