use std::path::Path;

use anyhow::Result;
use starpls_bazel::Builtins;

use crate::converter::convert_to_builtins;
use crate::error::StubError;
use crate::parser::parse_stub_file;

/// Loader for custom extension files.
pub struct StubLoader {
    builtins: Builtins,
}

impl StubLoader {
    pub fn new() -> Self {
        Self {
            builtins: Builtins::default(),
        }
    }

    /// Load a single extension file and merge it into the builtins.
    pub fn load_stub_file(&mut self, path: &Path) -> Result<(), StubError> {
        let stub_definition = parse_stub_file(path)?;
        let stub_builtins = convert_to_builtins(&stub_definition)?;
        self.merge_builtins(stub_builtins);
        Ok(())
    }

    /// Convert the loaded extensions to builtins format.
    pub fn into_builtins(self) -> Result<Builtins, StubError> {
        Ok(self.builtins)
    }

    fn merge_builtins(&mut self, other: Builtins) {
        self.builtins.global.extend(other.global);
        self.builtins.r#type.extend(other.r#type);
    }
}

impl Default for StubLoader {
    fn default() -> Self {
        Self::new()
    }
}
