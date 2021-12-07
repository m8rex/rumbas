use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use rumbas_support::preamble::*;
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq)]
pub enum Noneable<T> {
    None,
    NotNone(T),
}

impl<T: Examples> Examples for Noneable<T> {
    fn examples() -> Vec<Self> {
        T::examples()
            .into_iter()
            .map(|e| Self::NotNone(e))
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
            item.insert_template_value(&key, &val);
        }
    }
}

impl<T: Overwrite<T>> Overwrite<Noneable<T>> for Noneable<T> {
    fn overwrite(&mut self, other: &Noneable<T>) {
        if let Noneable::NotNone(ref mut val) = self {
            if let Noneable::NotNone(other_val) = &other {
                val.overwrite(&other_val);
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
