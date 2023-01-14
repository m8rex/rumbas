use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use comparable::Comparable;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use structdoc::StructDoc;

//TODO use derive for Input & overwrite
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, Eq)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(JMEString),
    Value(T),
}

// Done like the Option value in StructDoc
impl<T: StructDoc> StructDoc for VariableValued<T> {
    fn document() -> structdoc::Documentation {
        T::document().with_arity(structdoc::Arity::OrVariableValued)
    }
}

#[derive(PartialEq, Debug)]
pub enum VariableValuedDesc<T: Comparable + PartialEq + std::fmt::Debug> {
    Variable(<JMEString as comparable::Comparable>::Desc),
    Value(<T as comparable::Comparable>::Desc),
}

#[derive(PartialEq, Debug)]
pub enum VariableValuedChange<T: Comparable + PartialEq + std::fmt::Debug> {
    BothVariable(<JMEString as comparable::Comparable>::Change),
    BothValue(<T as comparable::Comparable>::Change),
    Different(
        <VariableValued<T> as comparable::Comparable>::Desc,
        <VariableValued<T> as comparable::Comparable>::Desc,
    ),
}
impl<T: Comparable + PartialEq + std::fmt::Debug> comparable::Comparable for VariableValued<T> {
    type Desc = VariableValuedDesc<T>;
    fn describe(&self) -> Self::Desc {
        match self {
            VariableValued::Variable(var0) => VariableValuedDesc::Variable(var0.describe()),
            VariableValued::Value(var0) => VariableValuedDesc::Value(var0.describe()),
        }
    }
    type Change = VariableValuedChange<T>;
    fn comparison(&self, other: &Self) -> comparable::Changed<Self::Change> {
        match (self, other) {
            (VariableValued::Variable(self_var0), VariableValued::Variable(other_var0)) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(VariableValuedChange::BothVariable)
            }
            (VariableValued::Value(self_var0), VariableValued::Value(other_var0)) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(VariableValuedChange::BothValue)
            }
            (_, _) => comparable::Changed::Changed(VariableValuedChange::Different(
                self.describe(),
                other.describe(),
            )),
        }
    }
}

impl<T: Examples> Examples for VariableValued<T> {
    fn examples() -> Vec<Self> {
        T::examples()
            .into_iter()
            .map(Self::Value)
            .chain(JMEString::examples().into_iter().map(Self::Variable))
            .collect()
    }
}

impl<T: RumbasCheck> RumbasCheck for VariableValued<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            VariableValued::Variable(s) => s.check(locale),
            VariableValued::Value(v) => v.check(locale),
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
    fn find_missing(&self) -> InputCheckResult {
        match self {
            VariableValued::Variable(s) => s.find_missing(),
            VariableValued::Value(v) => v.find_missing(),
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match *self {
            VariableValued::Variable(ref mut s) => s.insert_template_value(key, val),
            VariableValued::Value(ref mut v) => v.insert_template_value(key, val),
        };
    }
    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        match self {
            VariableValued::Variable(s) => s.files_to_load(main_file_path),
            VariableValued::Value(v) => v.files_to_load(main_file_path),
        }
    }
    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &std::collections::HashMap<FileToLoad, LoadedFile>,
    ) {
        match *self {
            VariableValued::Variable(ref mut s) => s.insert_loaded_files(main_file_path, files),
            VariableValued::Value(ref mut v) => v.insert_loaded_files(main_file_path, files),
        };
    }
    fn dependencies(
        &self,
        main_file_path: &RumbasPath,
    ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
        match self {
            VariableValued::Variable(s) => s.dependencies(main_file_path),
            VariableValued::Value(v) => v.dependencies(main_file_path),
        }
    }
}

impl<T: InputInverse> InputInverse for VariableValued<T> {
    type Input = VariableValued<<T as InputInverse>::Input>;
    type EnumInput = Self::Input;
}

impl<T: Overwrite<T> + DeserializeOwned> Overwrite<VariableValued<T>> for VariableValued<T> {
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
}

impl<V, T: ToNumbas<V> + RumbasCheck> ToNumbas<numbas::support::primitive::VariableValued<V>>
    for VariableValued<T>
{
    type ToNumbasHelper = T::ToNumbasHelper;
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> numbas::support::primitive::VariableValued<V> {
        match self {
            VariableValued::Variable(v) => {
                numbas::support::primitive::VariableValued::Variable(v.clone())
            }
            VariableValued::Value(v) => {
                numbas::support::primitive::VariableValued::Value(v.to_numbas(locale, data))
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

//TODO use derive for Input & overwrite
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema)]
#[serde(untagged)]
pub enum ReverseVariableValued<T> {
    Value(T), // Different ordering
    Variable(JMEString),
}

// Done like the Option value in StructDoc
impl<T: StructDoc> StructDoc for ReverseVariableValued<T> {
    fn document() -> structdoc::Documentation {
        T::document().with_arity(structdoc::Arity::OrVariableValued)
    }
}

impl<T: Comparable + PartialEq + std::fmt::Debug> comparable::Comparable
    for ReverseVariableValued<T>
{
    type Desc = VariableValuedDesc<T>;
    fn describe(&self) -> Self::Desc {
        match self {
            ReverseVariableValued::Variable(var0) => VariableValuedDesc::Variable(var0.describe()),
            ReverseVariableValued::Value(var0) => VariableValuedDesc::Value(var0.describe()),
        }
    }
    type Change = VariableValuedChange<T>;
    fn comparison(&self, other: &Self) -> comparable::Changed<Self::Change> {
        match (self, other) {
            (
                ReverseVariableValued::Variable(self_var0),
                ReverseVariableValued::Variable(other_var0),
            ) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(VariableValuedChange::BothVariable)
            }
            (ReverseVariableValued::Value(self_var0), ReverseVariableValued::Value(other_var0)) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(VariableValuedChange::BothValue)
            }
            (_, _) => comparable::Changed::Changed(VariableValuedChange::Different(
                self.describe(),
                other.describe(),
            )),
        }
    }
}

impl<T: Examples> Examples for ReverseVariableValued<T> {
    fn examples() -> Vec<Self> {
        T::examples()
            .into_iter()
            .map(Self::Value)
            .chain(JMEString::examples().into_iter().map(Self::Variable))
            .collect()
    }
}

impl<T: RumbasCheck> RumbasCheck for ReverseVariableValued<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            Self::Variable(s) => s.check(locale),
            Self::Value(v) => v.check(locale),
        }
    }
}

impl<T: Input> Input for ReverseVariableValued<T> {
    type Normal = ReverseVariableValued<<T as Input>::Normal>;
    fn to_normal(&self) -> <Self as Input>::Normal {
        self.clone().map(|a| a.to_normal())
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal.map(<T as Input>::from_normal)
    }
    fn find_missing(&self) -> InputCheckResult {
        match self {
            Self::Variable(s) => s.find_missing(),
            Self::Value(v) => v.find_missing(),
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match *self {
            Self::Variable(ref mut s) => s.insert_template_value(key, val),
            Self::Value(ref mut v) => v.insert_template_value(key, val),
        };
    }
    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        match self {
            Self::Variable(s) => s.files_to_load(main_file_path),
            Self::Value(v) => v.files_to_load(main_file_path),
        }
    }
    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &std::collections::HashMap<FileToLoad, LoadedFile>,
    ) {
        match *self {
            Self::Variable(ref mut s) => s.insert_loaded_files(main_file_path, files),
            Self::Value(ref mut v) => v.insert_loaded_files(main_file_path, files),
        };
    }
    fn dependencies(
        &self,
        main_file_path: &RumbasPath,
    ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
        match self {
            Self::Variable(s) => s.dependencies(main_file_path),
            Self::Value(v) => v.dependencies(main_file_path),
        }
    }
}

impl<T: InputInverse> InputInverse for ReverseVariableValued<T> {
    type Input = ReverseVariableValued<<T as InputInverse>::Input>;
    type EnumInput = Self::Input;
}

impl<T: Overwrite<T> + DeserializeOwned> Overwrite<ReverseVariableValued<T>>
    for ReverseVariableValued<T>
{
    fn overwrite(&mut self, other: &ReverseVariableValued<T>) {
        match (self, other) {
            (
                &mut ReverseVariableValued::Variable(ref mut val),
                &ReverseVariableValued::Variable(ref valo),
            ) => val.overwrite(valo),
            (
                &mut ReverseVariableValued::Value(ref mut val),
                &ReverseVariableValued::Value(ref valo),
            ) => val.overwrite(valo),
            _ => (),
        };
    }
}

impl<V, T: ToNumbas<V> + RumbasCheck> ToNumbas<numbas::support::primitive::VariableValued<V>>
    for ReverseVariableValued<T>
{
    type ToNumbasHelper = T::ToNumbasHelper;
    fn to_numbas(&self, locale: &str, data: &Self::ToNumbasHelper) -> numbas::support::primitive::VariableValued<V> {
        match self {
            Self::Variable(v) => numbas::support::primitive::VariableValued::Variable(v.clone()),
            Self::Value(v) => {
                numbas::support::primitive::VariableValued::Value(v.to_numbas(locale, data))
            }
        }
    }
}

impl<O, T: ToRumbas<O>> ToRumbas<ReverseVariableValued<O>>
    for numbas::support::primitive::VariableValued<T>
{
    fn to_rumbas(&self) -> ReverseVariableValued<O> {
        match self {
            numbas::support::primitive::VariableValued::Variable(v) => {
                ReverseVariableValued::Variable(v.clone())
            }
            numbas::support::primitive::VariableValued::Value(v) => {
                ReverseVariableValued::Value(v.to_rumbas())
            }
        }
    }
}

impl<T> ReverseVariableValued<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ReverseVariableValued<U> {
        match self {
            ReverseVariableValued::Variable(x) => ReverseVariableValued::Variable(x),
            ReverseVariableValued::Value(x) => ReverseVariableValued::Value(f(x)),
        }
    }
}
