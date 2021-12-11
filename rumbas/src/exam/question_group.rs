use crate::question::Question;
use crate::question::QuestionInput;
use crate::support::file_manager::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use rumbas_support::preamble::*;
use sanitize_filename::sanitize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "QuestionGroupInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
pub struct QuestionGroup {
    /// The name
    pub name: TranslatableString,
    /// The strategy to use to pick the questions to show
    #[serde(flatten)]
    pub picking_strategy: PickingStrategy,
    /// The questions
    pub questions: Vec<QuestionPath>,
}

impl ToNumbas<numbas::exam::question_group::QuestionGroup> for QuestionGroup {
    fn to_numbas(&self, locale: &str) -> numbas::exam::question_group::QuestionGroup {
        numbas::exam::question_group::QuestionGroup {
            name: Some(self.name.to_numbas(locale)),
            picking_strategy: self.picking_strategy.to_numbas(locale),
            questions: self.questions.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionGroup> for numbas::exam::question_group::QuestionGroup {
    fn to_rumbas(&self) -> QuestionGroup {
        QuestionGroup {
            name: self.name.clone().unwrap_or_default().to_rumbas(),
            picking_strategy: self.picking_strategy.to_rumbas(),
            questions: self.questions.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "PickingStrategyInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
pub struct PickingStrategyRandomSubset {
    pub pick_questions: usize,
}

// TODO: remove this JsonSchema
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct QuestionPath {
    pub question_name: String,
    pub question_data: Question,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct QuestionPathInput {
    pub question_name: String,
    pub question_data: Option<QuestionInput>,
    pub error_message: Option<String>,
}

impl InputInverse for QuestionPath {
    type Input = QuestionPathInput;
    type EnumInput = QuestionPathInput;
}

impl Examples for QuestionPathInput {
    fn examples() -> Vec<Self> {
        QuestionInput::examples()
            .into_iter()
            .map(|e| QuestionPathInput {
                question_name: "".to_string(),
                question_data: None,
                error_message: None,
            })
            .collect()
    }
}
impl QuestionPathInput {
    pub fn file_to_read(&self) -> FileToRead {
        FileToRead::Question(QuestionFileToRead::with_file_name(self.question_name))
    }
}

impl Input for QuestionPathInput {
    type Normal = QuestionPath;
    fn to_normal(&self) -> Self::Normal {
        Self::Normal {
            question_name: self.question_name.to_owned(),
            question_data: self.question_data.unwrap().to_normal(),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            question_name: normal.question_name,
            question_data: Some(Input::from_normal(normal.question_data)),
            error_message: None,
        }
    }
    fn find_missing(&self) -> InputCheckResult {
        if let Some(q) = self.question_data {
            q.find_missing()
        } else {
            InputCheckResult::from_missing(Some(self.question_name.clone()))
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        if let Some(ref mut q) = self.question_data {
            q.insert_template_value(key, val);
        }
    }
    fn files_to_load(&self) -> Vec<FileToLoad> {
        if let Some(q) = self.question_data {
            q.files_to_load()
        } else {
            let file = self.file_to_read();
            vec![file.into()]
        }
    }

    fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
        if let Some(q) = self.question_data {
            q.insert_loaded_files(files);
        } else {
            let file = self.file_to_read();
            if let Some(f) = file {
                let file: FileToLoad = f.into();
                let file = files.get(&file);
                match file {
                    Some(LoadedFile::Normal(n)) => {
                        let question_data_res = QuestionInput::from_str(&n.content[..]);
                        match question_data_res {
                            Ok(q) => self.question_data = Some(q.clone()),
                            Err(e) => self.error_message = Some(e.clone()),
                        }
                    }
                    Some(LoadedFile::Localized(l)) => {
                        unreachable!()
                    }
                    None => self.error_message = Some(format!("Missing content")),
                }
            }
        }
    }
}

impl RumbasCheck for QuestionPath {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        self.question_data.check(locale)
    }
}

impl Overwrite<QuestionPathInput> for QuestionPathInput {
    fn overwrite(&mut self, _other: &Self) {}
}

impl ToNumbas<numbas::question::Question> for QuestionPath {
    fn to_numbas(&self, locale: &str) -> numbas::question::Question {
        self.question_data
            .clone()
            .to_numbas_with_name(locale, self.question_name.clone())
    }
}

impl ToRumbas<QuestionPath> for numbas::question::Question {
    fn to_rumbas(&self) -> QuestionPath {
        QuestionPath {
            question_name: sanitize(&self.name),
            question_data: self.to_rumbas(),
        }
    }
}

impl JsonSchema for QuestionPathInput {
    fn schema_name() -> String {
        "QuestionPath".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

impl std::convert::From<String> for QuestionPathInput {
    fn try_from(s: String) -> Result<Self, Self::Error> {
        //let question_data = QuestionInput::from_name(&s).map_err(|e| e)?;
        Ok(QuestionPathInput {
            question_name: s,
            question_data: None,
            error_message: None,
        })
    }
}

impl std::convert::From<QuestionPathInput> for String {
    fn from(q: QuestionPathInput) -> Self {
        /*let q_yaml = crate::question::QuestionFileTypeInput::Normal(Box::new(q.question_data))
            .to_yaml()
            .unwrap();
        let file = format!("{}/{}.yaml", crate::QUESTIONS_FOLDER, q.question_name);
        log::info!("Writing to {}", file);
        std::fs::write(file, q_yaml).unwrap(); //fix handle result (try_from)
        */
        q.question_name
    }
}
