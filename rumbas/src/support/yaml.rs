use rumbas_support::path::RumbasPath;
use serde::de::DeserializeOwned;
use yaml_subset::yaml::{parse_yaml_file, YamlError as YamlSubsetError};

pub type YamlResult<T> = Result<T, YamlError>;

#[derive(Debug)]
pub struct YamlError {
    error: YamlErrorKind,
    file: RumbasPath,
}

#[derive(Debug)]
pub enum YamlErrorKind {
    SubsetError(YamlSubsetError),
    YamlError(serde_yaml::Error),
}

impl YamlError {
    pub fn from(error: YamlErrorKind, file: RumbasPath) -> YamlError {
        YamlError { error, file }
    }
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error {
            YamlErrorKind::SubsetError(ref e) => {
                let (line, column) = e.location();
                write!(
                    f,
                    "Error in {} on column {} of line {}. The error message is:\n{}",
                    self.file.display(),
                    column,
                    line,
                    e,
                )
            }
            YamlErrorKind::YamlError(ref e) => {
                if let Some(location) = e.location() {
                    write!(
                        f,
                        "Error in {} on column {} of line {}. The error message is {}",
                        self.file.display(),
                        location.column(),
                        location.line(),
                        e,
                    )
                } else {
                    write!(
                        f,
                        "Error in {}. The error message is {}",
                        self.file.display(),
                        e,
                    )
                }
            }
        }
    }
}

pub fn parse_yaml<T>(s: &str, file_path: RumbasPath) -> YamlResult<T>
where
    T: DeserializeOwned,
{
    let _test = parse_yaml_file(s)
        .map_err(|e| YamlError::from(YamlErrorKind::SubsetError(e), file_path.clone()))?;

    serde_yaml::from_str(s).map_err(|e| YamlError::from(YamlErrorKind::YamlError(e), file_path))
}
