# Rust Addon Skeleton

This template is a standalone Rust crate for building FRAME0 addons.

Run inside this directory:

```bash
cargo test
```

From the FRAME0 repository root:

```bash
cargo test --manifest-path templates/addon-rust/Cargo.toml
cargo run -p frame0_cli -- inspect templates/addon-rust/examples/basic_scene.yaml --json
```

Package contents:

- `addon.yaml`: addon manifest matching `schemas/addon.schema.json`
- `src/lib.rs`: minimal descriptor, processing function, and tests
- `examples/basic_scene.yaml`: scene manifest showing how the addon is referenced
