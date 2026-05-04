# shader_post_processing

This scene demonstrates a shader and post-processing stack with source render, luminance extraction, separable blur, LUT color grade, feedback trails, final composite, and capture.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/shader_post_processing/scene.yaml --json
cargo run -p frame0_cli -- graph examples/shader_post_processing/scene.yaml --json
```
