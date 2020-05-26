use crate::data::exam::{Feedback, Navigation, Question, Timing};
use std::fs;
use std::path::{Path, PathBuf};

//TODO Tests

#[derive(Debug)]
pub struct DefaultFile {
    r#type: DefaultFileType,
    path: PathBuf,
}

#[derive(Debug)]
pub enum DefaultFileType {
    Navigation,
    Timing,
    Feedback,
    Question,
}

pub enum DefaultData {
    Navigation(Navigation),
    Timing(Timing),
    Feedback(Feedback),
    Question(Question),
}

impl DefaultFileType {
    fn from(path: &Path) -> Option<DefaultFileType> {
        let file_name = path.file_stem();
        match file_name {
            Some(f) => match f.to_str() {
                Some("navigation") => Some(DefaultFileType::Navigation),
                Some("timing") => Some(DefaultFileType::Timing),
                Some("feedback") => Some(DefaultFileType::Feedback),
                Some("question") => Some(DefaultFileType::Question),
                _ => None,
            },
            None => None,
        }
    }
    fn read_as_data(&self, path: &PathBuf) -> serde_json::Result<DefaultData> {
        let json = fs::read_to_string(path).unwrap();
        match self {
            DefaultFileType::Navigation => {
                let n: Navigation = serde_json::from_str(&json)?;
                Ok(DefaultData::Navigation(n))
            }
            DefaultFileType::Timing => {
                let t: Timing = serde_json::from_str(&json)?;
                Ok(DefaultData::Timing(t))
            }
            DefaultFileType::Feedback => {
                let f: Feedback = serde_json::from_str(&json)?;
                Ok(DefaultData::Feedback(f))
            }
            DefaultFileType::Question => {
                let q: Question = serde_json::from_str(&json)?;
                Ok(DefaultData::Question(q))
            }
        }
    }
}

impl DefaultFile {
    fn from(path: &Path) -> Option<DefaultFile> {
        let default_type: Option<DefaultFileType> = DefaultFileType::from(&path);
        if let Some(t) = default_type {
            return Some(DefaultFile {
                r#type: t,
                path: path.to_path_buf(),
            });
        }
        None
    }

    pub fn read_as_data(&self) -> serde_json::Result<DefaultData> {
        self.r#type.read_as_data(&self.path)
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

pub fn default_files(path: &Path) -> Vec<DefaultFile> {
    let paths = default_file_paths(path);
    let usefull_paths = paths
        .into_iter()
        .map(|p| DefaultFile::from(&p))
        .filter(|p| p.is_some());
    usefull_paths.into_iter().map(|p| p.unwrap()).collect()
}

fn default_file_paths(path: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let ancestors = path.ancestors();
    for a in ancestors {
        let defaults_path = a.with_file_name("defaults");
        if defaults_path.is_dir() {
            for entry in defaults_path.read_dir().expect("read_dir call failed") {
                if let Ok(entry) = entry {
                    result.push(entry.path()); //TODO: order files from the folder
                    println!("{:?}", entry.path());
                }
            }
        }
    }

    result
}
