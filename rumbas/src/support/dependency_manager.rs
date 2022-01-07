use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::RwLock;

lazy_static! {
    pub static ref DEPENDENCIES: DependencyManager = DependencyManager::default();
}

#[derive(Debug)]
pub struct DependencyManager {
    depended_on_by: RwLock<HashMap<PathBuf, Mutex<HashSet<PathBuf>>>>,
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self {
            depended_on_by: RwLock::new(HashMap::new()),
        }
    }
}

impl DependencyManager {
    pub fn add_dependencies(&self, path: PathBuf, dependencies: HashSet<PathBuf>) {
        for dependency in dependencies.iter().chain(vec![path.clone()].iter()) {
            log::debug!("Reading depended on by map.");
            let map = self
                .depended_on_by
                .read()
                .expect("Can read depended_on_by map");
            log::debug!(
                "Checking if {} is in the depended on by map.",
                dependency.display()
            );

            if let Some(val) = map.get(dependency) {
                log::debug!("Found {} in the depended_on_by_map.", dependency.display());
                val.lock()
                    .expect("unlock loaded depended_on_by mutex")
                    .insert(path.clone());
            } else {
                std::mem::drop(map);
                let mut map = self
                    .depended_on_by
                    .write()
                    .expect("Can write depended_on_by map");

                map.insert(
                    dependency.clone(),
                    Mutex::new(vec![path.clone()].into_iter().collect()),
                );
            }
        }
    }

    pub fn get_dependants(&self, path: PathBuf) -> HashSet<PathBuf> {
        log::debug!("Reading depended on by map.");
        let map = self
            .depended_on_by
            .read()
            .expect("Can read depended_on_by map");
        log::debug!(
            "Checking if {} is in the depended on by map.",
            path.display()
        );

        if let Some(val) = map.get(&path) {
            log::debug!("Found {} in the depended_on_by_map.", path.display());
            val.lock()
                .expect("unlock loaded depended_on_by mutex")
                .clone()
        } else {
            log::debug!("Didn't find {} in the depended_on_by_map.", path.display());
            HashSet::new()
        }
    }

    pub fn log_debug(&self) {
        let map = self
            .depended_on_by
            .read()
            .expect("Can read depended_on_by map");
        log::debug!("{:?}", map);
    }
}
