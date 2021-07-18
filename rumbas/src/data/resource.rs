use crate::data::optional_overwrite::{
    Noneable, OptionalOverwrite, RumbasCheck, RumbasCheckResult,
};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::ToRumbas;
use serde::{Deserialize, Serialize};

// TODO TranslatableString

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
        let path = std::path::Path::new("resources").join(&s);
        if path.exists() {
            Ok(ResourcePath {
                resource_name: s,
                resource_path: path.to_path_buf(),
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

impl ToNumbas for ResourcePath {
    type NumbasType = numbas::exam::Resource;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::Resource([
                self.resource_name.clone(),
                self.resource_path
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ]))
        } else {
            Err(check)
        }
    }
}

impl ToRumbas for numbas::exam::Resource {
    type RumbasType = ResourcePath;
    fn to_rumbas(&self) -> Self::RumbasType {
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
