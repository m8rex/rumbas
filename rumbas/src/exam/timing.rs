use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Timing {
        duration_in_seconds: Noneable<usize>, // if "none" (or 0) -> unlimited time
        allow_pause: bool,
        /// Action to do on timeout
        on_timeout: TimeoutAction,
        /// Action to do five minutes before timeout
        timed_warning: TimeoutAction
    }
}

impl ToNumbas<numbas::exam::ExamTiming> for Timing {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamTiming {
        numbas::exam::ExamTiming {
            allow_pause: self.allow_pause.to_numbas(locale),
            timeout: self.on_timeout.to_numbas(locale),
            timed_warning: self.timed_warning.to_numbas(locale),
        }
    }
}

impl ToRumbas<Timing> for numbas::exam::Exam {
    fn to_rumbas(&self) -> Timing {
        Timing {
            duration_in_seconds: Value::Normal(
                self.basic_settings
                    .duration_in_seconds
                    .map(Noneable::NotNone)
                    .unwrap_or(Noneable::None),
            ),
            allow_pause: Value::Normal(self.timing.allow_pause),
            on_timeout: Value::Normal(self.timing.timeout.to_rumbas()),
            timed_warning: Value::Normal(self.timing.timed_warning.to_rumbas()),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum TimeoutAction {
    None,
    Warn { message: TranslatableString },
}
impl_optional_overwrite!(TimeoutAction);

impl ToNumbas<numbas::exam::ExamTimeoutAction> for TimeoutAction {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamTimeoutAction {
        match self {
            TimeoutAction::None => numbas::exam::ExamTimeoutAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            TimeoutAction::Warn { message } => numbas::exam::ExamTimeoutAction::Warn {
                message: message.to_string(locale).unwrap(),
            },
        }
    }
}

impl ToRumbas<TimeoutAction> for numbas::exam::ExamTimeoutAction {
    fn to_rumbas(&self) -> TimeoutAction {
        match self {
            numbas::exam::ExamTimeoutAction::None { message: _ } => TimeoutAction::None,
            numbas::exam::ExamTimeoutAction::Warn { message } => TimeoutAction::Warn {
                message: message.to_owned().into(),
            },
        }
    }
}
