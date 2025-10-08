use std::path::Path;

use anyhow::Result;

use crate::error::StubError;
use crate::types::StubDefinition;

/// Parse a stub file and return its definition.
///
/// This function detects the file format and parses accordingly.
pub fn parse_stub_file(path: &Path) -> Result<StubDefinition, StubError> {
    let content = std::fs::read_to_string(path).map_err(|e| StubError::FileReadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension {
        "json" => parse_json_stub(&content),
        "py" => parse_python_stub(&content),
        _ => Err(StubError::UnsupportedFormat {
            format: extension.to_string(),
        }),
    }
}

/// Parse a JSON stub file.
fn parse_json_stub(content: &str) -> Result<StubDefinition, StubError> {
    let definition: StubDefinition =
        serde_json::from_str(content).map_err(|e| StubError::ParseError {
            path: std::path::PathBuf::new(), // Will be set by caller
            source: e,
        })?;

    Ok(definition)
}

/// Parse a Python stub file.
///
/// This is a placeholder implementation. In a full implementation,
/// this would parse Python AST to extract type information.
fn parse_python_stub(_content: &str) -> Result<StubDefinition, StubError> {
    // TODO: Implement Python stub parsing
    Err(StubError::UnsupportedFormat {
        format: "python".to_string(),
    })
}
