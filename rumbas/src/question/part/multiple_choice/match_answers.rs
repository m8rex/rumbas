use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

optional_overwrite! {
    pub struct MatchAnswersItemMarks {
        marks: numbas::exam::Primitive,
        answer: TranslatableString
    }
}

optional_overwrite! {
    pub struct MatchAnswersItem {
        statement: TranslatableString,
        /// Map points to strings of answers ! use anchors in yaml
        answer_marks: Vec<MatchAnswersItemMarks>
    }
}

question_part_type! {
    pub struct QuestionPartMatchAnswersWithItems {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceMatchAnswerData,
        shuffle_answers: bool,
        shuffle_items: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
        /// !FLATTENED
        #[serde(flatten)]
        display: MatchAnswerWithItemsDisplay,
        layout: numbas::exam::MatchAnswersWithChoicesLayout,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type: numbas::exam::MultipleChoiceWarningType
        //min_marks & max_marks?
        //TODO wrong_nb_choices_warning:
        //TODO other?
    }
}
impl_optional_overwrite!(
    numbas::exam::MatchAnswersWithChoicesLayout,
    numbas::exam::MatchAnswersWithChoicesDisplayType
);
impl_to_numbas!(
    numbas::exam::MatchAnswersWithChoicesLayout,
    numbas::exam::MatchAnswersWithChoicesDisplayType
);

impl ToNumbas<numbas::exam::ExamQuestionPartMatchAnswersWithChoices>
    for QuestionPartMatchAnswersWithItems
{
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartMatchAnswersWithChoices {
        let (answers, choices, marking_matrix) = match self.answer_data.unwrap() {
            MultipleChoiceMatchAnswerData::ItemBased(data) => (
                VariableValued::Value(data.answers.clone()).to_numbas(locale),
                VariableValued::Value(
                    data.items
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|a| a.clone().unwrap().statement)
                                .collect::<Vec<_>>()
                        })
                        .unwrap(),
                )
                .to_numbas(locale),
                Some(
                    VariableValued::Value(
                        data.items // TODO: better handling
                            .unwrap(),
                    )
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                data.answers
                                    .clone()
                                    .unwrap()
                                    .iter()
                                    .map(|a| {
                                        i.unwrap()
                                            .answer_marks
                                            .unwrap()
                                            .iter()
                                            .find(|am| am.answer.unwrap() == a.unwrap())
                                            .map_or_else(|| 0usize.into(), |v| v.marks.unwrap())
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .to_numbas(locale),
                ),
            ),
            MultipleChoiceMatchAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                data.choices.to_numbas(locale),
                Some(data.marks.to_numbas(locale)),
            ),
        };
        numbas::exam::ExamQuestionPartMatchAnswersWithChoices {
            part_data: self.to_numbas_shared_data(locale),
            min_answers: Some(self.should_select_at_least.to_numbas(locale)),
            max_answers: self.should_select_at_most.to_numbas(locale),
            min_marks: Some(0.into()),
            max_marks: Some(0.into()),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            shuffle_choices: self.shuffle_items.to_numbas(locale),
            answers,
            choices,
            wrong_nb_choices_warning: self.wrong_nb_answers_warning_type.to_numbas(locale),
            layout: self.layout.to_numbas(locale),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale),
            marking_matrix,
            display_type: self.display.to_numbas(locale),
        }
    }
}

impl ToRumbas<MultipleChoiceMatchAnswerData>
    for numbas::exam::ExamQuestionPartMatchAnswersWithChoices
{
    fn to_rumbas(&self) -> MultipleChoiceMatchAnswerData {
        if let (
            numbas::exam::VariableValued::Value(answer_options),
            numbas::exam::VariableValued::Value(choice_options),
            Some(numbas::exam::VariableValued::Value(marking_matrix)),
        ) = (
            self.answers.clone(),
            self.choices.clone(),
            self.marking_matrix.clone(),
        ) {
            let items_data: Vec<_> = choice_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .collect();

            MultipleChoiceMatchAnswerData::ItemBased({
                let answers: Vec<_> = answer_options
                    .iter()
                    .map(|a| Value::Normal(a.clone().into()))
                    .collect();
                MultipleChoiceMatchAnswers {
                    answers: Value::Normal(answers.clone()),
                    items: Value::Normal(
                        items_data
                            .into_iter()
                            .map(|(statement, marks)| {
                                Value::Normal(MatchAnswersItem {
                                    statement: Value::Normal(statement.into()),
                                    answer_marks: Value::Normal(
                                        marks
                                            .into_iter()
                                            .enumerate()
                                            .map(|(i, m)| MatchAnswersItemMarks {
                                                marks: Value::Normal(m),
                                                answer: answers.get(i).unwrap().clone(),
                                            })
                                            .collect(),
                                    ),
                                })
                            })
                            .collect(),
                    ),
                }
            })
        } else {
            MultipleChoiceMatchAnswerData::NumbasLike(MultipleChoiceMatchAnswerDataNumbasLike {
                answers: Value::Normal(
                    self.answers
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|vv| vv.clone().into())
                                .collect::<Vec<TranslatableString>>()
                        })
                        .to_rumbas(),
                ),
                choices: Value::Normal(
                    self.choices
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|vv| vv.clone().into())
                                .collect::<Vec<TranslatableString>>()
                        })
                        .to_rumbas(),
                ),
                marks: Value::Normal(
                    self.marking_matrix
                        .clone()
                        .map(|m| m.to_rumbas())
                        .expect("How can the marking matrix be optional?"),
                ),
            })
        }
    }
}

impl ToRumbas<QuestionPartMatchAnswersWithItems>
    for numbas::exam::ExamQuestionPartMatchAnswersWithChoices
{
    fn to_rumbas(&self) -> QuestionPartMatchAnswersWithItems {
        let custom_marking_algorithm_notes: Option<_> =
            self.part_data.custom_marking_algorithm.to_rumbas();
        QuestionPartMatchAnswersWithItems {
            // Default section
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(extract_part_common_prompt(&self.part_data)),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&self.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&self.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&self.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &self.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&self.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &self.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &self.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                self.part_data.variable_replacement_strategy.to_rumbas(),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &self.part_data,
            )),
            custom_marking_algorithm_notes: Value::Normal(
                custom_marking_algorithm_notes.unwrap_or_default(),
            ),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),

            answer_data: Value::Normal(self.to_rumbas()),
            shuffle_answers: Value::Normal(self.shuffle_answers),
            shuffle_items: Value::Normal(self.shuffle_choices),
            show_cell_answer_state: Value::Normal(self.show_cell_answer_state),
            should_select_at_least: Value::Normal(
                self.min_answers
                    .unwrap_or(DEFAULTS.match_answers_with_items_min_answers)
                    .0,
            ),
            should_select_at_most: Value::Normal(
                self.max_answers
                    .map(|v| v.0)
                    .map(Noneable::NotNone)
                    .unwrap_or_else(Noneable::nn),
            ),
            display: Value::Normal(self.display_type.to_rumbas()),
            layout: Value::Normal(self.layout.clone()),
            wrong_nb_answers_warning_type: Value::Normal(self.wrong_nb_choices_warning),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum MatchAnswerWithItemsDisplay {
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "check")]
    Check,
}
impl_optional_overwrite!(MatchAnswerWithItemsDisplay);

impl ToNumbas<numbas::exam::MatchAnswersWithChoicesDisplayType> for MatchAnswerWithItemsDisplay {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::MatchAnswersWithChoicesDisplayType {
        match self {
            MatchAnswerWithItemsDisplay::Check => {
                numbas::exam::MatchAnswersWithChoicesDisplayType::Check
            }
            MatchAnswerWithItemsDisplay::Radio => {
                numbas::exam::MatchAnswersWithChoicesDisplayType::Radio
            }
        }
    }
}

impl ToRumbas<MatchAnswerWithItemsDisplay> for numbas::exam::MatchAnswersWithChoicesDisplayType {
    fn to_rumbas(&self) -> MatchAnswerWithItemsDisplay {
        match self {
            numbas::exam::MatchAnswersWithChoicesDisplayType::Check => {
                MatchAnswerWithItemsDisplay::Check
            }
            numbas::exam::MatchAnswersWithChoicesDisplayType::Radio => {
                MatchAnswerWithItemsDisplay::Radio
            }
        }
    }
}

optional_overwrite_enum! {
    #[serde(tag = "type")]
    pub enum MultipleChoiceMatchAnswerData {
        #[serde(rename = "item_based")]
        ItemBased(MultipleChoiceMatchAnswers),
        #[serde(rename = "numbas_like")]
        NumbasLike(MultipleChoiceMatchAnswerDataNumbasLike)
    }
}

optional_overwrite! {
    pub struct MultipleChoiceMatchAnswerDataNumbasLike {
        answers: VariableValued<Vec<TranslatableString>>,
        choices: VariableValued<Vec<TranslatableString>>,
        marks: VariableValued<Vec<Vec<numbas::exam::Primitive>>>
    }
}

optional_overwrite! {
    pub struct MultipleChoiceMatchAnswers {
        /// Values of the answers
        answers: Vec<Value<TranslatableString>>,
        /// Items for which the answer can be selected
        items: Vec<Value<MatchAnswersItem>>
    }
}
