# FRAME0 User Manual

This manual explains how to use the current FRAME0 scaffold as a CLI-first
creative runtime and development environment.

## 1. Install And Verify

From the repository root:

```bash
cargo test --all
cargo run -p frame0_cli -- doctor --json
```

`doctor` reports the CLI version, platform, schema names, mock devices, render
capabilities, and native boundary paths.

## 2. Inspect A Scene

```bash
cargo run -p frame0_cli -- inspect examples/hello_shader/scene.yaml --json
```

Use `inspect` first whenever you create or edit a scene. It checks schema-level
shape, node references, device requirements, and permission intent.

## 3. Inspect The Graph

```bash
cargo run -p frame0_cli -- graph examples/camera_to_shader/scene.yaml --json
```

The graph output shows nodes, edges, diagnostics, and topological order. Use it
before running complex media, ML, or extension scenes.

## 4. Dry Run And Event Stream

```bash
cargo run -p frame0_cli -- run examples/hello_shader/scene.yaml --dry-run --json
cargo run -p frame0_cli -- run examples/hello_shader/scene.yaml --events ndjson --frames 3
```

Dry runs are useful for validation and automation. NDJSON events are useful for
AI agents, regression tests, log capture, and timeline replay.

## 5. Render Headlessly

```bash
cargo run -p frame0_cli -- render examples/headless_render/scene.yaml --frames 60 --json
```

The current renderer is a deterministic mock backend. Real Metal rendering,
texture pooling, shader compilation, and GPU timing are intentionally future
work behind the same CLI surface.

## 6. Use Schemas

```bash
cargo run -p frame0_cli -- schema list --json
cargo run -p frame0_cli -- schema export all --json
```

Schemas are the contract for external tools and addon authors. Use them to
validate generated scene manifests before running FRAME0.

## 7. Explore Examples

```bash
cargo run -p frame0_cli -- examples list --json
cargo run -p frame0_cli -- examples run audio_visual_sync --frames 4
cargo run -p frame0_cli -- examples launch projection_mapping --frames 120 --out runs/examples/projection_mapping --json
cargo run -p frame0_cli -- examples launch-all --frames 24 --out runs/examples --json
scripts/launch_examples.sh
scripts/verify_examples.sh
```

Important example groups:

| Group | Examples |
| --- | --- |
| Rendering | `hello_shader`, `headless_render`, `shader_post_processing`, `visual_nodes` |
| Audio | `audio_reactive`, `audio_pipeline`, `audio_visual_sync` |
| Spatial audio | `spatial_audio_visualizer` |
| Native SDKs | `mock_sdk_input`, `cpp_external_bridge`, `native_ml`, `depth_pointcloud` |
| ML | `native_ml`, `ml_multimodal_pipeline`, `dataset_recorder`, `coreml_style_transfer`, `apple_native_features` |
| Apple native | `apple_native_features`, `camera_extension_output`, `auv3_audio_unit` |
| Control | `input_events`, `midi_osc_control_surface`, `parameter_automation` |
| Creative systems | `creative_primitives`, `cinder_geometry`, `operator_network`, `timeline_sequencing`, `projection_mapping`, `generative_typography` |

## 8. Work With Native Plugins

```bash
cargo build -p frame0_mock_sdk
cargo run -p frame0_plugin_host -- inspect plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- enumerate-devices plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
```

Native plugins are isolated behind a C ABI and loaded by the plugin host. Keep
vendor SDKs, C++ ABI types, and OS framework objects inside the adapter.

## 9. Work With Native ML

```bash
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
```

The mock ML adapter demonstrates model description and inference packets. Future
Core ML, MPSGraph, ANE, or vendor backends should preserve these runtime-facing
contracts.

## 10. Build Addons

Start from the Rust addon skeleton:

```bash
frame0 new my_addon --kind addon-rust
scripts/verify_addon_package.sh templates/addon-rust
```

Addon packages should include:

- `addon.yaml`
- README
- tests or verification script
- at least one example scene
- plugin or native external assets when needed

## 11. AI-Assisted Development Loop

Use this loop for automated development:

```bash
frame0 inspect path/to/scene.yaml --json
frame0 graph path/to/scene.yaml --json
frame0 run path/to/scene.yaml --dry-run --json
frame0 run path/to/scene.yaml --events ndjson --frames 3
frame0 explain error error.json --json
frame0 suggest fix path/to/scene.yaml --json
```

Do not use plain text logs as the only source of truth when JSON or NDJSON is
available.

## 12. Verify Public Examples

Before publishing changes to example scenes, run:

```bash
scripts/verify_examples.sh
```

The script inspects every `examples/*/scene.yaml` and verifies that the CLI can
generate the documented example index.

## 13. Launch Example Artifacts

Use `examples launch` when you want concrete files to inspect or open:

```bash
cargo run -p frame0_cli -- examples launch coreml_style_transfer --frames 120 --out runs/examples/coreml_style_transfer --json
cargo run -p frame0_cli -- examples launch-all --frames 24 --out runs/examples --json
```

The launch command writes:

- `preview.html`
- `launch.json`
- `events.ndjson`
- `frames.json`

`launch-all` also writes `index.html` at the output root.
