use crate::RC_FILE_NAME;
use rumbas_support::path::RumbasPath;
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
    pub fn from_path(r: &RumbasPath) -> Result<RC, serde_yaml::Error> {
        read(r.root())
    }
}

/// Reads the [RC_FILE_NAME] file
/// Returns None if the file does not exist
/// Returns Some(val) where val is the parsing result if the file does exist
pub fn read(p: &Path) -> Result<RC, serde_yaml::Error> {
    let root_opt = find_root(p);
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
pub fn find_root(p: &Path) -> Option<PathBuf> {
    log::debug!("Looking for root for {:?}", p);
    let start = p.canonicalize().unwrap();

    let mut current = Some(start.as_path());

    while let Some(f) = current {
        let possible_file = f.join(RC_FILE_NAME);
        if possible_file.exists() {
            log::debug!("Found root for {:?}", f);
            return Some(f.to_owned());
        }

        current = f.parent();
    }
    None
}

/// Find the relative path within the rumbas repo
pub fn within_repo(path: &Path) -> Option<RumbasPath> {
    find_root(&path)
        .map(|root| RumbasPath::create(path, root.as_path()))
        .flatten()
}
