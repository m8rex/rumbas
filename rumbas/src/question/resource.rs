use crate::support::template::{Value, ValueType};
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

// TODO TranslatableString

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ResourcePath {
    pub resource_name: String,
    pub resource_path: std::path::PathBuf,
}
impl_optional_overwrite!(ResourcePath);

impl std::convert::TryFrom<String> for ResourcePath {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let path = std::path::Path::new(crate::RESOURCES_FOLDER).join(&s);
        if path.exists() {
            Ok(ResourcePath {
                resource_name: s,
                resource_path: path,
            })
        } else {
            Err(format!("Missing resource {}", path.display()))
        }
    }
}

impl std::convert::From<ResourcePath> for String {
    fn from(q: ResourcePath) -> Self {
        q.resource_name
    }
}

impl ToNumbas<numbas::exam::Resource> for ResourcePath {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::Resource {
        numbas::exam::Resource([
            self.resource_name.clone(),
            self.resource_path
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ])
    }
}

impl ToRumbas<ResourcePath> for numbas::exam::Resource {
    fn to_rumbas(&self) -> ResourcePath {
        ResourcePath {
            resource_name: self.0[0].clone(),
            resource_path: std::path::Path::new(&self.0[1]).to_path_buf(),
        }
    }
}

impl ResourcePath {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl Hash for ResourcePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resource_name.hash(state);
    }
}

impl PartialEq for ResourcePath {
    fn eq(&self, other: &Self) -> bool {
        self.resource_name == other.resource_name
    }
}
impl Eq for ResourcePath {}
