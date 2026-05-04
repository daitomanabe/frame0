# Shader And Post-Processing Examples

FRAME0 models post-processing as explicit render nodes. Each pass names its input resources, output resource, shader path, and parameters.

The sample scene is:

```text
examples/shader_post_processing/scene.yaml
```

A richer camera/audio-reactive shader contract is also available:

```text
examples/analog_filter/scene.yaml
```

It models the `analog-filter` Swift/Metal app as FRAME0 nodes: camera input,
microphone metering, analog TV/VHS/CRT/RF artifact parameters, temporal
feedback, presets, adaptive render scale, and present/capture outputs.

Pass order:

1. `source`: generate the base texture.
2. `luminance_extract`: isolate bright regions.
3. `blur_horizontal`: horizontal Gaussian blur.
4. `blur_vertical`: vertical Gaussian blur.
5. `color_grade`: combine source/bloom and apply LUT-style controls.
6. `feedback`: feed previous frame history into trails.
7. `final_composite`: mix grade and trails, then output preview/capture.

Verify:

```bash
cargo run -p frame0_cli -- inspect examples/shader_post_processing/scene.yaml --json
cargo run -p frame0_cli -- graph examples/shader_post_processing/scene.yaml --json
cargo run -p frame0_cli -- inspect examples/analog_filter/scene.yaml --json
cargo run -p frame0_cli -- graph examples/analog_filter/scene.yaml --json
```
