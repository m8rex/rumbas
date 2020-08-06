use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::Value;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    Timing,
    duration_in_seconds: Noneable<usize>, // if "none" (or 0) -> unlimited time
    allow_pause: bool,
    on_timeout: TimeoutAction,
    timed_warning: TimeoutAction
}

impl ToNumbas for Timing {
    type NumbasType = numbas::exam::ExamTiming;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::ExamTiming> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::ExamTiming::new(
                self.allow_pause.unwrap(),
                self.on_timeout.clone().unwrap().to_numbas(&locale).unwrap(),
                self.timed_warning
                    .clone()
                    .unwrap()
                    .to_numbas(&locale)
                    .unwrap(),
            ))
        } else {
            Err(empty_fields)
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
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        Ok(match self {
            TimeoutAction::None => numbas::exam::ExamTimeoutAction::None {
                message: "".to_string(), // message doesn't mean anything
            },
            TimeoutAction::Warn { message } => numbas::exam::ExamTimeoutAction::Warn {
                message: message.to_string(&locale).unwrap(),
            },
        })
    }
}
