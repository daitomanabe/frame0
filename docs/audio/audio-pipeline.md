# Audio Pipeline Sample

FRAME0 audio examples use explicit resources for sample streams, analysis streams, meters, and file outputs.

The sample scene is:

```text
examples/audio_pipeline/scene.yaml
```

It covers:

- audio input contract
- oscillator generation
- noise burst generation
- envelope following
- FFT band analysis
- mixing and send metadata
- WAV recorder contract
- event output from meters

Verify:

```bash
cargo run -p frame0_cli -- inspect examples/audio_pipeline/scene.yaml --json
cargo run -p frame0_cli -- graph examples/audio_pipeline/scene.yaml --json
```
