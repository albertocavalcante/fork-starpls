use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtensionError {
    #[error("Failed to read extension file '{path}': {source}")]
    FileReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse extension file '{path}': {source}")]
    ParseError {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("Extension validation failed: {message}")]
    ValidationError { message: String },

    #[error("Unsupported extension format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Type conversion error: {message}")]
    TypeConversionError { message: String },

    #[error("Multiple extensions define the same symbol: {symbol}")]
    DuplicateSymbol { symbol: String },

    #[error("Invalid file pattern: {pattern}")]
    InvalidPattern { pattern: String },
}

impl From<serde_json::Error> for ExtensionError {
    fn from(err: serde_json::Error) -> Self {
        ExtensionError::ParseError {
            path: PathBuf::new(), // Will be set by caller
            source: err,
        }
    }
}
