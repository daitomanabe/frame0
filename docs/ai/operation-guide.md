# FRAME0 AI Agent Operation Guide

This guide describes how an AI agent should use FRAME0 without relying on
implicit GUI state, guessed hardware names, or human-only logs. FRAME0 is
designed as a CLI-first, JSON-readable creative runtime. An agent should treat
the command line, schemas, scene manifests, runtime snapshots, and NDJSON event
streams as the source of truth.

## 1. Operating Principles

Use the same commands that a human developer uses, but prefer machine-readable
outputs:

1. Discover available commands and contracts before editing.
2. Read or generate scene manifests as declarative inputs.
3. Validate scenes before running them.
4. Inspect the graph before assuming node order or resource references.
5. Query device and plugin capabilities before selecting them.
6. Run dry-run checks before execution.
7. During execution, consume NDJSON events instead of plain text logs.
8. When failures occur, parse diagnostics, then call the explanation and fix
   suggestion tools.

Priority order for evidence:

1. JSON and NDJSON command output
2. JSON Schemas exported by `frame0 schema`
3. Scene, plugin, and addon manifests
4. Documentation in `docs/`
5. Plain text terminal output

Plain text is useful for humans, but it is not the stable automation contract.

## 2. First Commands For Any Agent

Run these commands before making assumptions about the local FRAME0 build:

```bash
frame0 --version
frame0 doctor --json
frame0 docs index --json
frame0 docs examples --json
frame0 schema list --json
frame0 examples list --json
```

Use `frame0 docs index --json` as the machine-readable documentation map. Use
`frame0 docs examples --json` and `frame0 examples list --json` to choose a
known scene instead of guessing paths.

When a schema is needed, export it directly:

```bash
frame0 schema export scene --json
frame0 schema export all --json
```

## 3. Golden Scene Loop

The safe AI loop for creating, modifying, or executing a scene is:

```bash
frame0 inspect examples/hello_shader/scene.yaml --json
frame0 graph examples/hello_shader/scene.yaml --json
frame0 resources list --scene examples/hello_shader/scene.yaml --json
frame0 devices list --json
frame0 snapshot runtime --scene examples/hello_shader/scene.yaml --json
frame0 run examples/hello_shader/scene.yaml --dry-run --json
frame0 run examples/hello_shader/scene.yaml --events ndjson --frames 3
```

Interpretation rules:

- `inspect` validates the scene manifest and returns diagnostics.
- `graph` shows nodes, references, validation errors, cycles, and topological
  order.
- `resources list` shows the runtime resources derived from a scene.
- `devices list` is required before selecting or referring to a device ID.
- `snapshot runtime` captures the expected runtime state before execution.
- `run --dry-run --json` checks execution setup without consuming a live run.
- `run --events ndjson` is the primary execution stream for agents.

Do not continue to execution if `inspect`, `graph`, or `run --dry-run --json`
reports blocking diagnostics.

## 4. Reading Command Output

For JSON outputs, first check:

- `ok`
- `diagnostics`
- `errors`
- `warnings`
- `scene`
- `graph`
- `resources`
- `devices`
- `events`

For NDJSON outputs, parse one JSON object per line. Do not assume all events
share the same payload. Store the line number or event index when reporting a
failure so the execution can be reproduced.

Recommended event handling:

1. Ignore blank lines.
2. Parse each line as a complete JSON object.
3. Treat malformed JSON as a tooling failure.
4. Group events by type and timestamp.
5. Preserve the raw event object in regression artifacts when possible.

## 5. Scene Authoring Rules

When an AI agent writes or edits a scene:

- Generate against the exported scene schema, not memory.
- Keep node IDs stable unless the user asked for a rename.
- Prefer existing example patterns under `examples/`.
- Use explicit resource references.
- Use explicit clock policy values when timing matters.
- Use explicit permissions when a device, plugin, extension, or model is needed.
- Keep vendor SDK handles out of the scene manifest.
- Keep native OS extension bundle internals out of core scene nodes.
- Keep model runtime handles out of core scene nodes.
- Use relative repository paths for example assets when possible.
- Avoid private absolute paths and machine-specific usernames.

Agents must not:

- Infer device IDs by display name.
- Depend on GUI state.
- Depend on a window being visible.
- Assume 60 fps without reading the clock policy.
- Treat plain text logs as the source of truth when JSON or NDJSON exists.
- Write C++ ABI types, Core ML objects, Metal handles, Core Media I/O objects,
  AUv3 internals, or vendor SDK structs into core manifests.

## 6. Patching Scenes

Use `frame0 scene patch` for JSON merge-patch style changes instead of ad hoc
string edits when possible.

```bash
frame0 scene patch examples/hello_shader/scene.yaml patch.json --out /tmp/scene.yaml --json
frame0 inspect /tmp/scene.yaml --json
frame0 graph /tmp/scene.yaml --json
frame0 run /tmp/scene.yaml --dry-run --json
```

Patch workflow:

1. Inspect the original scene.
2. Build a small patch file.
3. Write patched output to a new file with `--out`.
4. Inspect and graph the patched scene.
5. Dry-run the patched scene.
6. Only replace the original scene after validation passes.

## 7. Examples Workflow

Examples are the safest way for an AI agent to verify FRAME0 behavior because
they are CLI-testable and deterministic.

List examples:

```bash
frame0 examples list --json
```

Run a named example:

```bash
frame0 examples run audio_visual_sync --frames 4
```

Launch a single example with artifacts:

```bash
frame0 examples launch projection_mapping --frames 120 --out runs/examples/projection_mapping --json
```

Launch every example:

```bash
frame0 examples launch-all --frames 24 --out runs/examples --json
```

Generated launch artifacts:

- `preview.html`: static visual preview for quick inspection
- `launch.json`: structured launch metadata
- `events.ndjson`: runtime event stream
- `frames.json`: deterministic frame report
- `index.html`: generated by `launch-all`

Read structured logs from a launch directory:

```bash
frame0 logs --root runs/examples --tail 5 --json
```

Agents should inspect these artifacts instead of claiming that a GUI, Metal
window, Core ML model, or macOS extension bundle was executed unless the
specific native command for that path was actually run.

## 8. Failure Handling

When a command fails:

1. Preserve the command, arguments, working directory, and JSON output.
2. Read `diagnostics`, `errors`, and `warnings`.
3. If an error JSON file exists, explain it:

   ```bash
   frame0 explain error error.json --json
   ```

4. Ask FRAME0 for a suggested fix:

   ```bash
   frame0 suggest fix examples/hello_shader/scene.yaml --json
   ```

5. Apply the smallest scene or manifest patch that addresses the diagnostic.
6. Re-run `inspect`, `graph`, and `run --dry-run --json`.

Do not hide warnings. Warnings can describe capability mismatches, missing
permissions, fallback paths, or mock-only behavior.

## 9. Native Plugins And Extensions

FRAME0 keeps native SDKs and OS extension details behind adapter boundaries.
AI agents should reason about native integrations through plugin manifests,
capabilities, packets, events, and schemas.

Plugin commands:

```bash
frame0 plugins list --json
frame0 plugins verify plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- inspect plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- enumerate-devices plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
```

Supervision smoke test:

```bash
cargo run -p frame0_plugin_host -- supervise plugins/mock/plugin.yaml --max-restarts 1 --crash-first --json
```

Rules for native paths:

- The C ABI is the stable external boundary.
- C++ may be used inside adapters, but it is not the public ABI boundary.
- Core Media I/O Camera Extension, AUv3, Syphon, Core ML, Metal, AVFoundation,
  and vendor SDK specifics belong in adapters, extension bundles, or plugin
  manifests, not in core scene contracts.
- Stub plugin manifests prove contract shape; they do not prove that a signed
  macOS extension bundle was installed or activated.

## 10. Machine Learning Workflow

Native ML is represented as model capability metadata plus inference packets.
Use mock ML commands for deterministic contract testing:

```bash
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
frame0 inspect examples/native_ml/scene.yaml --json
frame0 inspect examples/coreml_style_transfer/scene.yaml --json
```

ML agent rules:

- Treat Core ML, MPSGraph, and ANE execution as adapter implementation details.
- Do not put model runtime objects into scene YAML.
- Record model IDs, input/output tensor contracts, labels, checksums, and
  timestamps as structured metadata.
- Validate timestamp alignment when ML output is used with video or audio.
- Prefer deterministic mock inference when testing graph contracts.

## 11. Apple Native Feature Workflow

Apple native examples currently verify FRAME0 contracts and graph/resource
behavior. They should be described precisely:

```bash
frame0 inspect examples/camera_extension_output/scene.yaml --json
frame0 inspect examples/auv3_audio_unit/scene.yaml --json
frame0 inspect examples/extension_multi_output/scene.yaml --json
frame0 plugins verify plugins/camera_extension_stub/plugin.yaml --json
frame0 plugins verify plugins/audio_unit_stub/plugin.yaml --json
frame0 plugins verify plugins/syphon_stub/plugin.yaml --json
```

Agent language should distinguish between:

- Contract verified: schema, manifest, graph, permissions, and resource shape
  are valid.
- Adapter smoke tested: plugin host loaded an adapter and exchanged structured
  data.
- Native runtime executed: a real signed extension, real device, real Metal
  command queue, real Core ML model, or real CoreAudio path was launched.

Do not claim the third state unless the exact native runtime path has been
implemented and executed.

## 12. Addon And External Authoring

AI agents can scaffold third-party addon and external packages:

```bash
frame0 new my_addon --kind addon-rust
scripts/verify_addon_registry.sh
```

Use these docs when authoring public extensions:

- `docs/addons/authoring-guide.md`
- `docs/addons/registry.md`
- `docs/addons/verification.md`
- `templates/addon-rust/`
- `templates/external-c/`
- `templates/external-cpp/`
- `native/frame0_external_c_api/`

Addon rules:

- Keep package metadata explicit.
- Keep capabilities machine-readable.
- Include a small example scene or manifest.
- Include smoke-test commands in README files.
- Do not depend on private local paths.
- Do not assume an addon has access to core internals.

## 13. Verification Before Reporting Completion

For documentation-only changes, run:

```bash
cargo fmt --all -- --check
cargo test --all
```

For scene, example, addon, or schema changes, also run:

```bash
scripts/verify_examples.sh
scripts/verify_addon_registry.sh
frame0 examples launch-all --frames 2 --out /tmp/frame0-example-launch-all --json
frame0 logs --root /tmp/frame0-example-launch-all --tail 2 --json
```

For plugin or native adapter changes, also run the relevant host command:

```bash
cargo build -p frame0_mock_sdk
cargo run -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
```

For ML changes:

```bash
cargo build -p frame0_mock_ml
cargo run -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
```

## 14. Minimal Agent Checklist

Use this checklist before changing or running a scene:

- [ ] Run `frame0 doctor --json`.
- [ ] Read `frame0 docs index --json`.
- [ ] Read the exported schema for the object being edited.
- [ ] Inspect the scene with `frame0 inspect <scene> --json`.
- [ ] Inspect the graph with `frame0 graph <scene> --json`.
- [ ] Query devices with `frame0 devices list --json` if hardware is involved.
- [ ] Query plugins with `frame0 plugins list --json` or `plugins verify` if a
      plugin is involved.
- [ ] Run `frame0 run <scene> --dry-run --json`.
- [ ] Execute with `frame0 run <scene> --events ndjson --frames <n>` or
      `frame0 examples launch`.
- [ ] Preserve artifacts and structured logs.
- [ ] Explain and fix diagnostics before reporting success.

## 15. Safe Language For Reports

Use precise wording:

- "The scene contract validates."
- "The graph is acyclic and topologically ordered."
- "The dry run passes."
- "The mock runtime emitted NDJSON events."
- "The launch command wrote preview and event artifacts."
- "The plugin manifest verifies."
- "The plugin host smoke test passed."

Avoid overclaiming:

- Do not say "the camera extension runs" when only the stub manifest verifies.
- Do not say "Core ML inference ran" when only mock ML inference ran.
- Do not say "Metal rendered a window" when only a headless mock render report
  was generated.
- Do not say "audio hardware was captured" when only the audio graph contract
  was validated.

The goal is reproducible automation: every statement should map back to a
command, artifact, schema, or event stream.
