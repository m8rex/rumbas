use crate::support::file_reference::FileString;
use crate::support::file_reference::FileStringInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::Value;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
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

impl ToNumbas<numbas::exam::navigation::Navigation> for NormalNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::Navigation {
        numbas::exam::navigation::Navigation {
            allow_regenerate: self.to_shared_data().can_regenerate.to_numbas(locale),
            allow_steps: Some(self.to_shared_data().show_steps.to_numbas(locale)),
            show_frontpage: self.to_shared_data().show_title_page.to_numbas(locale),
            confirm_when_leaving: Some(
                self.to_shared_data().confirm_when_leaving.to_numbas(locale),
            ),
            start_password: Some(self.to_shared_data().start_password.to_numbas(locale)),
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

impl ToRumbas<NormalNavigation> for numbas::exam::exam::Exam {
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

optional_overwrite! {
    pub struct SequentialNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData,
        /// Whether the student can move back to previous question
        /// Old name was `reverse`
        #[serde(alias = "reverse")]
        can_move_to_previous: RumbasBool,
        /// Whether the student can jump to any question.
        browsing_enabled: RumbasBool,
        /// When the results page should be shown
        show_results_page: ShowResultsPage,
        /// Action to execute when a student changes question or tries to end the exam.
        on_leave: LeaveAction
    }
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

optional_overwrite! {
    pub struct MenuNavigation {
        /// (flattened field) The data shared between all types of navigation
        #[serde(flatten)]
        shared_data: NavigationSharedData
    }
}

impl ToNumbas<numbas::exam::navigation::NavigationMode> for MenuNavigation {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::navigation::NavigationMode {
        numbas::exam::navigation::NavigationMode::Menu // TODO: sequential
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

impl ToNumbas<numbas::exam::navigation::Navigation> for DiagnosticNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::exam::navigation::Navigation {
        numbas::exam::navigation::Navigation {
            allow_regenerate: self.shared_data.can_regenerate.to_numbas(locale),
            allow_steps: Some(self.shared_data.show_steps.to_numbas(locale)),
            show_frontpage: self.shared_data.show_title_page.to_numbas(locale),
            confirm_when_leaving: Some(self.shared_data.confirm_when_leaving.to_numbas(locale)),
            start_password: Some(self.shared_data.start_password.to_numbas(locale)),
            navigation_mode: numbas::exam::navigation::NavigationMode::Diagnostic(
                numbas::exam::navigation::NavigationModeDiagnostic {
                    on_leave: self.on_leave.clone().to_numbas(locale),
                },
            ),
        }
    }
}

impl ToRumbas<DiagnosticNavigation> for numbas::exam::exam::Exam {
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShowResultsPage {
    OnCompletion,
    Never,
}
impl_optional_overwrite!(ShowResultsPage);

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

#[derive(Debug, Clone)]
pub enum LeaveAction {
    None,
    WarnIfNotAttempted(LeaveActionMessage),
    PreventIfNotAttempted(LeaveActionMessage),
}

impl RumbasCheck for LeaveAction {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            Self::None => RumbasCheckResult::empty(),
            Self::WarnIfNotAttempted(l) => l.check(locale),
            Self::PreventIfNotAttempted(l) => l.check(locale),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum LeaveActionInput {
    None,
    WarnIfNotAttempted(LeaveActionMessageInput),
    PreventIfNotAttempted(LeaveActionMessageInput),
}

impl OptionalCheck for LeaveActionInput {
    fn find_missing(&self) -> OptionalCheckResult {
        match self {
            Self::None => OptionalCheckResult::empty(),
            Self::WarnIfNotAttempted(l) => l.find_missing(),
            Self::PreventIfNotAttempted(l) => l.find_missing(),
        }
    }
}

impl Input for LeaveActionInput {
    type Normal = LeaveAction;
    fn to_normal(&self) -> Self::Normal {
        match self {
            Self::None => Self::Normal::None,
            Self::WarnIfNotAttempted(t) => Self::Normal::WarnIfNotAttempted(t.to_normal()),
            Self::PreventIfNotAttempted(t) => Self::Normal::PreventIfNotAttempted(t.to_normal()),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        match normal {
            Self::Normal::None => Self::None,
            Self::Normal::WarnIfNotAttempted(t) => Self::WarnIfNotAttempted(Input::from_normal(t)),
            Self::Normal::PreventIfNotAttempted(t) => {
                Self::PreventIfNotAttempted(Input::from_normal(t))
            }
        }
    }
}

impl OptionalOverwrite<LeaveActionInput> for LeaveActionInput {
    fn overwrite(&mut self, other: &LeaveActionInput) {
        match (self, other) {
            (&mut Self::WarnIfNotAttempted(ref mut val), Self::WarnIfNotAttempted(ref valo)) => {
                val.overwrite(&valo)
            }
            (
                &mut Self::PreventIfNotAttempted(ref mut val),
                Self::PreventIfNotAttempted(ref valo),
            ) => val.overwrite(&valo),
            _ => (),
        };
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match self {
            &mut Self::WarnIfNotAttempted(ref mut enum_val) => {
                enum_val.insert_template_value(&key, &val)
            }
            &mut Self::PreventIfNotAttempted(ref mut enum_val) => {
                enum_val.insert_template_value(&key, &val)
            }
            _ => (),
        };
    }
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

optional_overwrite! {
    pub struct LeaveActionMessage {
        message: TranslatableString
    }
}

optional_overwrite! {
    pub struct NavigationSharedData {
        /// Password to begin the exam
        start_password: FileString, //TODO: Noneable, but "" is none in this case?
        /// Whether the student can regenerate questions
        /// Old name was `allow_regenerate`
        #[serde(alias = "allow_regenerate")]
        can_regenerate: RumbasBool,
        /// If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions
        /// Old name was `allow_steps`
        #[serde(alias = "allow_steps")]
        show_steps: RumbasBool,
        /// Whether the title page should be shown.
        /// Old name was `show_frontpage`
        #[serde(alias = "show_frontpage")]
        show_title_page: RumbasBool,
        /// Whether the student will be asked to confirm when leaving the exam.
        #[serde(alias = "prevent_leaving")]
        confirm_when_leaving: RumbasBool,
        show_names_of_question_groups: RumbasBool,
        /// Whether the student is allowed to print the exam
        allow_printing: RumbasBool
    }
}

impl ToRumbas<NavigationSharedData> for numbas::exam::exam::Exam {
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
