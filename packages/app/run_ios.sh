#!/bin/bash

PORT=1420
DX_OUTPUT_DIR="../../target/dx/main-ui/release/web/public"
TAURI_DIST_DIR="../app/dist"

pkill -f cargo
pkill -f dx
pkill -f tauri
rm -f ~/.cargo/.package-cache-lock
rm -f ~/.cargo/.crate-cache-lock

if lsof -i tcp:$PORT >/dev/null; then
  echo "Killing process on port $PORT..."
  kill -9 $(lsof -ti tcp:$PORT)
fi

# echo "Building static files with dx..."
# source ~/.zshrc && envs_ratel && cd ../main-ui && ENV=dev make build.mobile

# echo "Syncing static files to Tauri dist..."
# rm -rf "$TAURI_DIST_DIR"
# mkdir -p "$TAURI_DIST_DIR"
# cp -r "$DX_OUTPUT_DIR"/* "$TAURI_DIST_DIR"

echo "Launching Tauri ios Dev..."
pwd
ls -l
cd .. && cd app && cargo tauri ios dev