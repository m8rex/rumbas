use crate::question::Question;
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::yaml::YamlError;
use sanitize_filename::sanitize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct QuestionGroup {
        /// The name
        name: TranslatableString,
        /// The strategy to use to pick the questions to show
        #[serde(flatten)]
        picking_strategy: PickingStrategy,
        /// The questions
        questions: Vec<Value<QuestionPath>>
    }
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "picking_strategy")]
pub enum PickingStrategy {
    #[serde(rename = "all_ordered")]
    AllOrdered,
    #[serde(rename = "all_shuffled")]
    AllShuffled,
    #[serde(rename = "random_subset")]
    RandomSubset { pick_questions: usize },
}
impl_optional_overwrite!(PickingStrategy);

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
            PickingStrategy::RandomSubset { pick_questions } => {
                numbas::exam::question_group::QuestionGroupPickingStrategy::RandomSubset {
                    pick_questions: *pick_questions,
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
            } => PickingStrategy::RandomSubset {
                pick_questions: *pick_questions,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct QuestionPath {
    pub question_name: String,
    pub question_data: Question,
}

impl RumbasCheck for QuestionPath {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        self.question_data.check(locale)
    }
}

impl OptionalOverwrite<QuestionPath> for QuestionPath {
    fn overwrite(&mut self, _other: &Self) {}
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.question_data.insert_template_value(key, val);
    }
}
impl_optional_overwrite_value!(QuestionPath);

impl ToNumbas<numbas::question::question::Question> for QuestionPath {
    fn to_numbas(&self, locale: &str) -> numbas::question::question::Question {
        self.question_data
            .clone()
            .to_numbas_with_name(locale, self.question_name.clone())
    }
}

impl ToRumbas<QuestionPath> for numbas::question::question::Question {
    fn to_rumbas(&self) -> QuestionPath {
        QuestionPath {
            question_name: sanitize(&self.name),
            question_data: self.to_rumbas(),
        }
    }
}

impl JsonSchema for QuestionPath {
    fn schema_name() -> String {
        "QuestionPath".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

impl std::convert::TryFrom<String> for QuestionPath {
    type Error = YamlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let question_data = Question::from_name(&s).map_err(|e| e)?;
        Ok(QuestionPath {
            question_name: s,
            question_data,
        })
    }
}

impl std::convert::From<QuestionPath> for String {
    fn from(q: QuestionPath) -> Self {
        q.question_name
    }
}
