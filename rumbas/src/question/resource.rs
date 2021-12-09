use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

// TODO Optional overwrite
// TODO TranslatableString

// TODO: fix ovewrite?
#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "ResourcePathInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ResourcePath {
    pub resource_name: String,
    pub resource_path: PathBuf,
}

impl Examples for ResourcePathInput {
    fn examples() -> Vec<Self> {
        vec![] // TODO: create file somewhere?
    }
}

impl ToNumbas<numbas::question::resource::Resource> for ResourcePath {
    fn to_numbas(&self, _locale: &str) -> numbas::question::resource::Resource {
        numbas::question::resource::Resource([
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

impl ToRumbas<ResourcePath> for numbas::question::resource::Resource {
    fn to_rumbas(&self) -> ResourcePath {
        ResourcePath {
            resource_name: self.0[0].clone(),
            resource_path: std::path::Path::new(&self.0[1]).to_path_buf(),
        }
    }
}

impl std::convert::TryFrom<String> for ResourcePathInput {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let path = std::path::Path::new(crate::RESOURCES_FOLDER).join(&s);
        if path.exists() {
            Ok(ResourcePathInput {
                resource_name: Value::Normal(s),
                resource_path: Value::Normal(path),
            })
        } else {
            Err(format!("Missing resource {}", path.display()))
        }
    }
}

impl std::convert::From<ResourcePathInput> for String {
    fn from(q: ResourcePathInput) -> Self {
        q.resource_name.unwrap()
    }
}

impl std::convert::TryFrom<String> for ResourcePath {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let data: ResourcePathInput = s.try_into()?;
        Ok(data.to_normal())
    }
}

impl std::convert::From<ResourcePath> for String {
    fn from(q: ResourcePath) -> Self {
        q.resource_name
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

impl PartialEq for ResourcePathInput {
    fn eq(&self, other: &Self) -> bool {
        self.resource_name == other.resource_name
    }
}
impl Eq for ResourcePathInput {}

impl PartialEq for ResourcePathInputEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for ResourcePathInputEnum {}

impl ResourcePath {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn yaml() {
        let r = ResourcePath {
            resource_name: "test".to_string(),
            resource_path: Path::new("tmp").to_path_buf(),
        };
        assert_eq!(
            r.to_yaml().unwrap(),
            r"---
test
"
        );
        let rid = ResourcePathInputEnum(ResourcePathInput {
            resource_name: Value::Normal("test".to_string()),
            resource_path: Value::Normal(Path::new("tmp").to_path_buf()),
        });
        assert_eq!(r.to_yaml().unwrap(), serde_yaml::to_string(&rid).unwrap());
    }
}
