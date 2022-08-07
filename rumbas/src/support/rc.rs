use crate::RC_FILE_NAME;
use serde::{Deserialize, Serialize};
use semver::{Version};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// "Run commands" that specify how this rumbas repo should be executed
pub struct RC {
    version: Version,
}

impl Default for RC {
    fn default() -> Self {
        Self {
            version: Version::new(0,4,0)
        }
    }
}

impl RC {
    pub fn with_version(&self, version: Version) -> RC {
        let mut rc = self.clone();
        rc.version = version;
        rc
    }

    pub fn write(&self) -> std::io::Result<()> {
        let s = serde_yaml::to_string(self).expect("Failed converting RC to yaml file");
        std::fs::write(RC_FILE_NAME, s)
    }

    pub fn version(&self) -> Version {
        self.version.clone()
    }
}

/// Reads the [RC_FILE_NAME] file
/// Returns None if the file does not exist
/// Returns Some(val) where val is the parsing result if the file does exist
pub fn read() -> Result<RC, serde_yaml::Error> {
    let f = std::fs::read_to_string(RC_FILE_NAME);
    if let Ok(f) = f {
        serde_yaml::from_str(&f)
    } else {
        Ok(Default::default())
    }
}
