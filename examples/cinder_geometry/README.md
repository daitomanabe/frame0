# cinder_geometry

This scene maps Cinder-style concepts to FRAME0 graph resources:

- perspective camera
- procedural mesh
- PBR material parameters
- draw mesh node
- time-driven transform

Run:

```bash
cargo run -p frame0_cli -- inspect examples/cinder_geometry/scene.yaml --json
cargo run -p frame0_cli -- graph examples/cinder_geometry/scene.yaml --json
```

