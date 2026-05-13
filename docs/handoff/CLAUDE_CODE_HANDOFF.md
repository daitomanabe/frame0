# FRAME0 Claude Code Handoff

Date: 2026-05-13 JST
Repository: `frame0`
Current known head when this handoff was written: `11fe2af`

This handoff is for continuing FRAME0 development in Claude Code. Treat the
repository as the source of truth and re-check `git status`, `git log`, and the
docs before editing.

## Mission

Move FRAME0 from a CLI-verifiable contract scaffold toward a native-capable
runtime while preserving the current strengths:

- CLI-first operation
- JSON/NDJSON machine-readable output
- scene/schema/plugin contracts before native framework binding
- deterministic examples and tests
- adapter boundaries for native SDKs, Apple frameworks, ML, and extensions

Do not jump straight to GUI/native execution. The next work should make native
execution easier by improving runtime/plugin transport, resource mutation,
events, and validation.

## Current State

Implemented:

- CLI commands: `new`, `inspect`, `graph`, `run`, `render`, `devices`,
  `plugins`, `resources`, `resource`, `doctor`, `docs`, `schema`, `snapshot`,
  `explain`, `suggest`, `scene patch`, `scene controls`, `examples`,
  `benchmark`, and `logs`
- 32 CLI-verifiable examples
- `analog_filter` example contract with camera, microphone, controls, presets,
  uniforms, artifact groups, feedback, adaptive render scale, and launch
  artifacts
- `scene controls` extraction for parameter-heavy scenes
- Plugin host smoke paths, stream tests, crash supervision, and mock ML commands
- Addon skeleton generator and addon registry validation
- Public docs, API docs, user manual, AI operation guide, and compatibility
  notes

Still stubbed:

- Real runtime-to-plugin-host IPC
- Real Metal command queue / shader compile / texture pool
- AVFoundation camera capture and CVPixelBuffer to Metal bridge
- CoreAudio input and real FFT
- Core ML / MPSGraph / ANE-backed model execution
- Signed Camera Extension / AUv3 bundles
- Real vendor SDK adapter sample

Read first:

```bash
git status --short --branch
git log --oneline --decorate -5
cargo run -q -p frame0_cli -- doctor --json
cargo run -q -p frame0_cli -- docs index --json
cargo run -q -p frame0_cli -- examples list --json
```

## Engineering Rules

- Keep changes small and reviewable.
- One implementation item per commit.
- Prefer schema/contract/test surfaces before native binding.
- Keep vendor SDK headers out of core crates.
- Keep C++ ABI out of public boundaries; use the C ABI.
- Do not put Core ML, Metal, Core Media I/O, AVFoundation, AUv3, or vendor SDK
  runtime objects into scene manifests.
- Do not infer device IDs by name; use `frame0 devices list --json`.
- Prefer JSON/NDJSON over plain text logs for automation.
- Preserve all existing examples and tests.

Before every commit/push:

```bash
cargo fmt --all -- --check
cargo test --all
scripts/verify_examples.sh
scripts/verify_addon_registry.sh
run the repository privacy scan used by the current maintainer
run the repository license-holder scan used by the current maintainer
```

Expected privacy scan result: no output, exit code 1.
Expected copyright scan result: only `LICENSE`.

## Priority List

### P0: Runtime And IPC

- [ ] Add structured IPC between runtime and plugin host.
- [ ] Forward stream packets from plugin host smoke tests into runtime
      events/resources.
- [ ] Add runtime resource mutation transport for `frame0 resource set`.
- [ ] Add runtime event/resource state transitions with tests.

This should be the first active work area. It enables the later Metal,
CoreAudio, Camera Extension, and ML work.

### P1: Rendering And Metal

- [ ] Add a real Metal command queue behind `frame0_render`.
- [ ] Add shader compilation diagnostics and source mapping.
- [ ] Add texture pool lifecycle metrics.
- [ ] Add GPU timing packets to `benchmark` and launch artifacts.
- [ ] Strengthen `analog_filter` shader/uniform validation.

### P1: Native macOS Media

- [ ] Add AVFoundation camera discovery/capture adapter.
- [ ] Add CVPixelBuffer to CVMetalTexture bridge.
- [ ] Add CoreAudio input adapter.
- [ ] Add real FFT/analyzer node.
- [ ] Add audio clock synchronization tests.
- [ ] Add audio output device selection.

### P1: analog_filter Follow-Up

- [ ] Add FRAME0-side shader asset skeleton for `analog_filter`.
- [ ] Generate a uniform-buffer layout report from `scene controls`.
- [ ] Add preset diff and missing-uniform checks to `scene controls`.
- [ ] Add preset tables to launch previews.
- [ ] Add deterministic replay from mock camera/audio streams through the
      `analog_filter` graph.

### P2: Apple Native Extensions

- [ ] Add signed Core Media I/O Camera Extension scaffold.
- [ ] Add signed AUv3 extension scaffold.
- [ ] Add entitlement / codesign / install / uninstall scripts.
- [ ] Connect Camera Extension output to runtime events and launch artifacts.
- [ ] Add AUv3 parameter bridge smoke test.

### P2: Machine Learning

- [ ] Add Core ML model loading behind the native ML adapter.
- [ ] Add MPSGraph / ANE execution path behind the same inference packet
      contract.
- [ ] Add model asset discovery and checksum validation.
- [ ] Add video/audio/ML timestamp alignment tests.
- [ ] Strengthen multimodal batching examples.

### P2: Addon And External Ecosystem

- [ ] Add external C/C++ skeleton smoke tests.
- [ ] Add sample addon release checklist.
- [ ] Add richer addon package validation diagnostics.
- [ ] Standardize addon example launch artifacts.
- [ ] Expand third-party template README files.

### P3: Observability And QA

- [ ] Add structured trace IDs to runtime events.
- [ ] Add JSON schema for benchmark results.
- [ ] Add diagnostics summaries to launch artifacts.
- [ ] Add example regression snapshot comparison in CI.
- [ ] Extend `suggest fix` to understand controls, presets, and uniforms.

### P3: Documentation

- [ ] Native runtime implementation guide.
- [ ] Metal backend implementation guide.
- [ ] Plugin IPC protocol document.
- [ ] Apple extension signing manual.
- [ ] `analog_filter` porting manual.
- [ ] AI-agent command reading path update after IPC is implemented.

## Recommended First Task

Start with structured IPC, but keep the first commit narrow.

Goal for first commit:

- Define a runtime/plugin IPC packet model.
- Add JSON schema or Rust data structures for request/response/event packets.
- Add deterministic tests.
- Add one plugin-host command or helper that emits/accepts the IPC packet shape.
- Do not attempt real async process transport yet unless the packet contract is
  already solid.

Suggested files to inspect:

```bash
apps/frame0_plugin_host/src/main.rs
apps/frame0_plugin_host/src/lib.rs
apps/frame0_plugin_host/tests/host.rs
apps/frame0_cli/src/main.rs
crates/frame0_core/src/lib.rs
crates/frame0_plugin_api/src/lib.rs
crates/frame0_schema/src/lib.rs
schemas/event_packet.schema.json
schemas/resource.schema.json
schemas/plugin.schema.json
```

Possible implementation shape:

- Add `schemas/ipc_packet.schema.json`.
- Add Rust packet structs in `crates/frame0_schema`.
- Include packet kinds such as:
  - `runtime.hello`
  - `plugin.open_stream`
  - `plugin.stream_packet`
  - `plugin.resource_update`
  - `plugin.error`
  - `runtime.shutdown`
- Add `frame0_plugin_host ipc-smoke plugins/mock/plugin.yaml --json`.
- Add tests that parse, serialize, and round-trip packet envelopes.
- Add docs in `docs/native/` or `docs/api/reference.md`.

Definition of done:

```bash
cargo fmt --all -- --check
cargo test --all
cargo run -q -p frame0_plugin_host -- ipc-smoke plugins/mock/plugin.yaml --json
cargo run -q -p frame0_cli -- docs index --json
scripts/verify_examples.sh
```

## Suggested Claude Code Prompt

Use this prompt when starting Claude Code:

```text
You are working in the FRAME0 repository. Read docs/handoff/CLAUDE_CODE_HANDOFF.md first, then inspect the current git status and implementation status. Implement the first P0 Runtime And IPC item as a small, reviewable change: define structured runtime/plugin IPC packet contracts and add deterministic tests, without binding real native frameworks yet. Preserve existing examples and JSON/NDJSON CLI behavior. Run cargo fmt --all -- --check, cargo test --all, scripts/verify_examples.sh, scripts/verify_addon_registry.sh, and privacy/copyright scans before committing. Commit one logical change and push.
```

## Useful Commands

Scene and graph:

```bash
cargo run -q -p frame0_cli -- inspect examples/analog_filter/scene.yaml --json
cargo run -q -p frame0_cli -- graph examples/analog_filter/scene.yaml --json
cargo run -q -p frame0_cli -- scene controls examples/analog_filter/scene.yaml --json
cargo run -q -p frame0_cli -- run examples/analog_filter/scene.yaml --dry-run --json
```

Launch artifacts:

```bash
cargo run -q -p frame0_cli -- examples launch analog_filter --frames 4 --out /tmp/frame0-analog-filter-launch --json
cargo run -q -p frame0_cli -- logs --root /tmp/frame0-analog-filter-launch --tail 5 --json
```

Plugin host:

```bash
cargo build -p frame0_mock_sdk
cargo run -q -p frame0_plugin_host -- smoke plugins/mock/plugin.yaml --json
cargo run -q -p frame0_plugin_host -- stream-test plugins/mock/plugin.yaml --json
cargo run -q -p frame0_plugin_host -- supervise plugins/mock/plugin.yaml --max-restarts 1 --crash-first --json
```

ML:

```bash
cargo build -p frame0_mock_ml
cargo run -q -p frame0_plugin_host -- ml-describe plugins/mock_ml/plugin.yaml --json
cargo run -q -p frame0_plugin_host -- ml-infer plugins/mock_ml/plugin.yaml --model mock_classifier --json
```

Docs and schemas:

```bash
cargo run -q -p frame0_cli -- docs index --json
cargo run -q -p frame0_cli -- schema list --json
cargo run -q -p frame0_cli -- schema export all --json
```

## Commit Guidance

Use short, concrete commits:

- `Add IPC packet schema`
- `Wire plugin stream packets into runtime events`
- `Add resource mutation transport`
- `Add Metal shader diagnostics`
- `Add CoreAudio input adapter skeleton`

Avoid large mixed commits that include unrelated docs, examples, native
scaffolds, and runtime changes together.

## Handoff Notes

The immediate blocker for native work is not missing examples. It is transport:
runtime and plugin host still do not have a structured IPC path that can carry
stream packets, resource updates, errors, and state transitions. Solve that
first, then build Metal/CoreAudio/AVFoundation/ML/extension work behind the same
contracts.
