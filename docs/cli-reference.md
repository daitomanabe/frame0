# FRAME0 CLI Reference

Every operational command accepts `--json` for machine-readable output.

Core commands:

```bash
frame0 --version
frame0 new my_scene
frame0 inspect examples/hello_shader/scene.yaml --json
frame0 inspect examples/camera_extension_output/scene.yaml --json
frame0 inspect examples/auv3_audio_unit/scene.yaml --json
frame0 inspect examples/extension_multi_output/scene.yaml --json
frame0 graph examples/hello_shader/scene.yaml --json
frame0 graph diff before.yaml after.yaml --json
frame0 run examples/hello_shader/scene.yaml --dry-run --json
frame0 run examples/hello_shader/scene.yaml --events ndjson --frames 3
frame0 render examples/headless_render/scene.yaml --frames 60 --json
frame0 devices list --json
frame0 devices modes device.video_input.mock.0 --json
frame0 plugins list --json
frame0 plugins inspect io.frame0.mock.sdk --json
frame0 plugins verify plugins/mock/plugin.yaml --json
frame0 plugins verify plugins/camera_extension_stub/plugin.yaml --json
frame0 plugins verify plugins/audio_unit_stub/plugin.yaml --json
frame0 plugins verify plugins/syphon_stub/plugin.yaml --json
frame0 resources list --scene examples/hello_shader/scene.yaml --json
frame0 resource get node.color_shader --scene examples/hello_shader/scene.yaml --json
frame0 doctor --json
frame0 docs index --json
frame0 docs examples --json
frame0 schema list --json
frame0 schema export scene --json
frame0 schema export all --json
frame0 snapshot runtime --scene examples/hello_shader/scene.yaml --json
frame0 explain error error.json --json
frame0 suggest fix examples/hello_shader/scene.yaml --json
frame0 scene patch scene.yaml patch.json --json
frame0 examples list --json
frame0 examples run audio_visual_sync --frames 4
frame0 benchmark examples/hello_shader/scene.yaml --json
```

`run --events ndjson` is the primary AI-readable execution stream.

API and manual documentation:

- [API Documentation](api/README.md)
- [API Reference](api/reference.md)
- [Schema Reference](api/schemas.md)
- [User Manual](manual/user-manual.md)

Native plugin host commands:

```bash
cargo build -p frame0_mock_sdk
cargo run -p frame0_plugin_host -- inspect plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- enumerate-devices plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- supervise plugins/mock/plugin.yaml --max-restarts 1 --crash-first --json
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
```
