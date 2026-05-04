# FRAME0 Implementation Status

Date: 2026-05-04 JST

This repository now contains a working FRAME0 v0.1 scaffold built from `frame0_ai_bundle/`.

## Implemented

- CLI skeleton: `frame0 --version`, `new`, `inspect`, `graph`, `run`, `render`, `devices`, `plugins`, `resources`, `resource`, `doctor`, `schema`, `snapshot`, `explain`, `suggest`, `scene patch`, `examples`, `benchmark`, `logs`
- JSON output for operational commands
- NDJSON event stream: `frame0 run <scene> --events ndjson`
- JSON Schema v0 exports for scene, plugin, resource, device, capability, packets, events, errors, runtime snapshot, graph, parameter, and permissions
- Scene manifest parsing and static validation
- Graph extraction, reference validation, cycle detection, and topological order
- Resource Registry with mock devices and runtime snapshot output
- Manual, monotonic, and fixed-step timebase primitives
- Deterministic headless mock render reports
- Mock device capability model for video, audio, and OSC
- Plugin manifest loading and verification
- Out-of-process plugin host executable with dynamic library loading
- Mock native plugin dynamic library implementing the C ABI
- Plugin crash detection and one-shot restart supervision test path
- Plugin stream open/start/callback/stop/close smoke path with frame/audio packet capture
- Native ML capabilities, model schema, inference packet schema, and example scene
- Mock native ML plugin with `frame0_plugin_control_json`
- ML describe/infer host commands returning deterministic native inference JSON
- Extension schema and examples for Core Media I/O Camera Extension, AUv3, and Syphon-style output
- Stub plugin manifests for camera extension, audio unit extension, and Syphon output
- Native plugin C ABI header
- C++ SDK adapter interface
- Native external C ABI skeleton and C/C++ external templates
- AI operation guide, error explanation, fix suggestions, graph diff, and merge-patch helper
- Public API documentation, schema reference, user manual, and development TODO checklist
- Machine-readable documentation index via `frame0 docs index --json` and example documentation listing via `frame0 docs examples --json`
- API compatibility notes for schema, CLI JSON, NDJSON, and native ABI changes
- Repository example verification helper via `scripts/verify_examples.sh`
- Addon skeleton generator via `frame0 new <path> --kind addon-rust`
- Expanded examples for projection mapping, multi-camera switching, depth point clouds, MIDI/OSC control, generative typography, dataset recording, Core ML style transfer, and spatial audio visualization
- Examples: 31 CLI-verifiable scenes covering shader/rendering, audio, audio-visual sync, ML, Apple native APIs, native SDK bridges, projection mapping, multi-camera switching, depth point clouds, MIDI/OSC control, generative typography, dataset recording, extensions, and addon/external authoring patterns
- CI workflow for format and tests
- Unit and integration tests

## Explicitly Stubbed For Native Follow-up

These are represented by stable contracts and adapter boundaries, but not implemented as real native/hardware paths yet:

- Metal command queue, shader compilation, texture pool, GPU timing, and window preview
- AVFoundation camera capture and CVPixelBuffer to Metal texture bridge
- CoreAudio input and real FFT node
- IPC transport beyond process exit supervision
- Real SDK adapter sample such as DeckLink, RealSense, NDI, or AJA
- Real Core ML / MPSGraph / ANE-backed model execution
- OS Extension outputs such as Core Media I/O Camera Extension and AUv3
- Signed/installable macOS extension bundles for the stub manifests

The stubs are intentional: the initial spec requires CLI/schema/runtime/resource/timebase/mock contracts first, before binding vendor SDK or OS extension code.

## Verification

Passed:

```bash
cargo fmt --all -- --check
cargo test --all
cargo run -q -p frame0_cli -- inspect examples/hello_shader/scene.yaml --json
cargo run -q -p frame0_cli -- graph examples/camera_to_shader/scene.yaml --json
cargo run -q -p frame0_cli -- run examples/hello_shader/scene.yaml --events ndjson --frames 2
cargo run -q -p frame0_cli -- plugins verify plugins/mock/plugin.yaml --json
cargo run -q -p frame0_cli -- doctor --json
cargo run -q -p frame0_cli -- docs index --json
cargo run -q -p frame0_cli -- docs examples --json
cargo run -q -p frame0_cli -- new /tmp/frame0-addon-smoke --kind addon-rust --force
cargo run -q -p frame0_cli -- schema export scene --json
cargo run -q -p frame0_cli -- schema export all --json
scripts/verify_examples.sh
cargo run -q -p frame0_cli -- snapshot runtime --scene examples/audio_reactive/scene.yaml --json
cargo build -p frame0_mock_sdk
cargo run -q -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -q -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
cargo run -q -p frame0_plugin_host -- supervise plugins/mock/plugin.yaml --max-restarts 1 --crash-first --json
cargo build -p frame0_mock_ml
cargo run -q -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -q -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
cargo run -q -p frame0_cli -- inspect examples/native_ml/scene.yaml --json
cargo run -q -p frame0_cli -- inspect examples/camera_extension_output/scene.yaml --json
cargo run -q -p frame0_cli -- inspect examples/auv3_audio_unit/scene.yaml --json
cargo run -q -p frame0_cli -- inspect examples/extension_multi_output/scene.yaml --json
cargo run -q -p frame0_cli -- examples list --json
cargo run -q -p frame0_cli -- plugins verify plugins/camera_extension_stub/plugin.yaml --json
cargo run -q -p frame0_cli -- plugins verify plugins/audio_unit_stub/plugin.yaml --json
cargo run -q -p frame0_cli -- plugins verify plugins/syphon_stub/plugin.yaml --json
```

## Next Engineering Order

1. Add structured IPC between runtime and plugin host.
2. Move stream packet forwarding from host smoke output into runtime resource/event ingestion.
3. Add a macOS Metal bridge crate or Swift helper behind `frame0_render`.
4. Add signed macOS extension bundle scaffolds for Camera Extension and AUv3.
5. Add AVFoundation camera discovery/capture as an adapter, not core code.
6. Add CoreAudio input and FFT as an adapter-backed node.
7. Replace `frame0_mock_ml` internals with a real Core ML or MPSGraph adapter behind the same C ABI.
8. Choose and implement one real video SDK adapter sample.
