use std::path::Path;

use anyhow::Result;
use starpls_bazel::Builtins;

use crate::converter::convert_to_builtins;
use crate::error::ExtensionError;
use crate::parser::parse_extension_file;

/// Loader for custom extension files.
pub struct ExtensionLoader {
    builtins: Builtins,
}

impl ExtensionLoader {
    pub fn new() -> Self {
        Self {
            builtins: Builtins::default(),
        }
    }

    /// Load a single extension file and merge it into the builtins.
    pub fn load_extension_file(&mut self, path: &Path) -> Result<(), ExtensionError> {
        let extension_definition = parse_extension_file(path)?;
        let extension_builtins = convert_to_builtins(&extension_definition)?;
        self.merge_builtins(extension_builtins);
        Ok(())
    }

    /// Convert the loaded extensions to builtins format.
    pub fn into_builtins(self) -> Result<Builtins, ExtensionError> {
        Ok(self.builtins)
    }

    fn merge_builtins(&mut self, other: Builtins) {
        self.builtins.global.extend(other.global);
        self.builtins.r#type.extend(other.r#type);
    }
}

impl Default for ExtensionLoader {
    fn default() -> Self {
        Self::new()
    }
}
