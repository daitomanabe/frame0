# FRAME0 Schema Reference

FRAME0 publishes JSON Schemas for manifests, packets, runtime state, and addon
metadata. The schemas live in `schemas/` and are embedded into
`crates/frame0_schema` for CLI export.

## Commands

```bash
frame0 schema list --json
frame0 schema export scene --json
frame0 schema export plugin --json
frame0 schema export all --json
```

`schema export all` returns an object keyed by schema name. Use this when
generating validators or binding code for tools.

## Schema Families

| Family | Schemas |
| --- | --- |
| Scene and graph | `scene`, `graph`, `resource`, `runtime_snapshot` |
| Plugins and addons | `plugin`, `addon`, `extension`, `permission`, `capability` |
| Runtime packets | `frame_packet`, `audio_packet`, `event_packet`, `inference_packet` |
| Creative control | `parameter`, `automation`, `input_event`, `timeline`, `visual_node`, `operator_network`, `media_asset` |
| ML | `ml_model`, `inference_packet` |
| Diagnostics | `error` |

## Validation Expectations

External tools should validate against schemas before invoking the runtime.
Runtime commands still perform their own validation and return diagnostics.

Recommended validation loop:

```bash
frame0 inspect path/to/scene.yaml --json
frame0 graph path/to/scene.yaml --json
frame0 run path/to/scene.yaml --dry-run --json
```

## Versioning

The current schema set is v0.1 scaffold quality. Schema names are stable enough
for examples, tests, addon skeletons, and AI tooling. Breaking changes should be
paired with:

- schema updates
- documentation updates
- example updates
- CLI integration tests
- migration notes when existing examples change shape
