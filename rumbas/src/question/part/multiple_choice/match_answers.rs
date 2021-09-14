use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionParts;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStrings;
use crate::support::variable_valued::VariableValued;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck)]
    #[input(name = "QuestionPartMatchAnswersWithItemsInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct QuestionPartMatchAnswersWithItems {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceMatchAnswerData,
        shuffle_answers: RumbasBool,
        shuffle_items: RumbasBool,
        show_cell_answer_state: RumbasBool,
        should_select_at_least: RumbasNatural,
        should_select_at_most: NoneableNatural,
        /// !FLATTENED
        #[serde(flatten)]
        display: MatchAnswerWithItemsDisplay,
        layout: MatchAnswersWithChoicesLayout,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type: MultipleChoiceWarningType
        //min_marks & max_marks?
        //TODO wrong_nb_choices_warning:
        //TODO other?
    }
}

type MatchAnswersWithChoicesLayout =
    numbas::question::part::match_answers::MatchAnswersWithChoicesLayout;

type MultipleChoiceWarningType = numbas::question::part::match_answers::MultipleChoiceWarningType;

impl_to_numbas!(
    numbas::question::part::match_answers::MatchAnswersWithChoicesLayout,
    numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType
);

impl ToNumbas<numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices>
    for QuestionPartMatchAnswersWithItems
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices {
        let (answers, choices, marking_matrix) = match &self.answer_data {
            MultipleChoiceMatchAnswerData::ItemBased(data) => (
                VariableValued::Value(data.answers.clone()).to_numbas(locale),
                VariableValued::Value(
                    data.items
                        .iter()
                        .map(|a| a.clone().statement)
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                Some(
                    VariableValued::Value(
                        data.items.clone(), // TODO: better handling
                    )
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                data.answers
                                    .iter()
                                    .map(|a| {
                                        i.answer_marks
                                            .iter()
                                            .find(|am| &am.answer == a)
                                            .map_or_else(|| 0usize.into(), |v| v.marks.clone())
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
        numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices {
            part_data: self.to_numbas(locale),
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

impl ToRumbas<QuestionPartMatchAnswersWithItems>
    for numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices
{
    fn to_rumbas(&self) -> QuestionPartMatchAnswersWithItems {
        create_question_part! {
            QuestionPartMatchAnswersWithItems with &self.part_data => {

                answer_data: self.to_rumbas(),
                shuffle_answers: self.shuffle_answers.to_rumbas(),
                shuffle_items: self.shuffle_choices.to_rumbas(),
                show_cell_answer_state: self.show_cell_answer_state.to_rumbas(),
                should_select_at_least:
                    self.min_answers
                        .unwrap_or(DEFAULTS.match_answers_with_items_min_answers)
                        .0.to_rumbas(),
                should_select_at_most:
                    self.max_answers
                        .map(|v| v.0).to_rumbas()
                ,
                display: self.display_type.to_rumbas(),
                layout: self.layout.clone(),
                wrong_nb_answers_warning_type: self.wrong_nb_choices_warning.to_rumbas()
            }
        }
    }
}

impl ToRumbas<MultipleChoiceMatchAnswerData>
    for numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices
{
    fn to_rumbas(&self) -> MultipleChoiceMatchAnswerData {
        if let (
            numbas::support::primitive::VariableValued::Value(answer_options),
            numbas::support::primitive::VariableValued::Value(choice_options),
            Some(numbas::support::primitive::VariableValued::Value(marking_matrix)),
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
                let answers: Vec<_> = answer_options.iter().map(|a| a.clone().into()).collect();
                MultipleChoiceMatchAnswers {
                    answers: answers.clone(),
                    items: items_data
                        .into_iter()
                        .map(|(statement, marks)| {
                            MatchAnswersItem {
                                // TODO: extract to ToRumbas?
                                statement: statement.into(),
                                answer_marks: marks
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, m)| MatchAnswersItemMarks {
                                        marks: m,
                                        answer: answers.get(i).unwrap().clone(),
                                    })
                                    .collect(),
                            }
                        })
                        .collect(),
                }
            })
        } else {
            MultipleChoiceMatchAnswerData::NumbasLike(MultipleChoiceMatchAnswerDataNumbasLike {
                answers: self
                    .answers
                    .clone()
                    /* .map(|v| {
                        v.iter()
                            .map(|vv| vv.clone().into())
                            .collect::<Vec<TranslatableString>>()
                    })*/
                    .to_rumbas(),
                choices: self
                    .choices
                    .clone()
                    /* .map(|v| {
                        v.iter()
                            .map(|vv| vv.clone().into())
                            .collect::<Vec<TranslatableString>>()
                    })*/
                    .to_rumbas(),

                marks: self
                    .marking_matrix
                    .clone()
                    .map(|m| m.to_rumbas())
                    .expect("How can the marking matrix be optional?"),
            })
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatchAnswerWithItemsDisplayInput")]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(tag = "display")]
pub enum MatchAnswerWithItemsDisplay {
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "check")]
    Check,
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType>
    for MatchAnswerWithItemsDisplay
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType {
        match self {
            MatchAnswerWithItemsDisplay::Check => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Check
            }
            MatchAnswerWithItemsDisplay::Radio => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Radio
            }
        }
    }
}

impl ToRumbas<MatchAnswerWithItemsDisplay>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType
{
    fn to_rumbas(&self) -> MatchAnswerWithItemsDisplay {
        match self {
            numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Check => {
                MatchAnswerWithItemsDisplay::Check
            }
            numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Radio => {
                MatchAnswerWithItemsDisplay::Radio
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MultipleChoiceMatchAnswerDataInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "type")]
pub enum MultipleChoiceMatchAnswerData {
    #[serde(rename = "item_based")]
    ItemBased(MultipleChoiceMatchAnswers),
    #[serde(rename = "numbas_like")]
    NumbasLike(MultipleChoiceMatchAnswerDataNumbasLike),
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MultipleChoiceMatchAnswerDataNumbasLikeInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MultipleChoiceMatchAnswerDataNumbasLike {
    answers: VariableValuedTranslatableStrings,
    choices: VariableValuedTranslatableStrings,
    marks: VariableValuedPrimitivess,
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MultipleChoiceMatchAnswersInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MultipleChoiceMatchAnswers {
    /// Values of the answers
    answers: TranslatableStrings,
    /// Items for which the answer can be selected
    items: MatchAnswersItems,
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatchAnswersItemInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatchAnswersItem {
    statement: TranslatableString,
    /// Map points to strings of answers ! use anchors in yaml
    answer_marks: MatchAnswersItemMarksList,
}

type MatchAnswersItems = Vec<MatchAnswersItem>;

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatchAnswersItemMarksInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatchAnswersItemMarks {
    marks: Primitive,
    answer: TranslatableString,
}

type MatchAnswersItemMarksList = Vec<MatchAnswersItemMarks>;
