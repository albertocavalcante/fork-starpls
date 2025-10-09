use std::path::Path;

use anyhow::Result;

use crate::error::ExtensionError;
use crate::types::ExtensionDefinition;

/// Parse a JSON extension file and return its definition.
pub fn parse_extension_file(path: &Path) -> Result<ExtensionDefinition, ExtensionError> {
    let content = std::fs::read_to_string(path).map_err(|e| ExtensionError::FileReadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    if extension != "json" {
        return Err(ExtensionError::UnsupportedFormat {
            format: extension.to_string(),
        });
    }

    parse_json_extension(&content)
}

/// Parse a JSON extension file.
fn parse_json_extension(content: &str) -> Result<ExtensionDefinition, ExtensionError> {
    let definition: ExtensionDefinition =
        serde_json::from_str(content).map_err(|e| ExtensionError::ParseError {
            path: std::path::PathBuf::new(), // Will be set by caller
            source: e,
        })?;

    Ok(definition)
}

