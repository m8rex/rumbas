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
