# Build the dioxus-web bundle and assemble it into a Tauri Android APK.
#
# Inputs you may override:
#   ENV=local|dev|staging|prod   (default: local)
#   MOBILE_API_URL=https://...   (default: depends on ENV)
#   MODE=debug|release           (default: debug; release for store builds)
#   ANDROID_TARGET=aarch64-linux-android | x86_64-linux-android

ENV ?= local
MODE ?= debug
PORT ?= 8000
# Rustup-style target triple (for cross-compile / verification commands).
ANDROID_TARGET ?= aarch64-linux-android
# Tauri's `--target` accepts short names: aarch64 | armv7 | i686 | x86_64.
# Derive from ANDROID_TARGET so a single override stays consistent.
TAURI_TARGET ?= $(firstword $(subst -, ,$(ANDROID_TARGET)))

# Auto-detect host LAN IP so a physical device on the same Wi-Fi can reach
# `dx serve` and the backend running on this workstation. Same detection
# logic as app/ratel/Makefile. Falls back to 10.0.2.2 (Android emulator's
# alias for host loopback). Override with `make HOST_IP=10.20.30.40 ...`.
ifeq ($(shell uname -s),Darwin)
  HOST_IP ?= $(shell iface=$$(route -n get default 2>/dev/null | awk '/interface:/ {print $$2}'); \
                     [ -n "$$iface" ] && ipconfig getifaddr "$$iface" || echo 10.0.2.2)
else
  HOST_IP ?= $(shell ip route get 1.1.1.1 2>/dev/null | awk '/src/ {for(i=1;i<=NF;i++) if($$i=="src") {print $$(i+1); exit}}')
  ifeq ($(strip $(HOST_IP)),)
    HOST_IP := 10.0.2.2
  endif
endif

ifeq ($(ENV),prod)
  MOBILE_API_URL ?= https://ratel.foundation
else ifeq ($(ENV),staging)
  MOBILE_API_URL ?= https://stg.ratel.foundation
else ifeq ($(ENV),dev)
  MOBILE_API_URL ?= https://dev.ratel.foundation
else
  MOBILE_API_URL ?= http://$(HOST_IP):$(PORT)
endif

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
RUSTFLAGS ?= -D warnings

# Android toolchain — same paths the existing app/ratel/Makefile uses for the
# legacy dioxus-native build (matched here so a single env profile works for
# both). Override in CI or non-standard installs.
HOST_OS := $(shell uname -s | tr '[:upper:]' '[:lower:]')
# Force-override these even if the shell exports stale values pointing at
# empty directories (which is common — e.g. ANDROID_HOME=/opt/android-sdk
# left over from an Android Studio install that's not actually there).
# Use `make ANDROID_HOME=... ANDROID_NDK_HOME=... JAVA_HOME=...` on the
# command line to override per-invocation.
ifeq ($(HOST_OS),darwin)
  ANDROID_HOME := $(if $(filter override,$(origin ANDROID_HOME)),$(ANDROID_HOME),/opt/homebrew/share/android-commandlinetools)
  ANDROID_NDK_HOME := $(if $(filter override,$(origin ANDROID_NDK_HOME)),$(ANDROID_NDK_HOME),/opt/homebrew/share/android-ndk)
  JAVA_HOME := $(if $(filter override,$(origin JAVA_HOME)),$(JAVA_HOME),/Library/Java/JavaVirtualMachines/temurin-21.jdk/Contents/Home)
else
  ANDROID_HOME := $(if $(filter override,$(origin ANDROID_HOME)),$(ANDROID_HOME),/opt/android-sdk)
  ANDROID_NDK_HOME := $(if $(filter override,$(origin ANDROID_NDK_HOME)),$(ANDROID_NDK_HOME),/opt/android-ndk)
  JAVA_HOME := $(if $(filter override,$(origin JAVA_HOME)),$(JAVA_HOME),/usr/lib/jvm/java-21-openjdk)
endif
NDK_HOME ?= $(ANDROID_NDK_HOME)

ANDROID_ENV = \
	ANDROID_HOME=$(ANDROID_HOME) \
	ANDROID_SDK_ROOT=$(ANDROID_HOME) \
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) \
	NDK_HOME=$(NDK_HOME) \
	JAVA_HOME=$(JAVA_HOME)

# WebView dev source URL for `make android-dev`. Defaults to the auto-detected
# host LAN IP (so physical devices on the same Wi-Fi work out of the box).
# Falls back to 10.0.2.2 (emulator loopback alias) if detection fails.
# Override with `make android-dev DEV_URL=http://10.20.30.40:8000` when needed.
DEV_URL ?= http://$(HOST_IP):$(PORT)

DIST_DIR := $(CURDIR)/dist
RATEL_DIR := $(CURDIR)/../ratel
# dx writes its output under the workspace root's target/dx, not app/ratel/target/dx,
# because the cargo workspace shares one target directory.
WORKSPACE_ROOT := $(CURDIR)/../..

ifneq ($(CARGO_TARGET_DIR),)
	DX_BUILD_DIR = $(CARGO_TARGET_DIR)/dx/app-shell/release/web/public
else
	DX_BUILD_DIR = $(RATEL_DIR)/target/dx/app-shell/release/web/public
endif

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
	cd $(RATEL_DIR) && make build-tauri
	rsync -a --delete "$(DX_BUILD_DIR)/" "$(DIST_DIR)/"

	@# dx 0.7's bundle JS unconditionally calls atob(window.initial_dioxus_hydration_data)
	@# at boot. In a fullstack/SSR deployment the server injects this variable into the
	@# HTML response. Tauri serves the static bundle without SSR, so atob() chokes on
	@# undefined.
	@#
	@# Stub `atob` to return "" when its argument is undefined/null. The wasm's
	@# hydration check separately tests whether real hydration data is present (via a
	@# different code path) and falls through to client-only mounting when absent.
	@# This is less invasive than setting initial_dioxus_hydration_data="" which the
	@# wasm misinterprets as "empty manifest, try to hydrate" and then panics.
	@if ! grep -q "patched_atob" $(DIST_DIR)/index.html; then \
		echo "Injecting atob shim into index.html"; \
		python3 -c "p='$(DIST_DIR)/index.html'; t=open(p).read(); s='<script>(function(){var __o=window.atob;window.atob=function(x){return (x===undefined||x===null)?\"\":__o.call(this,x);};window.__patched_atob=true;})();</script>'; open(p,'w').write(t.replace('<body>','<body>'+s,1))"; \
	fi

# Live dev — dioxus serves WASM on :8000, Tauri WebView points there via devUrl.
# Pass DEV_URL=http://<lan-ip>:8000 to override the default emulator-loopback
# address baked into tauri.conf.json (useful for physical devices).
# Copy the canonical google-services.json into the Gradle module where the
# Google Services Gradle plugin expects it. Single source of truth lives at
# src-tauri/google-services.json (gitignored); this target keeps the Gradle
# copy in sync before any Android build runs.
src-tauri/gen/android/app/google-services.json: src-tauri/google-services.json
	@cp $< $@
	@echo "Synced google-services.json -> gen/android/app/"

android-dev: install-tauri-cli src-tauri/gen/android/app/google-services.json
	@echo "Run 'cd ../ratel && PORT=8000 make run-tauri' in another terminal first."
	cd src-tauri && $(ANDROID_ENV) cargo tauri android dev \
		$(if $(DEV_URL),--config '{"build":{"devUrl":"$(DEV_URL)"}}',)

build-release: install-tauri-cli dx-build src-tauri/gen/android/app/google-services.json
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(TAURI_TARGET) $(if $(filter debug,$(MODE)),--debug,)
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*.apk" -newer Makefile 2>/dev/null

install-debug: build-release
	@APK=$$(find src-tauri/gen/android -name "app-*.apk" -not -name "*unsigned*" 2>/dev/null \
	       | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$APK" ]; then echo "ERROR: no debug APK found under src-tauri/gen/android"; exit 1; fi; \
	echo "Installing $$APK"; \
	adb install -r "$$APK"; \
	adb shell am start -n "co.biyard.ratel/.MainActivity"

clean:
	rm -rf $(DIST_DIR)
	cd src-tauri && cargo clean
	rm -rf src-tauri/gen/android/.gradle src-tauri/gen/android/build src-tauri/gen/android/app/build
