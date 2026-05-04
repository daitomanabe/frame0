# operator_network

This scene captures a TouchDesigner-style operator network as inspectable FRAME0 manifest data.

Operator families:

- TOP: texture operators
- CHOP: channel operators
- DAT: table/data operators
- SOP/MAT/COMP: reserved for geometry, materials, and subgraphs

Run:

```bash
cargo run -p frame0_cli -- inspect examples/operator_network/scene.yaml --json
cargo run -p frame0_cli -- schema export operator_network --json
```

