use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct NavigationSharedData {
        /// Password to begin the exam
        start_password: FileString, //TODO: Noneable, but "" is none in this case?
        /// Whether the student can regenerate questions (alias of `can_regenerate`)
        #[serde(alias = "allow_regenerate")]
        can_regenerate: bool,
        /// If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions
        #[serde(alias = "allow_steps")]
        show_steps: bool,
        /// Whether the title page should be shown.
        #[serde(alias = "show_frontpage")]
        show_title_page: bool,
        /// Whether the student will be asked to confirm when leaving the exam.
        prevent_leaving: bool,
        show_names_of_question_groups: bool
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "mode")]
pub enum NormalNavigation {
    Sequential(SequentialNavigation),
    Menu(MenuNavigation),
}
impl_optional_overwrite!(NormalNavigation);

impl NormalNavigation {
    pub fn to_shared_data(&self) -> NavigationSharedData {
        match self {
            NormalNavigation::Menu(m) => m.shared_data.clone().unwrap(),
            NormalNavigation::Sequential(m) => m.shared_data.clone().unwrap(),
        }
    }
    pub fn can_move_to_previous(&self) -> Option<bool> {
        match self {
            NormalNavigation::Menu(_m) => Some(false),
            NormalNavigation::Sequential(m) => Some(m.can_move_to_previous.clone().unwrap()),
        }
    }
    pub fn to_navigation_mode(&self) -> numbas::exam::ExamNavigationMode {
        match self {
            NormalNavigation::Menu(_m) => numbas::exam::ExamNavigationMode::Menu,
            NormalNavigation::Sequential(_m) => numbas::exam::ExamNavigationMode::Sequence,
        }
    }
    pub fn browsing_enabled(&self) -> bool {
        match self {
            NormalNavigation::Menu(_m) => false,
            NormalNavigation::Sequential(m) => m.browsing_enabled.clone().unwrap(),
        }
    }
    pub fn show_results_page(&self) -> Option<ShowResultsPage> {
        match self {
            NormalNavigation::Menu(_m) => None,
            NormalNavigation::Sequential(m) => Some(m.show_results_page.clone().unwrap()),
        }
    }
    pub fn on_leave(&self) -> Option<LeaveAction> {
        match self {
            NormalNavigation::Menu(_m) => None,
            NormalNavigation::Sequential(m) => Some(m.on_leave.clone().unwrap()),
        }
    }
}

optional_overwrite! {
    pub struct SequentialNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData,
        /// Whether the student can move back to previous question (alias of `reverse`)
        #[serde(alias = "reverse")]
        can_move_to_previous: bool,
        /// Whether the student can jump to any question.
        browsing_enabled: bool,
        /// When the results page should be shown
        show_results_page: ShowResultsPage,
        /// Action to execute when a student changes question or tries to end the exam.
        on_leave: LeaveAction
    }
}

optional_overwrite! {
    pub struct MenuNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData
    }
}

impl ToNumbas for NormalNavigation {
    type NumbasType = numbas::exam::ExamNavigation;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamNavigation::new(
                self.to_shared_data().can_regenerate.unwrap(),
                self.to_navigation_mode(),
                self.can_move_to_previous(),
                Some(self.browsing_enabled()),
                Some(self.to_shared_data().show_steps.clone().unwrap()),
                self.to_shared_data().show_title_page.clone().unwrap(),
                self.show_results_page() // TODO
                    .clone()
                    .map(|s| s.to_numbas(&locale).unwrap()),
                Some(self.to_shared_data().prevent_leaving.clone().unwrap()),
                self.on_leave()
                    .clone()
                    .map(|s| s.to_numbas(&locale).unwrap()),
                self.to_shared_data()
                    .start_password
                    .clone()
                    .map(|s| s.get_content(&locale)),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

optional_overwrite! {
    pub struct DiagnosticNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData,
        /// Action to execute when a student changes question or tries to end the exam.
        on_leave: LeaveAction
    }
}

impl ToNumbas for DiagnosticNavigation {
    type NumbasType = numbas::exam::ExamNavigation;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamNavigation::new(
                self.shared_data.clone().unwrap().can_regenerate.unwrap(),
                numbas::exam::ExamNavigationMode::Diagnostic,
                None,
                None,
                Some(self.shared_data.clone().unwrap().show_steps.unwrap()),
                self.shared_data
                    .clone()
                    .unwrap()
                    .show_title_page
                    .clone()
                    .unwrap(),
                None,
                Some(
                    self.shared_data
                        .clone()
                        .unwrap()
                        .prevent_leaving
                        .clone()
                        .unwrap(),
                ),
                self.on_leave.clone().map(|s| s.to_numbas(&locale).unwrap()),
                self.shared_data
                    .clone()
                    .unwrap()
                    .start_password
                    .clone()
                    .map(|s| s.get_content(&locale)),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShowResultsPage {
    OnCompletion,
    Never,
}
impl_optional_overwrite!(ShowResultsPage);

impl ToNumbas for ShowResultsPage {
    type NumbasType = numbas::exam::ExamShowResultsPage;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            ShowResultsPage::OnCompletion => numbas::exam::ExamShowResultsPage::OnCompletion,
            ShowResultsPage::Never => numbas::exam::ExamShowResultsPage::Never,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum LeaveAction {
    None,
    WarnIfNotAttempted { message: TranslatableString },
    PreventIfNotAttempted { message: TranslatableString },
}
impl_optional_overwrite!(LeaveAction);

impl ToNumbas for LeaveAction {
    type NumbasType = numbas::exam::ExamLeaveAction;
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            LeaveAction::None => numbas::exam::ExamLeaveAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            LeaveAction::WarnIfNotAttempted { message } => {
                numbas::exam::ExamLeaveAction::WarnIfNotAttempted {
                    message: message.to_string(&locale).unwrap(),
                }
            }
            LeaveAction::PreventIfNotAttempted { message } => {
                numbas::exam::ExamLeaveAction::PreventIfNotAttempted {
                    message: message.to_string(&locale).unwrap(),
                }
            }
        })
    }
}

optional_overwrite! {
    pub struct QuestionNavigation {
        allow_regenerate: bool,
        show_frontpage: bool,
        prevent_leaving: bool
    }
}

impl ToNumbas for QuestionNavigation {
    type NumbasType = numbas::exam::QuestionNavigation;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::QuestionNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::QuestionNavigation::new(
                self.allow_regenerate.unwrap(),
                self.show_frontpage.unwrap(),
                Some(self.prevent_leaving.clone().unwrap()),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
