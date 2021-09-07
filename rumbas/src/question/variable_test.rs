use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::jme::JMEString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type JMEStringInput = numbas::jme::JMEString;

optional_overwrite! {
    pub struct VariablesTest {
        condition: JMEString,
        max_runs: RumbasNatural
    }
}

impl ToNumbas<numbas::question::question::QuestionVariablesTest> for VariablesTest {
    fn to_numbas(&self, locale: &str) -> numbas::question::question::QuestionVariablesTest {
        numbas::question::question::QuestionVariablesTest {
            condition: self.condition.to_numbas(locale),
            max_runs: self.max_runs.to_numbas(locale),
        }
    }
}

impl ToRumbas<VariablesTest> for numbas::question::question::QuestionVariablesTest {
    fn to_rumbas(&self) -> VariablesTest {
        VariablesTest {
            condition: self.condition.to_rumbas(),
            max_runs: self.max_runs.0.to_rumbas(),
        }
    }
}
