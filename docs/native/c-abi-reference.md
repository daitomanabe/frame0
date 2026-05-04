# FRAME0 Native C ABI Reference

The public native plugin boundary is `native/frame0_plugin_c_api/frame0_plugin_api.h`.

Native processing externals use a separate skeleton at `native/frame0_external_c_api/frame0_external_api.h`. See [Native External C ABI](external-c-abi.md).

Rules:

- `FRAME0_PLUGIN_API_VERSION` must match the runtime.
- Exported functions must never throw exceptions.
- Strings returned by plugins must be released by `frame0_plugin_free_string`.
- Device discovery returns FRAME0 resources and capability metadata, not vendor classes.
- Frame and audio callbacks carry metadata and storage references. They do not own rendering or scene logic.
- Optional adapter-specific control goes through `frame0_plugin_control_json`, which returns JSON and keeps SDK handles behind the native boundary.

Implemented v0.1 native smoke path:

```bash
cargo build -p frame0_mock_sdk
cargo run -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
```

Crash/restart supervision can be tested with:

```bash
cargo run -p frame0_plugin_host -- supervise plugins/mock/plugin.yaml --max-restarts 1 --crash-first --json
```

Native ML control can be tested with:

```bash
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
```
