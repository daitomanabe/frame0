# Addon Verification Flow

FRAME0 provides a repository-local helper for basic addon package checks:

```bash
scripts/verify_addon_package.sh addons/org.example.frame0.visual_warp/addon.yaml
scripts/verify_addon_package.sh addons/org.example.frame0.c_external_gain/addon.yaml
```

The helper checks that the manifest has the required addon keys, resolves example scenes relative to the package root, and runs:

```bash
cargo run -q -p frame0_cli -- inspect <example-scene> --json
```

Native externals should also run their build checks:

```bash
clang -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external.c
clang++ -std=c++17 -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external_cpp.cpp
```

Rust addons should also run:

```bash
cargo test --manifest-path templates/addon-rust/Cargo.toml
```

This helper is intentionally conservative. It does not execute arbitrary `tests` commands from third-party manifests.
