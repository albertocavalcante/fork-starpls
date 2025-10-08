/*!
Starlark extensions for Starpls.

This crate provides functionality for loading custom extension files (JSON format)
and converting them to Starpls builtin format, following the same pattern as
`starpls_bazel` for modular extension support.
*/

use std::path::Path;

use anyhow::Result;
pub use starpls_bazel::APIContext;
// Re-export starpls_bazel types for compatibility
pub use starpls_bazel::Builtins;

pub mod converter;
pub mod error;
pub mod loader;
pub mod parser;
pub mod types;
pub mod validator;

use error::StubError;
use loader::StubLoader;

/// Extension trait for Builtins to add merge functionality
pub trait BuiltinsExt {
    /// Merge another Builtins into this one, adding all types and globals
    fn merge(&mut self, other: Builtins);
}

impl BuiltinsExt for Builtins {
    fn merge(&mut self, other: Builtins) {
        self.r#type.extend(other.r#type);
        self.global.extend(other.global);
    }
}

/// Main entry point for loading custom extensions.
///
/// This function loads JSON extension files and converts them to Starpls builtin format,
/// following the same pattern as `starpls_bazel::load_builtins()`.
pub fn load_custom_extensions(ext_paths: &[impl AsRef<Path>]) -> Result<Builtins, StubError> {
    let mut loader = StubLoader::new();

    for path in ext_paths {
        loader.load_stub_file(path.as_ref())?;
    }

    loader.into_builtins()
}

/// Load a single JSON extension file and return builtin definitions.
pub fn load_single_extension(path: impl AsRef<Path>) -> Result<Builtins, StubError> {
    let mut loader = StubLoader::new();
    loader.load_stub_file(path.as_ref())?;
    loader.into_builtins()
}

/// Validate a JSON extension file without loading it.
pub fn validate_extension_file(path: impl AsRef<Path>) -> Result<(), StubError> {
    let definition = parser::parse_stub_file(path.as_ref())?;
    validator::validate_stub_definition(&definition)?;
    Ok(())
}
