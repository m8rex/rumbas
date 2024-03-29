use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RumbasPath {
    root_path: PathBuf,
    project_path: PathBuf,
    absolute_path: PathBuf,
}

impl Hash for RumbasPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.absolute_path.hash(state);
    }
}

impl PartialEq for RumbasPath {
    fn eq(&self, other: &Self) -> bool {
        self.absolute_path == other.absolute_path
    }
}
impl Eq for RumbasPath {}

impl AsRef<Path> for RumbasPath {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.absolute_path.as_ref()
    }
}

impl RumbasPath {
    pub fn is_file(&self) -> bool {
        self.absolute_path.is_file()
    }
    pub fn is_dir(&self) -> bool {
        self.absolute_path.is_dir()
    }
    pub fn in_main_folder(&self, s: &str) -> bool {
        self.project_path.starts_with(s)
    }
    pub fn display(&self) -> std::path::Display {
        self.project_path.display()
    }
    pub fn root(&self) -> &Path {
        self.root_path.as_path()
    }
    pub fn project(&self) -> &Path {
        self.project_path.as_path()
    }
    pub fn absolute(&self) -> &Path {
        self.absolute_path.as_path()
    }
    pub fn extension(&self) -> Option<&std::ffi::OsStr> {
        self.project_path.extension()
    }
    pub fn parent(&self) -> Option<RumbasPath> {
        if let Some(abs) = self.absolute_path.parent() {
            if let Ok(project) = abs.strip_prefix(&self.root_path) {
                Some(RumbasPath {
                    absolute_path: abs.to_path_buf(),
                    project_path: project.to_path_buf(),
                    root_path: self.root_path.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn keep_root(&self, p: &Path) -> Self {
        let absolute_path = self.root_path.join(p);
        Self {
            absolute_path,
            root_path: self.root_path.clone(),
            project_path: p.to_path_buf(),
        }
    }
    pub fn in_root(&self, p: &Path) -> Option<Self> {
        Self::create(p, self.root_path.as_path())
    }
    pub fn create(path: &Path, root: &Path) -> Option<Self> {
        let absolute = canonicalize_keep_symlink(path);
        log::debug!("Stripping {:?} from {:?}", root, absolute);
        absolute.strip_prefix(root).ok().map(|r| RumbasPath {
            root_path: root.to_path_buf(),
            project_path: r.to_path_buf(),
            absolute_path: absolute.to_path_buf(),
        })
    }
}

fn canonicalize_keep_symlink(path: &Path) -> PathBuf {
    if let Some(p) = path.parent() {
        if p != Path::new("") {
            return canonicalize_keep_symlink(p).join(path.file_name().unwrap());
        }
    }
    if path.is_relative() {
        if let Some(file_name) = path.file_name() {
            Path::new(".").canonicalize().unwrap().join(file_name)
        } else {
            Path::new(".").canonicalize().unwrap()
        }
    } else {
        path.canonicalize().unwrap()
    }
}

impl RumbasPath {
    pub fn test_make(path: &Path, root: &Path) -> Self {
        RumbasPath {
            root_path: root.to_path_buf(),
            project_path: path.to_path_buf(),
            absolute_path: root.join(path),
        }
    }
}
