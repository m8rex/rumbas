use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::question::Question;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use crate::data::yaml::YamlError;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    QuestionGroup,
    name: TranslatableString,
    picking_strategy: PickingStrategy: serde(flatten),
    questions: Vec<QuestionPath>
}

impl ToNumbas for QuestionGroup {
    type NumbasType = numbas::exam::ExamQuestionGroup;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamQuestionGroup> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamQuestionGroup::new(
                self.name.clone().map(|s| s.to_string(&locale)).flatten(),
                self.picking_strategy
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
                self.questions //TODO: add ToNumbas to QuestionPath?
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|q| q.to_numbas(&locale).unwrap())
                    .collect(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
        _locale: &String,
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

optional_overwrite! {
    QuestionPath: serde(try_from = "String"),
    question_name: String,
    question_data: Question
}

impl std::convert::TryFrom<String> for QuestionPath {
    type Error = YamlError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let question_data = Question::from_name(&s).map_err(|e| {
            println!("{}", e);
            e
        })?;
        Ok(QuestionPath {
            question_name: Some(s),
            question_data: Some(question_data),
        })
    }
}

impl ToNumbas for QuestionPath {
    type NumbasType = numbas::exam::ExamQuestion;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(self
                .question_data
                .clone()
                .unwrap()
                .to_numbas_with_name(&locale, self.question_name.clone().unwrap())
                .unwrap())
        } else {
            Err(empty_fields)
        }
    }
}
