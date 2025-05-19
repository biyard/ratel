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

echo "Copying libratel_lib.a → libapp.a for Xcode..."

ARCHS=("x86_64")
CONFIGURATION="debug"

for ARCH in "${ARCHS[@]}"; do
  SRC="$HOME/Projects/ratel/target/${ARCH}-apple-ios/${CONFIGURATION}/libratel_lib.a"
  DEST="$HOME/Projects/ratel/packages/app/gen/apple/Externals/${ARCH}/${CONFIGURATION}/libapp.a"

  ls $HOME/Projects/ratel/target/x86_64-apple-ios/debug/
  if [ -f "$SRC" ]; then
    mkdir -p "$(dirname "$DEST")"
    cp "$SRC" "$DEST"
    echo "Copied: $SRC → $DEST"
  else
    echo "Missing: $SRC"
  fi
done

echo "📦 Installing CocoaPods (if needed)..."
cd "$HOME/Projects/ratel/packages/app/gen/apple"
pod install

# echo "🚀 Building and launching iOS Simulator app..."
# xcodebuild -workspace ratel.xcworkspace \
#   -scheme ratel_iOS \
#   -sdk iphonesimulator \
#   -destination "platform=iOS Simulator,name=iPhone 16" \
#   -configuration $CONFIGURATION \
#   build

echo "Launching Tauri ios Dev..."
pwd
ls -l
cd ../../../ && cd app && cargo tauri ios dev