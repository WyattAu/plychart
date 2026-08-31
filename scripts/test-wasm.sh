#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

BROWSER=""
if command -v google-chrome &>/dev/null || command -v chromium-browser &>/dev/null || command -v chromium &>/dev/null; then
  BROWSER="chrome"
elif command -v firefox &>/dev/null; then
  BROWSER="firefox"
else
  echo "WARNING: No supported browser found (chrome/firefox). Skipping wasm-pack tests."
  exit 0
fi

echo "==> Running wasm-pack tests with $BROWSER..."
wasm-pack test --headless "--$BROWSER" --workspace

echo "==> wasm-pack tests complete."