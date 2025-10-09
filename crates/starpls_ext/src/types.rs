use serde::Deserialize;

/// An extension definition containing symbols to be loaded.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionDefinition {
    /// List of symbols defined in this extension file.
    pub symbols: Vec<ExtensionSymbol>,
}

/// A single symbol definition from an extension file.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionSymbol {
    /// Name of the symbol.
    pub name: String,

    /// Type of the symbol (e.g., "function", "string", "object").
    pub r#type: String,

    /// Documentation string.
    #[serde(default)]
    pub doc: String,

    /// Function signature if this is a callable symbol.
    #[serde(default)]
    pub callable: Option<ExtensionCallable>,
}

/// Function signature for callable symbols.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionCallable {
    /// Function parameters.
    pub params: Vec<ExtensionParam>,

    /// Return type.
    pub return_type: String,
}

/// Function parameter definition.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionParam {
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
