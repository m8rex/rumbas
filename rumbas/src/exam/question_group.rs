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

impl ToNumbas<numbas::exam::ExamQuestionGroup> for QuestionGroup {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionGroup {
        numbas::exam::ExamQuestionGroup {
            name: Some(self.name.to_numbas(locale)),
            picking_strategy: self.picking_strategy.to_numbas(locale),
            questions: self.questions.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionGroup> for numbas::exam::ExamQuestionGroup {
    fn to_rumbas(&self) -> QuestionGroup {
        QuestionGroup {
            name: Value::Normal(self.name.clone().unwrap_or_default().into()),
            picking_strategy: Value::Normal(self.picking_strategy.to_rumbas()),
            questions: Value::Normal(self.questions.to_rumbas()),
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

impl ToNumbas<numbas::exam::ExamQuestionGroupPickingStrategy> for PickingStrategy {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamQuestionGroupPickingStrategy {
        match self {
            PickingStrategy::AllOrdered => {
                numbas::exam::ExamQuestionGroupPickingStrategy::AllOrdered
            }
            PickingStrategy::AllShuffled => {
                numbas::exam::ExamQuestionGroupPickingStrategy::AllShuffled
            }
            PickingStrategy::RandomSubset { pick_questions } => {
                numbas::exam::ExamQuestionGroupPickingStrategy::RandomSubset {
                    pick_questions: *pick_questions,
                }
            }
        }
    }
}

impl ToRumbas<PickingStrategy> for numbas::exam::ExamQuestionGroupPickingStrategy {
    fn to_rumbas(&self) -> PickingStrategy {
        match self {
            numbas::exam::ExamQuestionGroupPickingStrategy::AllOrdered => {
                PickingStrategy::AllOrdered
            }
            numbas::exam::ExamQuestionGroupPickingStrategy::AllShuffled => {
                PickingStrategy::AllShuffled
            }
            numbas::exam::ExamQuestionGroupPickingStrategy::RandomSubset { pick_questions } => {
                PickingStrategy::RandomSubset {
                    pick_questions: *pick_questions,
                }
            }
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

impl ToNumbas<numbas::exam::ExamQuestion> for QuestionPath {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestion {
        self.question_data
            .clone()
            .to_numbas_with_name(locale, self.question_name.clone())
    }
}

impl ToRumbas<QuestionPath> for numbas::exam::ExamQuestion {
    fn to_rumbas(&self) -> QuestionPath {
        QuestionPath {
            question_name: sanitize(&self.name),
            question_data: self.to_rumbas(),
        }
    }
}
