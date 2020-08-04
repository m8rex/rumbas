use std::path::PathBuf;

#[derive(Debug)]
pub struct YamlError {
    error: serde_yaml::Error,
    file: PathBuf,
}

impl YamlError {
    pub fn from(error: serde_yaml::Error, file: PathBuf) -> YamlError {
        YamlError { error, file }
    }
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.error.location() {
            write!(
                f,
                "Error in {} on column {} of line {}. The error message is {}",
                self.file.display(),
                location.column(),
                location.line(),
                self,
            )
        } else {
            write!(
                f,
                "Error in {}. The error message is {}",
                self.file.display(),
                self.error,
            )
        }
    }
}

pub type YamlResult<T> = Result<T, YamlError>;
