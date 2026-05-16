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
GOOGLE_OAUTH_CLIENT_ID ?=

ANDROID_ENV = \
	ANDROID_HOME=$(ANDROID_HOME) \
	ANDROID_SDK_ROOT=$(ANDROID_HOME) \
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) \
	NDK_HOME=$(NDK_HOME) \
	GOOGLE_OAUTH_CLIENT_ID=$(GOOGLE_OAUTH_CLIENT_ID) \
	JAVA_HOME=$(JAVA_HOME) \

# WebView dev source URL for `make android-dev`. Defaults to the auto-detected
# host LAN IP (so physical devices on the same Wi-Fi work out of the box).
# Falls back to 10.0.2.2 (emulator loopback alias) if detection fails.
# Override with `make android-dev DEV_URL=http://10.20.30.40:8000` when needed.
DEV_URL ?= http://$(HOST_IP):$(PORT)

# CSP override per environment. Dev allows any http/https origin so a
# physical device or emulator can reach `dx serve` on any LAN IP, ngrok
# tunnel, etc. without rebuilding the APK for each address change. Prod
# narrows back down to known origins. Pass via cargo tauri's --config flag
# at build/run time (see build-android / run-android below).
ifeq ($(ENV),prod)
  CSP_CONNECT := 'self' tauri: ipc: http://tauri.localhost https://ratel.foundation https://*.ratel.foundation https://*.googleapis.com https://*.firebaseio.com https://identitytoolkit.googleapis.com https://securetoken.googleapis.com wss://*.firebaseio.com
else
  CSP_CONNECT := 'self' tauri: ipc: http: https: wss:
endif

# `'unsafe-eval'` is required because dioxus / wasm-bindgen call JS eval() or
# `new Function(...)` for some dynamic dispatch paths. `'wasm-unsafe-eval'`
# alone only allows WebAssembly.compile/instantiate, not JS string-eval.
# Without it the wasm bundle aborts with `EvalError: Refused to evaluate a
# string as JavaScript`. Acceptable in dev; consider narrowing for prod.
CSP_OVERRIDE := default-src 'self' tauri: http://tauri.localhost https://ratel.foundation https://*.ratel.foundation; script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval' tauri: http://tauri.localhost https://www.gstatic.com; connect-src $(CSP_CONNECT); img-src 'self' data: blob: https:; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com

DIST_DIR := $(CURDIR)/dist
RATEL_DIR := $(CURDIR)/../ratel
# dx writes its output under the workspace root's target/dx, not app/ratel/target/dx,
# because the cargo workspace shares one target directory.
WORKSPACE_ROOT := $(CURDIR)/../..

DX_CARGO_TARGET_DIR ?= $(PWD)/dx-tauri-target
DX_BUILD_DIR = $(DX_CARGO_TARGET_DIR)/dx/app-shell/release/web/public

.PHONY: help install-tauri-cli dx-build run-android build-android build-android-debug sign-apk run-apk debug-apk clean keystore.sha1 apk.sha1

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

$(HOME)/.android/debug.keystore:
	keytool -genkey -v -keystore $@ -storepass android -keypass android -alias androiddebugkey -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Korea,O=Biyard,C=KR"

	keytool -list -v -keystore ~/.android/debug.keystore -alias androiddebugkey -storepass android | grep SHA1
	@echo "Add SHA1 to your Firebase console"

# Read the SHA-1 of the keystore that the next APK build will use. Source
# of truth for the value to register in Firebase Console -> Project Settings
# -> Android app -> "Add fingerprint".
keystore.sha1:
	@keytool -list -v -keystore ~/.android/debug.keystore -alias androiddebugkey -storepass android | grep SHA1 \
		| awk '{print $$2}'

# Read the SHA-1 of the actually-signed APK. Use this AFTER a build to
# confirm the cert that got embedded — accounts for any signingConfig
# override in build.gradle.kts that keystore.sha1 wouldn't see.
apk.sha1:
	@APK=$$(find src-tauri/gen/android -name "app-*.apk" -not -name "*unsigned*" 2>/dev/null \
	       | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$APK" ]; then echo "ERROR: no signed APK found; run 'make build-release' first"; exit 1; fi; \
	APKSIGNER=$$(find $(ANDROID_HOME)/build-tools -name apksigner 2>/dev/null | sort -r | head -1); \
	if [ -z "$$APKSIGNER" ]; then echo "ERROR: apksigner not found under $(ANDROID_HOME)/build-tools"; exit 1; fi; \
	"$$APKSIGNER" verify --print-certs "$$APK" | grep "SHA-1 digest" | awk '{print $$NF}'

# Run `dx build` and copy the produced web bundle into ./dist for Tauri.
dx-build:
	cd $(RATEL_DIR) && CARGO_TARGET_DIR=$(DX_CARGO_TARGET_DIR) make build-tauri
	rsync -a --delete "$(DX_BUILD_DIR)/" "$(DIST_DIR)/"

	@if ! grep -q "patched_atob" $(DIST_DIR)/index.html; then \
		echo "Injecting SSR-global shims into index.html"; \
		python3 -c "p='$(DIST_DIR)/index.html'; t=open(p).read(); s='<script>(function(){var __o=window.atob;window.atob=function(x){return (x===undefined||x===null)?String.fromCharCode(0x80):__o.call(this,x);};if(!window.hydrate_queue)window.hydrate_queue=[];if(!window.hydration_callback)window.hydration_callback=function(){};window.__patched_atob=true;})();</script>'; open(p,'w').write(t.replace('<body>','<body>'+s,1))"; \
	fi

run: run-android

# Write a one-off JSON override merging per-ENV CSP and the DEV_URL into
# /tmp/tauri-override.json. Tauri's `--config` accepts a file path, which
# avoids shell-quoting hell with the embedded `'self'` quotes in CSP.
# Always regenerated so DEV_URL / ENV changes are picked up.
TAURI_OVERRIDE_FILE := $(PWD)/tauri-override.json

$(TAURI_OVERRIDE_FILE):
	@printf '{"build":{"devUrl":"%s"},"app":{"security":{"csp":"%s"}}}' \
			"$(DEV_URL)" "$(CSP_OVERRIDE)" > $(TAURI_OVERRIDE_FILE)


run-android: install-tauri-cli $(TAURI_OVERRIDE_FILE)
	@echo "Run 'cd ../ratel && PORT=8000 make run-tauri' in another terminal first."
	cd src-tauri && $(ANDROID_ENV) cargo tauri android dev --config $(TAURI_OVERRIDE_FILE)

build-android: install-tauri-cli dx-build
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(TAURI_TARGET) --apk
	@$(MAKE) sign-apk
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*.apk" -newer Makefile -not -name "*unsigned*" 2>/dev/null

# Sign every fresh release-unsigned APK with the local debug keystore so it
# can be `adb install`ed without the user manually invoking apksigner.
# Output: same path with `-unsigned` stripped from the filename. Idempotent:
# skips files that are already signed (apksigner verify succeeds).
sign-apk: $(HOME)/.android/debug.keystore
	@APKSIGNER=$$(find $(ANDROID_HOME)/build-tools -name apksigner 2>/dev/null | sort -r | head -1); \
	if [ -z "$$APKSIGNER" ]; then echo "ERROR: apksigner not found under $(ANDROID_HOME)/build-tools"; exit 1; fi; \
	for APK in $$(find src-tauri/gen/android -name "*-unsigned.apk" 2>/dev/null); do \
		OUT=$$(echo "$$APK" | sed 's/-unsigned\.apk$$/.apk/'); \
		echo "Signing $$APK -> $$OUT"; \
		"$$APKSIGNER" sign \
			--ks $(HOME)/.android/debug.keystore \
			--ks-pass pass:android \
			--key-pass pass:android \
			--ks-key-alias androiddebugkey \
			--in "$$APK" \
			--out "$$OUT"; \
	done

run-apk: build-android
	adb install -r "src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk"
	adb shell am start -n "co.biyard.ratel/.MainActivity"

# Debug variant of build-android. Passes --debug to `cargo tauri android build`
# so AGP produces a debug-signed APK directly (no separate sign-apk step) and
# enables: WebView chrome://inspect, lldb attach, BuildConfig.DEBUG=true,
# no R8 minify. Useful when you need Chrome DevTools on the WebView or to
# attach a debugger. Slower compile + larger APK than the release variant.
build-android-debug: install-tauri-cli dx-build $(TAURI_OVERRIDE_FILE)
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(TAURI_TARGET) --apk --debug --config $(TAURI_OVERRIDE_FILE)
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*-debug.apk" -newer Makefile 2>/dev/null

# Install and launch the debug-signed APK on the connected adb device.
# Mirrors run-apk but for the debug variant.
debug-apk: build-android-debug
	@APK=$$(find src-tauri/gen/android -name "app-*-debug.apk" 2>/dev/null \
	       | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$APK" ]; then echo "ERROR: no debug APK found under src-tauri/gen/android"; exit 1; fi; \
	echo "Installing $$APK"; \
	adb install -r "$$APK"
	adb shell am start -n "co.biyard.ratel/.MainActivity"

clean:
	rm -rf $(DIST_DIR)
	cd src-tauri && cargo clean
	rm -rf src-tauri/gen/android/.gradle src-tauri/gen/android/build src-tauri/gen/android/app/build
