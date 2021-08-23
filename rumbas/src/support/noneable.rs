use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq)]
pub enum Noneable<T> {
    None,
    NotNone(T),
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
