# {{project-name}}

{{description}}

## Building

To build this plugin:

```bash
make
```

## Installation

After building, the plugin can be installed in witmproxy by running `witm plugin add <path-to-wasm-file>`.

## Template Variables

This plugin was generated from the witmproxy Rust plugin template with the following variables:

- **Plugin Name**: {{plugin-name}}
- **Authors**: {{author}}
- **Description**: {{description}}

## Customization

- Modify the `manifest()` function in `src/lib.rs` to update plugin metadata
- Implement your plugin logic in the `handle_request()` and `handle_response()` functions
- Update the `filter` CEL expressions in the `manifest()` function to specify which connections/requests/responses the plugin should process
- Add any additional dependencies to `Cargo.toml`