#!/usr/bin/env bash
set -euo pipefail

CHECKLIST="${1:-docs/creative/FEATURE_CHECKLIST.md}"

if [ ! -f "${CHECKLIST}" ]; then
  echo "checklist not found: ${CHECKLIST}" >&2
  exit 1
fi

echo "Next unchecked creative feature:"
grep -n '^- \[ \]' "${CHECKLIST}" | head -1 || {
  echo "all checklist items are complete"
  exit 0
}

echo
echo "Loop contract:"
echo "1. Implement the feature above."
echo "2. Mark only that checkbox complete."
echo "3. Run: cargo fmt --all -- --check && cargo test --all"
echo "4. Run privacy/copyright scans."
echo "5. Commit and push."

