#!/usr/bin/env bash
# Build plychart WASM + JS glue, verify exports, stamp pkg metadata.
#
# Steps:
#   1. wasm-pack build (target web, release)
#   2. Guard: pkg/plychart.js must export >= 20 functions. A build that
#      silently produces 0 exports (e.g. wasm-bindgen schema mismatch)
#      fails here instead of shipping a dead module.
#   3. Copy package.json + types.d.ts templates into pkg/ for npm use.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PKG="$ROOT/crates/plychart/pkg"

cd "$ROOT"
wasm-pack build --target web --release --out-dir pkg crates/plychart

EXPORTS=$(grep -c '^export function' "$PKG/plychart.js" || true)
if [ "$EXPORTS" -lt 20 ]; then
  echo "FATAL: plychart.js has only $EXPORTS top-level exports (expected >= 20)." >&2
  echo "The wasm-bindgen schema likely mismatched the crate. Do not ship this build." >&2
  exit 1
fi
echo "export guard OK: $EXPORTS functions"

# npm metadata + ergonomic types (templates live in scripts/templates/)
cp "$SCRIPT_DIR/templates/package.json" "$PKG/package.json"
cp "$SCRIPT_DIR/templates/types.d.ts" "$PKG/types.d.ts"

# Content hash for cache busting: consumers embed this in their loader's
# query string so a replaced WASM binary is never served stale.
HASH=$(sha256sum "$PKG/plychart_bg.wasm" | cut -c1-8)
echo "p$HASH" > "$PKG/version.txt"
echo "version: p$HASH"

echo "pkg ready: $PKG"
