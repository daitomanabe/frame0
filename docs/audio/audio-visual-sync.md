# Audio Visual Sync Sample

The audio-visual sample connects analysis outputs to visual parameters with explicit mapping nodes.

Scene:

```text
examples/audio_visual_sync/scene.yaml
```

Signal flow:

1. Audio input produces sample packets.
2. Beat tracker outputs phase and transport.
3. Onset detector emits transient events.
4. FFT and envelope nodes expose continuous controls.
5. Mapper converts audio features to visual parameter targets.
6. Particles, instancing, and post-processing consume mapped params.
7. Timeline/replay keeps deterministic event capture available.

Verify:

```bash
cargo run -p frame0_cli -- inspect examples/audio_visual_sync/scene.yaml --json
cargo run -p frame0_cli -- graph examples/audio_visual_sync/scene.yaml --json
```
