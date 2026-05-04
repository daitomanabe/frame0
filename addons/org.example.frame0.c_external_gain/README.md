# org.example.frame0.c_external_gain

Example third-party native external package. It points at the C external skeleton entry points and shows how a package manifest declares platform-specific native library outputs.

Verify from the FRAME0 repository root:

```bash
clang -fsyntax-only -I native/frame0_external_c_api -I templates/external-c/include templates/external-c/src/frame0_example_external.c
cargo run -p frame0_cli -- inspect addons/org.example.frame0.c_external_gain/examples/gain_scene.yaml --json
```
