use crate::question::answer_simplification::AnswerSimplificationType;
use std::convert::TryInto;
use std::fmt::Display;
use std::str::FromStr;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

pub fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(serde_json::Value::String(s)) => T::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(serde_json::Value::Number(n)) => {
            let s = n.to_string();
            T::from_str(&s)
                .map_err(serde::de::Error::custom)
                .map(Option::from)
        }
        Ok(v) => Err(serde::de::Error::custom(format!(
            "string or number expected but found something else: {}",
            v
        ))),
        Err(_) => Ok(None),
    }
}

pub fn answer_simplification_deserialize_string<'de, D>(
    deserializer: D,
) -> Result<Vec<AnswerSimplificationType>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(serde_json::Value::String(s)) => {
            let mut r = Vec::new();
            for whole_item in s.split(',') {
                let new_item = whole_item.try_into();
                match new_item {
                    Ok(a) => r.push(a),
                    Err(m) => {
                        return Err(serde::de::Error::custom(format!(
                            "unknown answer simplification type {}",
                            m
                        )))
                    }
                }
            }
            Ok(r)
        }
        Ok(v) => Err(serde::de::Error::custom(format!(
            "string expected but found something else: {}",
            v
        ))),
        Err(_) => Err(serde::de::Error::custom("Invalid string expected for answer simplifcations".to_string())),
    }
}

pub fn answer_simplification_serialize_string<S>(
    values: &Vec<AnswerSimplificationType>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut parts: Vec<String> = Vec::new();
    for value in values {
        let new_item = value.to_string();
        parts.push(new_item);
    }
    s.serialize_str(&parts.join(",")[..])
}
