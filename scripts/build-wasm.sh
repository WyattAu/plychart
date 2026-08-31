#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "==> Building plychart for web target (wasm-pack)..."
wasm-pack build --target web --out-dir pkg crates/plychart

echo "==> Building plychart for nodejs target (wasm-pack)..."
wasm-pack build --target nodejs --out-dir pkg crates/plychart

echo "==> wasm-pack builds complete. Output in pkg/"