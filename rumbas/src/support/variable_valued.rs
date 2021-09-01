use crate::support::optional_overwrite::*;
use crate::support::rumbas_check::{RumbasCheck, RumbasCheckResult};
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

impl<T: OptionalCheck> OptionalCheck for VariableValued<T> {
    fn find_missing(&self) -> OptionalCheckResult {
        match self {
            VariableValued::Variable(s) => s.find_missing(),
            VariableValued::Value(v) => v.find_missing(),
        }
    }
}

impl<T: Input> Input for VariableValued<T> {
    type Normal = VariableValued<<T as Input>::Normal>;
    fn to_normal(&self) -> <Self as Input>::Normal {
        self.clone().map(|a| a.to_normal())
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal.map(<T as Input>::from_normal)
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

impl<V, T: ToNumbas<V> + RumbasCheck> ToNumbas<numbas::support::primitive::VariableValued<V>>
    for VariableValued<T>
{
    fn to_numbas(&self, locale: &str) -> numbas::support::primitive::VariableValued<V> {
        match self {
            VariableValued::Variable(v) => {
                numbas::support::primitive::VariableValued::Variable(v.clone())
            }
            VariableValued::Value(v) => {
                numbas::support::primitive::VariableValued::Value(v.to_numbas(locale))
            }
        }
    }
}

impl<O, T: ToRumbas<O>> ToRumbas<VariableValued<O>>
    for numbas::support::primitive::VariableValued<T>
{
    fn to_rumbas(&self) -> VariableValued<O> {
        match self {
            numbas::support::primitive::VariableValued::Variable(v) => {
                VariableValued::Variable(v.clone())
            }
            numbas::support::primitive::VariableValued::Value(v) => {
                VariableValued::Value(v.to_rumbas())
            }
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