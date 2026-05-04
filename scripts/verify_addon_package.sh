#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: scripts/verify_addon_package.sh <path/to/addon.yaml>" >&2
  exit 2
fi

manifest="$1"
if [ ! -f "$manifest" ]; then
  echo "addon manifest not found: $manifest" >&2
  exit 2
fi

package_dir="$(dirname "$manifest")"

require_key() {
  key="$1"
  if ! awk -v key="$key" '$1 == key ":" { found = 1 } END { exit found ? 0 : 1 }' "$manifest"; then
    echo "missing required addon key: $key" >&2
    exit 1
  fi
}

require_key "id"
require_key "version"
require_key "kind"
require_key "capabilities"
require_key "entrypoints"
require_key "permissions"
require_key "examples"
require_key "tests"

examples="$(
  awk '
    /^[[:space:]]*examples:/ { in_examples = 1; next }
    /^[[:space:]]*[a-zA-Z_]+:/ && in_examples { in_examples = 0 }
    in_examples && /^[[:space:]]*-/ {
      sub(/^[[:space:]]*-[[:space:]]*/, "")
      gsub(/"/, "")
      print
    }
  ' "$manifest"
)"

if [ -z "$examples" ]; then
  echo "addon manifest has no examples" >&2
  exit 1
fi

while IFS= read -r example; do
  [ -n "$example" ] || continue
  scene="$package_dir/$example"
  if [ ! -f "$scene" ]; then
    echo "example scene not found: $scene" >&2
    exit 1
  fi
  echo "inspect $scene"
  cargo run -q -p frame0_cli -- inspect "$scene" --json >/dev/null
done <<< "$examples"

echo "addon package ok: $manifest"
