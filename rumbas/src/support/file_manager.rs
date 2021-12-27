use crate::exam::ExamInput;
use crate::question::custom_part_type::CustomPartTypeDefinitionInput;
use crate::question::QuestionInput;
use rayon::prelude::*;
use rumbas_support::input::{FileToLoad, LoadedFile, LoadedLocalizedFile, LoadedNormalFile};
use std::convert::TryInto;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

lazy_static! {
    pub static ref CACHE: FileManager = FileManager::new();
}

#[derive(Debug)]
pub struct FileManager {
    cache: RwLock<HashMap<FileToLoad, Mutex<LoadedFile>>>,
    dir_cache: RwLock<HashMap<PathBuf, Mutex<RumbasRepoFolderEntries>>>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            dir_cache: RwLock::new(HashMap::new()),
        }
    }
}

impl FileManager {
    pub fn read_file(&self, file: FileToLoad) -> Option<LoadedFile> {
        let map = self.cache.read().expect("Can read cache map");
        log::debug!("Checking if {} is in the cache.", file.file_path.display());
        if let Some(val) = map.get(&file) {
            log::debug!("Found {} in the cache.", file.file_path.display());
            Some(val.lock().expect("unlock loaded file mutex").clone())
        } else {
            std::mem::drop(map); // remove the read lock

            let res = match file.locale_dependant {
                true => Self::read_localized_file(&file.file_path)
                    .map(rumbas_support::input::LoadedFile::Localized),
                false => Self::read_normal_file(&file.file_path)
                    .map(rumbas_support::input::LoadedFile::Normal),
            };
            match res {
                Ok(r) => {
                    let mut map = self.cache.write().expect("Can write cache map");
                    map.insert(file.clone(), Mutex::new(r.clone()));
                    Some(r.clone())
                }
                Err(()) => {
                    log::error!("Couldn't resolve {}", file.file_path.display());
                    None
                }
            }
        }
    }

    pub fn read(&self, files: Vec<FileToLoad>) -> HashMap<FileToLoad, LoadedFile> {
        let result: HashMap<_, _> = files
            .into_par_iter()
            .filter_map(|file| self.read_file(file.clone()).map(|l| (file, l.clone())))
            .collect();
        result
    }

    fn read_normal_file(file_path: &PathBuf) -> Result<LoadedNormalFile, ()> {
        log::debug!("Reading file {}.", file_path.display());
        match std::fs::read_to_string(&file_path) {
            Ok(content) => Ok(LoadedNormalFile {
                content,
                file_path: file_path.clone(),
            }),
            Err(e) => {
                log::error!(
                    "Failed read content of {} with error {}",
                    file_path.display(),
                    e
                );
                Err(())
            }
        }
    }

    fn read_localized_file(file_path: &PathBuf) -> Result<LoadedLocalizedFile, ()> {
        log::debug!("Reading localized file {}.", file_path.display());
        let file_name = file_path.file_name().unwrap().to_str().unwrap(); //TODO
        let file_dir = file_path.parent().ok_or(())?;
        //Look for translation dirs
        let mut translated_content = HashMap::new();
        for entry in file_dir.read_dir().expect("read_dir call failed").flatten()
        // We only care about the ones that are 'Ok'
        {
            if let Ok(entry_name) = entry.file_name().into_string() {
                if entry_name.starts_with(crate::LOCALE_FOLDER_PREFIX) {
                    let locale = entry_name
                        .splitn(2, crate::LOCALE_FOLDER_PREFIX)
                        .collect::<Vec<_>>()
                        .get(1)
                        .unwrap()
                        .to_string();
                    let locale_file_path = file_dir.join(entry_name).join(file_name);
                    if locale_file_path.exists() {
                        if let Ok(s) = std::fs::read_to_string(&locale_file_path) {
                            if let Ok(s) = s.clone().try_into() {
                                translated_content.insert(locale, s);
                            } else {
                                log::warn!(
                                    "Failed converting content in {}",
                                    locale_file_path.display()
                                );
                            }
                        } else {
                            log::warn!("Failed reading {}", locale_file_path.display());
                        }
                    }
                }
            }
        }

        let content = match std::fs::read_to_string(&file_path) {
            Ok(s) => Some(s),
            Err(e) => {
                log::debug!(
                    "Failed reading content for localized file {} with error {}",
                    file_path.display(),
                    e
                );
                None
            }
        };
        Ok(LoadedLocalizedFile {
            file_path: file_path.clone(),
            content,
            localized_content: translated_content,
        })
    }
}

impl FileManager {
    pub fn read_folder(&self, path: &PathBuf) -> Vec<RumbasRepoEntry> {
        // TODO: handle symlinks...
        let map = self.dir_cache.read().expect("Can read dir cache map");
        log::debug!("Checking if {} is in the dir_cache.", path.display());
        if let Some(val) = map.get(path) {
            log::debug!("Found {} in the dir_cache.", path.display());
            val.lock()
                .expect("unlock loaded file mutex")
                .entries
                .clone()
        } else {
            std::mem::drop(map); // remove the read lock
            let mut entries = Vec::new();
            if path.is_dir() {
                for entry in path.read_dir().expect("read_dir call failed").flatten()
                // We only care about the ones that are 'Ok'
                {
                    entries.push(RumbasRepoEntry::from(&entry.path()));
                }
                let mut map = self.dir_cache.write().expect("Can write dir_cache map");
                map.insert(
                    path.clone(),
                    Mutex::new(RumbasRepoFolderEntries {
                        r#type: RumbasRepoFolderType::from(&path),
                        entries: entries.clone(),
                    }),
                );
                entries
            } else {
                Vec::new()
            }
        }
    }
    fn read_all_folders(&self, path: &PathBuf) -> Vec<RumbasRepoFolderData> {
        self.read_folder(&path.to_path_buf())
            .into_iter()
            .filter_map(|e| match e {
                RumbasRepoEntry::Folder(f) => Some(
                    self.read_all_folders(&f.path)
                        .into_iter()
                        .chain(vec![f].into_iter())
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .flat_map(|e| e)
            .collect()
    }
    pub fn find_default_folders(&self) -> Vec<RumbasRepoFolderData> {
        // TODO find repo base
        self.read_all_folders(&std::path::Path::new(".").to_path_buf())
            .into_iter()
            .filter(|f| f.r#type == RumbasRepoFolderType::DefaultFolder)
            .collect()
    }
}

impl FileManager {
    fn find_all_yaml_files(path: PathBuf, wanted_file_type: RumbasRepoFileType) -> Vec<FileToLoad> {
        let mut files = Vec::new();
        for entry in CACHE.read_folder(&path) {
            match entry {
                RumbasRepoEntry::File(file) => {
                    if file.r#type == wanted_file_type {
                        files.push(FileToLoad {
                            file_path: file.path.clone(),
                            locale_dependant: false,
                        })
                    }
                }
                RumbasRepoEntry::Folder(folder) => match folder.r#type {
                    RumbasRepoFolderType::Folder => files.extend(Self::find_all_yaml_files(
                        folder.path,
                        wanted_file_type.clone(),
                    )),
                    _ => (),
                },
            }
        }
        files
    }
    pub fn read_all_questions(&self) -> Vec<LoadedFile> {
        let path = std::path::Path::new(crate::QUESTIONS_FOLDER); // TODO, find root of rumbas repo by looking for rc file
        let files = Self::find_all_yaml_files(path.to_path_buf(), RumbasRepoFileType::QuestionFile);
        self.read(files).into_iter().map(|(_, l)| l).collect()
    }
    pub fn read_all_question_templates(&self) -> Vec<LoadedFile> {
        let path = std::path::Path::new(crate::QUESTION_TEMPLATES_FOLDER); // TODO, find root of rumbas repo by looking for rc file
        let files =
            Self::find_all_yaml_files(path.to_path_buf(), RumbasRepoFileType::QuestionTemplateFile);
        self.read(files).into_iter().map(|(_, l)| l).collect()
    }
    pub fn read_all_exams(&self) -> Vec<LoadedFile> {
        let path = std::path::Path::new(crate::EXAMS_FOLDER); // TODO, find root of rumbas repo by looking for rc file
        let files = Self::find_all_yaml_files(path.to_path_buf(), RumbasRepoFileType::ExamFile);
        self.read(files).into_iter().map(|(_, l)| l).collect()
    }
    pub fn read_all_exam_templates(&self) -> Vec<LoadedFile> {
        let path = std::path::Path::new(crate::EXAM_TEMPLATES_FOLDER); // TODO, find root of rumbas repo by looking for rc file
        let files =
            Self::find_all_yaml_files(path.to_path_buf(), RumbasRepoFileType::ExamTemplateFile);
        self.read(files).into_iter().map(|(_, l)| l).collect()
    }
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
        let file_path = std::path::Path::new(crate::CUSTOM_PART_TYPES_FOLDER)
            .join(file_name)
            .with_extension("yaml");
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
        let file_path = std::path::Path::new(crate::QUESTIONS_FOLDER)
            .join(file_name)
            .with_extension("yaml");
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

#[derive(Debug, Clone)]
pub enum RumbasRepoEntry {
    File(RumbasRepoFileData),
    Folder(RumbasRepoFolderData),
}
impl RumbasRepoEntry {
    pub fn from(p: &PathBuf) -> Self {
        if p.is_dir() {
            Self::Folder(RumbasRepoFolderData::from(p))
        } else {
            // if p.is_file() { TODO: symlink?
            Self::File(RumbasRepoFileData::from(p))
        }
    }
}
#[derive(Debug, Clone)]
pub struct RumbasRepoFileData {
    r#type: RumbasRepoFileType,
    path: PathBuf,
}

impl RumbasRepoFileData {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl RumbasRepoFileData {
    pub fn from(p: &PathBuf) -> Self {
        Self {
            r#type: RumbasRepoFileType::from(p),
            path: p.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RumbasRepoFileType {
    QuestionFile,
    QuestionTemplateFile,
    ExamFile,
    ExamTemplateFile,
    CustomPartTypeFile,
    DefaultFile,
    LocaleFile,
    File,
}
impl RumbasRepoFileType {
    pub fn from(p: &PathBuf) -> Self {
        let components: Vec<_> = p.iter().collect();
        if components.len() > 1
            && components
                .get(components.len() - 1)
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(crate::LOCALE_FOLDER_PREFIX)
        {
            Self::LocaleFile
        } else if let Some(ext) = p.extension() {
            if ext == "yaml" {
                if p.starts_with(crate::DEFAULTS_FOLDER) {
                    Self::DefaultFile
                } else if p.starts_with(crate::QUESTIONS_FOLDER) {
                    Self::QuestionFile
                } else if p.starts_with(crate::QUESTION_TEMPLATES_FOLDER) {
                    Self::QuestionTemplateFile
                } else if p.starts_with(crate::EXAMS_FOLDER) {
                    Self::ExamFile
                } else if p.starts_with(crate::EXAM_TEMPLATES_FOLDER) {
                    Self::ExamTemplateFile
                } else if p.starts_with(crate::CUSTOM_PART_TYPES_FOLDER) {
                    Self::CustomPartTypeFile
                } else {
                    Self::File
                }
            } else {
                Self::File
            }
        } else {
            Self::File
        }
    }
}
#[derive(Debug, Clone)]
pub struct RumbasRepoFolderData {
    r#type: RumbasRepoFolderType,
    path: PathBuf,
}

impl RumbasRepoFolderData {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl RumbasRepoFolderData {
    pub fn from(p: &PathBuf) -> Self {
        Self {
            r#type: RumbasRepoFolderType::from(p),
            path: p.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RumbasRepoFolderEntries {
    r#type: RumbasRepoFolderType,
    entries: Vec<RumbasRepoEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RumbasRepoFolderType {
    DefaultFolder,
    LocalizedFolder { locale: String },
    Folder,
}

impl RumbasRepoFolderType {
    pub fn from(p: &PathBuf) -> Self {
        if let Some(stem) = p.file_stem() {
            if stem == crate::DEFAULTS_FOLDER {
                Self::DefaultFolder
            } else if stem
                .to_str()
                .unwrap()
                .starts_with(crate::LOCALE_FOLDER_PREFIX)
            {
                let locale = stem
                    .to_str()
                    .unwrap()
                    .splitn(2, crate::LOCALE_FOLDER_PREFIX)
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .to_string();
                Self::LocalizedFolder { locale }
            } else {
                Self::Folder
            }
        } else {
            log::debug!(
                "Unkown file_stem for {}, assuming it is just a folder",
                p.display()
            );
            Self::Folder
        }
    }
}

macro_rules! create_from_string_type {
    ($t: ident, $ti: ident, $data: ty, $datai: ty, $read_type: ty, $n_type: ty, $schema: literal, $combine: expr, $filename_field: ident) => {
        // TODO: remove this JsonSchema
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
        #[serde(into = "String")]
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
                if let Some(_) = &self.data {
                    None
                } else {
                    Some(<$read_type>::with_file_name(self.file_name.clone()).into())
                }
            }
        }

        impl Input for $ti {
            type Normal = $t;
            fn to_normal(&self) -> Self::Normal {
                Self::Normal {
                    file_name: self.file_name.to_owned(),
                    data: self.data.as_ref().map(|d| d.to_normal()).unwrap(),
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
                if let Some(ref q) = self.data {
                    let mut previous_result = q.find_missing();
                    previous_result.extend_path(self.file_name.clone());
                    previous_result
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
                } else if let Some(ref q) = self.data {
                    q.files_to_load()
                } else {
                    unreachable!();
                }
            }

            fn insert_loaded_files(
                &mut self,
                files: &std::collections::HashMap<FileToLoad, LoadedFile>,
            ) {
                if let Some(ref mut q) = self.data {
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
                                    Ok(q) => {
                                        let mut input = q.clone();
                                        $combine(&file_to_load.file_path, &mut input);
                                        let files_to_load = input.files_to_load();
                                        let loaded_files =
                                            crate::support::file_manager::CACHE.read(files_to_load);
                                        input.insert_loaded_files(&loaded_files);

                                        self.data = Some(input)
                                    }
                                    Err(e) => self.error_message = Some(e.to_string()),
                                }
                            }
                            Some(LoadedFile::Localized(_l)) => {
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
                    file_name: sanitize(&self.$filename_field),
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
                Self {
                    file_name: s,
                    data: None,
                    error_message: None,
                }
            }
        }

        impl std::convert::From<$t> for String {
            fn from(q: $t) -> Self {
                q.file_name
            }
        }

        impl std::convert::From<$ti> for String {
            fn from(q: $ti) -> Self {
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
