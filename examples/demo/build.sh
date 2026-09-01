#!/usr/bin/env bash
# Build and serve plychart demo
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Building plychart WASM..."
cd "$ROOT_DIR"
wasm-pack build --target web --out-dir examples/demo/pkg

echo "Starting demo server on http://localhost:8080"
cd "$SCRIPT_DIR"
python3 -m http.server 8080
