use crate::RC_FILE_NAME;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// "Run commands" that specify how this rumbas repo should be executed
pub struct RC {
    version: Version,
}

impl Default for RC {
    fn default() -> Self {
        Self {
            version: Version::new(0, 4, 0),
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
    let root_opt = find_root();
    if let Some(root) = root_opt {
        let f = std::fs::read_to_string(root.join(RC_FILE_NAME));
        if let Ok(f) = f {
            serde_yaml::from_str(&f)
        } else {
            Ok(Default::default())
        }
    } else {
        Ok(Default::default())
    }
}

/// Reads the [RC_FILE_NAME] file
/// Returns None if the file does not exist
/// Returns Some(val) where val is the parsing result if the file does exist
pub fn find_root() -> Option<PathBuf> {
    let start = Path::new(".").canonicalize().unwrap();

    let mut current = Some(start.as_path());

    while let Some(f) = current {
        let possible_file = f.join(RC_FILE_NAME);
        if possible_file.exists() {
            return Some(f.to_owned());
        }

        current = f.parent();
    }
    None
}
