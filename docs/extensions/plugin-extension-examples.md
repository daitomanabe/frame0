# FRAME0 Plugin And Extension Examples

FRAME0 models macOS extensions as resources behind plugin manifests. Scene logic stays in the runtime graph; OS extension bundles only adapt input/output transport.

## Core Media I/O Camera Extension

Example:

```bash
cargo run -p frame0_cli -- inspect examples/camera_extension_output/scene.yaml --json
cargo run -p frame0_cli -- plugins verify plugins/camera_extension_stub/plugin.yaml --json
```

Files:

- `examples/camera_extension_output/scene.yaml`
- `plugins/camera_extension_stub/plugin.yaml`

The scene renders a video graph output into an `os.camera_extension` output resource. The stub declares an IOSurface-to-CMSampleBuffer transport, which is the adapter boundary a real Camera Extension bundle would consume.

## Audio Unit v3

Example:

```bash
cargo run -p frame0_cli -- inspect examples/auv3_audio_unit/scene.yaml --json
cargo run -p frame0_cli -- plugins verify plugins/audio_unit_stub/plugin.yaml --json
```

Files:

- `examples/auv3_audio_unit/scene.yaml`
- `plugins/audio_unit_stub/plugin.yaml`

The scene maps FFT bands into AUv3 parameters. This keeps DAW-facing parameter transport outside the core graph while still making the relationship inspectable as JSON.

## Multi Output Extension Graph

Example:

```bash
cargo run -p frame0_cli -- inspect examples/extension_multi_output/scene.yaml --json
cargo run -p frame0_cli -- plugins verify plugins/syphon_stub/plugin.yaml --json
```

Files:

- `examples/extension_multi_output/scene.yaml`
- `plugins/syphon_stub/plugin.yaml`

The scene sends one render node to screen preview, virtual camera, and Syphon-style output. It exists to validate output fan-out and extension metadata before implementing signed macOS bundles.

## Current Boundary

Implemented now:

- extension schema export
- stub plugin manifests
- inspectable scenes
- plugin verification through the CLI

Not implemented yet:

- signed `.appex` bundles
- entitlement setup
- install/uninstall flow
- Core Media I/O provider implementation
- AUv3 component registration

