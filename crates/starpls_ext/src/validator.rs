use crate::error::StubError;
use crate::types::StubDefinition;

/// Validate a stub definition.
pub fn validate_stub_definition(definition: &StubDefinition) -> Result<(), StubError> {
    // Validate that all symbols have unique names
    let mut seen_symbols = std::collections::HashSet::new();

    for symbol in &definition.symbols {
        if !seen_symbols.insert(&symbol.name) {
            return Err(StubError::DuplicateSymbol {
                symbol: symbol.name.clone(),
            });
        }

        // Validate symbol properties
        validate_symbol(symbol)?;
    }

    Ok(())
}

fn validate_symbol(symbol: &crate::types::StubSymbol) -> Result<(), StubError> {
    if symbol.name.trim().is_empty() {
        return Err(StubError::ValidationError {
            message: "Symbol name cannot be empty".to_string(),
        });
    }

    // Validate that the symbol name is a valid identifier
    if !is_valid_identifier(&symbol.name) {
        return Err(StubError::ValidationError {
            message: format!("Invalid symbol name: '{}'", symbol.name),
        });
    }

    Ok(())
}

fn is_valid_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();
    let first = chars.next().unwrap();

    // First character must be letter or underscore
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    // Remaining characters must be letter, digit, or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
