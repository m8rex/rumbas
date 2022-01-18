use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};
use std::collections::HashMap;

pub const TEMPLATE_PREFIX: &str = "template";

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "TemplateFileInput")]
#[input(no_examples)]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema)]
pub struct TemplateFile {
    #[serde(rename = "template")]
    pub relative_template_path: String,
    #[serde(flatten)]
    pub data: HashMap<String, MyYamlValue>,
}

impl Examples for TemplateFileInput {
    fn examples() -> Vec<Self> {
        vec![Self {
            relative_template_path: Value::Normal("templatefile".to_string()),
            data: vec![(
                "key".to_string(),
                ValueType::Normal(MyYamlValue(serde_yaml::Value::String("value".to_string()))),
            )]
            .into_iter()
            .collect(),
        }]
    }
}

impl Examples for TemplateFileInputEnum {
    fn examples() -> Vec<Self> {
        TemplateFileInput::examples()
            .into_iter()
            .map(Self)
            .collect()
    }
}

impl PartialEq for TemplateFile {
    fn eq(&self, other: &Self) -> bool {
        self.relative_template_path == other.relative_template_path
    }
}
impl Eq for TemplateFile {}

impl PartialEq for TemplateFileInput {
    fn eq(&self, other: &Self) -> bool {
        self.relative_template_path == other.relative_template_path
    }
}
impl Eq for TemplateFileInput {}

impl PartialEq for TemplateFileInputEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for TemplateFileInputEnum {}

#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone)]
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
    fn files_to_load(&self) -> Vec<FileToLoad> {
        vec![]
    }
    fn insert_loaded_files(&mut self, _files: &HashMap<FileToLoad, LoadedFile>) {}
    fn dependencies(&self) -> std::collections::HashSet<std::path::PathBuf> {
        std::collections::HashSet::new()
    }
}

impl InputInverse for MyYamlValue {
    type Input = MyYamlValue;
    type EnumInput = Self::Input;
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
