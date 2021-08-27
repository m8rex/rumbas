use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::Value;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Timing {
        duration_in_seconds: NoneableNatural, // if "none" (or 0) -> unlimited time
        allow_pause: RumbasBool,
        /// Action to do on timeout
        on_timeout: TimeoutAction,
        /// Action to do five minutes before timeout
        timed_warning: TimeoutAction
    }
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

impl ToRumbas<Timing> for numbas::exam::exam::Exam {
    fn to_rumbas(&self) -> Timing {
        Timing {
            duration_in_seconds: self.basic_settings.duration_in_seconds.to_rumbas(),
            allow_pause: self.timing.allow_pause.to_rumbas(),
            on_timeout: self.timing.timeout.to_rumbas(),
            timed_warning: self.timing.timed_warning.to_rumbas(),
        }
    }
}

// TODO: optional_overwrite
#[derive(Debug, Clone)]
pub enum TimeoutAction {
    None,
    Warn(TimeoutActionWarn),
}

impl RumbasCheck for TimeoutAction {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            Self::None => RumbasCheckResult::empty(),
            Self::Warn(l) => l.check(locale),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum TimeoutActionInput {
    None,
    Warn(TimeoutActionWarnInput),
}

impl Input for TimeoutActionInput {
    type Normal = TimeoutAction;
    fn from_normal(normal: Self::Normal) -> Self {
        match normal {
            Self::Normal::None => Self::None,
            Self::Normal::Warn(c) => Self::Warn(Input::from_normal(c)),
        }
    }
    fn to_normal(&self) -> Self::Normal {
        match self {
            Self::None => Self::Normal::None,
            Self::Warn(c) => Self::Normal::Warn(c.to_normal()),
        }
    }
}

impl OptionalOverwrite<TimeoutActionInput> for TimeoutActionInput {
    fn overwrite(&mut self, other: &TimeoutActionInput) {
        match (self, other) {
            (&mut Self::Warn(ref mut val), Self::Warn(ref valo)) => val.overwrite(&valo),
            _ => (),
        };
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match self {
            &mut Self::Warn(ref mut enum_val) => enum_val.insert_template_value(&key, &val),
            _ => (),
        };
    }
}

impl OptionalCheck for TimeoutActionInput {
    fn find_missing(&self) -> OptionalCheckResult {
        match self {
            Self::None => OptionalCheckResult::empty(),
            Self::Warn(l) => l.find_missing(),
        }
    }
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

optional_overwrite! {
    pub struct TimeoutActionWarn {
        message: TranslatableString
    }
}
