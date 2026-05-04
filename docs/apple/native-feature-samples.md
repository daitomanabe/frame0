# Apple Native Feature Samples

The Apple native sample collects platform-specific contracts into one inspectable scene:

```text
examples/apple_native_features/scene.yaml
```

Covered contracts:

- AVFoundation camera input
- CoreAudio input
- ScreenCaptureKit display capture
- Metal compute preprocessing
- Vision face landmarks
- Core ML inference
- MPSGraph-style inference
- AUv3 parameter bridge
- Core Media I/O Camera Extension output

The sample is a contract scaffold. It intentionally does not install signed `.appex` bundles, entitlements, or real Apple framework bridges.

Verify:

```bash
cargo run -p frame0_cli -- inspect examples/apple_native_features/scene.yaml --json
cargo run -p frame0_cli -- graph examples/apple_native_features/scene.yaml --json
```
