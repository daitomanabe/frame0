# audio_pipeline

This scene demonstrates audio-only routing: mock microphone input, oscillator, noise generator, envelope follower, FFT, mixer, recorder, meter, and output.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/audio_pipeline/scene.yaml --json
cargo run -p frame0_cli -- graph examples/audio_pipeline/scene.yaml --json
```
