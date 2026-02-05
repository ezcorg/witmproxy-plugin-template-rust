# {{project-name}}

{{description}}

## Building

Ensure you have the `wasm32-wasip2` Rust toolchain installed:
```bash
rustup target add wasm32-wasip2
```

Then execute the default make target (which runs `cargo build --target wasm32-wasip2` and signs the produced WASM component):

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
- Implement your plugins event handling logic in the `handle()` function
- Update the `capabilities` in the manifest to request additional permissions to host resources or events
- Add any additional dependencies to `Cargo.toml`