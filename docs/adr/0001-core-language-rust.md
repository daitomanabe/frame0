# ADR 0001: Core Runtime Language

Status: accepted for v0.1 scaffold

FRAME0 Core uses Rust for schema handling, CLI, graph validation, resource registry, timebase, AI diagnostics, and supervisor-facing data models.

Rationale:

- Rust gives a stable C ABI integration story without exposing C++ ABI types.
- The CLI, schema validation, JSON/NDJSON output, and deterministic tests fit Rust well.
- Native Apple framework work can still be isolated behind Swift, Objective-C++, or C adapters.

Consequences:

- Core crates must not include vendor SDK headers.
- macOS-specific media and render bridges are adapter crates/processes, not core dependencies.
- Public plugin boundaries remain C ABI, not Rust ABI.

