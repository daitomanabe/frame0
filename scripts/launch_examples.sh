#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
cd "$repo_root"

frames="${1:-24}"
out="${2:-runs/examples}"

cargo run -q -p frame0_cli -- examples launch-all --frames "$frames" --out "$out" --json

if [ ! -f "$out/index.html" ]; then
  echo "launch index not found: $out/index.html" >&2
  exit 1
fi

echo "example launch index: $out/index.html"
