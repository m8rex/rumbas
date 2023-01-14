use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use comparable::Comparable;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "VariablesTestInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct VariablesTest {
    /// A JME expression which should evaluate to true when the set of variables generated has the properties you want. For example, if a, b and c are the coefficients of a quadratic equation and you want it to have real roots, the condition could be b^2-4*a*c>=0.
    pub condition: JMEString,
    /// The maximum number of times the system should regenerate the set of variables without finding a set which satisfies the condition before giving up. If the system exceeds this number in a compiled exam, the entire exam will fail, so try to avoid it!
    pub max_runs: usize,
}

impl ToNumbas<numbas::question::QuestionVariablesTest> for VariablesTest {
    type ToNumbasHelper = ();
    fn to_numbas(&self, locale: &str, _data: &Self::ToNumbasHelper) -> numbas::question::QuestionVariablesTest {
        numbas::question::QuestionVariablesTest {
            condition: self.condition.to_numbas(locale, &()),
            max_runs: self.max_runs.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<VariablesTest> for numbas::question::QuestionVariablesTest {
    fn to_rumbas(&self) -> VariablesTest {
        VariablesTest {
            condition: self.condition.to_rumbas(),
            max_runs: self.max_runs.0.to_rumbas(),
        }
    }
}
