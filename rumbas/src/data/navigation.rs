use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    Navigation,
    allow_regenerate: bool,
    reverse: bool,
    browsing_enabled: bool,
    navigation_mode: ExamNavigationMode,
    allow_steps: bool,
    show_frontpage: bool,
    show_results_page: ShowResultsPage,
    prevent_leaving: bool,
    on_leave: LeaveAction,
    start_password: FileString, //TODO: Noneable, but "" is none in this case?
    show_names_of_question_groups: bool
}

impl ToNumbas for Navigation {
    type NumbasType = numbas::exam::ExamNavigation;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamNavigation> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamNavigation::new(
                self.allow_regenerate.unwrap(),
                Some(self.reverse.clone().unwrap()),
                Some(self.browsing_enabled.clone().unwrap()),
                self.navigation_mode
                    .clone()
                    .map(|s| s.to_numbas(&locale).unwrap()),
                Some(self.allow_steps.clone().unwrap()),
                self.show_frontpage.unwrap(),
                self.show_results_page
                    .clone()
                    .map(|s| s.to_numbas(&locale).unwrap()),
                Some(self.prevent_leaving.clone().unwrap()),
                self.on_leave.clone().map(|s| s.to_numbas(&locale).unwrap()),
                self.start_password.clone().map(|s| s.get_content(&locale)),
            ))
        } else {
            Err(empty_fields)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExamNavigationMode {
    Sequence,
    Menu,
}
impl_optional_overwrite!(ExamNavigationMode);

impl ToNumbas for ExamNavigationMode {
    type NumbasType = numbas::exam::ExamNavigationMode;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            ExamNavigationMode::Sequence => numbas::exam::ExamNavigationMode::Sequence,
            ExamNavigationMode::Menu => numbas::exam::ExamNavigationMode::Menu,
        })
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
    QuestionNavigation,
    allow_regenerate: bool,
    show_frontpage: bool,
    prevent_leaving: bool
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
