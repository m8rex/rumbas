use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::{file_reference::FileString, noneable::Noneable};
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NormalNavigationInput")]
#[derive(Deserialize, Serialize, Comparable, JsonSchema, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "mode")]
pub enum NormalNavigation {
    Sequential(SequentialNavigation),
    Menu(MenuNavigation),
}

impl ToNumbas<numbas::exam::navigation::Navigation> for NormalNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::Navigation {
        numbas::exam::navigation::Navigation {
            allow_regenerate: self.to_shared_data().can_regenerate.to_numbas(locale),
            allow_steps: self.to_shared_data().show_steps.to_numbas(locale),
            show_frontpage: self.to_shared_data().show_title_page.to_numbas(locale),
            confirm_when_leaving: self.to_shared_data().confirm_when_leaving.to_numbas(locale),
            start_password: self
                .to_shared_data()
                .start_password
                .to_numbas(locale)
                .unwrap_or_default(),
            navigation_mode: self.to_numbas(locale),
        }
    }
}

impl ToNumbas<numbas::exam::navigation::NavigationMode> for NormalNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::NavigationMode {
        match self {
            NormalNavigation::Menu(n) => n.to_numbas(locale),
            NormalNavigation::Sequential(n) => n.to_numbas(locale),
        }
    }
}

impl ToRumbas<NormalNavigation> for numbas::exam::Exam {
    fn to_rumbas(&self) -> NormalNavigation {
        match &self.navigation.navigation_mode {
            numbas::exam::navigation::NavigationMode::Sequential(s) => {
                NormalNavigation::Sequential(SequentialNavigation {
                    shared_data: self.to_rumbas(),
                    can_move_to_previous: s.can_move_to_previous.to_rumbas(),
                    browsing_enabled: s.browsing_enabled.to_rumbas(),
                    show_results_page: s.show_results_page.to_rumbas(),
                    on_leave: s.on_leave.to_rumbas(),
                })
            }
            numbas::exam::navigation::NavigationMode::Menu => {
                NormalNavigation::Menu(MenuNavigation {
                    shared_data: self.to_rumbas(),
                })
            }
            numbas::exam::navigation::NavigationMode::Diagnostic(_d) => {
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
            NormalNavigation::Menu(m) => m.shared_data.clone(),
            NormalNavigation::Sequential(m) => m.shared_data.clone(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "SequentialNavigationInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct SequentialNavigation {
    /// (flattened field) The data shared between all types of navigation
    #[serde(flatten)]
    pub shared_data: NavigationSharedData,
    /// Whether the student can move back to previous question
    /// Old name was `reverse`
    #[serde(alias = "reverse")]
    pub can_move_to_previous: bool,
    /// Whether the student can jump to any question.
    pub browsing_enabled: bool,
    /// When the results page should be shown
    pub show_results_page: ShowResultsPage,
    /// Action to execute when a student changes question or tries to end the exam.
    pub on_leave: LeaveAction,
}

impl ToNumbas<numbas::exam::navigation::NavigationMode> for SequentialNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::NavigationMode {
        numbas::exam::navigation::NavigationMode::Sequential(
            numbas::exam::navigation::NavigationModeSequential {
                on_leave: self.on_leave.to_numbas(locale),
                show_results_page: self.show_results_page.to_numbas(locale),
                can_move_to_previous: self.can_move_to_previous.to_numbas(locale),
                browsing_enabled: self.browsing_enabled.to_numbas(locale),
            },
        )
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MenuNavigationInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MenuNavigation {
    /// (flattened field) The data shared between all types of navigation
    #[serde(flatten)]
    pub shared_data: NavigationSharedData,
}

impl ToNumbas<numbas::exam::navigation::NavigationMode> for MenuNavigation {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::navigation::NavigationMode {
        numbas::exam::navigation::NavigationMode::Menu // TODO: sequential
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "DiagnosticNavigationInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct DiagnosticNavigation {
    /// (flattened field) The data shared between all types of navigation
    #[serde(flatten)]
    pub shared_data: NavigationSharedData,
    /// Action to execute when a student changes question or tries to end the exam.
    pub on_leave: LeaveAction,
}

impl ToNumbas<numbas::exam::navigation::Navigation> for DiagnosticNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::Navigation {
        numbas::exam::navigation::Navigation {
            allow_regenerate: self.shared_data.can_regenerate.to_numbas(locale),
            allow_steps: self.shared_data.show_steps.to_numbas(locale),
            show_frontpage: self.shared_data.show_title_page.to_numbas(locale),
            confirm_when_leaving: self.shared_data.confirm_when_leaving.to_numbas(locale),
            start_password: self
                .shared_data
                .start_password
                .to_numbas(locale)
                .unwrap_or_default(),
            navigation_mode: numbas::exam::navigation::NavigationMode::Diagnostic(
                numbas::exam::navigation::NavigationModeDiagnostic {
                    on_leave: self.on_leave.clone().to_numbas(locale),
                },
            ),
        }
    }
}

impl ToRumbas<DiagnosticNavigation> for numbas::exam::Exam {
    fn to_rumbas(&self) -> DiagnosticNavigation {
        match &self.navigation.navigation_mode {
            numbas::exam::navigation::NavigationMode::Sequential(_s) => {
                panic!(
                    "{}",
                    "Bug in rumbas: can' create diagnostic exam from normal one."
                )
            }
            numbas::exam::navigation::NavigationMode::Menu => {
                panic!(
                    "{}",
                    "Bug in rumbas: can' create diagnostic exam from normal one."
                )
            }
            numbas::exam::navigation::NavigationMode::Diagnostic(d) => DiagnosticNavigation {
                shared_data: self.to_rumbas(),
                on_leave: d.on_leave.to_rumbas(),
            },
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "ShowResultsPageInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowResultsPage {
    OnCompletion,
    Never,
}

impl ToNumbas<numbas::exam::navigation::ShowResultsPage> for ShowResultsPage {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::navigation::ShowResultsPage {
        match self {
            ShowResultsPage::OnCompletion => {
                numbas::exam::navigation::ShowResultsPage::OnCompletion
            }
            ShowResultsPage::Never => numbas::exam::navigation::ShowResultsPage::Never,
        }
    }
}

impl ToRumbas<ShowResultsPage> for numbas::exam::navigation::ShowResultsPage {
    fn to_rumbas(&self) -> ShowResultsPage {
        match self {
            numbas::exam::navigation::ShowResultsPage::Never => ShowResultsPage::Never,
            numbas::exam::navigation::ShowResultsPage::OnCompletion => {
                ShowResultsPage::OnCompletion
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "LeaveActionInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum LeaveAction {
    None,
    WarnIfNotAttempted(LeaveActionMessage),
    PreventIfNotAttempted(LeaveActionMessage),
}

impl ToNumbas<numbas::exam::navigation::LeaveAction> for LeaveAction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::LeaveAction {
        match self {
            LeaveAction::None => numbas::exam::navigation::LeaveAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            LeaveAction::WarnIfNotAttempted(m) => {
                numbas::exam::navigation::LeaveAction::WarnIfNotAttempted {
                    message: m.message.to_string(locale).unwrap(),
                }
            }
            LeaveAction::PreventIfNotAttempted(m) => {
                numbas::exam::navigation::LeaveAction::PreventIfNotAttempted {
                    message: m.message.to_string(locale).unwrap(),
                }
            }
        }
    }
}

impl ToRumbas<LeaveAction> for numbas::exam::navigation::LeaveAction {
    fn to_rumbas(&self) -> LeaveAction {
        match self {
            numbas::exam::navigation::LeaveAction::None { message: _ } => LeaveAction::None,
            numbas::exam::navigation::LeaveAction::WarnIfNotAttempted { message } => {
                LeaveAction::WarnIfNotAttempted(LeaveActionMessage {
                    message: message.clone().into(),
                })
            }
            numbas::exam::navigation::LeaveAction::PreventIfNotAttempted { message } => {
                LeaveAction::PreventIfNotAttempted(LeaveActionMessage {
                    message: message.clone().into(),
                })
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "LeaveActionMessageInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct LeaveActionMessage {
    pub message: TranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NavigationSharedDataInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct NavigationSharedData {
    /// Password to begin the exam
    /// none and "" are the same
    pub start_password: Noneable<FileString>,
    /// Whether the student can regenerate questions
    /// Old name was `allow_regenerate`
    #[serde(alias = "allow_regenerate")]
    pub can_regenerate: bool,
    /// If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions
    /// Old name was `allow_steps`
    #[serde(alias = "allow_steps")]
    pub show_steps: bool,
    /// Whether the title page should be shown.
    /// Old name was `show_frontpage`
    #[serde(alias = "show_frontpage")]
    pub show_title_page: bool,
    /// Whether the student will be asked to confirm when leaving the exam.
    #[serde(alias = "prevent_leaving")]
    pub confirm_when_leaving: bool,
    pub show_names_of_question_groups: bool,
    /// Whether the student is allowed to print the exam
    pub allow_printing: bool,
}

impl ToRumbas<NavigationSharedData> for numbas::exam::Exam {
    fn to_rumbas(&self) -> NavigationSharedData {
        NavigationSharedData {
            start_password: if self.navigation.start_password.is_empty() {
                None
            } else {
                Some(self.navigation.start_password.clone())
            }
            .to_rumbas(),
            can_regenerate: self.navigation.allow_regenerate.to_rumbas(),
            show_steps: self.navigation.allow_steps.to_rumbas(),

            show_title_page: self.navigation.show_frontpage.to_rumbas(),
            confirm_when_leaving: self.navigation.confirm_when_leaving.to_rumbas(),
            show_names_of_question_groups: self
                .basic_settings
                .show_question_group_names
                .to_rumbas(),
            allow_printing: self.basic_settings.allow_printing.to_rumbas(),
        }
    }
}
