use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::jme::JMEString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct VariablesTest {
        condition: JMEString,
        max_runs: usize
    }
}

impl ToNumbas<numbas::exam::ExamQuestionVariablesTest> for VariablesTest {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamQuestionVariablesTest {
        numbas::exam::ExamQuestionVariablesTest {
            condition: self.condition.to_numbas(locale),
            max_runs: self.max_runs.to_numbas(locale),
        }
    }
}

impl ToRumbas<VariablesTest> for numbas::exam::ExamQuestionVariablesTest {
    fn to_rumbas(&self) -> VariablesTest {
        VariablesTest {
            condition: Value::Normal(self.condition.clone()),
            max_runs: Value::Normal(self.max_runs.0),
        }
    }
}
