use crate::support::noneable::Noneable;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_check::{RumbasCheck, RumbasCheckResult};
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::jme::{ContentAreaString, EmbracedJMEString, JMEString};
use schemars::JsonSchema;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(JMEString),
    Value(T),
}
impl_optional_overwrite!(JMEString);
impl_optional_overwrite!(EmbracedJMEString);
impl_optional_overwrite!(ContentAreaString);

impl<T: RumbasCheck> RumbasCheck for VariableValued<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            VariableValued::Variable(s) => s.check(locale),
            VariableValued::Value(v) => v.check(locale),
        }
    }
}
impl<T: OptionalOverwrite<T> + DeserializeOwned> OptionalOverwrite<VariableValued<T>>
    for VariableValued<T>
{
    fn overwrite(&mut self, other: &VariableValued<T>) {
        match (self, other) {
            (&mut VariableValued::Variable(ref mut val), &VariableValued::Variable(ref valo)) => {
                val.overwrite(valo)
            }
            (&mut VariableValued::Value(ref mut val), &VariableValued::Value(ref valo)) => {
                val.overwrite(valo)
            }
            _ => (),
        };
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match *self {
            VariableValued::Variable(ref mut s) => s.insert_template_value(key, val),
            VariableValued::Value(ref mut v) => v.insert_template_value(key, val),
        };
    }
}
impl_optional_overwrite_value!(VariableValued<T>[T]);

impl<V, T: ToNumbas<V> + RumbasCheck> ToNumbas<numbas::exam::VariableValued<V>>
    for VariableValued<T>
{
    fn to_numbas(&self, locale: &str) -> numbas::exam::VariableValued<V> {
        match self {
            VariableValued::Variable(v) => numbas::exam::VariableValued::Variable(v.clone()),
            VariableValued::Value(v) => numbas::exam::VariableValued::Value(v.to_numbas(locale)),
        }
    }
}

impl<T> VariableValued<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> VariableValued<U> {
        match self {
            VariableValued::Variable(x) => VariableValued::Variable(x),
            VariableValued::Value(x) => VariableValued::Value(f(x)),
        }
    }
}

impl<O, T: ToRumbas<O>> ToRumbas<VariableValued<O>> for numbas::exam::VariableValued<T> {
    fn to_rumbas(&self) -> VariableValued<O> {
        match self {
            numbas::exam::VariableValued::Variable(v) => VariableValued::Variable(v.clone()),
            numbas::exam::VariableValued::Value(v) => VariableValued::Value(v.to_rumbas()),
        }
    }
}
