# audio_visual_sync

This scene connects audio analysis to visual state: beat phase, onsets, FFT bands, and envelope are mapped into particles, instancing, post-processing, timeline transport, and deterministic replay.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/audio_visual_sync/scene.yaml --json
cargo run -p frame0_cli -- graph examples/audio_visual_sync/scene.yaml --json
```
