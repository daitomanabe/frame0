# Addon Registry Convention

FRAME0 discovers third-party addon packages through an explicit registry file. The registry is intentionally simple so it works in source repositories, vendored folders, and package-manager exports.

## Registry Shape

```yaml
registry:
  version: "0.1"
  root: addons
  entries:
    - id: org.example.visual_nodes
      path: addons/org.example.visual_nodes/addon.yaml
```

## Package Rules

- `id` must be stable and reverse-DNS style.
- `path` points to an `addon.yaml` file.
- `addon.yaml` must match `schemas/addon.schema.json`.
- An addon must declare capabilities and permissions before FRAME0 loads code or assets.
- Examples and tests must use repo-relative paths.
- Native code must use a plugin/external boundary rather than linking vendor SDK handles into FRAME0 Core.

## Verification Flow

```bash
cargo run -p frame0_cli -- schema export addon --json
cargo run -p frame0_cli -- plugins list --json
scripts/verify_addon_registry.sh
```

The registry helper checks that each registered manifest exists, matches its registry id, and passes the package verification flow.
