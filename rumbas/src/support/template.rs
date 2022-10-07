use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub const TEMPLATE_PREFIX: &str = "template";

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
pub struct TemplateFile {
    #[serde(rename = "template")]
    pub relative_template_path: String,
    #[serde(flatten)]
    #[comparable_ignore]
    pub data: BTreeMap<String, MyYamlValue>,
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
pub struct TemplateFileInput {
    #[serde(rename = "template")]
    pub relative_template_path: ValueType<String>,
    #[serde(flatten)]
    #[comparable_ignore]
    pub data: BTreeMap<String, MyYamlValue>,
}

impl TemplateFileInput {
    pub fn template_key(&self) -> Option<String> {
        self.relative_template_path.template_key()
    }
    pub fn has_unknown_parent(&self) -> bool {
        self.template_key().is_some()
    }
    pub fn set_template(&mut self, other: &TemplateFile) {
        if let Some(key) = self.template_key() {
            if let Some(value) = other.data.get(&key) {
                self.relative_template_path
                    .insert_template_value(&key, &(value.0))
            }
        }
    }
}

pub type TemplateFileInputEnum = TemplateFileInput;

impl Overwrite<TemplateFileInput> for TemplateFileInput {
    fn overwrite(&mut self, other: &Self) {}
}

impl RumbasCheck for TemplateFile {
    fn check(&self, _locale: &str) -> RumbasCheckResult {
        RumbasCheckResult::empty()
    }
}

impl Input for TemplateFileInput {
    type Normal = TemplateFile;
    fn to_normal(&self) -> Self::Normal {
        TemplateFile {
            relative_template_path: self.relative_template_path.clone().unwrap(),
            data: self.data.clone(),
        }
    }
    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            relative_template_path: ValueType::Normal(normal.relative_template_path.clone()),
            data: normal.data.clone(),
        }
    }
    fn find_missing(&self) -> InputCheckResult {
        InputCheckResult::empty()
    }
    fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {}
    fn files_to_load(&self, _main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        vec![]
    }
    fn insert_loaded_files(
        &mut self,
        _main_file_path: &RumbasPath,
        _files: &HashMap<FileToLoad, LoadedFile>,
    ) {
    }
    fn dependencies(
        &self,
        _main_file_path: &RumbasPath,
    ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
        std::collections::HashSet::new()
    }
}

impl InputInverse for TemplateFile {
    type Input = TemplateFileInput;
    type EnumInput = TemplateFileInputEnum;
}

impl Examples for TemplateFileInput {
    fn examples() -> Vec<Self> {
        vec![Self {
            relative_template_path: ValueType::Normal("templatefile".to_string()),
            data: vec![(
                "key".to_string(),
                MyYamlValue(serde_yaml::Value::String("value".to_string())),
            )]
            .into_iter()
            .collect(),
        }]
    }
}

impl std::hash::Hash for TemplateFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.relative_template_path.hash(state);
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

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
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
    fn files_to_load(&self, _main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        vec![]
    }
    fn insert_loaded_files(
        &mut self,
        _main_file_path: &RumbasPath,
        _files: &HashMap<FileToLoad, LoadedFile>,
    ) {
    }
    fn dependencies(
        &self,
        _main_file_path: &RumbasPath,
    ) -> std::collections::HashSet<rumbas_support::path::RumbasPath> {
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
