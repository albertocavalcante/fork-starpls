# StarPLS Extension System - Quick Start

## Overview
StarPLS supports file-specific extensions that add custom symbols (functions, objects, types) to your Starlark files based on file patterns.

## Basic Usage
```bash
starpls check myfile.star --experimental_load_extensions config.json
```

## Extension Configuration

### Simple Global Extension
```json
{
  "globals": [
    {
      "name": "my_function",
      "type": "function",
      "doc": "My custom function",
      "callable": {
        "params": [{"name": "arg", "type": "string"}],
        "return_type": "None"
      }
    }
  ]
}
```
→ Available in **all** files

### File-Specific Extension
```json
{
  "when": {
    "file_patterns": ["*.star"]
  },
  "globals": [
    {
      "name": "dev_config",
      "type": "DevConfig",
      "as_type": true,
      "properties": {
        "debug": {
          "type": "function",
          "callable": {
            "params": [{"name": "enabled", "type": "bool"}]
          }
        }
      }
    }
  ]
}
```
→ Available **only** in `.star` files

## Key Features

**File Pattern Matching**: Use `when.file_patterns` to limit extensions to specific files
```json
"when": {"file_patterns": ["*.star", "Tiltfile", "*.bzl"]}
```

**Object Types**: Set `as_type: true` + `properties` for objects with methods
```json
{
  "name": "config",
  "as_type": true,
  "properties": {
    "get": {"type": "function", "callable": {...}},
    "value": {"type": "string"}
  }
}
```

**Functions**: Use `callable` field for function definitions
```json
{
  "name": "notify",
  "type": "function",
  "callable": {
    "params": [{"name": "msg", "type": "string"}],
    "return_type": "None"
  }
}
```

## Result
- Extensions respect file patterns (`.star` vs `.bzl` isolation)
- Symbols are recognized by LSP (completion, hover, etc.)
- Type checking works for function parameters and object methods

## Example Usage
```python
# In .star file with dev_config extension
dev_config.debug(True)  # ✅ Works
notify("Build started") # ✅ Works

# In .bzl file (if extension has when: ["*.star"])
dev_config.debug(True)  # ❌ "not defined" - correctly isolated
```