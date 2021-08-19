use crate::data::optional_overwrite::VariableValued;
use crate::data::optional_overwrite::*;
use crate::data::question_part::JMENotes;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::ToNumbas;
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
use crate::data::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

//TODO: defaults
question_part_type! {
    pub struct QuestionPartChooseOne {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        /// Whether the student has to select an option (if false: can submit without selecting)
        has_to_select_option: bool,
        /// !FLATTENED: all its attributes should be added to [QuestionPartChooseOne]
        #[serde(flatten)]
        display: ChooseOneDisplay
        //TODO wrong_nb_choices_warning:
    }
}

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum MultipleChoiceAnswerData {
        ItemBased(Vec<MultipleChoiceAnswer>),
        NumbasLike(Box<MultipleChoiceAnswerDataNumbasLike>)
    }
}

optional_overwrite! {
    pub struct MultipleChoiceAnswerDataNumbasLike {
        answers: VariableValued<Vec<TranslatableString>>,
        marks: VariableValued<Vec<numbas::exam::Primitive>>,
        feedback: Noneable<Vec<TranslatableString>>
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixRowPrimitive(Vec<numbas::exam::Primitive>);
impl_optional_overwrite!(MatrixRowPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixRowPrimitive {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Row(self.0.clone())
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixRow(Vec<TranslatableString>);
impl_optional_overwrite!(MatrixRow); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixRow {
    fn to_numbas(&self, locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Row(
            self.0
                .to_numbas(locale)
                .into_iter()
                .map(|a| a.into())
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixPrimitive(Vec<VariableValued<Vec<numbas::exam::Primitive>>>);
impl_optional_overwrite!(MatrixPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixPrimitive {
    fn to_numbas(&self, locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Matrix(self.0.to_numbas(locale))
    }
}
impl ToNumbas<numbas::exam::ExamQuestionPartChooseOne> for QuestionPartChooseOne {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartChooseOne {
        let (answers, marking_matrix, distractors) = match self.answer_data.unwrap() {
            MultipleChoiceAnswerData::ItemBased(answers) => (
                VariableValued::Value(
                    answers
                        .iter()
                        .map(|a| a.statement.clone().unwrap())
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                Some(
                    VariableValued::Value(
                        answers
                            .iter()
                            .map(|a| a.marks.clone().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .to_numbas(locale),
                ),
                Some(
                    answers
                        .iter()
                        .map(|a| {
                            a.feedback.clone().unwrap() //TODO
                        })
                        .collect::<Vec<_>>()
                        .to_numbas(locale),
                ),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                Some(data.marks.to_numbas(locale)),
                data.feedback.map(|f| f.to_numbas(locale)).flatten(),
            ),
        };
        numbas::exam::ExamQuestionPartChooseOne {
            part_data: self.to_numbas_shared_data(locale),
            min_answers: Some(if self.has_to_select_option.to_numbas(locale) {
                1
            } else {
                0
            }),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            answers,
            display_type: self.display.unwrap().get_numbas_type(),
            columns: self.display.unwrap().get_nb_columns().into(),
            wrong_nb_choices_warning: Some(numbas::exam::MultipleChoiceWarningType::None), //TODO
            show_cell_answer_state: Some(self.show_cell_answer_state.to_numbas(locale)),
            marking_matrix,
            distractors,
        }
    }
}

fn extract_multiple_choice_answer_data(
    answers: &numbas::exam::VariableValued<Vec<String>>,
    marking_matrix: &Option<numbas::exam::VariableValued<Vec<numbas::exam::Primitive>>>,
    distractors: &Option<Vec<String>>,
) -> MultipleChoiceAnswerData {
    if let (
        numbas::exam::VariableValued::Value(answer_options),
        Some(numbas::exam::VariableValued::Value(marking_matrix)),
    ) = (answers.clone(), marking_matrix.clone())
    {
        let answers_data: Vec<_> = match distractors.clone() {
            None => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .map(|(a, b)| (a, b, "".to_string()))
                .collect(),
            Some(d) => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .zip(d.into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect(),
        };
        MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(a, b, c)| MultipleChoiceAnswer {
                    statement: Value::Normal(a.into()),
                    marks: Value::Normal(b),
                    feedback: Value::Normal(c.into()),
                })
                .collect(),
        )
    } else {
        MultipleChoiceAnswerData::NumbasLike(Box::new(MultipleChoiceAnswerDataNumbasLike {
            answers: Value::Normal(
                answers
                    .clone()
                    .map(|v| {
                        v.iter()
                            .map(|vv| vv.clone().into())
                            .collect::<Vec<TranslatableString>>()
                    })
                    .to_rumbas(),
            ),
            marks: Value::Normal(
                marking_matrix
                    .clone()
                    .map(|m| m.to_rumbas())
                    .expect("How can the marking matrix be optional?"),
            ),
            feedback: Value::Normal(
                distractors
                    .clone()
                    .map(|v| {
                        Noneable::NotNone(
                            v.into_iter()
                                .map(|f| f.into())
                                .collect::<Vec<TranslatableString>>()
                                .to_rumbas(),
                        )
                    })
                    .unwrap_or_else(Noneable::nn),
            ),
        }))
    }
}

impl ToRumbas<MultipleChoiceAnswerData> for numbas::exam::ExamQuestionPartChooseOne {
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.answers, &self.marking_matrix, &self.distractors)
    }
}

impl ToRumbas<QuestionPartChooseOne> for numbas::exam::ExamQuestionPartChooseOne {
    fn to_rumbas(&self) -> QuestionPartChooseOne {
        QuestionPartChooseOne {
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
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
            ),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),
            answer_data: Value::Normal(self.to_rumbas()),
            display: Value::Normal(self.to_rumbas()),
            shuffle_answers: Value::Normal(self.shuffle_answers),
            show_cell_answer_state: Value::Normal(
                self.show_cell_answer_state
                    .unwrap_or(DEFAULTS.choose_one_show_cell_answer_state),
            ),
            has_to_select_option: Value::Normal(
                self.min_answers
                    .map(|v| v == 1)
                    .unwrap_or(DEFAULTS.choose_one_has_to_select_option),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum ChooseOneDisplay {
    #[serde(rename = "dropdown")]
    DropDown,
    #[serde(rename = "radio")]
    Radio { columns: usize },
}
impl_optional_overwrite!(ChooseOneDisplay);

impl ChooseOneDisplay {
    pub fn get_numbas_type(&self) -> numbas::exam::ChooseOneDisplayType {
        match self {
            ChooseOneDisplay::DropDown => numbas::exam::ChooseOneDisplayType::DropDown,
            ChooseOneDisplay::Radio { columns: _ } => numbas::exam::ChooseOneDisplayType::Radio,
        }
    }

    pub fn get_nb_columns(&self) -> usize {
        match self {
            ChooseOneDisplay::DropDown => 0,
            ChooseOneDisplay::Radio { columns } => *columns,
        }
    }
}

impl ToRumbas<ChooseOneDisplay> for numbas::exam::ExamQuestionPartChooseOne {
    fn to_rumbas(&self) -> ChooseOneDisplay {
        match self.display_type {
            numbas::exam::ChooseOneDisplayType::Radio => ChooseOneDisplay::Radio {
                columns: self.columns.0,
            },
            numbas::exam::ChooseOneDisplayType::DropDown => ChooseOneDisplay::DropDown,
        }
    }
}

impl_to_numbas!(numbas::exam::MultipleChoiceMatrix);
impl_optional_overwrite!(numbas::exam::MultipleChoiceMatrix);

impl_to_numbas!(numbas::exam::Primitive);
impl_optional_overwrite!(numbas::exam::Primitive);

optional_overwrite! {
    pub struct MultipleChoiceAnswer {
        statement: TranslatableString,
        feedback: TranslatableString,
        marks: numbas::exam::Primitive // TODO: variable valued?
    }
}

question_part_type! {
    pub struct QuestionPartChooseMultiple {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
        columns: usize,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type:  numbas::exam::MultipleChoiceWarningType
        //min_marks & max_marks?
        //TODO other?
    }
}
impl_optional_overwrite!(numbas::exam::MultipleChoiceWarningType);
impl_to_numbas!(numbas::exam::MultipleChoiceWarningType);

impl ToNumbas<numbas::exam::ExamQuestionPartChooseMultiple> for QuestionPartChooseMultiple {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionPartChooseMultiple {
        // TODO: below is duplicated in CHooseOne
        let (choices, marking_matrix, distractors) = match self.answer_data.unwrap() {
            MultipleChoiceAnswerData::ItemBased(answers) => (
                VariableValued::Value(
                    answers
                        .iter()
                        .map(|a| a.statement.clone().unwrap())
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                Some(
                    VariableValued::Value(
                        answers
                            .iter()
                            .map(|a| a.marks.clone().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .to_numbas(locale),
                ),
                Some(
                    answers
                        .iter()
                        .map(|a| {
                            a.feedback.clone().unwrap() //TODO
                        })
                        .collect::<Vec<_>>()
                        .to_numbas(locale),
                ),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                Some(data.marks.to_numbas(locale)),
                data.feedback.map(|f| f.to_numbas(locale)).flatten(),
            ),
        };
        numbas::exam::ExamQuestionPartChooseMultiple {
            part_data: self.to_numbas_shared_data(locale),
            min_answers: Some(self.should_select_at_least.to_numbas(locale)),
            max_answers: self.should_select_at_most.to_numbas(locale),
            min_marks: Some(0usize), // todo?
            max_marks: Some(0usize.into()),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            choices,
            display_columns: self.columns.to_numbas(locale),
            wrong_nb_choices_warning: self.wrong_nb_answers_warning_type.to_numbas(locale),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale),
            marking_matrix,
            distractors,
        }
    }
}

impl ToRumbas<MultipleChoiceAnswerData> for numbas::exam::ExamQuestionPartChooseMultiple {
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.choices, &self.marking_matrix, &self.distractors)
    }
}

impl ToRumbas<QuestionPartChooseMultiple> for numbas::exam::ExamQuestionPartChooseMultiple {
    fn to_rumbas(&self) -> QuestionPartChooseMultiple {
        QuestionPartChooseMultiple {
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
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
            ),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),

            answer_data: Value::Normal(self.to_rumbas()),
            shuffle_answers: Value::Normal(self.shuffle_answers),
            show_cell_answer_state: Value::Normal(self.show_cell_answer_state),
            should_select_at_least: Value::Normal(
                self.min_answers
                    .unwrap_or(DEFAULTS.choose_multiple_min_answers)
                    .0,
            ),
            should_select_at_most: Value::Normal(
                self.max_answers
                    .map(|v| v.0)
                    .map(Noneable::NotNone)
                    .unwrap_or_else(Noneable::nn),
            ),
            columns: Value::Normal(self.display_columns.0),
            wrong_nb_answers_warning_type: Value::Normal(self.wrong_nb_choices_warning),
        }
    }
}

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
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
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
