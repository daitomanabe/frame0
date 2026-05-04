# cpp_external_bridge

This scene demonstrates how FRAME0 references a C++ native external through the stable C ABI.

Run:

```bash
clang++ -std=c++17 -fsyntax-only -I native/frame0_external_c_api -I templates/external-cpp/include templates/external-cpp/src/frame0_cpp_external_adapter.cpp
cargo run -p frame0_cli -- inspect examples/cpp_external_bridge/scene.yaml --json
```
