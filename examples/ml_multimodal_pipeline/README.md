# ml_multimodal_pipeline

This scene demonstrates a richer ML pipeline: model registry, video preprocessing, audio preprocessing, native mock inference, multimodal postprocess, deterministic mock outputs, overlay rendering, and inference event output.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/ml_multimodal_pipeline/scene.yaml --json
cargo run -p frame0_cli -- graph examples/ml_multimodal_pipeline/scene.yaml --json
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
```
