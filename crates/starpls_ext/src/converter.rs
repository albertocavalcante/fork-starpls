use anyhow::Result;
use starpls_bazel::builtin::Callable;
use starpls_bazel::builtin::Param;
use starpls_bazel::builtin::Value;
use starpls_bazel::APIContext;
use starpls_bazel::Builtins;

use crate::error::StubError;
use crate::types::StubCallable;
use crate::types::StubDefinition;
use crate::types::StubParam;
use crate::types::StubSymbol;

/// Convert a stub definition to Starpls builtin format.
///
/// This follows the same pattern as `starpls_bazel::env` for converting
/// external definitions to the internal builtin format.
pub fn convert_to_builtins(definition: &StubDefinition) -> Result<Builtins, StubError> {
    let mut builtins = Builtins::default();

    for symbol in &definition.symbols {
        let value = convert_symbol_to_value(symbol)?;
        builtins.global.push(value);
    }

    Ok(builtins)
}

fn convert_symbol_to_value(symbol: &StubSymbol) -> Result<Value, StubError> {
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

fn convert_callable(callable: &StubCallable) -> Result<Callable, StubError> {
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

fn convert_param(param: &StubParam) -> Result<Param, StubError> {
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
