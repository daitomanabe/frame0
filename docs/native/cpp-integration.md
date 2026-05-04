# C++ Integration Mechanism

FRAME0 supports C++ through a C ABI boundary. C++ symbols, exceptions, STL types, vendor SDK handles, and object lifetimes should not cross into FRAME0 Core.

## Recommended Shape

```text
FRAME0 Core
  -> stable C ABI: frame0_external_api.h
    -> C ABI exported functions
      -> private C++ adapter class
        -> vendor SDK or custom C++ implementation
```

Use `templates/external-cpp` for a minimal implementation.

## Rules

- Export only C ABI functions from `native/frame0_external_c_api/frame0_external_api.h`.
- Put C++ classes in private headers or source files.
- Catch all exceptions before returning to FRAME0.
- Use JSON for params, metadata, control requests, and control responses.
- Use packet structs for borrowed data and named output resources.
- Keep build products out of the repo.

## Verification

```bash
clang++ -std=c++17 -fsyntax-only -I native/frame0_external_c_api -I templates/external-cpp/include templates/external-cpp/src/frame0_cpp_external_adapter.cpp
cargo run -p frame0_cli -- inspect examples/cpp_external_bridge/scene.yaml --json
```
