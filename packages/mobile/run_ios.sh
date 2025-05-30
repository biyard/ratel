#!/bin/bash

PORT=1420
DX_OUTPUT_DIR="target/dx/mobile-ui/release/web/public"
TAURI_DIST_DIR="src-tauri/../dist"

pkill -f cargo
pkill -f dx
pkill -f tauri
rm -f ~/.cargo/.package-cache-lock
rm -f ~/.cargo/.crate-cache-lock

if lsof -i tcp:$PORT >/dev/null; then
  echo "Killing process on port $PORT..."
  kill -9 $(lsof -ti tcp:$PORT)
fi

echo "📦 Building static files with dx..."
dx build --release

echo "🧹 Syncing static files to Tauri dist..."
rm -rf "$TAURI_DIST_DIR"
mkdir -p "$TAURI_DIST_DIR"
cp -r "$DX_OUTPUT_DIR"/* "$TAURI_DIST_DIR"

echo "🚀 Launching Tauri iOS Dev..."
cargo tauri ios dev