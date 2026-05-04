# Dataset Recorder

Synchronized capture manifest for camera, audio, pose/labels, and metadata.
Useful as a starting point for ML dataset collection and reproducible replay.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/dataset_recorder/scene.yaml --json
cargo run -p frame0_cli -- graph examples/dataset_recorder/scene.yaml --json
```
