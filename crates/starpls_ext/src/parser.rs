use std::path::Path;

use anyhow::Result;

use crate::error::StubError;
use crate::types::StubDefinition;

/// Parse a JSON extension file and return its definition.
pub fn parse_stub_file(path: &Path) -> Result<StubDefinition, StubError> {
    let content = std::fs::read_to_string(path).map_err(|e| StubError::FileReadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    if extension != "json" {
        return Err(StubError::UnsupportedFormat {
            format: extension.to_string(),
        });
    }

    parse_json_stub(&content)
}

/// Parse a JSON extension file.
fn parse_json_stub(content: &str) -> Result<StubDefinition, StubError> {
    let definition: StubDefinition =
        serde_json::from_str(content).map_err(|e| StubError::ParseError {
            path: std::path::PathBuf::new(), // Will be set by caller
            source: e,
        })?;

    Ok(definition)
}

