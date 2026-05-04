# FRAME0 Native ML Adapter

FRAME0 treats machine learning as a native adapter capability, not as core runtime logic.

Native ML plugins should expose:

- `ml.inference`
- `ml.model.load`
- `ml.tensor`
- task-specific capabilities such as `ml.classification`, `ml.embedding`, `ml.segmentation`, `ml.pose`, or `ml.depth`
- backend capabilities such as `coreml.model`, `metal.mps`, and `ane.inference`

The v0.1 control path is `frame0_plugin_control_json` in `native/frame0_plugin_c_api/frame0_plugin_api.h`.

Current smoke commands:

```bash
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
cargo run -p frame0_cli -- inspect examples/native_ml/scene.yaml --json
cargo run -p frame0_cli -- inspect examples/ml_multimodal_pipeline/scene.yaml --json
```

The mock implementation lives in `native/adapters/mock_ml`. It returns deterministic classification and embedding outputs so runtime and AI agents can develop against stable JSON before real Core ML, MPSGraph, or ANE-backed code is added.

Core rules:

- Do not link Core ML, MPS, or model runtime APIs into `frame0_core`.
- Do not expose vendor model handles through the stable API.
- Move tensors by resource references or storage handles, not by unbounded JSON blobs.
- Keep inference results timestamped and inspectable as `InferencePacket`.

## Rich Sample

`examples/ml_multimodal_pipeline/scene.yaml` expands the basic `native_ml` sample with:

- model registry metadata
- video tensor preprocessing
- audio/mel preprocessing
- two native mock inference nodes
- multimodal postprocess
- deterministic mock outputs
- overlay rendering
- inference event output
