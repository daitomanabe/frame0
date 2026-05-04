# C++ External Template

This template shows the recommended FRAME0 C++ mechanism:

1. Export the stable C ABI from `native/frame0_external_c_api/frame0_external_api.h`.
2. Keep C++ classes in private implementation files.
3. Catch exceptions before they cross the C ABI.
4. Exchange node params, metadata, and control messages as JSON.

Verify from the FRAME0 repository root:

```bash
clang++ -std=c++17 -fsyntax-only -I native/frame0_external_c_api -I templates/external-cpp/include templates/external-cpp/src/frame0_cpp_external_adapter.cpp
cargo run -p frame0_cli -- inspect templates/external-cpp/examples/cpp_external_scene.yaml --json
```

Build with CMake:

```bash
cmake -S templates/external-cpp -B templates/external-cpp/build
cmake --build templates/external-cpp/build
```
