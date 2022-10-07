use crate::exam::ParseError;
use crate::question::Question;
use crate::question::{QuestionFileTypeInput, QuestionInput};
use crate::support::default::combine_question_with_default_files;
use crate::support::file_manager::*;
use crate::support::sanitize::sanitize;
use crate::support::template::TemplateFile;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "QuestionGroupInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct QuestionGroup {
    /// The name
    pub name: TranslatableString,
    /// The strategy to use to pick the questions to show
    #[serde(flatten)]
    pub picking_strategy: PickingStrategy,
    /// The questions
    pub questions: Vec<QuestionFromTemplate>,
}

impl ToNumbas<numbas::exam::question_group::QuestionGroup> for QuestionGroup {
    fn to_numbas(&self, locale: &str) -> numbas::exam::question_group::QuestionGroup {
        numbas::exam::question_group::QuestionGroup {
            name: self.name.to_numbas(locale),
            picking_strategy: self.picking_strategy.to_numbas(locale),
            questions: self.questions.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionGroup> for numbas::exam::question_group::QuestionGroup {
    fn to_rumbas(&self) -> QuestionGroup {
        QuestionGroup {
            name: self.name.to_rumbas(),
            picking_strategy: self.picking_strategy.to_rumbas(),
            questions: self.questions.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "PickingStrategyInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(tag = "picking_strategy")]
pub enum PickingStrategy {
    #[serde(rename = "all_ordered")]
    AllOrdered,
    #[serde(rename = "all_shuffled")]
    AllShuffled,
    #[serde(rename = "random_subset")]
    RandomSubset(PickingStrategyRandomSubset),
}

impl ToNumbas<numbas::exam::question_group::QuestionGroupPickingStrategy> for PickingStrategy {
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::exam::question_group::QuestionGroupPickingStrategy {
        match self {
            PickingStrategy::AllOrdered => {
                numbas::exam::question_group::QuestionGroupPickingStrategy::AllOrdered
            }
            PickingStrategy::AllShuffled => {
                numbas::exam::question_group::QuestionGroupPickingStrategy::AllShuffled
            }
            PickingStrategy::RandomSubset(p) => {
                numbas::exam::question_group::QuestionGroupPickingStrategy::RandomSubset {
                    pick_questions: p.pick_questions,
                }
            }
        }
    }
}

impl ToRumbas<PickingStrategy> for numbas::exam::question_group::QuestionGroupPickingStrategy {
    fn to_rumbas(&self) -> PickingStrategy {
        match self {
            numbas::exam::question_group::QuestionGroupPickingStrategy::AllOrdered => {
                PickingStrategy::AllOrdered
            }
            numbas::exam::question_group::QuestionGroupPickingStrategy::AllShuffled => {
                PickingStrategy::AllShuffled
            }
            numbas::exam::question_group::QuestionGroupPickingStrategy::RandomSubset {
                pick_questions,
            } => PickingStrategy::RandomSubset(PickingStrategyRandomSubset {
                pick_questions: *pick_questions,
            }),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "PickingStrategyRandomSubsetInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct PickingStrategyRandomSubset {
    pub pick_questions: usize,
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum QuestionPathOrTemplate {
    QuestionPath(String),
    Template(TemplateFile),
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(into = "QuestionPathOrTemplate")]
pub struct QuestionFromTemplate {
    pub template_data: Vec<TemplateFile>,
    pub question_path: Option<String>,
    pub data: Question,
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(from = "QuestionPathOrTemplate")]
#[serde(into = "QuestionPathOrTemplate")]
pub struct QuestionFromTemplateInput {
    pub template_data: Vec<TemplateFile>,
    pub question_path: Option<String>,
    pub data: Option<QuestionInput>,
    pub error_message: Option<String>,
}

impl std::convert::From<QuestionFromTemplateInput> for QuestionPathOrTemplate {
    fn from(qft: QuestionFromTemplateInput) -> Self {
        match qft.question_path {
            Some(path) => QuestionPathOrTemplate::QuestionPath(path.clone()),
            None => QuestionPathOrTemplate::Template(qft.template_data[0].clone()),
        }
    }
}

impl std::convert::From<QuestionFromTemplate> for QuestionPathOrTemplate {
    fn from(qft: QuestionFromTemplate) -> Self {
        match qft.question_path {
            Some(path) => QuestionPathOrTemplate::QuestionPath(path.clone()),
            None => QuestionPathOrTemplate::Template(qft.template_data[0].clone()),
        }
    }
}

impl std::convert::From<QuestionPathOrTemplate> for QuestionFromTemplateInput {
    fn from(qpt: QuestionPathOrTemplate) -> Self {
        match qpt {
            QuestionPathOrTemplate::QuestionPath(path) => Self {
                template_data: Vec::new(),
                question_path: Some(path),
                data: None,
                error_message: None,
            },
            QuestionPathOrTemplate::Template(t) => t.into(),
        }
    }
}

impl std::convert::From<TemplateFile> for QuestionFromTemplateInput {
    fn from(template_file: TemplateFile) -> Self {
        Self {
            template_data: vec![template_file],
            question_path: None,
            data: None,
            error_message: None,
        }
    }
}

impl InputInverse for QuestionFromTemplate {
    type Input = QuestionFromTemplateInput;
    type EnumInput = QuestionFromTemplateInput;
}

impl Examples for QuestionFromTemplate {
    fn examples() -> Vec<Self> {
        Vec::new()
    }
}
impl Examples for QuestionFromTemplateInput {
    fn examples() -> Vec<Self> {
        Vec::new()
    }
}
impl QuestionFromTemplateInput {
    fn dependency(&self, main_file_path: &RumbasPath) -> FileToRead {
        crate::support::file_manager::QuestionFileToRead::with_file_name(
            if self.template_data.len() > 0 {
                self.template_data
                    .last()
                    .unwrap()
                    .relative_template_path
                    .clone()
            } else {
                self.question_path.clone().unwrap()
            },
            main_file_path,
        )
        .into()
    }

    pub fn file_to_read(&self, main_file_path: &RumbasPath) -> Option<FileToRead> {
        if self.data.is_some() {
            None
        } else {
            Some(self.dependency(main_file_path).into())
        }
    }
}

impl Input for QuestionFromTemplateInput {
    type Normal = QuestionFromTemplate;
    fn to_normal(&self) -> Self::Normal {
        Self::Normal {
            template_data: self.template_data.to_owned(),
            question_path: self.question_path.to_owned(),
            data: self.data.as_ref().map(|d| d.to_normal()).unwrap(),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            template_data: normal.template_data,
            question_path: normal.question_path,
            data: Some(Input::from_normal(normal.data)),
            error_message: None,
        }
    }
    fn find_missing(&self) -> InputCheckResult {
        let path = if let Some(p) = self.question_path.as_ref() {
            p.to_owned()
        } else {
            self.template_data
                .first()
                .as_ref()
                .unwrap()
                .relative_template_path
                .clone()
        };
        if let Some(ref q) = self.data {
            let mut previous_result = q.find_missing();
            previous_result.extend_path(path.clone());
            previous_result
        } else if let Some(e) = self.error_message.as_ref() {
            InputCheckResult::from_error_message(e.clone())
        } else {
            InputCheckResult::from_missing(Some(path.clone()))
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
                        let data_res: Result<QuestionFileTypeInput, _> =
                            serde_yaml::from_str(&n.content[..]).map_err(|e| {
                                ParseError::YamlError(YamlError::from(
                                    e,
                                    file_to_load.file_path.clone(),
                                ))
                            });
                        match data_res {
                            Ok(QuestionFileTypeInput::Normal(q)) => {
                                let mut input = (*q.clone()).0;
                                self.template_data.iter().rev().for_each(|template| {
                                    template.data.iter().for_each(|(k, v)| {
                                        input.insert_template_value(k, &v.0);
                                    })
                                });
                                combine_question_with_default_files(
                                    file_to_load.file_path,
                                    &mut input,
                                );
                                let files_to_load = input.files_to_load(main_file_path);
                                let loaded_files =
                                    crate::support::file_manager::CACHE.read_files(files_to_load);
                                input.insert_loaded_files(main_file_path, &loaded_files);

                                self.data = Some(input);
                            }
                            Ok(QuestionFileTypeInput::Template(template_file)) => {
                                let mut template_file = template_file.clone();
                                if template_file.has_unknown_parent() {
                                    for previous in self.template_data.iter().rev() {
                                        template_file.set_template(previous);
                                        if !template_file.has_unknown_parent() {
                                            break;
                                        }
                                    }
                                    if let Some(key) = template_file.template_key() {
                                        self.error_message = Some(format!("Parent template not found, the template key {} is not set for {}", key, file_to_load.file_path.display()));
                                        return;
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

impl RumbasCheck for QuestionFromTemplate {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        let mut previous_result = self.data.check(locale);
        previous_result.extend_path(if let Some(p) = self.question_path.as_ref() {
            p.clone()
        } else {
            self.template_data
                .first()
                .unwrap()
                .relative_template_path
                .clone()
        });
        previous_result
    }
}

impl Overwrite<QuestionFromTemplateInput> for QuestionFromTemplateInput {
    fn overwrite(&mut self, _other: &Self) {}
}

impl ToNumbas<numbas::question::Question> for QuestionFromTemplate {
    fn to_numbas(&self, locale: &str) -> numbas::question::Question {
        self.data.clone().to_numbas_with_name(
            locale,
            if let Some(n) = self.question_path.as_ref() {
                n.clone()
            } else {
                self.template_data
                    .first()
                    .unwrap()
                    .relative_template_path
                    .clone()
            },
        )
    }
}

impl ToRumbas<QuestionFromTemplate> for numbas::question::Question {
    fn to_rumbas(&self) -> QuestionFromTemplate {
        QuestionFromTemplate {
            template_data: Vec::new(),
            data: self.to_rumbas(),
            question_path: Some(sanitize(&self.name[..])),
        }
        // TODO: handle variable overrride
    }
}

impl std::hash::Hash for QuestionFromTemplate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.template_data.hash(state);
    }
}
impl PartialEq for QuestionFromTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.template_data == other.template_data
    }
}
impl Eq for QuestionFromTemplate {}

impl PartialEq for QuestionFromTemplateInput {
    fn eq(&self, other: &Self) -> bool {
        self.template_data == other.template_data
    }
}
impl Eq for QuestionFromTemplateInput {}
