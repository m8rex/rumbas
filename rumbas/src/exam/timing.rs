use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "TimingInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct Timing {
    pub duration_in_seconds: Noneable<usize>, // if "none" (or 0) -> unlimited time
    pub allow_pause: bool,
    /// Action to do on timeout
    pub on_timeout: TimeoutAction,
    /// Action to do five minutes before timeout
    pub timed_warning: TimeoutAction,
}

impl ToNumbas<numbas::exam::timing::Timing> for Timing {
    fn to_numbas(&self, locale: &str) -> numbas::exam::timing::Timing {
        numbas::exam::timing::Timing {
            allow_pause: self.allow_pause.to_numbas(locale),
            timeout: self.on_timeout.to_numbas(locale),
            timed_warning: self.timed_warning.to_numbas(locale),
        }
    }
}

impl ToRumbas<Timing> for numbas::exam::Exam {
    fn to_rumbas(&self) -> Timing {
        Timing {
            duration_in_seconds: self.basic_settings.duration_in_seconds.to_rumbas(),
            allow_pause: self.timing.allow_pause.to_rumbas(),
            on_timeout: self.timing.timeout.to_rumbas(),
            timed_warning: self.timing.timed_warning.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "TimeoutActionInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum TimeoutAction {
    None,
    Warn(TimeoutActionWarn),
}

impl ToNumbas<numbas::exam::timing::TimeoutAction> for TimeoutAction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::timing::TimeoutAction {
        match self {
            TimeoutAction::None => numbas::exam::timing::TimeoutAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            TimeoutAction::Warn(wm) => numbas::exam::timing::TimeoutAction::Warn {
                message: wm.message.to_string(locale).unwrap(),
            },
        }
    }
}

impl ToRumbas<TimeoutAction> for numbas::exam::timing::TimeoutAction {
    fn to_rumbas(&self) -> TimeoutAction {
        match self {
            numbas::exam::timing::TimeoutAction::None { message: _ } => TimeoutAction::None,
            numbas::exam::timing::TimeoutAction::Warn { message } => {
                TimeoutAction::Warn(TimeoutActionWarn {
                    message: message.to_owned().into(),
                })
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "TimeoutActionWarnInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct TimeoutActionWarn {
    pub message: TranslatableString,
}
