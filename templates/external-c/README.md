# C/C++ External Skeleton

This template shows a native external that implements the FRAME0 External C ABI.

Build checks from the FRAME0 repository root:

```bash
clang -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external.c
clang++ -std=c++17 -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external_cpp.cpp
cargo run -p frame0_cli -- inspect templates/external-c/examples/external_scene.yaml --json
```

Entry points implemented in `src/frame0_example_external.c`:

- `frame0_external_get_descriptor`
- `frame0_external_initialize`
- `frame0_external_shutdown`
- `frame0_external_describe_ports`
- `frame0_external_create_node`
- `frame0_external_destroy_node`
- `frame0_external_process`
- `frame0_external_control_json`
- `frame0_external_last_error_json`
- `frame0_external_free_string`

The C++ file demonstrates how to keep C++ implementation code behind exported C ABI functions.
