# FRAME0 Addon Registry

This directory is the public convention for third-party addon packages.

Registry file:

```yaml
registry:
  version: "0.1"
  root: addons
  entries:
    - id: org.example.my_addon
      path: addons/org.example.my_addon/addon.yaml
```

Each addon package should include:

- `addon.yaml` matching `schemas/addon.schema.json`
- at least one CLI-verifiable example
- tests or smoke commands that can run without private machine paths
- README documentation for capabilities, permissions, and supported platforms

FRAME0 addon IDs should use reverse-DNS naming. Paths inside manifests should be relative to the addon package root unless the schema explicitly says otherwise.

Verify the registry with:

```bash
scripts/verify_addon_registry.sh
```
