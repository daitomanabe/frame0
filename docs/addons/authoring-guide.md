# Addon Authoring Guide

This guide describes the minimum shape of a third-party FRAME0 addon package.

## Package Layout

```text
addons/org.example.my_addon/
  addon.yaml
  README.md
  examples/
    basic_scene.yaml
  schemas/
    optional_node_params.schema.json
  src/
    optional native or Rust sources
```

The package root should be relocatable. Use paths relative to the package root in `addon.yaml` unless a command explicitly runs from the FRAME0 repository root.

## Manifest

Every package must include `addon.yaml` matching `schemas/addon.schema.json`.

Required fields:

- `id`: stable reverse-DNS identifier
- `name`: human-readable addon name
- `version`: addon package version
- `api_version`: FRAME0 addon API version
- `kind`: `rust_crate`, `native_external`, `wasm`, `script`, `schema_pack`, or `asset_pack`
- `capabilities`: feature strings the addon provides
- `entrypoints`: code, schema, asset, or native-library entry paths
- `permissions`: required runtime permissions
- `examples`: CLI-verifiable scene examples
- `tests`: commands or scripts that verify the package

## Capabilities

Capabilities should be specific and composable:

```yaml
capabilities:
  - visual.node
  - shader_pass
  - texture.processor
```

Avoid broad strings such as `everything` or `native`. FRAME0 uses capabilities for discovery, verification, permission review, and future loading policy.

## Permissions

Declare the narrowest permissions:

```yaml
permissions:
  camera: false
  microphone: false
  network: false
  file_read: true
  file_write: false
```

Native SDKs, camera extensions, audio units, and network transports should not be hidden behind generic visual or shader packages.

## Versioning

- Use semantic versions for addon releases.
- Increment patch for manifest/docs/examples fixes.
- Increment minor for new compatible capabilities or parameters.
- Increment major when scenes using the old addon can break.
- Keep `api_version` separate from package `version`.

## Rust Addons

Start from `templates/addon-rust`, or generate a copy:

```bash
frame0 new my_addon --kind addon-rust
```

Verification:

```bash
cargo test --manifest-path templates/addon-rust/Cargo.toml
cargo run -p frame0_cli -- inspect templates/addon-rust/examples/basic_scene.yaml --json
```

Rust addon packages should expose a small descriptor and keep runtime-facing behavior testable without private hardware.

## Native Externals

Start from `templates/external-c`.

Verification:

```bash
clang -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external.c
clang++ -std=c++17 -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external_cpp.cpp
cargo run -p frame0_cli -- inspect templates/external-c/examples/external_scene.yaml --json
```

Native externals should export C ABI functions from `native/frame0_external_c_api/frame0_external_api.h`. C++ implementation code should remain behind that C ABI.

## Examples

Every addon should provide at least one scene that passes:

```bash
cargo run -p frame0_cli -- inspect addons/org.example.my_addon/examples/basic_scene.yaml --json
```

Examples should use mock devices, generated data, or deterministic assets. Avoid private paths and machine-specific device IDs.

## Tests

Good `tests` entries are commands that a maintainer can run from the repository root:

```yaml
tests:
  - cargo run -p frame0_cli -- inspect addons/org.example.my_addon/examples/basic_scene.yaml --json
  - cargo test --manifest-path addons/org.example.my_addon/Cargo.toml
```

Native packages can use syntax-only compile checks before full dynamic library loading exists.

For the built-in repository helper, run:

```bash
scripts/verify_addon_package.sh addons/org.example.my_addon/addon.yaml
```

The helper inspects example scenes and does not execute arbitrary manifest test commands.

## Publishing Checklist

1. Add or update `addon.yaml`.
2. Add at least one inspectable example.
3. Add README usage notes.
4. Add tests that avoid private machine paths.
5. Add the package to `addons/registry.yaml`.
6. Run the package tests and repository test suite.
7. Run copyright and privacy scans before pushing.
