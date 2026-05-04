# FRAME0 API Documentation

This directory collects the public contracts that tools, native adapters, AI
agents, and third-party addons should depend on.

## Public API Surfaces

| Surface | Stability | Entry point |
| --- | --- | --- |
| CLI API | v0.1 scaffold | `frame0 --help`, `docs/cli-reference.md` |
| Scene manifest API | v0 schema | `schemas/scene.schema.json`, `crates/frame0_schema` |
| Runtime event API | v0 schema | `frame0 run <scene> --events ndjson` |
| Runtime snapshot API | v0 schema | `frame0 snapshot runtime --json` |
| Resource API | v0 schema | `frame0 resources list --json`, `frame0 resource get --json` |
| Plugin manifest API | v0 schema | `schemas/plugin.schema.json` |
| Native plugin ABI | v0 C ABI | `native/frame0_plugin_c_api/frame0_plugin_api.h` |
| Native external ABI | v0 C ABI skeleton | `native/frame0_external_c_api/frame0_external_api.h` |
| Addon package API | v0 convention | `docs/addons/authoring-guide.md` |
| ML adapter API | v0 schema + C ABI control JSON | `docs/ml/native-ml-adapter.md` |

## Reading Order

1. Start with [Reference](reference.md) for the practical command and data
   contracts.
2. Use [Schemas](schemas.md) when implementing validators, code generators, or
   external tools.
3. Use [User Manual](../manual/user-manual.md) when running the scaffold as a
   creative runtime.
4. Use the native docs under `docs/native/` when implementing C/C++ adapters.

## Compatibility Rule

The stable boundary is JSON, NDJSON, YAML manifests, JSON Schema, and C ABI
headers. Rust crate internals are implementation details unless a document in
`docs/api/` explicitly marks them as public.
