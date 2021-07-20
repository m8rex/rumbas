use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use crate::data::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
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
        show_names_of_question_groups: bool,
        /// Whether the student is allowed to print the exam
        allow_printing: bool
    }
}

impl ToRumbas<NavigationSharedData> for numbas::exam::Exam {
    fn to_rumbas(&self) -> NavigationSharedData {
        NavigationSharedData {
            start_password: Value::Normal(FileString::s(
                &self
                    .navigation
                    .start_password
                    .clone()
                    .unwrap_or(DEFAULTS.navigation_start_password),
            )),
            can_regenerate: Value::Normal(self.navigation.allow_regenerate),
            show_steps: Value::Normal(
                self.navigation
                    .allow_steps
                    .unwrap_or(DEFAULTS.navigation_allow_steps),
            ),
            show_title_page: Value::Normal(self.navigation.show_frontpage),
            prevent_leaving: Value::Normal(
                self.navigation
                    .prevent_leaving
                    .unwrap_or(DEFAULTS.navigation_prevent_leaving),
            ),
            show_names_of_question_groups: Value::Normal(
                self.basic_settings
                    .show_question_group_names
                    .unwrap_or(DEFAULTS.navigation_show_names_of_question_groups),
            ),
            allow_printing: Value::Normal(
                self.basic_settings
                    .allow_printing
                    .unwrap_or(DEFAULTS.basic_settings_allow_printing),
            ),
        }
    }
}

optional_overwrite_enum! {
    #[serde(rename_all = "snake_case")]
    #[serde(tag = "mode")]
    pub enum NormalNavigation {
        Sequential(SequentialNavigation),
        Menu(MenuNavigation)
    }
}

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
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamNavigation> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamNavigation {
                allow_regenerate: self.to_shared_data().can_regenerate.unwrap(),
                navigation_mode: self.to_navigation_mode(),
                reverse: self.can_move_to_previous(),
                browsing_enabled: Some(self.browsing_enabled()),
                allow_steps: Some(self.to_shared_data().show_steps.unwrap()),
                show_frontpage: self.to_shared_data().show_title_page.unwrap(),
                show_results_page: self
                    .show_results_page() // TODO
                    .map(|s| s.to_numbas(locale).unwrap()),
                prevent_leaving: Some(self.to_shared_data().prevent_leaving.unwrap()),
                on_leave: self.on_leave().map(|s| s.to_numbas(locale).unwrap()),
                start_password: self
                    .to_shared_data()
                    .start_password
                    .map(|s| s.get_content(locale)),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<NormalNavigation> for numbas::exam::Exam {
    fn to_rumbas(&self) -> NormalNavigation {
        match self.navigation.navigation_mode {
            numbas::exam::ExamNavigationMode::Sequence => {
                NormalNavigation::Sequential(SequentialNavigation {
                    shared_data: Value::Normal(self.to_rumbas()),
                    can_move_to_previous: Value::Normal(
                        self.navigation
                            .reverse
                            .unwrap_or(DEFAULTS.navigation_reverse),
                    ),
                    browsing_enabled: Value::Normal(
                        self.navigation
                            .browsing_enabled
                            .unwrap_or(DEFAULTS.navigation_browsing_enabled),
                    ),
                    show_results_page: Value::Normal(
                        self.navigation
                            .show_results_page
                            .clone()
                            .unwrap_or(DEFAULTS.navigation_show_results_page)
                            .to_rumbas(),
                    ),
                    on_leave: Value::Normal(
                        self.navigation
                            .on_leave
                            .clone()
                            .unwrap_or(DEFAULTS.navigation_on_leave)
                            .to_rumbas(),
                    ),
                })
            }
            numbas::exam::ExamNavigationMode::Menu => NormalNavigation::Menu(MenuNavigation {
                shared_data: Value::Normal(self.to_rumbas()),
            }),
            numbas::exam::ExamNavigationMode::Diagnostic => {
                panic!(
                    "{}",
                    "Bug in rumbas: can' create normal exam from diagnostic one."
                )
            }
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
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamNavigation> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamNavigation {
                allow_regenerate: self.shared_data.clone().unwrap().can_regenerate.unwrap(),
                navigation_mode: numbas::exam::ExamNavigationMode::Diagnostic,
                reverse: None,
                browsing_enabled: None,
                allow_steps: Some(self.shared_data.clone().unwrap().show_steps.unwrap()),
                show_frontpage: self.shared_data.clone().unwrap().show_title_page.unwrap(),
                show_results_page: None,
                prevent_leaving: Some(self.shared_data.clone().unwrap().prevent_leaving.unwrap()),
                on_leave: self.on_leave.clone().map(|s| s.to_numbas(locale).unwrap()),
                start_password: self
                    .shared_data
                    .clone()
                    .unwrap()
                    .start_password
                    .map(|s| s.get_content(locale)),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<DiagnosticNavigation> for numbas::exam::Exam {
    fn to_rumbas(&self) -> DiagnosticNavigation {
        DiagnosticNavigation {
            shared_data: Value::Normal(self.to_rumbas()),
            on_leave: Value::Normal(
                self.navigation
                    .on_leave
                    .clone()
                    .unwrap_or(DEFAULTS.navigation_on_leave)
                    .to_rumbas(),
            ),
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
    fn to_numbas(&self, _locale: &str) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            ShowResultsPage::OnCompletion => numbas::exam::ExamShowResultsPage::OnCompletion,
            ShowResultsPage::Never => numbas::exam::ExamShowResultsPage::Never,
        })
    }
}

impl ToRumbas<ShowResultsPage> for numbas::exam::ExamShowResultsPage {
    fn to_rumbas(&self) -> ShowResultsPage {
        match self {
            numbas::exam::ExamShowResultsPage::Never => ShowResultsPage::Never,
            numbas::exam::ExamShowResultsPage::OnCompletion => ShowResultsPage::OnCompletion,
        }
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
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            LeaveAction::None => numbas::exam::ExamLeaveAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            LeaveAction::WarnIfNotAttempted { message } => {
                numbas::exam::ExamLeaveAction::WarnIfNotAttempted {
                    message: message.to_string(locale).unwrap(),
                }
            }
            LeaveAction::PreventIfNotAttempted { message } => {
                numbas::exam::ExamLeaveAction::PreventIfNotAttempted {
                    message: message.to_string(locale).unwrap(),
                }
            }
        })
    }
}

impl ToRumbas<LeaveAction> for numbas::exam::ExamLeaveAction {
    fn to_rumbas(&self) -> LeaveAction {
        match self {
            numbas::exam::ExamLeaveAction::None { message: _ } => LeaveAction::None,
            numbas::exam::ExamLeaveAction::WarnIfNotAttempted { message } => {
                LeaveAction::WarnIfNotAttempted {
                    message: TranslatableString::s(&message),
                }
            }
            numbas::exam::ExamLeaveAction::PreventIfNotAttempted { message } => {
                LeaveAction::PreventIfNotAttempted {
                    message: TranslatableString::s(&message),
                }
            }
        }
    }
}

optional_overwrite! {
    pub struct QuestionNavigation {
        #[serde(alias = "allow_regenerate")]
        can_regenerate: bool,
        #[serde(alias = "show_frontpage")]
        show_title_page: bool,
        prevent_leaving: bool
    }
}

impl ToNumbas for QuestionNavigation {
    type NumbasType = numbas::exam::QuestionNavigation;
    fn to_numbas(&self, _locale: &str) -> NumbasResult<numbas::exam::QuestionNavigation> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::QuestionNavigation {
                allow_regenerate: self.can_regenerate.unwrap(),
                show_frontpage: self.show_title_page.unwrap(),
                prevent_leaving: Some(self.prevent_leaving.clone().unwrap()),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionNavigation> for numbas::exam::QuestionNavigation {
    fn to_rumbas(&self) -> QuestionNavigation {
        QuestionNavigation {
            can_regenerate: Value::Normal(self.allow_regenerate),
            show_title_page: Value::Normal(self.show_frontpage),
            prevent_leaving: Value::Normal(
                self.prevent_leaving
                    .unwrap_or(DEFAULTS.question_navigation_prevent_leaving),
            ),
        }
    }
}
