use serde::Deserialize;

/// A stub definition containing symbols to be loaded.
///
/// This follows the same pattern as the extension system but is focused
/// on stub file loading rather than runtime extension.
#[derive(Debug, Clone, Deserialize)]
pub struct StubDefinition {
    /// List of symbols defined in this stub file.
    pub symbols: Vec<StubSymbol>,
}

/// A single symbol definition from a stub file.
#[derive(Debug, Clone, Deserialize)]
pub struct StubSymbol {
    /// Name of the symbol.
    pub name: String,

    /// Type of the symbol (e.g., "function", "string", "object").
    pub r#type: String,

    /// Documentation string.
    #[serde(default)]
    pub doc: String,

    /// Function signature if this is a callable symbol.
    #[serde(default)]
    pub callable: Option<StubCallable>,
}

/// Function signature for callable symbols.
#[derive(Debug, Clone, Deserialize)]
pub struct StubCallable {
    /// Function parameters.
    pub params: Vec<StubParam>,

    /// Return type.
    pub return_type: String,
}

/// Function parameter definition.
#[derive(Debug, Clone, Deserialize)]
pub struct StubParam {
    /// Parameter name.
    pub name: String,

    /// Parameter type.
    pub r#type: String,

    /// Parameter documentation.
    #[serde(default)]
    pub doc: String,

    /// Default value for the parameter.
    #[serde(default)]
    pub default_value: String,

    /// Whether the parameter is mandatory.
    #[serde(default)]
    pub is_mandatory: bool,

    /// Whether this is a star argument (*args).
    #[serde(default)]
    pub is_star_arg: bool,

    /// Whether this is a star-star argument (**kwargs).
    #[serde(default)]
    pub is_star_star_arg: bool,
}
