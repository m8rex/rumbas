use crate::support::file_reference::FileString;
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite_enum! {
    #[serde(rename_all = "snake_case")]
    #[serde(tag = "mode")]
    pub enum NormalNavigation {
        Sequential(SequentialNavigation),
        Menu(MenuNavigation)
    }
}

impl ToNumbas<numbas::exam::ExamNavigation> for NormalNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamNavigation {
        numbas::exam::ExamNavigation {
            allow_regenerate: self.to_shared_data().can_regenerate.to_numbas(locale),
            allow_steps: Some(self.to_shared_data().show_steps.to_numbas(locale)),
            show_frontpage: self.to_shared_data().show_title_page.to_numbas(locale),
            confirm_when_leaving: Some(
                self.to_shared_data().confirm_when_leaving.to_numbas(locale),
            ),
            start_password: self
                .to_shared_data()
                .start_password
                .map(|s| s.to_numbas(locale)),
            navigation_mode: self.to_numbas(locale),
        }
    }
}

impl ToNumbas<numbas::exam::ExamNavigationMode> for NormalNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamNavigationMode {
        match self {
            NormalNavigation::Menu(n) => n.to_numbas(locale),
            NormalNavigation::Sequential(n) => n.to_numbas(locale),
        }
    }
}

impl ToRumbas<NormalNavigation> for numbas::exam::Exam {
    fn to_rumbas(&self) -> NormalNavigation {
        match &self.navigation.navigation_mode {
            numbas::exam::ExamNavigationMode::Sequential(s) => {
                NormalNavigation::Sequential(SequentialNavigation {
                    shared_data: self.to_rumbas(),
                    can_move_to_previous: s.can_move_to_previous.to_rumbas(),
                    browsing_enabled: s.browsing_enabled.to_rumbas(),
                    show_results_page: s.show_results_page.to_rumbas(),
                    on_leave: s.on_leave.to_rumbas(),
                })
            }
            numbas::exam::ExamNavigationMode::Menu => NormalNavigation::Menu(MenuNavigation {
                shared_data: self.to_rumbas(),
            }),
            numbas::exam::ExamNavigationMode::Diagnostic(_d) => {
                panic!(
                    "{}",
                    "Bug in rumbas: can' create normal exam from diagnostic one."
                )
            }
        }
    }
}

impl NormalNavigation {
    pub fn to_shared_data(&self) -> NavigationSharedData {
        match self {
            NormalNavigation::Menu(m) => m.shared_data.clone().unwrap(),
            NormalNavigation::Sequential(m) => m.shared_data.clone().unwrap(),
        }
    }
}

optional_overwrite! {
    pub struct SequentialNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData,
        /// Whether the student can move back to previous question
        /// Old name was `reverse`
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

impl ToNumbas<numbas::exam::ExamNavigationMode> for SequentialNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamNavigationMode {
        numbas::exam::ExamNavigationMode::Sequential(numbas::exam::ExamNavigationModeSequential {
            on_leave: self.on_leave.to_numbas(locale),
            show_results_page: self.show_results_page.to_numbas(locale),
            can_move_to_previous: self.can_move_to_previous.to_numbas(locale),
            browsing_enabled: self.browsing_enabled.to_numbas(locale),
        })
    }
}

optional_overwrite! {
    pub struct MenuNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData
    }
}

impl ToNumbas<numbas::exam::ExamNavigationMode> for MenuNavigation {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamNavigationMode {
        numbas::exam::ExamNavigationMode::Menu // TODO: sequential
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

impl ToNumbas<numbas::exam::ExamNavigation> for DiagnosticNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamNavigation {
        numbas::exam::ExamNavigation {
            allow_regenerate: self
                .shared_data
                .clone()
                .unwrap()
                .can_regenerate
                .to_numbas(locale),
            allow_steps: Some(
                self.shared_data
                    .clone()
                    .unwrap()
                    .show_steps
                    .to_numbas(locale),
            ),
            show_frontpage: self
                .shared_data
                .clone()
                .unwrap()
                .show_title_page
                .to_numbas(locale),
            confirm_when_leaving: Some(
                self.shared_data
                    .clone()
                    .unwrap()
                    .confirm_when_leaving
                    .to_numbas(locale),
            ),
            start_password: self
                .shared_data
                .clone()
                .unwrap()
                .start_password
                .map(|s| s.to_numbas(locale)),
            navigation_mode: numbas::exam::ExamNavigationMode::Diagnostic(
                numbas::exam::ExamNavigationModeDiagnostic {
                    on_leave: self.on_leave.clone().to_numbas(locale),
                },
            ),
        }
    }
}

impl ToRumbas<DiagnosticNavigation> for numbas::exam::Exam {
    fn to_rumbas(&self) -> DiagnosticNavigation {
        match &self.navigation.navigation_mode {
            numbas::exam::ExamNavigationMode::Sequential(_s) => {
                panic!(
                    "{}",
                    "Bug in rumbas: can' create diagnostic exam from normal one."
                )
            }
            numbas::exam::ExamNavigationMode::Menu => {
                panic!(
                    "{}",
                    "Bug in rumbas: can' create diagnostic exam from normal one."
                )
            }
            numbas::exam::ExamNavigationMode::Diagnostic(d) => DiagnosticNavigation {
                shared_data: self.to_rumbas(),
                on_leave: d.on_leave.to_rumbas(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShowResultsPage {
    OnCompletion,
    Never,
}
impl_optional_overwrite!(ShowResultsPage);

impl ToNumbas<numbas::exam::ExamShowResultsPage> for ShowResultsPage {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamShowResultsPage {
        match self {
            ShowResultsPage::OnCompletion => numbas::exam::ExamShowResultsPage::OnCompletion,
            ShowResultsPage::Never => numbas::exam::ExamShowResultsPage::Never,
        }
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum LeaveAction {
    None,
    WarnIfNotAttempted { message: TranslatableString },
    PreventIfNotAttempted { message: TranslatableString },
}
impl_optional_overwrite!(LeaveAction);

impl ToNumbas<numbas::exam::ExamLeaveAction> for LeaveAction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamLeaveAction {
        match self {
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
        }
    }
}

impl ToRumbas<LeaveAction> for numbas::exam::ExamLeaveAction {
    fn to_rumbas(&self) -> LeaveAction {
        match self {
            numbas::exam::ExamLeaveAction::None { message: _ } => LeaveAction::None,
            numbas::exam::ExamLeaveAction::WarnIfNotAttempted { message } => {
                LeaveAction::WarnIfNotAttempted {
                    message: message.clone().into(),
                }
            }
            numbas::exam::ExamLeaveAction::PreventIfNotAttempted { message } => {
                LeaveAction::PreventIfNotAttempted {
                    message: message.clone().into(),
                }
            }
        }
    }
}

optional_overwrite! {
    pub struct NavigationSharedData {
        /// Password to begin the exam
        start_password: FileString, //TODO: Noneable, but "" is none in this case?
        /// Whether the student can regenerate questions
        /// Old name was `allow_regenerate`
        #[serde(alias = "allow_regenerate")]
        can_regenerate: bool,
        /// If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions
        /// Old name was `allow_steps`
        #[serde(alias = "allow_steps")]
        show_steps: bool,
        /// Whether the title page should be shown.
        /// Old name was `show_frontpage`
        #[serde(alias = "show_frontpage")]
        show_title_page: bool,
        /// Whether the student will be asked to confirm when leaving the exam.
        #[serde(alias = "prevent_leaving")]
        confirm_when_leaving: bool,
        show_names_of_question_groups: bool,
        /// Whether the student is allowed to print the exam
        allow_printing: bool
    }
}

impl ToRumbas<NavigationSharedData> for numbas::exam::Exam {
    fn to_rumbas(&self) -> NavigationSharedData {
        NavigationSharedData {
            start_password: self
                .navigation
                .start_password
                .clone()
                .unwrap_or(DEFAULTS.navigation_start_password)
                .to_rumbas(),
            can_regenerate: self.navigation.allow_regenerate.to_rumbas(),
            show_steps: self
                .navigation
                .allow_steps
                .unwrap_or(DEFAULTS.navigation_allow_steps)
                .to_rumbas(),

            show_title_page: self.navigation.show_frontpage.to_rumbas(),
            confirm_when_leaving: self
                .navigation
                .confirm_when_leaving
                .unwrap_or(DEFAULTS.navigation_prevent_leaving)
                .to_rumbas(),
            show_names_of_question_groups: self
                .basic_settings
                .show_question_group_names
                .unwrap_or(DEFAULTS.navigation_show_names_of_question_groups)
                .to_rumbas(),
            allow_printing: self
                .basic_settings
                .allow_printing
                .unwrap_or(DEFAULTS.basic_settings_allow_printing)
                .to_rumbas(),
        }
    }
}
