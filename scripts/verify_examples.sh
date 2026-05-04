#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
cd "$repo_root"

if [ ! -d examples ]; then
  echo "examples directory not found" >&2
  exit 2
fi

count=0
while IFS= read -r scene; do
  [ -n "$scene" ] || continue
  echo "inspect $scene"
  output="$(cargo run -q -p frame0_cli -- inspect "$scene" --json)"
  if ! printf '%s\n' "$output" | rg -q '"ok": true'; then
    echo "example inspect reported diagnostics: $scene" >&2
    printf '%s\n' "$output" >&2
    exit 1
  fi
  count=$((count + 1))
done < <(find examples -mindepth 2 -maxdepth 2 -name scene.yaml | sort)

if [ "$count" -eq 0 ]; then
  echo "no example scenes found" >&2
  exit 1
fi

cargo run -q -p frame0_cli -- docs examples --json >/dev/null

echo "examples ok: $count scenes"
