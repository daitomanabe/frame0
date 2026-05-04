# FRAME0

FRAME0 is a CLI-first creative runtime for realtime video, audio, device IO, native SDK adapters, OS extension output, native ML inference, and AI-controllable execution.

The project is not a Processing/openFrameworks-style drawing API. It is an inspectable runtime foundation for media systems where time, resources, plugin isolation, GPU/media transport, and machine-readable diagnostics matter as much as rendering.

## Current Status

This repository is an executable v0.1 scaffold. It already builds and tests the contracts that should exist before real hardware SDKs, signed macOS extensions, or native Apple framework bridges are added.

Implemented now:

- CLI with JSON output for inspection, graph extraction, dry runs, snapshots, schemas, examples, diagnostics, and plugin verification
- NDJSON runtime event stream for AI agents and automated tooling
- Scene, plugin, resource, packet, graph, runtime, permission, extension, ML model, and inference JSON schemas
- Static scene validation and graph topological ordering
- Resource registry with mock devices and extension resources
- Manual, monotonic, and fixed-step timebase primitives
- Out-of-process native plugin host
- C ABI native plugin boundary plus C++ adapter interface
- Mock native video/audio SDK plugin as a real `cdylib`
- Plugin crash detection and restart supervision smoke path
- Stream open/start/callback/stop/close smoke path with frame/audio packet capture
- Native ML plugin contract with deterministic mock inference output
- Deterministic example launcher that writes runnable HTML previews, launch reports, frame reports, and NDJSON events
- Core Media I/O Camera Extension, AUv3, and Syphon-style output example manifests
- Unit and integration tests for the CLI, plugin host, native mock SDK, and native mock ML adapter

Next native integration work:

- Real Metal command queue, shader compilation, texture pool, GPU timing, and window preview
- Real AVFoundation camera capture and CVPixelBuffer/CVMetalTexture bridge
- Real CoreAudio input and FFT implementation
- Runtime-to-plugin-host IPC beyond process-level smoke tests
- Signed/installable macOS `.appex` bundles and entitlement flow
- Real Core ML / MPSGraph / ANE-backed model execution
- Real vendor SDK adapters such as DeckLink, RealSense, NDI, or AJA

## Quick Start

```bash
cargo test --all
cargo run -p frame0_cli -- --version
cargo run -p frame0_cli -- doctor --json
cargo run -p frame0_cli -- new my_addon --kind addon-rust
```

Inspect a scene:

```bash
cargo run -p frame0_cli -- inspect examples/hello_shader/scene.yaml --json
cargo run -p frame0_cli -- graph examples/camera_to_shader/scene.yaml --json
cargo run -p frame0_cli -- run examples/hello_shader/scene.yaml --dry-run --json
cargo run -p frame0_cli -- run examples/hello_shader/scene.yaml --events ndjson --frames 3
```

Work with schemas and runtime snapshots:

```bash
cargo run -p frame0_cli -- schema list --json
cargo run -p frame0_cli -- schema export scene --json
cargo run -p frame0_cli -- snapshot runtime --scene examples/audio_reactive/scene.yaml --json
```

Start from the docs map when building against FRAME0:

```bash
cargo run -p frame0_cli -- docs index --json
cargo run -p frame0_cli -- docs examples --json
cargo run -p frame0_cli -- schema export all --json
cargo run -p frame0_cli -- examples list --json
```

## Native Plugin Host

Build and smoke-test the mock native SDK plugin:

```bash
cargo build -p frame0_mock_sdk
cargo run -p frame0_plugin_host -- inspect plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- enumerate-devices plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
```

Test crash detection and restart supervision:

```bash
cargo run -p frame0_plugin_host -- supervise plugins/mock/plugin.yaml --max-restarts 1 --crash-first --json
```

The native boundary is defined in [native/frame0_plugin_c_api/frame0_plugin_api.h](native/frame0_plugin_c_api/frame0_plugin_api.h). C++ SDK adapters should wrap vendor SDKs behind that C ABI instead of exposing C++ ABI, exceptions, STL types, or vendor handles to FRAME0 Core.

## Native ML

FRAME0 treats ML inference as a native adapter capability, not as core runtime logic. The mock ML plugin uses the same dynamic plugin host path and returns deterministic inference JSON.

```bash
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
cargo run -p frame0_cli -- inspect examples/native_ml/scene.yaml --json
```

The mock implementation is in [native/adapters/mock_ml](native/adapters/mock_ml). It is structured so the internals can later be replaced by Core ML, MPSGraph, or ANE-backed execution without changing the runtime-facing contract.

## OS Extension Examples

These examples run through the deterministic example launcher and materialize the FRAME0-side plugin/extension contracts as launch reports, NDJSON events, and HTML previews.

```bash
cargo run -p frame0_cli -- examples launch camera_extension_output --frames 120 --out runs/examples/camera_extension_output --json
cargo run -p frame0_cli -- examples launch auv3_audio_unit --frames 120 --out runs/examples/auv3_audio_unit --json
cargo run -p frame0_cli -- examples launch extension_multi_output --frames 120 --out runs/examples/extension_multi_output --json
cargo run -p frame0_cli -- inspect examples/camera_extension_output/scene.yaml --json
cargo run -p frame0_cli -- inspect examples/auv3_audio_unit/scene.yaml --json
cargo run -p frame0_cli -- inspect examples/extension_multi_output/scene.yaml --json
cargo run -p frame0_cli -- plugins verify plugins/camera_extension_stub/plugin.yaml --json
cargo run -p frame0_cli -- plugins verify plugins/audio_unit_stub/plugin.yaml --json
cargo run -p frame0_cli -- plugins verify plugins/syphon_stub/plugin.yaml --json
```

Included extension examples:

- `camera_extension_output`: render graph output to a Core Media I/O Camera Extension-style output resource
- `auv3_audio_unit`: map FFT analysis into AUv3 parameters
- `extension_multi_output`: fan one render output to screen preview, virtual camera, and Syphon-style output

## Examples

| Example | Purpose |
| --- | --- |
| `hello_shader` | Minimal render shader scene |
| `headless_render` | Deterministic headless render path |
| `camera_to_shader` | Mock camera input to render shader |
| `audio_reactive` | Mock audio input and FFT-driven shader path |
| `mock_sdk_input` | Mock native SDK video/audio input scene |
| `native_ml` | Native ML tensor/inference/overlay graph |
| `creative_primitives` | Processing/openFrameworks-style draw primitive manifest |
| `cinder_geometry` | Cinder-style geometry, camera, and material manifest |
| `operator_network` | TouchDesigner-style TOP/CHOP/DAT operator network manifest |
| `parameter_automation` | LFO, ramp, smoothing, mapping, and preset automation |
| `input_events` | Mouse, keyboard, MIDI, OSC, HID, serial, and multitouch event contracts |
| `timeline_sequencing` | Clips, cues, keyframes, beat tracks, transport, and deterministic replay |
| `media_utilities` | Image sequence, movie, audio buffer, feedback, capture, and playback manifests |
| `visual_nodes` | Feedback, instancing, particles, shader passes, text, and 2D/3D composition |
| `projection_mapping` | Quad warp, calibration, masking, edge blend, and projector output |
| `multi_camera_switching` | Four-camera sync, color match, motion scoring, auto switching, and recording |
| `depth_pointcloud` | Depth camera filtering, point cloud conversion, particles, and point cache |
| `midi_osc_control_surface` | MIDI, OSC, keyboard mappings, smoothing, presets, and shader control |
| `generative_typography` | SDF font atlas, text layout, glyph instancing, feedback, and still capture |
| `dataset_recorder` | Synchronized camera/audio/labels capture for ML datasets |
| `coreml_style_transfer` | AVFoundation, Metal preprocessing, Core ML inference, temporal stabilization |
| `spatial_audio_visualizer` | Ambisonic audio analysis, source tracking, instanced geometry, and recording |
| `cpp_external_bridge` | C++ native external through the stable C ABI |
| `shader_post_processing` | Multipass shader stack with bloom, LUT color, feedback, and capture |
| `audio_pipeline` | Audio input, generator, analyzer, mixer, recorder, meter, and output routing |
| `audio_visual_sync` | Beat, onset, FFT, and envelope mapping into visual parameters and replay |
| `ml_multimodal_pipeline` | Model registry, video/audio preprocessing, native mock inference, postprocess, and overlay |
| `apple_native_features` | AVFoundation, CoreAudio, ScreenCaptureKit, Metal, Vision, Core ML, AUv3, and Camera Extension contracts |
| `camera_extension_output` | Core Media I/O Camera Extension output contract |
| `auv3_audio_unit` | Audio Unit v3 parameter-control contract |
| `extension_multi_output` | Screen + virtual camera + Syphon-style fan-out |

List examples with:

```bash
cargo run -p frame0_cli -- examples list --json
cargo run -p frame0_cli -- examples launch projection_mapping --frames 120 --out runs/examples/projection_mapping --json
cargo run -p frame0_cli -- examples launch-all --frames 24 --out runs/examples --json
cargo run -p frame0_cli -- logs --root runs/examples --tail 5 --json
scripts/launch_examples.sh
scripts/verify_examples.sh
```

## Repository Layout

```text
apps/frame0_cli/              FRAME0 CLI
apps/frame0_plugin_host/      Native plugin host executable
crates/frame0_schema/         Manifest, packet, error, and schema exports
crates/frame0_core/           Resource registry, events, runtime snapshots
crates/frame0_time/           Monotonic/manual/fixed-step clocks
crates/frame0_graph/          Graph extraction, validation, topological order
crates/frame0_device/         Mock device and extension capability model
crates/frame0_render/         Render capability and headless mock reports
crates/frame0_plugin_api/     Plugin manifest loading and verification
crates/frame0_ai_tools/       AI diagnostics, graph diff, scene patch helpers
schemas/                      JSON Schema v0 files
native/frame0_plugin_c_api/   Stable C ABI header
native/frame0_external_c_api/ Stable C ABI skeleton for native externals
native/frame0_cpp_sdk/        C++ adapter interface
native/adapters/mock_sdk/     Mock native video/audio SDK plugin
native/adapters/mock_ml/      Mock native ML plugin
addons/                       Third-party addon registry convention
addons/org.example.*          Example third-party addon/external packages
templates/addon-rust/         Standalone Rust addon skeleton with tests
templates/external-c/         C/C++ native external skeleton with ABI entry points
templates/external-cpp/       C++ external adapter template behind the C ABI
plugins/                      Plugin and extension manifests
examples/                     CLI-testable scene manifests
docs/                         ADRs, operation guides, native/ML/extension docs
scripts/verify_examples.sh    Example scene verification helper
```

## AI Operation Contract

FRAME0 is designed so AI agents and humans use the same commands:

1. Write or patch a scene manifest.
2. Run `frame0 inspect <scene> --json`.
3. Run `frame0 graph <scene> --json`.
4. Run `frame0 devices list --json`.
5. Run `frame0 run <scene> --dry-run --json`.
6. Execute with `frame0 run <scene> --events ndjson`.
7. Use `frame0 explain error <error.json> --json` and `frame0 suggest fix <scene-or-error> --json`.

AI agents should not infer device IDs by name, depend on GUI state, write vendor SDK types into core manifests, assume a fixed frame rate without reading the clock policy, or treat plain text logs as the source of truth when JSON/NDJSON exists.

## Design Rules

- Core Runtime must not include vendor SDK headers.
- Native SDKs are wrapped by adapter processes and the stable C ABI.
- C++ ABI is not a public boundary.
- Timebase is independent of the render loop.
- Extension bundles adapt transport; scene logic stays in FRAME0 graph resources.
- ML inference is an adapter capability; model runtime handles do not leak into core APIs.
- Every operational feature should be inspectable through JSON.

## Documentation

- [API Documentation](docs/api/README.md)
- [API Reference](docs/api/reference.md)
- [Schema Reference](docs/api/schemas.md)
- [API Compatibility Notes](docs/api/compatibility.md)
- [User Manual](docs/manual/user-manual.md)
- [CLI Reference](docs/cli-reference.md)
- [AI Operation Guide](docs/ai/operation-guide.md)
- [Native C ABI Reference](docs/native/c-abi-reference.md)
- [Native External C ABI](docs/native/external-c-abi.md)
- [C++ Integration Mechanism](docs/native/cpp-integration.md)
- [Native ML Adapter](docs/ml/native-ml-adapter.md)
- [Audio Pipeline Sample](docs/audio/audio-pipeline.md)
- [Audio Visual Sync Sample](docs/audio/audio-visual-sync.md)
- [Shader And Post-Processing Examples](docs/rendering/shader-post-processing.md)
- [Apple Native Feature Samples](docs/apple/native-feature-samples.md)
- [Plugin And Extension Examples](docs/extensions/plugin-extension-examples.md)
- [Creative Tool Concept Map](docs/creative/CONCEPT_MAP.md)
- [Creative Parity Checklist](docs/creative/FEATURE_CHECKLIST.md)
- [Native Sample TODO Checklist](docs/creative/NATIVE_SAMPLE_TODO.md)
- [Development TODO Checklist](docs/roadmap/DEVELOPMENT_TODO.md)
- [Addon Registry Convention](docs/addons/registry.md)
- [Addon Authoring Guide](docs/addons/authoring-guide.md)
- [Addon Verification Flow](docs/addons/verification.md)
- [Implementation Status](FRAME0_IMPLEMENTATION_STATUS.md)
- [Architecture Decisions](docs/adr)

## License

MIT. See [LICENSE](LICENSE).
