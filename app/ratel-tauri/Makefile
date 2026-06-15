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
# Defaults match standard local installs. Override by exporting env vars
# (e.g. CI) or via `make VAR=... <target>`. CI workflows set ANDROID_HOME
# / ANDROID_NDK_HOME / JAVA_HOME explicitly via env so the runner's
# toolcache paths win over these defaults.
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
GOOGLE_OAUTH_CLIENT_ID ?=

# adb target device for install/launch. Override: `make debug-apk DEVICE=emulator-5554`.
# Defaults to ANDROID_SERIAL, else the first attached device — so `adb` doesn't
# fail with "more than one device/emulator" when both an emulator and a phone
# are connected. List candidates with `adb devices`.
DEVICE ?= $(or $(ANDROID_SERIAL),$(shell adb devices 2>/dev/null | awk 'NR>1 && $$2=="device"{print $$1; exit}'))
ADB := adb $(if $(strip $(DEVICE)),-s $(DEVICE),)

ANDROID_ENV = \
	ANDROID_HOME="$(ANDROID_HOME)" \
	ANDROID_SDK_ROOT="$(ANDROID_HOME)" \
	ANDROID_NDK_HOME="$(ANDROID_NDK_HOME)" \
	NDK_HOME="$(NDK_HOME)" \
	GOOGLE_OAUTH_CLIENT_ID="$(GOOGLE_OAUTH_CLIENT_ID)" \
	JAVA_HOME="$(JAVA_HOME)" \
	PATH="$(NDK_SHIMS):$$PATH" \

# ── NDK tool-name shims (vendored OpenSSL build) ─────────────────────
# The native HTTP transport (src-tauri reqwest + native-tls-vendored)
# compiles OpenSSL from source for Android. openssl-src's generated
# Makefile invokes the LEGACY per-target archiver names
# (aarch64-linux-android-ranlib, -ar, ...). NDK r23+ ships only the
# UNIFIED llvm-* tools, so those prefixed names aren't on PATH and the
# vendored build dies with "ranlib: command not found" (Error 127)
# during `make install_dev`. We symlink the prefixed names to the
# llvm-* tools and prepend them to PATH via ANDROID_ENV (above).
NDK_TOOLBIN := $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(HOST_OS)-x86_64/bin
NDK_SHIMS := $(CURDIR)/.ndk-shims

.PHONY: ndk-shims
ndk-shims:
	@mkdir -p "$(NDK_SHIMS)"
	@for triple in aarch64-linux-android armv7a-linux-androideabi i686-linux-android x86_64-linux-android; do \
		for tool in ranlib ar nm strip; do \
			ln -sf "$(NDK_TOOLBIN)/llvm-$$tool" "$(NDK_SHIMS)/$$triple-$$tool"; \
		done; \
	done

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
  CSP_CONNECT := 'self' tauri: ipc: http://tauri.localhost http://localhost:8000 https://ratel.foundation https://*.ratel.foundation https://*.amazonaws.com https://*.googleapis.com https://*.firebaseio.com https://identitytoolkit.googleapis.com https://securetoken.googleapis.com wss://*.firebaseio.com https://*.portone.io https://*.iamport.co https://*.iamport.kr
else
  CSP_CONNECT := 'self' tauri: ipc: http: https: wss:
endif

# `'unsafe-eval'` is required because dioxus / wasm-bindgen call JS eval() or
# `new Function(...)` for some dynamic dispatch paths. `'wasm-unsafe-eval'`
# alone only allows WebAssembly.compile/instantiate, not JS string-eval.
# Without it the wasm bundle aborts with `EvalError: Refused to evaluate a
# string as JavaScript`. Acceptable in dev; consider narrowing for prod.
CSP_OVERRIDE := default-src 'self' tauri: http://tauri.localhost http://localhost:8000 http://192.168.219.103:8000 https://ratel.foundation https://*.ratel.foundation; script-src 'self'  https://cdn.portone.io 'wasm-unsafe-eval' 'unsafe-eval' tauri: http://tauri.localhost https://www.gstatic.com; connect-src $(CSP_CONNECT); img-src 'self' data: blob: https:; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com

DIST_DIR := $(CURDIR)/dist
RATEL_DIR := $(CURDIR)/../ratel
# dx writes its output under the workspace root's target/dx, not app/ratel/target/dx,
# because the cargo workspace shares one target directory.
WORKSPACE_ROOT := $(CURDIR)/../..

DX_CARGO_TARGET_DIR ?= $(PWD)/dx-tauri-target
DX_BUILD_DIR = $(DX_CARGO_TARGET_DIR)/dx/app-shell/release/web/public

.PHONY: help install-tauri-cli dx-build run-android build-android build-android-debug sign-apk run-apk debug-apk release-aab ios-init run-ios build-ios build-ios-sim testflight clean keystore.sha1 apk.sha1 FORCE

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

run: run-android

# ── iOS ──────────────────────────────────────────────────────────────
# iOS uses an iOS-TYPE OAuth client ID, NOT the web/Android one: the
# GoogleSignIn iOS SDK (via tauri-plugin-google-auth) keys off the reversed
# iOS client-ID URL scheme registered in gen/apple/ratel-tauri_iOS/Info.plist.
# The client ID is public (ships in the app binary + Info.plist), not a secret.
# Override with `make build-ios IOS_GOOGLE_OAUTH_CLIENT_ID=...`.
IOS_GOOGLE_OAUTH_CLIENT_ID ?= 925748630857-f17s6e6dg64b3mugn0dnu35i9d4694dh.apps.googleusercontent.com
# iOS targets bake the iOS client ID (propagates to their IOS_CARGO_CONFIG /
# IOS_ENV prerequisites). A command-line GOOGLE_OAUTH_CLIENT_ID still wins.
ios-init run-ios build-ios build-ios-sim testflight: GOOGLE_OAUTH_CLIENT_ID := $(IOS_GOOGLE_OAUTH_CLIENT_ID)

# CocoaPods / xcodegen need a UTF-8 locale; the HOMEBREW_* flags keep
# tauri's dependency check quiet.
IOS_ENV = \
	LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 \
	HOMEBREW_NO_ENV_HINTS=1 HOMEBREW_NO_INSTALL_CLEANUP=1 \
	GOOGLE_OAUTH_CLIENT_ID="$(GOOGLE_OAUTH_CLIENT_ID)"

# `GOOGLE_OAUTH_CLIENT_ID` is a compile-time `option_env!` in the native lib.
# On Android it's passed through Gradle's env, but the iOS build runs `cargo`
# from an Xcode build phase whose environment xcodebuild SANITIZES — the shell
# env never reaches the compile. So we inject it through cargo's `[env]` table
# in a GENERATED `.cargo/config.toml`, written here from the build env on every
# iOS build. This keeps it per-build configurable (dev/prod) instead of a
# hardcoded value. The file is gitignored. `force = false` so an explicit env
# (e.g. the Android/Gradle path) still wins.
IOS_CARGO_CONFIG := $(CURDIR)/src-tauri/.cargo/config.toml
$(IOS_CARGO_CONFIG): FORCE
	@mkdir -p $(dir $(IOS_CARGO_CONFIG))
	@printf '# AUTO-GENERATED by app/ratel-tauri/Makefile for iOS builds.\n# Do NOT edit or commit (gitignored). GOOGLE_OAUTH_CLIENT_ID is injected from\n# the build env because xcodebuild sanitizes the iOS build-phase environment.\n[env]\nGOOGLE_OAUTH_CLIENT_ID = { value = "%s", force = false }\n' '$(GOOGLE_OAUTH_CLIENT_ID)' > $(IOS_CARGO_CONFIG)

# Regenerate the Xcode project (pbxproj) from project.yml. `cargo tauri ios
# build` does NOT re-run xcodegen, so edits to project.yml — manual release
# signing (CODE_SIGN_IDENTITY / PROVISIONING_PROFILE_SPECIFIER) and the
# `Externals buildPhase: none` fix that keeps libapp.a out of the App Store
# bundle — only take effect after this runs. Required before the App Store
# build. Needs `xcodegen` (installed by `cargo tauri ios init`; otherwise
# `brew install xcodegen`).
.PHONY: gen-xcodeproj
gen-xcodeproj:
	cd src-tauri/gen/apple && xcodegen generate --spec project.yml

# Generate the Xcode project (src-tauri/gen/apple). Run once, or after
# changing the bundle identifier / tauri.conf iOS settings.
ios-init: install-tauri-cli
	cd src-tauri && $(IOS_ENV) cargo tauri ios init

# Live-reload dev on a booted Simulator. Run `cd ../ratel && PORT=8000 make
# run-tauri` in another terminal first (serves the web bundle the WebView loads).
run-ios: install-tauri-cli $(IOS_CARGO_CONFIG) $(TAURI_OVERRIDE_FILE)
	cd src-tauri && $(IOS_ENV) cargo tauri ios dev --config $(TAURI_OVERRIDE_FILE)

# Release .ipa for App Store / TestFlight. Requires a configured Apple
# development team + signing (set in Xcode, or APPLE_DEVELOPMENT_TEAM /
# the generated ExportOptions.plist). Pass the FULL prod env, same as
# release-aab:
#   ratel_envs; export ENV=prod MOBILE_API_URL=https://ratel.foundation
#   export GOOGLE_OAUTH_CLIENT_ID=...
#   make build-ios
build-ios: install-tauri-cli $(IOS_CARGO_CONFIG) gen-xcodeproj dx-build
	cd src-tauri && $(IOS_ENV) cargo tauri ios build
	@echo
	@echo "iOS output (.ipa):"
	@find src-tauri/gen/apple -name "*.ipa" -newer Makefile 2>/dev/null

# Simulator build — no Apple team / code signing needed. Verifies the iOS
# app compiles + bundles and produces a .app runnable on the Simulator.
build-ios-sim: install-tauri-cli $(IOS_CARGO_CONFIG) dx-build
	cd src-tauri && $(IOS_ENV) cargo tauri ios build --target aarch64-sim --debug

# ── TestFlight (App Store Connect) ───────────────────────────────────
# Build a release .ipa (App Store distribution signing) and upload it to
# App Store Connect → TestFlight, fully from the CLI.
#
# ONE-TIME SETUP on this Mac (you must do these — they need your Apple account):
#   1. Apple Developer Program membership; note your 10-char Team ID.
#   2. In App Store Connect, create the app record for bundle id
#      `co.biyard.ratel` (Apps → + → New App).
#   3. Distribution signing must resolve for the archive. Easiest path:
#      open `make ios-init`'s Xcode project once, sign in with your Apple ID
#      under Signing & Capabilities, and let Xcode manage an "App Store"
#      provisioning profile for co.biyard.ratel. (Or install an Apple
#      Distribution cert + App Store profile manually.)
#   4. App Store Connect API key for the upload (no 2FA prompts):
#      App Store Connect → Users and Access → Integrations → App Store
#      Connect API → generate a key with the "App Manager" role.
#      Download AuthKey_<KEYID>.p8 ONCE and place it at:
#        ~/.appstoreconnect/private_keys/AuthKey_<KEYID>.p8
#      (xcrun altool auto-discovers it there. Keep the .p8 OUT of git.)
#
# REQUIRED env (pass on the command line or `export` them):
#   APPLE_DEVELOPMENT_TEAM  10-char Apple Team ID (e.g. AB12CD34EF)
#   ASC_KEY_ID              App Store Connect API Key ID
#   ASC_ISSUER_ID           App Store Connect API Issuer ID (UUID)
# Plus the usual prod build env (same as release-aab):
#   export ENV=prod MOBILE_API_URL=https://ratel.foundation
#
# Build number defaults to the Unix epoch (seconds) so every upload is unique
# and monotonically increasing (App Store Connect rejects a duplicate
# version+build). Epoch is used instead of a YYYYMMDDHHMM stamp because
# `--build-number` is a u32 (max 4294967295) and a 12-digit datetime overflows
# it. Override with TF_BUILD_NUMBER=... Bump CFBundleShortVersionString in
# tauri.conf.json when you ship a new marketing version.
#
# Example:
#   ratel_envs
#   export ENV=prod MOBILE_API_URL=https://ratel.foundation
#   make testflight APPLE_DEVELOPMENT_TEAM=AB12CD34EF \
#        ASC_KEY_ID=XXXXXXXXXX ASC_ISSUER_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
APPLE_DEVELOPMENT_TEAM ?=
ASC_KEY_ID ?=
ASC_ISSUER_ID ?=
TF_BUILD_NUMBER ?= $(shell date +%s)

testflight: install-tauri-cli $(IOS_CARGO_CONFIG) gen-xcodeproj dx-build
	@test -n "$(APPLE_DEVELOPMENT_TEAM)" || { echo "ERROR: set APPLE_DEVELOPMENT_TEAM (10-char Apple Team ID)"; exit 1; }
	@test -n "$(ASC_KEY_ID)"             || { echo "ERROR: set ASC_KEY_ID (App Store Connect API Key ID)"; exit 1; }
	@test -n "$(ASC_ISSUER_ID)"          || { echo "ERROR: set ASC_ISSUER_ID (App Store Connect API Issuer ID)"; exit 1; }
	cd src-tauri && $(IOS_ENV) APPLE_DEVELOPMENT_TEAM="$(APPLE_DEVELOPMENT_TEAM)" \
		cargo tauri ios build --export-method app-store-connect --build-number "$(TF_BUILD_NUMBER)"
	@IPA=$$(find src-tauri/gen/apple/build -name "*.ipa" 2>/dev/null | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$IPA" ]; then echo "ERROR: no .ipa produced under src-tauri/gen/apple/build"; exit 1; fi; \
	echo "==> Validating $$IPA"; \
	xcrun altool --validate-app --type ios --file "$$IPA" \
		--apiKey "$(ASC_KEY_ID)" --apiIssuer "$(ASC_ISSUER_ID)" || \
		{ echo "ERROR: validation failed (fix before uploading)"; exit 1; }; \
	echo "==> Uploading $$IPA to App Store Connect / TestFlight"; \
	xcrun altool --upload-app --type ios --file "$$IPA" \
		--apiKey "$(ASC_KEY_ID)" --apiIssuer "$(ASC_ISSUER_ID)"
	@echo
	@echo "Uploaded. App Store Connect → TestFlight processes the build in a few"
	@echo "minutes; add it to a test group (and complete export-compliance) to install."

# Write a one-off JSON override merging per-ENV CSP and the DEV_URL into
# /tmp/tauri-override.json. Tauri's `--config` accepts a file path, which
# avoids shell-quoting hell with the embedded `'self'` quotes in CSP.
# Always regenerated so DEV_URL / ENV changes are picked up.
TAURI_OVERRIDE_FILE := $(PWD)/tauri-override.json

# Depend on FORCE so the override is regenerated on every invocation —
# otherwise make treats the existing file as up-to-date and DEV_URL / ENV
# changes are silently ignored (stale devUrl).
$(TAURI_OVERRIDE_FILE): FORCE
	@printf '{"build":{"devUrl":"%s"},"app":{"security":{"csp":"%s"}}}' \
			"$(DEV_URL)" "$(CSP_OVERRIDE)" > $(TAURI_OVERRIDE_FILE)

FORCE:


run-android: install-tauri-cli ndk-shims $(TAURI_OVERRIDE_FILE)
	@echo "Run 'cd ../ratel && PORT=8000 make run-tauri' in another terminal first."
	cd src-tauri && $(ANDROID_ENV) cargo tauri android dev --config $(TAURI_OVERRIDE_FILE)

build-android: install-tauri-cli ndk-shims dx-build
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(TAURI_TARGET) --apk
	@$(MAKE) sign-apk
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*.apk" -newer Makefile -not -name "*unsigned*" 2>/dev/null

# Release AAB (Android App Bundle) for Google Play Console upload.
# Builds the prod web bundle then a release-signed .aab. Signing uses
# src-tauri/gen/android/keystore.properties (wired into app/build.gradle.kts);
# without it the bundle is unsigned and Play will reject it.
#
# Pass the FULL prod env (same profile as a normal build, not just one var).
# NOTE: the inner app/ratel Makefile defaults MOBILE_API_URL to HOST_IP:PORT
# regardless of ENV, so you MUST export MOBILE_API_URL explicitly:
#   ratel_envs
#   export ENV=prod
#   export MOBILE_API_URL=https://ratel.foundation
#   export PORTONE_STORE_ID=... PORTONE_INICIS_CHANNEL_KEY=...   # PRODUCTION (contracted) MID
#   export GOOGLE_OAUTH_CLIENT_ID=...   # Web client id; register the Play/upload SHA-1 in GCP
#   make release-aab
#
# Bump the version in tauri.conf.json before each upload — Play requires a
# strictly increasing versionCode.
release-aab: install-tauri-cli ndk-shims dx-build
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(TAURI_TARGET) --aab
	@echo
	@echo "AAB output (upload this .aab to Play Console):"
	@find src-tauri/gen/android -name "*.aab" -newer Makefile 2>/dev/null

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
	$(ADB) install -r "src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk"
	$(ADB) shell am start -n "co.biyard.ratel/.MainActivity"

# Debug variant of build-android. Passes --debug to `cargo tauri android build`
# so AGP produces a debug-signed APK directly (no separate sign-apk step) and
# enables: WebView chrome://inspect, lldb attach, BuildConfig.DEBUG=true,
# no R8 minify. Useful when you need Chrome DevTools on the WebView or to
# attach a debugger. Slower compile + larger APK than the release variant.
build-android-debug: install-tauri-cli ndk-shims
	cd src-tauri && $(ANDROID_ENV) cargo tauri android build --target $(TAURI_TARGET) --apk --debug
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*-debug.apk" -newer Makefile 2>/dev/null

# Install and launch the debug-signed APK on the connected adb device.
# Mirrors run-apk but for the debug variant.
debug-apk: build-android-debug
	@APK=$$(find src-tauri/gen/android -name "app-*-debug.apk" 2>/dev/null \
	       | xargs ls -t 2>/dev/null | head -1); \
	if [ -z "$$APK" ]; then echo "ERROR: no debug APK found under src-tauri/gen/android"; exit 1; fi; \
	echo "Installing $$APK to $(if $(strip $(DEVICE)),$(DEVICE),<default device>)"; \
	$(ADB) install -r "$$APK"
	$(ADB) shell am start -n "co.biyard.ratel/.MainActivity"

clean:
	rm -rf $(DIST_DIR)
	cd src-tauri && cargo clean
	rm -rf src-tauri/gen/android/.gradle src-tauri/gen/android/build src-tauri/gen/android/app/build
