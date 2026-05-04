#!/usr/bin/env bash
set -euo pipefail

registry="${1:-addons/registry.yaml}"

if [ ! -f "$registry" ]; then
  echo "addon registry not found: $registry" >&2
  exit 2
fi

count=0
current_id=""

while IFS= read -r line; do
  case "$line" in
    "    - id:"*)
      current_id="${line#*id: }"
      ;;
    "      path:"*)
      manifest="${line#*path: }"
      if [ -z "$current_id" ]; then
        echo "registry entry path found before id: $manifest" >&2
        exit 1
      fi
      if [ ! -f "$manifest" ]; then
        echo "registered addon manifest not found: $manifest" >&2
        exit 1
      fi
      manifest_id="$(
        awk '$1 == "id:" { print $2; exit }' "$manifest"
      )"
      if [ "$manifest_id" != "$current_id" ]; then
        echo "registry id mismatch: $current_id != $manifest_id in $manifest" >&2
        exit 1
      fi
      scripts/verify_addon_package.sh "$manifest"
      count=$((count + 1))
      current_id=""
      ;;
  esac
done < "$registry"

if [ "$count" -eq 0 ]; then
  echo "addon registry has no entries: $registry" >&2
  exit 1
fi

echo "addon registry ok: $registry ($count entries)"
