use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use serde_diff::{Apply, Diff, SerdeDiff};

//TODO use derive for Input & overwrite
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum VariableValued<T> {
    Variable(JMEString),
    Value(T),
}

impl<T: SerdeDiff + Serialize + DeserializeOwned> serde_diff::SerdeDiff for VariableValued<T> {
    fn diff<'a, S: serde_diff::_serde::ser::SerializeSeq>(
        &self,
        ctx: &mut serde_diff::DiffContext<'a, S>,
        other: &Self,
    ) -> Result<bool, S::Error> {
        let mut __changed__ = false;
        match (self, other) {
            (VariableValued::Variable(l0), VariableValued::Variable(r0)) => {
                ctx.push_variant("Variable");
                {
                    {
                        ctx.push_field_index(0u16);
                        __changed__ |= <JMEString as serde_diff::SerdeDiff>::diff(&l0, ctx, &r0)?;
                        ctx.pop_path_element()?;
                    }
                }
                ctx.pop_path_element()?;
            }
            (VariableValued::Value(l0), VariableValued::Value(r0)) => {
                ctx.push_variant("Value");
                {
                    {
                        ctx.push_field_index(0u16);
                        __changed__ |= <T as serde_diff::SerdeDiff>::diff(&l0, ctx, &r0)?;
                        ctx.pop_path_element()?;
                    }
                }
                ctx.pop_path_element()?;
            }
            (_, VariableValued::Variable(r0)) => {
                ctx.push_full_variant();
                ctx.save_value(other)?;
                ctx.pop_path_element()?;
            }
            (_, VariableValued::Value(r0)) => {
                ctx.push_full_variant();
                ctx.save_value(other)?;
                ctx.pop_path_element()?;
            }
        }
        Ok(__changed__)
    }
    fn apply<'de, A>(
        &mut self,
        seq: &mut A,
        ctx: &mut serde_diff::ApplyContext,
    ) -> Result<bool, <A as serde_diff::_serde::de::SeqAccess<'de>>::Error>
    where
        A: serde_diff::_serde::de::SeqAccess<'de>,
    {
        let mut __changed__ = false;
        match (self, ctx.next_path_element(seq)?) {
            (this, Some(serde_diff::DiffPathElementValue::FullEnumVariant)) => {
                ctx.read_value(seq, this)?;
                __changed__ = true;
            }
            (
                &mut VariableValued::Variable(ref mut l0),
                Some(serde_diff::DiffPathElementValue::EnumVariant(variant)),
            ) if variant == "Variable" => {
                while let Some(element) = ctx.next_path_element(seq)? {
                    match element {
                        serde_diff::DiffPathElementValue::FieldIndex(0u16) => {
                            __changed__ |=
                                <JMEString as serde_diff::SerdeDiff>::apply(l0, seq, ctx)?
                        }
                        _ => ctx.skip_value(seq)?,
                    }
                }
            }
            (
                &mut VariableValued::Value(ref mut l0),
                Some(serde_diff::DiffPathElementValue::EnumVariant(variant)),
            ) if variant == "Value" => {
                while let Some(element) = ctx.next_path_element(seq)? {
                    match element {
                        serde_diff::DiffPathElementValue::FieldIndex(0u16) => {
                            __changed__ |= <T as serde_diff::SerdeDiff>::apply(l0, seq, ctx)?
                        }
                        _ => ctx.skip_value(seq)?,
                    }
                }
            }
            _ => ctx.skip_value(seq)?,
        }
        Ok(__changed__)
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
    fn files_to_load(&self) -> Vec<FileToLoad> {
        match self {
            VariableValued::Variable(s) => s.files_to_load(),
            VariableValued::Value(v) => v.files_to_load(),
        }
    }
    fn insert_loaded_files(&mut self, files: &std::collections::HashMap<FileToLoad, LoadedFile>) {
        match *self {
            VariableValued::Variable(ref mut s) => s.insert_loaded_files(files),
            VariableValued::Value(ref mut v) => v.insert_loaded_files(files),
        };
    }
    fn dependencies(&self) -> std::collections::HashSet<std::path::PathBuf> {
        match self {
            VariableValued::Variable(s) => s.dependencies(),
            VariableValued::Value(v) => v.dependencies(),
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
