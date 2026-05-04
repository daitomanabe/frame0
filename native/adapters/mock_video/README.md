# FRAME0 Mock Native Adapter

The mock adapter is the first native SDK target. It must behave like a vendor SDK without requiring hardware:

- enumerate deterministic video/audio/timecode devices
- negotiate modes from `plugins/mock/plugin.yaml`
- emit `FramePacket` and `AudioPacket` metadata
- simulate dropped frames, slow callbacks, hot unplug, and crash modes
- cross the runtime boundary only through `native/frame0_plugin_c_api/frame0_plugin_api.h`

The current repository includes the ABI, manifest, and a Rust `cdylib` mock implementation in `native/adapters/mock_sdk`.

```bash
cargo build -p frame0_mock_sdk
cargo run -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
```
