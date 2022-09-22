use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Noneable<T> {
    None,
    NotNone(T),
}

#[derive(Debug, PartialEq)]
pub enum NoneableDesc<T: Comparable + PartialEq + std::fmt::Debug> {
    None,
    NotNone(<T as comparable::Comparable>::Desc),
}
#[derive(Debug, PartialEq)]
pub enum NoneableChange<T: Comparable + PartialEq + std::fmt::Debug> {
    BothNotNone(<T as comparable::Comparable>::Change),
    Different(
        <Noneable<T> as comparable::Comparable>::Desc,
        <Noneable<T> as comparable::Comparable>::Desc,
    ),
}
impl<T: Comparable + PartialEq + std::fmt::Debug> comparable::Comparable for Noneable<T> {
    type Desc = NoneableDesc<T>;
    fn describe(&self) -> Self::Desc {
        match self {
            Noneable::None => NoneableDesc::None,
            Noneable::NotNone(var0) => NoneableDesc::NotNone(var0.describe()),
        }
    }
    type Change = NoneableChange<T>;
    fn comparison(&self, other: &Self) -> comparable::Changed<Self::Change> {
        match (self, other) {
            (Noneable::None, Noneable::None) => comparable::Changed::Unchanged,
            (Noneable::NotNone(self_var0), Noneable::NotNone(other_var0)) => {
                let changes_var0 = self_var0.comparison(other_var0);
                changes_var0.map(NoneableChange::BothNotNone)
            }
            (_, _) => comparable::Changed::Changed(NoneableChange::Different(
                self.describe(),
                other.describe(),
            )),
        }
    }
}

impl<T: Examples> Examples for Noneable<T> {
    fn examples() -> Vec<Self> {
        T::examples()
            .into_iter()
            .map(Self::NotNone)
            .chain(vec![Noneable::None].into_iter())
            .collect()
    }
}

impl<T: InputInverse> InputInverse for Noneable<T> {
    type Input = Noneable<<T as InputInverse>::Input>;
    type EnumInput = Self::Input;
}

impl<T: Input> Input for Noneable<T> {
    type Normal = Noneable<<T as Input>::Normal>;
    fn to_normal(&self) -> Noneable<<T as Input>::Normal> {
        self.clone().map(|a| a.to_normal())
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal.map(<T as Input>::from_normal)
    }
    fn find_missing(&self) -> InputCheckResult {
        match self {
            Noneable::NotNone(val) => val.find_missing(),
            _ => InputCheckResult::empty(),
        }
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        if let Noneable::NotNone(item) = self {
            item.insert_template_value(key, val);
        }
    }
    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        match self {
            Noneable::NotNone(val) => val.files_to_load(main_file_path),
            _ => Vec::new(),
        }
    }
    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &std::collections::HashMap<FileToLoad, LoadedFile>,
    ) {
        if let Noneable::NotNone(ref mut item) = self {
            item.insert_loaded_files(main_file_path, files);
        }
    }
    fn dependencies(
        &self,
        main_file_path: &RumbasPath,
    ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
        match self {
            Noneable::NotNone(val) => val.dependencies(main_file_path),
            _ => std::collections::HashSet::new(),
        }
    }
}

impl<T: Overwrite<T>> Overwrite<Noneable<T>> for Noneable<T> {
    fn overwrite(&mut self, other: &Noneable<T>) {
        if let Noneable::NotNone(ref mut val) = self {
            if let Noneable::NotNone(other_val) = &other {
                val.overwrite(other_val);
            }
        } else {
            // Do nothing, none is a valid value
        }
    }
}

impl<T: RumbasCheck> RumbasCheck for Noneable<T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            Noneable::NotNone(val) => val.check(locale),
            _ => RumbasCheckResult::empty(),
        }
    }
}

impl<S, T: ToNumbas<S> + RumbasCheck> ToNumbas<Option<S>> for Noneable<T> {
    fn to_numbas(&self, locale: &str) -> Option<S> {
        match self {
            Noneable::NotNone(val) => Some(val.clone().to_numbas(locale)),
            _ => None,
        }
    }
    fn to_numbas_with_name(&self, locale: &str, name: String) -> Option<S> {
        match self {
            Noneable::NotNone(val) => Some(val.clone().to_numbas_with_name(locale, name)),
            _ => None,
        }
    }
}

impl<T, O: ToRumbas<T>> ToRumbas<Noneable<T>> for Option<O> {
    fn to_rumbas(&self) -> Noneable<T> {
        self.clone()
            .map(|item| item.to_rumbas())
            .map_or(Noneable::None, Noneable::NotNone)
    }
}

impl<T: JsonSchema> JsonSchema for Noneable<T> {
    fn schema_name() -> String {
        format!("Noneable_{}", T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let none_schema = schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            enum_values: Some(vec![serde_json::json!("none")]),
            ..Default::default()
        };
        schemars::schema::SchemaObject {
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                any_of: Some(vec![none_schema.into(), gen.subschema_for::<T>()]),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl<T: std::clone::Clone> Noneable<T> {
    #[inline]
    pub fn unwrap_or(&self, other: T) -> T {
        match self {
            Noneable::None => other,
            Noneable::NotNone(nn) => nn.clone(),
        }
    }
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Noneable::NotNone(x) => x,
            Noneable::None => f(),
        }
    }
}

impl<T: std::default::Default> Noneable<T> {
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        match self {
            Noneable::None => T::default(),
            Noneable::NotNone(x) => x,
        }
    }
}

impl<T> Noneable<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Noneable<U> {
        match self {
            Noneable::None => Noneable::None,
            Noneable::NotNone(x) => Noneable::NotNone(f(x)),
        }
    }
}

impl<T> std::convert::From<Noneable<T>> for Option<T> {
    fn from(n: Noneable<T>) -> Self {
        match n {
            Noneable::None => Self::None,
            Noneable::NotNone(n) => Self::Some(n),
        }
    }
}

mod serde_noneable {
    use super::Noneable;
    use serde::Deserialize;
    use serde::Serialize;
    impl<T> Serialize for Noneable<T>
    where
        T: serde::Serialize,
    {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Noneable::None => s.serialize_str("none"),
                Noneable::NotNone(v) => v.serialize(s),
            }
        }
    }

    impl<'de, T> Deserialize<'de> for Noneable<T>
    where
        T: serde::Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Noneable<T>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let deser_res: Result<NoneableDeserialize<T>, _> =
                serde::Deserialize::deserialize(deserializer);
            deser_res.map(|res| match res {
                NoneableDeserialize::None(_v) => Noneable::None,
                NoneableDeserialize::NotNone(v) => Noneable::NotNone(v),
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    enum NoneEnum {
        #[serde(rename = "none")]
        None,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    #[serde(untagged)]
    enum NoneableDeserialize<T> {
        None(NoneEnum),
        NotNone(T),
    }
}
