use crate::support::rumbas_types::*;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const TEMPLATE_PREFIX: &str = "template";

optional_overwrite! {
    pub struct TemplateFile {
        #[serde(rename = "template")]
        relative_template_path: RumbasString,
        #[serde(flatten)]
        data: TemplateData
    }
}

type TemplateData = HashMap<String, MyYamlValue>;
type TemplateDataInput = HashMap<String, MyYamlValue>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyYamlValue(pub serde_yaml::Value);

impl Overwrite<MyYamlValue> for MyYamlValue {
    fn overwrite(&mut self, _other: &Self) {}
}

impl RumbasCheck for MyYamlValue {
    fn check(&self, _locale: &str) -> RumbasCheckResult {
        RumbasCheckResult::empty()
    }
}

impl Input for MyYamlValue {
    type Normal = MyYamlValue;
    fn to_normal(&self) -> Self::Normal {
        self.to_owned()
    }
    fn from_normal(normal: Self::Normal) -> Self {
        normal
    }
    fn find_missing(&self) -> InputCheckResult {
        InputCheckResult::empty()
    }
    fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
}

impl JsonSchema for MyYamlValue {
    fn schema_name() -> String {
        "YamlValue".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Bool(true)
    }
}

impl std::convert::From<serde_yaml::Value> for MyYamlValue {
    fn from(v: serde_yaml::Value) -> Self {
        Self(v)
    }
}
