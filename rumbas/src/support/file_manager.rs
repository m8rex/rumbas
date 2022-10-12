use crate::support::rc::within_repo;
use rayon::prelude::*;
use rumbas_support::input::{FileToLoad, LoadedFile, LoadedLocalizedFile, LoadedNormalFile};
use rumbas_support::path::RumbasPath;
use std::path::Path;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

lazy_static! {
    pub static ref CACHE: FileManager = FileManager::default();
}

#[derive(Debug)]
pub struct FileManager {
    cache: RwLock<HashMap<FileToLoad, Mutex<LoadedFile>>>,
    dir_cache: RwLock<HashMap<PathBuf, Mutex<RumbasRepoFolderEntries>>>,
}

impl Default for FileManager {
    fn default() -> Self {
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
                true => self
                    .read_localized_file(&file.file_path)
                    .map(rumbas_support::input::LoadedFile::Localized),
                false => Self::read_normal_file(file.file_path.clone())
                    .map(rumbas_support::input::LoadedFile::Normal),
            };
            match res {
                Ok(r) => {
                    let mut map = self.cache.write().expect("Can write cache map");
                    map.insert(file.clone(), Mutex::new(r.clone()));
                    Some(r)
                }
                Err(()) => {
                    log::error!("Couldn't resolve {}", file.file_path.display());
                    None
                }
            }
        }
    }

    pub fn read_files(&self, files: Vec<FileToLoad>) -> HashMap<FileToLoad, LoadedFile> {
        let result: HashMap<_, _> = files
            .into_par_iter()
            .filter_map(|file| self.read_file(file.clone()).map(|l| (file, l)))
            .collect();
        result
    }

    fn read_normal_file(file_path: RumbasPath) -> Result<LoadedNormalFile, ()> {
        log::debug!("Reading normal file {}.", file_path.display());
        match std::fs::read_to_string(file_path.absolute()) {
            Ok(content) => Ok(LoadedNormalFile { content, file_path }),
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

    fn read_localized_file(&self, file_path: &RumbasPath) -> Result<LoadedLocalizedFile, ()> {
        log::debug!("Reading localized file {}.", file_path.display());
        let file_name = file_path.project().file_name().unwrap().to_str().unwrap(); //TODO
        let file_dir = file_path.keep_root(file_path.project().parent().ok_or(())?);
        log::debug!("Looking for localized files in {}.", file_dir.display());
        //Look for translation dirs
        let mut translated_content = HashMap::new();
        for (path, locale) in self
            .read_folder(&file_dir)
            .into_iter()
            .filter_map(|e| match e {
                RumbasRepoEntry::File(_f) => None,
                RumbasRepoEntry::Folder(f) => match f.r#type {
                    RumbasRepoFolderType::LocalizedFolder { locale } => Some((f.path, locale)),
                    _ => None,
                },
            })
        {
            let locale_file_path = path.absolute().join(file_name);
            if locale_file_path.exists() {
                if let Ok(s) = std::fs::read_to_string(&locale_file_path) {
                    log::debug!("Found localized file {}.", locale_file_path.display());
                    translated_content.insert(locale, s);
                } else {
                    log::warn!("Failed reading {}", locale_file_path.display());
                }
            }
        }

        let content = match std::fs::read_to_string(file_path.absolute()) {
            Ok(s) => Some(s),
            Err(e) => {
                log::debug!(
                    "Failed reading content for default localized file {} with error {}",
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
    pub fn read_folder(&self, path: &RumbasPath) -> Vec<RumbasRepoEntry> {
        let map = self.dir_cache.read().expect("Can read dir cache map");
        log::debug!("Checking if {} is in the dir_cache.", path.display());
        if let Some(val) = map.get(path.absolute()) {
            log::debug!("Found {} in the dir_cache.", path.display());
            val.lock()
                .expect("unlock loaded file mutex")
                .entries
                .clone()
        } else {
            std::mem::drop(map); // remove the read lock
            let mut entries = Vec::new();
            if path.is_dir() {
                for entry in path
                    .absolute()
                    .read_dir()
                    .expect("read_dir call failed")
                    .flatten()
                // We only care about the ones that are 'Ok'
                {
                    if let Some(rumbas_path) = path.in_root(&entry.path()) {
                        entries.push(RumbasRepoEntry::from(rumbas_path));
                    }
                }
                let mut map = self.dir_cache.write().expect("Can write dir_cache map");
                map.insert(
                    path.absolute().to_path_buf(),
                    Mutex::new(RumbasRepoFolderEntries {
                        r#type: RumbasRepoFolderType::from(path),
                        entries: entries.clone(),
                    }),
                );
                entries
            } else {
                Vec::new()
            }
        }
    }
    fn read_all_folders(&self, path: &RumbasPath) -> Vec<RumbasRepoFolderData> {
        self.read_folder(path)
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
            .flatten()
            .collect()
    }
    pub fn find_default_folders(&self, path: &RumbasPath) -> Vec<RumbasRepoFolderData> {
        self.read_all_folders(&path.keep_root(path.root()))
            .into_iter()
            .filter(|f| f.r#type == RumbasRepoFolderType::DefaultFolder)
            .collect()
    }
}

impl FileManager {
    fn find_all_yaml_files(
        &self,
        path: RumbasPath,
        wanted_file_type: RumbasRepoFileType,
    ) -> Vec<FileToLoad> {
        let mut files = Vec::new();
        for entry in self.read_folder(&path) {
            match entry {
                RumbasRepoEntry::File(file) => {
                    if file.r#type == wanted_file_type {
                        files.push(FileToLoad {
                            file_path: file.path.clone(),
                            locale_dependant: false,
                        })
                    }
                }
                RumbasRepoEntry::Folder(folder) => {
                    if folder.r#type == RumbasRepoFolderType::Folder {
                        files
                            .extend(self.find_all_yaml_files(folder.path, wanted_file_type.clone()))
                    }
                }
            }
        }
        files
    }
    pub fn find_all_questions_in_folder(&self, folder_path: RumbasPath) -> Vec<FileToLoad> {
        self.find_all_yaml_files(folder_path, RumbasRepoFileType::QuestionFile)
    }
    pub fn read_all_questions_in_folder(&self, folder_path: RumbasPath) -> Vec<LoadedFile> {
        let files = self.find_all_questions_in_folder(folder_path);
        self.read_files(files).into_iter().map(|(_, l)| l).collect()
    }
    pub fn read_all_questions(&self, path: &RumbasPath) -> Vec<LoadedFile> {
        let folder_path = std::path::Path::new(crate::QUESTIONS_FOLDER).to_path_buf();
        let folder_path = path.keep_root(&folder_path);
        self.read_all_questions_in_folder(folder_path)
    }
    pub fn find_all_exams_in_folder(&self, folder_path: RumbasPath) -> Vec<FileToLoad> {
        self.find_all_yaml_files(folder_path, RumbasRepoFileType::ExamFile)
    }
    pub fn read_all_exams_in_folder(&self, folder_path: RumbasPath) -> Vec<LoadedFile> {
        let files = self.find_all_exams_in_folder(folder_path);
        self.read_files(files).into_iter().map(|(_, l)| l).collect()
    }
    pub fn read_all_exams(&self, path: &RumbasPath) -> Vec<LoadedFile> {
        let folder_path = std::path::Path::new(crate::EXAMS_FOLDER).to_path_buf();
        let folder_path = path.keep_root(&folder_path);
        self.read_all_exams_in_folder(folder_path)
    }
}

impl FileManager {
    pub fn delete_file(&self, file: FileToLoad) {
        let mut map = self.cache.write().expect("Can write cache map");
        if map.contains_key(&file) {
            log::debug!("Deleting {} from the cache.", file.file_path.display());
            map.remove(&file);
        }
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

impl std::convert::From<FileToRead> for RumbasPath {
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
    file_path: RumbasPath,
}

impl TextFileToRead {
    pub fn with_file_name(file_name: String, main_file_path: &RumbasPath) -> Self {
        let file_path = std::path::Path::new(crate::QUESTIONS_FOLDER).join(file_name);
        let file_path = main_file_path.keep_root(file_path.as_path());
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

impl std::convert::From<TextFileToRead> for RumbasPath {
    fn from(s: TextFileToRead) -> Self {
        s.file_path
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct CustomPartTypeFileToRead {
    file_path: RumbasPath,
}

impl CustomPartTypeFileToRead {
    pub fn with_file_name(file_name: String, main_file_path: &RumbasPath) -> Self {
        let file_path = std::path::Path::new(crate::CUSTOM_PART_TYPES_FOLDER)
            .join(file_name)
            .with_extension("yaml");
        let file_path = main_file_path.keep_root(file_path.as_path());
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

impl std::convert::From<CustomPartTypeFileToRead> for RumbasPath {
    fn from(s: CustomPartTypeFileToRead) -> Self {
        s.file_path
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct QuestionFileToRead {
    file_path: RumbasPath,
}

impl QuestionFileToRead {
    pub fn with_file_name(file_name: String, main_file_path: &RumbasPath) -> Self {
        let file_path = std::path::Path::new(crate::QUESTIONS_FOLDER)
            .join(file_name)
            .with_extension("yaml");
        let file_path = main_file_path.keep_root(file_path.as_path());
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

impl std::convert::From<QuestionFileToRead> for RumbasPath {
    fn from(s: QuestionFileToRead) -> Self {
        s.file_path
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ExamFileToRead {
    file_path: RumbasPath,
}

impl ExamFileToRead {
    pub fn with_file_name(file_name: String, main_file_path: &RumbasPath) -> Self {
        let file_path = std::path::Path::new(crate::EXAMS_FOLDER)
            .join(file_name)
            .with_extension("yaml");
        let file_path = main_file_path.keep_root(file_path.as_path());
        Self { file_path }
    }
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

impl std::convert::From<ExamFileToRead> for RumbasPath {
    fn from(s: ExamFileToRead) -> Self {
        s.file_path
    }
}

#[derive(Debug, Clone)]
pub enum RumbasRepoEntry {
    File(RumbasRepoFileData),
    Folder(RumbasRepoFolderData),
}
impl RumbasRepoEntry {
    pub fn from(p: RumbasPath) -> Self {
        if p.is_dir() {
            Self::Folder(RumbasRepoFolderData::from(p))
        } else {
            Self::File(RumbasRepoFileData::from(p))
        }
    }
}
#[derive(Debug, Clone)]
pub struct RumbasRepoFileData {
    r#type: RumbasRepoFileType,
    path: RumbasPath,
}

impl RumbasRepoFileData {
    pub fn path(&self) -> RumbasPath {
        self.path.clone()
    }
    pub fn dependency_path(&self) -> RumbasPath {
        match self.r#type.clone() {
            RumbasRepoFileType::LocaleFile(_, p) => p.clone(),
            _ => self.path(),
        }
    }
}

impl std::convert::From<RumbasRepoFileData> for FileToLoad {
    fn from(r: RumbasRepoFileData) -> Self {
        match r.r#type.clone() {
            RumbasRepoFileType::LocaleFile(_, p) => FileToLoad {
                file_path: p,
                locale_dependant: true,
            },
            _ => FileToLoad {
                file_path: r.path,
                locale_dependant: false,
            },
        }
    }
}

impl RumbasRepoFileData {
    pub fn from(path: RumbasPath) -> Self {
        Self {
            r#type: RumbasRepoFileType::from(&path),
            path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RumbasRepoFileType {
    QuestionFile,
    ExamFile,
    CustomPartTypeFile,
    DefaultFile,
    LocaleFile(String, RumbasPath),
    File,
}
impl RumbasRepoFileType {
    pub fn from(p: &RumbasPath) -> Self {
        let folder_type = RumbasRepoFolderType::from(&p.parent().unwrap());

        if let RumbasRepoFolderType::LocalizedFolder { locale } = folder_type {
            let resource_path = p.keep_root(
                p.project()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(p.project().file_name().unwrap())
                    .as_path(),
            );

            Self::LocaleFile(locale, resource_path)
        } else if let RumbasRepoFolderType::DefaultFolder = folder_type {
            Self::DefaultFile // TODO: fix DefaultFile
        } else if let Some(ext) = p.extension() {
            if ext == "yaml" {
                if p.in_main_folder(crate::DEFAULTS_FOLDER) {
                    Self::DefaultFile
                } else if p.in_main_folder(crate::QUESTIONS_FOLDER) {
                    Self::QuestionFile
                } else if p.in_main_folder(crate::EXAMS_FOLDER) {
                    Self::ExamFile
                } else if p.in_main_folder(crate::CUSTOM_PART_TYPES_FOLDER) {
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

#[cfg(test)]
mod test {
    use super::RumbasRepoFileType;
    use std::path::Path;

    fn rumbas_path(s: &str) -> rumbas_support::path::RumbasPath {
        rumbas_support::path::RumbasPath::test_make(&Path::new(s), &Path::new("."))
    }

    #[test]
    fn rumbas_repo_file_type() {
        assert_eq!(
            RumbasRepoFileType::DefaultFile,
            RumbasRepoFileType::from(&rumbas_path("defaults/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::QuestionFile,
            RumbasRepoFileType::from(&rumbas_path("questions/something/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::DefaultFile,
            RumbasRepoFileType::from(&rumbas_path("questions/defaults/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::LocaleFile("a".to_string(), rumbas_path("questions/file.yaml")),
            RumbasRepoFileType::from(&rumbas_path("questions/locale-a/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::ExamFile,
            RumbasRepoFileType::from(&rumbas_path("exams/something/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::DefaultFile,
            RumbasRepoFileType::from(&rumbas_path("exams/defaults/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::LocaleFile("e".to_string(), rumbas_path("exams/file.yaml")),
            RumbasRepoFileType::from(&rumbas_path("exams/locale-e/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::QuestionFile,
            RumbasRepoFileType::from(&rumbas_path("questions/templates/something/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::ExamFile,
            RumbasRepoFileType::from(&rumbas_path("exams/templates/something/file.yaml"))
        );
        assert_eq!(
            RumbasRepoFileType::CustomPartTypeFile,
            RumbasRepoFileType::from(&rumbas_path("custom_part_types/something/file.yaml"))
        );
    }
}

#[derive(Debug, Clone)]
pub struct RumbasRepoFolderData {
    r#type: RumbasRepoFolderType,
    path: RumbasPath,
}

impl RumbasRepoFolderData {
    pub fn path(&self) -> RumbasPath {
        self.path.clone()
    }
}

impl RumbasRepoFolderData {
    pub fn from(path: RumbasPath) -> Self {
        Self {
            r#type: RumbasRepoFolderType::from(&path),
            path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RumbasRepoFolderEntries {
    r#type: RumbasRepoFolderType,
    entries: Vec<RumbasRepoEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RumbasRepoFolderType {
    DefaultFolder,
    LocalizedFolder { locale: String },
    Folder,
}

impl RumbasRepoFolderType {
    pub fn from(p: &RumbasPath) -> Self {
        if let Some(stem) = p.project().file_stem() {
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
            log::warn!(
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
        #[derive(Debug, Clone, Serialize, Deserialize, Comparable, JsonSchema)]
        #[serde(into = "String")]
        pub struct $t {
            pub file_name: String,
            pub data: $data,
        }
        #[derive(Serialize, Deserialize, Comparable, Debug, Clone)]
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
            fn dependency(&self, main_file_path: &RumbasPath) -> FileToRead {
                <$read_type>::with_file_name(self.file_name.clone(), main_file_path).into()
            }

            pub fn file_to_read(&self, main_file_path: &RumbasPath) -> Option<FileToRead> {
                if let Some(_) = &self.data {
                    None
                } else {
                    Some(self.dependency(main_file_path).into())
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
                } else if let Some(e) = self.error_message.as_ref() {
                    InputCheckResult::from_error_message(e.clone())
                } else {
                    InputCheckResult::from_missing(Some(self.file_name.clone()))
                }
            }
            fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
                if let Some(ref mut q) = self.data {
                    q.insert_template_value(key, val);
                }
            }
            fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
                if let Some(file) = self.file_to_read(main_file_path) {
                    vec![file.into()]
                } else if let Some(ref q) = self.data {
                    // TODO: is this used like this?
                    q.files_to_load(main_file_path)
                } else {
                    unreachable!();
                }
            }
            fn dependencies(
                &self,
                main_file_path: &RumbasPath,
            ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
                let path: rumbas_support::path::RumbasPath = self.dependency(main_file_path).into();
                let deps: std::collections::HashSet<_> = vec![path].into_iter().collect();

                let deps = if let Some(ref data) = self.data {
                    data.dependencies(main_file_path)
                        .into_iter()
                        .chain(deps.into_iter())
                        .collect()
                } else {
                    deps
                };

                deps
            }
            fn insert_loaded_files(
                &mut self,
                main_file_path: &RumbasPath,
                files: &std::collections::HashMap<FileToLoad, LoadedFile>,
            ) {
                if let Some(ref mut q) = self.data {
                    q.insert_loaded_files(main_file_path, files);
                } else {
                    let file = self.file_to_read(main_file_path);
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
                                        $combine(file_to_load.file_path, &mut input);
                                        let files_to_load = input.files_to_load(main_file_path);
                                        let loaded_files = crate::support::file_manager::CACHE
                                            .read_files(files_to_load);
                                        input.insert_loaded_files(main_file_path, &loaded_files);

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
                let mut previous_result = self.data.check(locale);
                previous_result.extend_path(self.file_name.clone());
                previous_result
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
