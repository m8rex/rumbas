use crate::question::Question;
use crate::question::QuestionInput;
use crate::support::default::combine_question_with_default_files;
use crate::support::file_manager::*;
use crate::support::sanitize::sanitize;
use crate::support::template::TemplateFile;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
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
    pub questions: Vec<QuestionOrTemplate>,
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

crate::support::file_manager::create_from_string_type!(
    QuestionPath,
    QuestionPathInput,
    Question,
    QuestionInput,
    QuestionFileToRead,
    numbas::question::Question,
    "QuestionPath",
    combine_question_with_default_files,
    name
);

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "QuestionOrTemplateInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum QuestionOrTemplate {
    Template(QuestionFromTemplate),
    Normal(QuestionPath),
}

impl QuestionOrTemplate {
    pub fn data(self) -> Question {
        match self {
            Self::Template(q) => q.data,
            Self::Normal(q) => q.data,
        }
    }
}

impl ToRumbas<QuestionOrTemplate> for numbas::question::Question {
    fn to_rumbas(&self) -> QuestionOrTemplate {
        QuestionOrTemplate::Normal(self.to_rumbas())
    }
}

impl ToNumbas<numbas::question::Question> for QuestionOrTemplate {
    fn to_numbas(&self, locale: &str) -> numbas::question::Question {
        match self {
            Self::Template(t) => t.to_numbas(locale),
            Self::Normal(t) => t.to_numbas(locale),
        }
    }
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
pub struct QuestionFromTemplate {
    pub content: TemplateFile,
    pub data: Question,
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(from = "TemplateFile")]
pub struct QuestionFromTemplateInput {
    pub content: TemplateFile,
    pub data: Option<QuestionInput>,
    pub error_message: Option<String>,
}

impl std::convert::From<TemplateFile> for QuestionFromTemplateInput {
    fn from(content: TemplateFile) -> Self {
        Self {
            content,
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
            self.content.relative_template_path.clone(),
            main_file_path,
        )
        .into()
    }

    pub fn file_to_read(&self, main_file_path: &RumbasPath) -> Option<FileToRead> {
        if let Some(_) = &self.data {
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
            content: self.content.to_owned(),
            data: self.data.as_ref().map(|d| d.to_normal()).unwrap(),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            content: normal.content,
            data: Some(Input::from_normal(normal.data)),
            error_message: None,
        }
    }
    fn find_missing(&self) -> InputCheckResult {
        if let Some(ref q) = self.data {
            let mut previous_result = q.find_missing();
            previous_result.extend_path(self.content.relative_template_path.clone());
            previous_result
        } else {
            InputCheckResult::from_missing(Some(self.content.relative_template_path.clone()))
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
                        let data_res = <QuestionInput>::from_str(
                            &n.content[..],
                            file_to_load.file_path.clone(),
                        );
                        match data_res {
                            Ok(q) => {
                                let mut input = q.clone();
                                combine_question_with_default_files(
                                    file_to_load.file_path,
                                    &mut input,
                                );
                                let files_to_load = input.files_to_load(main_file_path);
                                let loaded_files =
                                    crate::support::file_manager::CACHE.read_files(files_to_load);
                                input.insert_loaded_files(main_file_path, &loaded_files);

                                self.content.data.iter().for_each(|(k, v)| {
                                    input.insert_template_value(k, &v.0);
                                });
                                self.data = Some(input)
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
                            self.content.relative_template_path
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
        previous_result.extend_path(self.content.relative_template_path.clone());
        previous_result
    }
}

impl Overwrite<QuestionFromTemplateInput> for QuestionFromTemplateInput {
    fn overwrite(&mut self, _other: &Self) {}
}

impl ToNumbas<numbas::question::Question> for QuestionFromTemplate {
    fn to_numbas(&self, locale: &str) -> numbas::question::Question {
        self.data
            .clone()
            .to_numbas_with_name(locale, self.content.relative_template_path.clone())
    }
}

impl ToRumbas<QuestionFromTemplate> for numbas::question::Question {
    fn to_rumbas(&self) -> QuestionFromTemplate {
        unreachable!()
        // TODO: handle variable overrride
    }
}

impl std::hash::Hash for QuestionFromTemplate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.content.hash(state);
    }
}
impl PartialEq for QuestionFromTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
impl Eq for QuestionFromTemplate {}

impl PartialEq for QuestionFromTemplateInput {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
impl Eq for QuestionFromTemplateInput {}
