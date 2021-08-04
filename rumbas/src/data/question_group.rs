use crate::data::optional_overwrite::*;
use crate::data::question::Question;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use crate::data::translatable::TranslatableString;
use crate::data::yaml::YamlError;
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

impl ToNumbas for QuestionGroup {
    type NumbasType = numbas::exam::ExamQuestionGroup;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamQuestionGroup> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamQuestionGroup {
                name: self.name.clone().map(|s| s.to_string(locale)).flatten(),
                picking_strategy: self
                    .picking_strategy
                    .clone()
                    .unwrap()
                    .to_numbas(locale)
                    .unwrap(),
                questions: self
                    .questions
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|q| q.to_numbas(locale).unwrap())
                    .collect(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionGroup> for numbas::exam::ExamQuestionGroup {
    fn to_rumbas(&self) -> QuestionGroup {
        QuestionGroup {
            name: Value::Normal(TranslatableString::s(
                &self.name.clone().unwrap_or_default(),
            )),
            picking_strategy: Value::Normal(self.picking_strategy.to_rumbas()),
            questions: Value::Normal(
                self.questions
                    .to_rumbas()
                    .into_iter()
                    .map(Value::Normal)
                    .collect(),
            ),
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

impl ToNumbas for PickingStrategy {
    type NumbasType = numbas::exam::ExamQuestionGroupPickingStrategy;
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> NumbasResult<numbas::exam::ExamQuestionGroupPickingStrategy> {
        Ok(match self {
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
        })
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
    fn check(&self) -> RumbasCheckResult {
        self.question_data.check()
    }
}
impl OptionalOverwrite<QuestionPath> for QuestionPath {
    fn overwrite(&mut self, other: &Self) {}
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.question_data.insert_template_value(key, val);
    }
}
impl_optional_overwrite_value!(QuestionPath);

impl JsonSchema for QuestionPath {
    fn schema_name() -> String {
        format!("QuestionPath")
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

impl ToNumbas for QuestionPath {
    type NumbasType = numbas::exam::ExamQuestion;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(self
                .question_data
                .clone()
                .to_numbas_with_name(locale, self.question_name.clone())
                .unwrap())
        } else {
            Err(check)
        }
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
