use anyhow::Result;
use starpls_bazel::builtin::Callable;
use starpls_bazel::builtin::Param;
use starpls_bazel::builtin::Value;
use starpls_bazel::APIContext;
use starpls_bazel::Builtins;

use crate::error::ExtensionError;
use crate::types::ExtensionCallable;
use crate::types::ExtensionDefinition;
use crate::types::ExtensionParam;
use crate::types::ExtensionSymbol;

/// Convert an extension definition to Starpls builtin format.
pub fn convert_to_builtins(definition: &ExtensionDefinition) -> Result<Builtins, ExtensionError> {
    let mut builtins = Builtins::default();

    for symbol in &definition.symbols {
        let value = convert_symbol_to_value(symbol)?;
        builtins.global.push(value);
    }

    Ok(builtins)
}

fn convert_symbol_to_value(symbol: &ExtensionSymbol) -> Result<Value, ExtensionError> {
    let callable = if let Some(ref callable) = symbol.callable {
        Some(convert_callable(callable)?)
    } else {
        None
    };

    Ok(Value {
        name: symbol.name.clone(),
        r#type: symbol.r#type.clone(),
        doc: symbol.doc.clone(),
        callable,
        api_context: APIContext::Bzl as i32,
    })
}

fn convert_callable(callable: &ExtensionCallable) -> Result<Callable, ExtensionError> {
    let params = callable
        .params
        .iter()
        .map(convert_param)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Callable {
        param: params,
        return_type: callable.return_type.clone(),
    })
}

fn convert_param(param: &ExtensionParam) -> Result<Param, ExtensionError> {
    Ok(Param {
        name: param.name.clone(),
        r#type: param.r#type.clone(),
        doc: param.doc.clone(),
        default_value: param.default_value.clone(),
        is_mandatory: param.is_mandatory,
        is_star_arg: param.is_star_arg,
        is_star_star_arg: param.is_star_star_arg,
    })
}
