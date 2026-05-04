# FRAME0 AI Operation Guide

AI agents should treat FRAME0 as an inspectable execution environment.

Required flow:

1. Write or patch a scene manifest.
2. Run `frame0 inspect <scene> --json`.
3. Run `frame0 graph <scene> --json`.
4. Run `frame0 devices list --json` before assuming device IDs.
5. Run `frame0 run <scene> --dry-run --json`.
6. For execution, read `frame0 run <scene> --events ndjson`.
7. Use `frame0 explain error <error.json> --json` and `frame0 suggest fix <scene-or-error> --json`.

Agents must not:

- infer device IDs by name
- depend on GUI state
- write vendor SDK types into core manifests
- assume 60 fps without reading clock policy
- treat plain text logs as the source of truth when JSON/NDJSON exists

