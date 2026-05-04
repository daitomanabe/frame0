# Depth Point Cloud

Depth-camera scene manifest that converts synchronized color and depth frames
into a filtered point cloud, particles, and a shaded 3D preview.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/depth_pointcloud/scene.yaml --json
cargo run -p frame0_cli -- graph examples/depth_pointcloud/scene.yaml --json
```
