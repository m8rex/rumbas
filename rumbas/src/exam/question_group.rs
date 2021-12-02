use crate::question::Question;
use crate::question::QuestionInput;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use rumbas_support::preamble::*;
use sanitize_filename::sanitize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "QuestionGroupInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "PickingStrategyInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
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

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "PickingStrategyRandomSubsetInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PickingStrategyRandomSubset {
    pub pick_questions: usize,
}

// TODO: remove this JsonSchema
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuestionPath {
    pub question_name: String,
    pub question_data: Question,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct QuestionPathInput {
    pub question_name: String,
    pub question_data: QuestionInput,
}

impl InputInverse for QuestionPath {
    type Input = QuestionPathInput;
    type EnumInput = QuestionPathInput;
}

impl Input for QuestionPathInput {
    type Normal = QuestionPath;
    fn to_normal(&self) -> Self::Normal {
        Self::Normal {
            question_name: self.question_name.to_owned(),
            question_data: self.question_data.to_normal(),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            question_name: normal.question_name,
            question_data: Input::from_normal(normal.question_data),
        }
    }
    fn find_missing(&self) -> InputCheckResult {
        self.question_data.find_missing()
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.question_data.insert_template_value(key, val);
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

impl std::convert::TryFrom<String> for QuestionPathInput {
    type Error = YamlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let question_data = QuestionInput::from_name(&s).map_err(|e| e)?;
        Ok(QuestionPathInput {
            question_name: s,
            question_data,
        })
    }
}

impl std::convert::From<QuestionPathInput> for String {
    fn from(q: QuestionPathInput) -> Self {
        q.question_name
    }
}
