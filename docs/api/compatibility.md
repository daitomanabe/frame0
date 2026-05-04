# FRAME0 API Compatibility Notes

FRAME0 v0.1 is a scaffold, but public tooling should still treat the documented
API surfaces as contracts. This file defines how changes should be made without
surprising addon authors, AI agents, or native adapter maintainers.

## Public Contract Surfaces

The public compatibility boundary includes:

- CLI JSON and NDJSON output
- YAML/JSON scene manifests
- JSON Schemas under `schemas/`
- native C ABI headers under `native/frame0_*_c_api/`
- plugin and addon manifests
- documented example scene structure

Rust crate internals are not public unless explicitly documented in
`docs/api/`.

## Change Classes

| Class | Examples | Required action |
| --- | --- | --- |
| Patch-compatible | Add optional JSON fields, add examples, clarify docs | Update docs and tests |
| Minor-compatible | Add CLI subcommand, add schema, add optional manifest section | Update docs, tests, and docs index |
| Breaking | Rename schema field, remove CLI JSON key, change ABI struct layout | Add migration notes and update all examples |

## Schema Change Rules

1. Prefer additive optional fields.
2. Keep existing enum values valid unless there is a strong runtime reason.
3. Do not remove a schema without replacing it in `schema_names()`.
4. Update `schemas/*.schema.json`, `crates/frame0_schema`, examples, docs, and
   integration tests in the same change.
5. Run:

```bash
cargo run -p frame0_cli -- schema export all --json
scripts/verify_examples.sh
cargo test --all
```

## CLI Change Rules

CLI output intended for automation must remain machine-readable. When changing a
JSON output shape:

- preserve existing keys where possible
- add new keys instead of repurposing old keys
- keep NDJSON one event per line
- update `docs/api/reference.md`
- update `frame0 docs index --json` when a new public command appears
- add or update CLI integration tests

## Native ABI Change Rules

Native C ABI changes are the highest-risk compatibility surface.

Allowed without an ABI version bump:

- adding new functions behind optional symbol lookup
- adding new JSON control messages
- extending reserved fields that already exist for extension

Requires an ABI version bump:

- changing struct layout
- changing function signatures
- changing ownership or lifetime rules
- changing packet memory layout

Do not expose C++ ABI types, Objective-C objects, Swift objects, STL containers,
exceptions, or vendor SDK handles across FRAME0 native boundaries.

## Deprecation Policy

When a field, command, schema, or ABI function must be replaced:

1. Add the replacement first.
2. Keep the old path working for at least one documented transition window.
3. Emit a diagnostic or documentation warning.
4. Update examples to the new path.
5. Remove the old path only in a clearly marked breaking change.

## Migration Checklist

- [ ] Update affected schemas and schema exports.
- [ ] Update all examples that use the old shape.
- [ ] Update API docs, manual, README, and CLI reference.
- [ ] Update `frame0 docs index --json` if public docs or commands changed.
- [ ] Run `scripts/verify_examples.sh`.
- [ ] Run `cargo test --all`.
- [ ] Include migration notes in the commit or release notes.
