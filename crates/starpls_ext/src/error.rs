use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StubError {
    #[error("Failed to read stub file '{path}': {source}")]
    FileReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse stub file '{path}': {source}")]
    ParseError {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("Stub file validation failed: {message}")]
    ValidationError { message: String },

    #[error("Unsupported stub file format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Type conversion error: {message}")]
    TypeConversionError { message: String },

    #[error("Multiple stub files define the same symbol: {symbol}")]
    DuplicateSymbol { symbol: String },

    #[error("Invalid file pattern: {pattern}")]
    InvalidPattern { pattern: String },
}

impl From<serde_json::Error> for StubError {
    fn from(err: serde_json::Error) -> Self {
        StubError::ParseError {
            path: PathBuf::new(), // Will be set by caller
            source: err,
        }
    }
}
