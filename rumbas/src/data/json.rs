use std::path::PathBuf;

#[derive(Debug)]
pub struct JsonError {
    error: serde_json::error::Error,
    file: PathBuf,
}

impl JsonError {
    pub fn from(error: serde_json::error::Error, file: PathBuf) -> JsonError {
        JsonError { error, file }
    }
}
impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error in {} on column {} of line {}. The type of the error is {:?}. The error message is {}",
            self.file.display(),
            self.error.column(),
            self.error.line(),
            self.error.classify(),
            self.error,
        ) // Better explanation: Eof -> end of file, Data: wrong datatype or missing field, Syntax: syntax error
    }
}
pub type JsonResult<T> = Result<T, JsonError>;
