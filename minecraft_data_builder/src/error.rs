use std::fmt::Debug;
use thiserror::Error;
pub type GenResult< T> = Result<T, GenError>;

#[derive(Debug, Error)]
pub enum GenError {
    #[error("Git2 error: {0}")]
    Git2(git2::Error),
    #[error("Failed to parse error: {0}")]
    Serde(&'static str, serde_json::Error),
    #[error("Io error: {0}")]
    Io(std::io::Error),
}

impl From<serde_json::Error> for GenError {
    fn from(err: serde_json::Error) -> Self {
        GenError::Serde("", err)
    }
}
impl From<std::io::Error> for GenError {
    fn from(err: std::io::Error) -> Self {
        GenError::Io(err)
    }
}
