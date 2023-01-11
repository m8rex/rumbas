//! Contains all the exam types

pub mod diagnostic;
pub mod feedback;
pub mod locale;
pub mod navigation;
pub mod normal;
pub mod numbas_settings;
pub mod question_group;
pub mod timing;

use crate::exam::diagnostic::convert_diagnostic_numbas_exam;
use crate::exam::diagnostic::DiagnosticExam;
use crate::exam::locale::Locale;
use crate::exam::normal::convert_normal_numbas_exam;
use crate::exam::normal::NormalExam;
use crate::exam::question_group::QuestionFromTemplate;
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::support::default::combine_exam_with_default_files;
use crate::support::file_manager::{FileToRead, CACHE};
use crate::support::template::{TemplateFile, TemplateFileInputEnum};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::yaml::YamlError;
use comparable::Comparable;
use rumbas_support::path::RumbasPath;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::path::Path;
use structdoc::StructDoc;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "ExamInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Exam {
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}

impl ToNumbas<numbas::exam::Exam> for Exam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        match self {
            Exam::Normal(n) => n.to_numbas(locale),
            Exam::Diagnostic(n) => n.to_numbas(locale),
        }
    }
}

impl Exam {
    pub fn locales(&self) -> Vec<Locale> {
        match self {
            Exam::Normal(n) => n.locales.clone(),
            Exam::Diagnostic(n) => n.locales.clone(),
        }
    }

    pub fn numbas_settings(&self) -> crate::exam::numbas_settings::NumbasSettings {
        match self {
            Exam::Normal(n) => n.numbas_settings.clone(),
            Exam::Diagnostic(n) => n.numbas_settings.clone(),
        }
    }
}
impl ExamInput {
    pub fn combine_with_defaults(&mut self, path: &RumbasPath) {
        combine_exam_with_default_files(path.clone(), self);
    }
}

#[derive(Debug, Display)]
pub enum ParseError {
    YamlError(YamlError),
    FileReadError(FileReadError),
    InvalidPath(InvalidExamPathError),
    RecursiveTemplates(RecursiveTemplatesError),
}

#[derive(Debug)]
pub struct RecursiveTemplatesError(pub RumbasPath, pub Vec<RumbasPath>);

impl Display for RecursiveTemplatesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid templates setup: the template recursion contains a loop: {} has already been loaded in {}",
            self.0.display(),
            self.1.iter().map(|e| e.display().to_string()).collect::<Vec<_>>().join(" "),
        )
    }
}

#[derive(Debug)]
pub struct InvalidExamPathError(RumbasPath);

impl Display for InvalidExamPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid compilation path: {} should start with {}/ or {}/",
            self.0.display(),
            crate::EXAMS_FOLDER,
            crate::QUESTIONS_FOLDER
        )
    }
}

#[derive(Debug)]
pub struct FileReadError(pub RumbasPath);

impl Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed reading file: {}", self.0.display(),)
    }
}

#[derive(Input, Overwrite, RumbasCheck, StructDoc)]
#[input(name = "ExamFileTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ExamFileType {
    /// An exam that uses a template.
    Template(TemplateFile),
    /// A normal exam.
    Normal(NormalExam),
    /// An exam in diagnostic mode.
    Diagnostic(DiagnosticExam),
}

impl ExamFileTypeInput {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
    pub fn find_question_of_preview(&self) -> crate::question::QuestionInput {
        match self {
            Self::Template(_) => unreachable!(),
            Self::Normal(n) => &n.0.question_groups,
            Self::Diagnostic(n) => &n.0.question_groups,
        }
        .clone()
        .unwrap()[0]
            .unwrap()
            .questions
            .unwrap()[0]
            .unwrap()
            .data
            .unwrap()
    }
}

impl ExamFileType {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        ExamFileTypeInput::from_normal(self.to_owned()).to_yaml()
    }
}

/// Convert a numbas exam to rumbas data
/// Returns the name of the exam, the resulting exam (as ExamFileType)
/// and vectors of questions and custom part type definitions
pub fn convert_numbas_exam(
    exam: numbas::exam::Exam,
) -> (
    String,
    ExamFileType,
    Vec<QuestionFromTemplate>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let (name, exam, qgs, cpts) = match exam.navigation.navigation_mode {
        numbas::exam::navigation::NavigationMode::Diagnostic(ref _d) => {
            let (exam, qgs, cpts) = convert_diagnostic_numbas_exam(exam);
            (exam.name.clone(), ExamFileType::Diagnostic(exam), qgs, cpts)
        }
        _ => {
            let (exam, qgs, cpts) = convert_normal_numbas_exam(exam);
            (exam.name.clone(), ExamFileType::Normal(exam), qgs, cpts)
        }
    };
    (
        name.to_string("").expect("no locale needed"),
        exam,
        qgs,
        cpts,
    )
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
pub struct RecursiveTemplateExam {
    pub template_data: Vec<TemplateFile>,
    pub data: Exam,
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(from = "ExamFileTypeInput")]
#[serde(into = "ExamFileTypeInput")]
pub struct RecursiveTemplateExamInput {
    pub template_data: Vec<TemplateFile>,
    pub data: Option<ExamInput>,
    pub error_message: Option<String>,
}

impl std::convert::From<RecursiveTemplateExamInput> for ExamFileTypeInput {
    fn from(rtei: RecursiveTemplateExamInput) -> Self {
        match rtei.data {
            Some(ExamInput::Normal(n)) => Self::Normal(n),
            Some(ExamInput::Diagnostic(n)) => Self::Diagnostic(n),
            None => Self::Template(Input::from_normal(
                rtei.template_data.first().clone().unwrap().clone(),
            )),
        }
    }
}

impl std::convert::From<ExamFileTypeInput> for RecursiveTemplateExamInput {
    fn from(efti: ExamFileTypeInput) -> Self {
        match efti {
            ExamFileTypeInput::Normal(n) => Self {
                template_data: Vec::new(),
                data: Some(ExamInput::Normal(n)),
                error_message: None,
            },
            ExamFileTypeInput::Diagnostic(n) => Self {
                template_data: Vec::new(),
                data: Some(ExamInput::Diagnostic(n)),
                error_message: None,
            },
            ExamFileTypeInput::Template(t) => Self {
                template_data: vec![t.to_normal()],
                data: None,
                error_message: None,
            },
        }
    }
}

impl InputInverse for RecursiveTemplateExam {
    type Input = RecursiveTemplateExamInput;
    type EnumInput = RecursiveTemplateExamInput;
}
/* TODO
impl Examples for RecursiveTemplateExamInput {
    fn examples() -> Vec<Self> {
        vec![QuestionPathOrTemplate::QuestionPath("path".to_string())]
            .into_iter()
            .map(|e| e.into())
            .collect()
    }
}
*/
impl RecursiveTemplateExamInput {
    pub fn file_to_read(&self, main_file_path: &RumbasPath) -> Option<FileToRead> {
        if self.data.is_some() {
            None
        } else if let Some(rel_path) = self
            .template_data
            .last()
            .clone()
            .map(|a| a.relative_template_path.clone())
        {
            Some(
                crate::support::file_manager::ExamFileToRead::with_file_name(
                    rel_path.clone(),
                    main_file_path,
                )
                .into(),
            )
        } else {
            None
        }
    }
}

impl Input for RecursiveTemplateExamInput {
    type Normal = RecursiveTemplateExam;
    fn to_normal(&self) -> Self::Normal {
        Self::Normal {
            template_data: self.template_data.iter().map(|t| t.to_owned()).collect(),
            data: self.data.as_ref().map(|d| d.to_normal()).unwrap(),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            template_data: normal.template_data.into_iter().skip(1).collect(),
            data: Some(Input::from_normal(normal.data)),
            error_message: None,
        }
    }
    fn find_missing(&self) -> InputCheckResult {
        /*let path = if let Some(p) = self.question_path.as_ref() {
            p.to_owned()
        } else {
            let first = self.template_data.first().clone().unwrap();
            first.relative_template_path.clone()
        };*/
        if let Some(ref q) = self.data {
            let mut previous_result = q.find_missing();
            //previous_result.extend_path(path.clone());
            previous_result
        } else if let Some(e) = self.error_message.as_ref() {
            InputCheckResult::from_error_message(e.clone())
        } else {
            InputCheckResult::from_missing(Some(
                self.template_data
                    .first()
                    .clone()
                    .unwrap()
                    .relative_template_path
                    .clone(),
            ))
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        if let Some(ref mut q) = self.data {
            q.insert_template_value(key, val);
        }
    }
    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        if self.error_message.is_some() {
            vec![]
        } else if let Some(file) = self.file_to_read(main_file_path) {
            vec![file.into()]
        } else if let Some(ref q) = self.data {
            // TODO: is this used like this?
            q.files_to_load(main_file_path)
        } else {
            vec![]
        }
    }
    fn dependencies(
        &self,
        main_file_path: &RumbasPath,
    ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
        let mut deps: std::collections::HashSet<_> = Default::default();

        for template_file in self.template_data.iter() {
            deps.insert(
                crate::support::file_manager::ExamFileToRead::with_file_name(
                    template_file.relative_template_path.clone(),
                    main_file_path,
                )
                .into(),
            );
        }

        if let Some(ref data) = self.data {
            data.dependencies(main_file_path)
                .into_iter()
                .chain(deps.into_iter())
                .collect()
        } else {
            deps
        }
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
                        let data_res: Result<ExamFileTypeInput, _> =
                            serde_yaml::from_str(&n.content[..]).map_err(|e| {
                                ParseError::YamlError(YamlError::from(
                                    e,
                                    file_to_load.file_path.clone(),
                                ))
                            });
                        match data_res {
                            Ok(ExamFileTypeInput::Template(template_file)) => {
                                let mut template_file = template_file.clone();
                                if template_file.has_unknown_parent() {
                                    for previous in self.template_data.iter().rev() {
                                        template_file.set_template(previous);
                                        if !template_file.has_unknown_parent() {
                                            break;
                                        }
                                    }
                                    if !template_file.relative_template_path.is_set() {
                                        if let Some(key) = template_file.template_key() {
                                            self.error_message = Some(format!("Parent template not found, the template key {} is not set for {}", key, file_to_load.file_path.display()));
                                            return;
                                        }
                                    }
                                }
                                let template_file = template_file.to_normal();

                                if self.template_data.contains(&template_file) {
                                    self.error_message = Some(format!(
                                        "Loop in templates: {} is a parent of itself. The fill template parent structure is {}",
                                        template_file.relative_template_path,
                                        self.template_data.iter().map(|t| t.relative_template_path.clone()).chain(vec![template_file.relative_template_path.clone()].into_iter()).collect::<Vec<_>>().join(" -> ")
                                    ));
                                } else {
                                    self.template_data.push(template_file);
                                }
                                // todo: change when allowing template field to be templatable
                            }
                            Ok(e) => {
                                let mut input = match e {
                                    ExamFileTypeInput::Normal(q) => ExamInput::Normal(q.clone()),
                                    ExamFileTypeInput::Diagnostic(d) => {
                                        ExamInput::Diagnostic(d.clone())
                                    }
                                    _ => unreachable!(),
                                };
                                self.template_data.iter().rev().for_each(|template| {
                                    template.data.iter().for_each(|(k, v)| {
                                        input.insert_template_value(k, &v.0);
                                    })
                                });

                                self.data = Some(input);
                            }
                            Err(e) => self.error_message = Some(e.to_string()),
                        }
                    }
                    Some(LoadedFile::Localized(_l)) => {
                        unreachable!()
                    }
                    None => {
                        self.error_message = Some(format!(
                            "Missing file: {}",
                            file_to_load.file_path.display()
                        ))
                    }
                }
            }
        }
    }
}

impl RumbasCheck for RecursiveTemplateExam {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        let mut previous_result = self.data.check(locale);
        /*
        previous_result.extend_path(if let Some(p) = self.question_path.as_ref() {
            p.clone()
        } else {
            self.template_data
                .first()
                .unwrap()
                .relative_template_path
                .clone()
        });
        */
        previous_result
    }
}

impl Overwrite<RecursiveTemplateExamInput> for RecursiveTemplateExamInput {
    fn overwrite(&mut self, _other: &Self) {}
}

impl ToNumbas<numbas::exam::Exam> for RecursiveTemplateExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        self.data.clone().to_numbas(locale)
    }
}

//TODO: shouldn't I check the data as well?
impl std::hash::Hash for RecursiveTemplateExam {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.template_data.hash(state);
    }
}
impl PartialEq for RecursiveTemplateExam {
    fn eq(&self, other: &Self) -> bool {
        self.template_data == other.template_data
    }
}
impl Eq for RecursiveTemplateExam {}

impl PartialEq for RecursiveTemplateExamInput {
    fn eq(&self, other: &Self) -> bool {
        self.template_data == other.template_data
    }
}
impl Eq for RecursiveTemplateExamInput {}

impl RecursiveTemplateExamInput {
    pub fn from_file(file: &RumbasPath) -> Result<Self, ParseError> {
        if file.in_main_folder(crate::EXAMS_FOLDER) {
            let yaml = CACHE
                .read_file(FileToLoad {
                    file_path: file.clone(),
                    locale_dependant: false,
                })
                .and_then(|lf| match lf {
                    LoadedFile::Normal(n) => Some(n.content),
                    LoadedFile::Localized(_) => None,
                })
                .ok_or_else(|| ParseError::FileReadError(FileReadError(file.clone())))?;

            serde_yaml::from_str(&yaml)
                .map_err(|e| ParseError::YamlError(YamlError::from(e, file.clone())))
        } else if file.in_main_folder(crate::QUESTIONS_FOLDER) {
            let mut data = BTreeMap::new();
            data.insert(
                "question".to_string(),
                serde_yaml::Value::String(
                    file.project()
                        .with_extension("")
                        .strip_prefix(crate::QUESTIONS_FOLDER)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                )
                .into(),
            );
            let t = TemplateFile {
                relative_template_path: crate::QUESTION_PREVIEW_TEMPLATE_NAME.to_string(),
                data,
            };
            Ok(ExamFileTypeInput::Template(TemplateFileInputEnum::from_normal(t)).into())
        } else {
            Err(ParseError::InvalidPath(InvalidExamPathError(file.clone())))
        }
    }
    pub fn load_files(&mut self, path: &RumbasPath) {
        loop {
            let files_to_load = self.files_to_load(path);
            if files_to_load.is_empty() {
                break;
            }
            let loaded_files = CACHE.read_files(files_to_load);
            self.insert_loaded_files(path, &loaded_files);
        }
    }
    pub fn combine_with_defaults(&mut self, path: &RumbasPath) {
        if let Some(ref mut data) = self.data {
            data.combine_with_defaults(path)
        }
    }
    pub fn normalize(&mut self, path: &RumbasPath) {
        // Load template files for exam
        self.load_files(path);

        self.combine_with_defaults(path);
        self.load_files(path);
    }
}
