use crate::question::part::multiple_choice::MultipleChoiceMarkingMethod;
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionPart;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::variable_valued::VariableValued;
use comparable::Comparable;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::convert::TryInto;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartMatchAnswersWithItemsInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartMatchAnswersWithItems {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceMatchAnswerData,
        shuffle_answers: bool,
        shuffle_items: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
        display: MatchAnswerWithItemsDisplay,
        layout: MatchAnswersWithChoicesLayout,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type: MultipleChoiceWarningType,
        /// If the student would have scored less than this many marks, they are instead awarded this many. Useful in combination with negative marking.
        minimal_achievable_marks: Noneable<usize>,
        /// If the student would have scored more than this many marks, they are instead awarded this many. The value 0 means “no maximum mark”.
        maximal_achievable_marks: Noneable<usize>

        //TODO other?
    }
}

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
                                        .map_or_else(
                                            || "0".to_string().try_into().unwrap(),
                                            |v| v.marks.clone(),
                                        )
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .to_numbas(locale),
            ),
            MultipleChoiceMatchAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                data.choices.to_numbas(locale),
                data.marks.to_numbas(locale),
            ),
        };
        numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices {
            part_data: self.to_numbas(locale),
            min_answers: self.should_select_at_least.into(),
            max_answers: self
                .should_select_at_most
                .to_numbas(locale)
                .unwrap_or_default(),
            min_marks: self
                .minimal_achievable_marks
                .to_numbas(locale)
                .unwrap_or_default(),
            max_marks: self
                .maximal_achievable_marks
                .to_numbas(locale)
                .unwrap_or_default(),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            shuffle_choices: self.shuffle_items.to_numbas(locale),
            answers,
            choices,
            wrong_nb_answers_warning: self.wrong_nb_answers_warning_type.to_numbas(locale),
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
                    self.min_answers.0.to_rumbas(),
                should_select_at_most: if self.max_answers.0 > 0 {
                    Noneable::NotNone(self.max_answers.0.to_rumbas())
                } else {
                    Noneable::None
                },
                display: self.display_type.to_rumbas(),
                layout: self.layout.to_rumbas(),
                wrong_nb_answers_warning_type: self.wrong_nb_answers_warning.to_rumbas(),
                minimal_achievable_marks: if self.min_marks.0 > 0 { Noneable::NotNone(self.min_marks.0.to_rumbas()) } else { Noneable::None },
                maximal_achievable_marks: if self.max_marks.0 > 0 { Noneable::NotNone(self.max_marks.0.to_rumbas()) } else { Noneable::None }
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
            numbas::support::primitive::VariableValued::Value(marking_matrix),
        ) = (
            self.answers.clone(),
            self.choices.clone(),
            self.marking_matrix.clone(),
        ) {
            let items_data = choice_options.into_iter().zip(marking_matrix.into_iter());

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
                                    .map(|(i, marks)| MatchAnswersItemMarks {
                                        marks,
                                        answer: answers.get(i).unwrap().clone(),
                                    })
                                    .collect(),
                            }
                        })
                        .collect(),
                }
            })
        } else {
            MultipleChoiceMatchAnswerData::NumbasLike(Box::new(
                MultipleChoiceMatchAnswerDataNumbasLike {
                    answers: self.answers.to_rumbas(),
                    choices: self.choices.to_rumbas(),

                    marks: self.marking_matrix.to_rumbas(),
                },
            ))
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MatchAnswerWithItemsDisplayInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum MatchAnswerWithItemsDisplay {
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "check")]
    Check(MatchAnswersWithChoicesDisplayCheck),
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType>
    for MatchAnswerWithItemsDisplay
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType {
        match self {
            MatchAnswerWithItemsDisplay::Check(c) => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Check(
                    c.to_numbas(locale),
                )
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
            numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Check(c) => {
                MatchAnswerWithItemsDisplay::Check(c.to_rumbas())
            }
            numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Radio => {
                MatchAnswerWithItemsDisplay::Radio
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MatchAnswersWithChoicesDisplayCheckInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
pub struct MatchAnswersWithChoicesDisplayCheck {
    marking_method: MultipleChoiceMarkingMethod,
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck>
    for MatchAnswersWithChoicesDisplayCheck
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck {
        numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck {
            marking_method: self.marking_method.to_numbas(locale),
        }
    }
}

impl ToRumbas<MatchAnswersWithChoicesDisplayCheck>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck
{
    fn to_rumbas(&self) -> MatchAnswersWithChoicesDisplayCheck {
        MatchAnswersWithChoicesDisplayCheck {
            marking_method: self.marking_method.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceMatchAnswerDataInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum MultipleChoiceMatchAnswerData {
    #[serde(rename = "item_based")]
    ItemBased(MultipleChoiceMatchAnswers),
    #[serde(rename = "numbas_like")]
    NumbasLike(Box<MultipleChoiceMatchAnswerDataNumbasLike>),
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceMatchAnswerDataNumbasLikeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MultipleChoiceMatchAnswerDataNumbasLike {
    pub answers: VariableValued<Vec<ContentAreaTranslatableString>>,
    pub choices: VariableValued<Vec<ContentAreaTranslatableString>>,
    pub marks: VariableValued<Vec<Vec<JMEString>>>,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceMatchAnswersInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MultipleChoiceMatchAnswers {
    /// Values of the answers
    pub answers: Vec<ContentAreaTranslatableString>,
    /// Items for which the answer can be selected
    pub items: Vec<MatchAnswersItem>,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MatchAnswersItemInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MatchAnswersItem {
    pub statement: ContentAreaTranslatableString,
    /// Map points to strings of answers ! use anchors in yaml
    pub answer_marks: Vec<MatchAnswersItemMarks>,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MatchAnswersItemMarksInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MatchAnswersItemMarks {
    pub marks: JMEString,
    pub answer: ContentAreaTranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceWarningTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MultipleChoiceWarningType {
    None,
    Prevent,
}

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceWarningType>
    for MultipleChoiceWarningType
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceWarningType {
        match self {
            Self::None => numbas::question::part::match_answers::MultipleChoiceWarningType::None,
            Self::Prevent => {
                numbas::question::part::match_answers::MultipleChoiceWarningType::Prevent
            }
        }
    }
}

impl ToRumbas<MultipleChoiceWarningType>
    for numbas::question::part::match_answers::MultipleChoiceWarningType
{
    fn to_rumbas(&self) -> MultipleChoiceWarningType {
        match self {
            Self::None => MultipleChoiceWarningType::None,
            Self::Prevent => MultipleChoiceWarningType::Prevent,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MatchAnswersWithChoicesLayoutTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MatchAnswersWithChoicesLayoutType {
    All,
    LowerTriangle,
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType>
    for MatchAnswersWithChoicesLayoutType
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType {
        match self {
            Self::All => numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType::All,
            Self::LowerTriangle => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType::LowerTriangle
            }
        }
    }
}

impl ToRumbas<MatchAnswersWithChoicesLayoutType>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType
{
    fn to_rumbas(&self) -> MatchAnswersWithChoicesLayoutType {
        match self {
            Self::All => MatchAnswersWithChoicesLayoutType::All,
            Self::LowerTriangle => MatchAnswersWithChoicesLayoutType::LowerTriangle,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MatchAnswersWithChoicesLayoutInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MatchAnswersWithChoicesLayout {
    #[serde(rename = "type")]
    layout_type: MatchAnswersWithChoicesLayoutType, // Renamed because of bug in Comparable
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesLayout>
    for MatchAnswersWithChoicesLayout
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesLayout {
        numbas::question::part::match_answers::MatchAnswersWithChoicesLayout {
            r#type: self.layout_type.to_numbas(locale),
            expression: String::new(),
        }
    }
}

impl ToRumbas<MatchAnswersWithChoicesLayout>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesLayout
{
    fn to_rumbas(&self) -> MatchAnswersWithChoicesLayout {
        MatchAnswersWithChoicesLayout {
            layout_type: self.r#type.to_rumbas(),
        }
    }
}
