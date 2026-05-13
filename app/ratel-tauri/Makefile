# Build the dioxus-web bundle and assemble it into a Tauri Android APK.
#
# Inputs you may override:
#   ENV=local|dev|staging|prod   (default: local)
#   MOBILE_API_URL=https://...   (default: depends on ENV)
#   MODE=debug|release           (default: debug; release for store builds)
#   ANDROID_TARGET=aarch64-linux-android | x86_64-linux-android

ENV ?= local
MODE ?= debug
ANDROID_TARGET ?= aarch64-linux-android

ifeq ($(ENV),prod)
  MOBILE_API_URL ?= https://ratel.foundation
else ifeq ($(ENV),staging)
  MOBILE_API_URL ?= https://stg.ratel.foundation
else ifeq ($(ENV),dev)
  MOBILE_API_URL ?= https://dev.ratel.foundation
else
  # Android emulator alias for the host loopback. Override with the
  # workstation's LAN IP when targeting a physical device.
  MOBILE_API_URL ?= http://10.0.2.2:8000
endif

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
RUSTFLAGS ?= -D warnings

# Android toolchain — same paths the existing app/ratel/Makefile uses for the
# legacy dioxus-native build (matched here so a single env profile works for
# both). Override in CI or non-standard installs.
HOST_OS := $(shell uname -s | tr '[:upper:]' '[:lower:]')
ifeq ($(HOST_OS),darwin)
  ANDROID_HOME ?= /opt/homebrew/share/android-commandlinetools
  ANDROID_NDK_HOME ?= /opt/homebrew/share/android-ndk
  JAVA_HOME ?= /Library/Java/JavaVirtualMachines/temurin-21.jdk/Contents/Home
else
  ANDROID_HOME ?= /opt/android-sdk
  ANDROID_NDK_HOME ?= /opt/android-ndk
  JAVA_HOME ?= /usr/lib/jvm/java-21-openjdk
endif
NDK_HOME ?= $(ANDROID_NDK_HOME)

ANDROID_ENV = \
	ANDROID_HOME=$(ANDROID_HOME) \
	ANDROID_SDK_ROOT=$(ANDROID_HOME) \
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) \
	NDK_HOME=$(NDK_HOME) \
	JAVA_HOME=$(JAVA_HOME)

DIST_DIR := $(CURDIR)/dist
RATEL_DIR := $(CURDIR)/../ratel
# dx writes its output under the workspace root's target/dx, not app/ratel/target/dx,
# because the cargo workspace shares one target directory.
WORKSPACE_ROOT := $(CURDIR)/../..

.PHONY: help install-tauri-cli dx-build android-dev build-release install-debug clean

help:
	@echo "Targets:"
	@echo "  dx-build         — build the dioxus-web bundle and copy into ./dist"
	@echo "  android-dev      — cargo tauri android dev (live reload, requires dx serve)"
	@echo "  build-release    — full APK build: dx-build + cargo tauri android build"
	@echo "  install-debug    — build-release + adb install + launch on connected device"
	@echo "  clean            — remove ./dist and src-tauri/target"
	@echo
	@echo "Common overrides: ENV=local|dev|staging|prod, MOBILE_API_URL=...,"
	@echo "                  MODE=debug|release, ANDROID_TARGET=..."

install-tauri-cli:
	@if ! cargo tauri --version >/dev/null 2>&1; then \
		cargo install tauri-cli --version "^2.0.0" --locked; \
	fi

# Run `dx build` and copy the produced web bundle into ./dist for Tauri.
dx-build:
	@echo "Building dioxus-web bundle with MOBILE_API_URL=$(MOBILE_API_URL) ENV=$(ENV)"
	cd $(RATEL_DIR) && \
		MOBILE_API_URL=$(MOBILE_API_URL) \
		DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
		ENV=$(ENV) \
		RUSTFLAGS='$(RUSTFLAGS)' \
		dx build --web --release --features tauri-web --no-default-features
	@mkdir -p $(DIST_DIR)
	@# dx output location has drifted across versions. Locate the most-recently
	@# produced index.html under target/dx/**/release/** and rsync its directory
	@# tree into ./dist. Portable across macOS (BSD find) and Linux (GNU find).
	@INDEX=$$(find $(WORKSPACE_ROOT)/target/dx $(RATEL_DIR)/target/dx -name index.html -path "*release*" 2>/dev/null \
	         | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$INDEX" ]; then \
		echo "ERROR: could not locate dx-built index.html under target/dx"; \
		exit 1; \
	fi; \
	SRC=$$(dirname "$$INDEX"); \
	echo "Copying $$SRC -> $(DIST_DIR)"; \
	rsync -a --delete "$$SRC/" "$(DIST_DIR)/"

# Live dev — dioxus serves WASM on :8000, Tauri WebView points there via devUrl.
android-dev: install-tauri-cli
	@echo "Run 'cd ../ratel && DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) MOBILE_API_URL=$(MOBILE_API_URL) dx serve --features tauri-web --no-default-features --port 8000' in another terminal first."
	cd src-tauri && $(ANDROID_ENV) cargo tauri android dev --target $(ANDROID_TARGET)

build-release: install-tauri-cli dx-build
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(ANDROID_TARGET) $(if $(filter release,$(MODE)),--release,)
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*.apk" -newer Makefile 2>/dev/null

install-debug: build-release
	@APK=$$(find src-tauri/gen/android -name "app-*-debug.apk" 2>/dev/null \
	       | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$APK" ]; then echo "ERROR: no debug APK found under src-tauri/gen/android"; exit 1; fi; \
	echo "Installing $$APK"; \
	adb install -r "$$APK"; \
	adb shell am start -n "co.biyard.ratel/.MainActivity"

clean:
	rm -rf $(DIST_DIR)
	cd src-tauri && cargo clean
	rm -rf src-tauri/gen/android/.gradle src-tauri/gen/android/build src-tauri/gen/android/app/build
