use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
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

impl ToNumbas for Timing {
    type NumbasType = numbas::exam::ExamTiming;
    fn to_numbas(&self, locale: &str) -> NumbasResult<numbas::exam::ExamTiming> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::ExamTiming {
                allow_pause: self.allow_pause.unwrap(),
                timeout: self.on_timeout.clone().unwrap().to_numbas(locale).unwrap(),
                timed_warning: self
                    .timed_warning
                    .clone()
                    .unwrap()
                    .to_numbas(locale)
                    .unwrap(),
            })
        } else {
            Err(check)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum TimeoutAction {
    None,
    Warn { message: TranslatableString },
}
impl_optional_overwrite!(TimeoutAction);

impl ToNumbas for TimeoutAction {
    type NumbasType = numbas::exam::ExamTimeoutAction;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            TimeoutAction::None => numbas::exam::ExamTimeoutAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            TimeoutAction::Warn { message } => numbas::exam::ExamTimeoutAction::Warn {
                message: message.to_string(locale).unwrap(),
            },
        })
    }
}
