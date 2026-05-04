# apple_native_features

This scene models Apple-native feature contracts: AVFoundation, CoreAudio, ScreenCaptureKit, Metal, Vision, Core ML, MPSGraph, AUv3, and Core Media I/O Camera Extension output.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/apple_native_features/scene.yaml --json
cargo run -p frame0_cli -- graph examples/apple_native_features/scene.yaml --json
```

This sample does not install signed macOS bundles. It defines the FRAME0-side resources, selectors, and graph contracts.
