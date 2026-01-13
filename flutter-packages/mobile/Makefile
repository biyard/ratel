PROJECT ?= ratel
EMULATOR ?= $(PROJECT)
ENV ?= dev
LOG_LEVEL ?= debug
FIXTURE_CONFIG_FILE=fixtures/config.json
BUILD_DIR=.build
BUILD_CONFIG_FILE=$(BUILD_DIR)/config.json
NET ?= local
MOBILE ?= android
PORT ?= 5000

UNAME_S := $(shell uname -s)

MAIN_IP := $(shell ip route get 1.1.1.1 | awk '{print $$7}')
IOS_SIMULATOR_ID := $(shell xcrun simctl list devices available | grep 'iPhone' | grep 'Booted' | head -1 | grep -o '([A-F0-9-]*)'  | tr -d '()')

ANDROID_OUT_APK_DIR ?= build/app/outputs/flutter-apk
ANDROID_OUT_APK_GLOB ?= $(ANDROID_OUT_APK_DIR)/*.apk
ANDROID_OUT_AAB_DIR ?= build/app/outputs/bundle/release
ANDROID_OUT_AAB_GLOB ?= $(ANDROID_OUT_AAB_DIR)/*.aab

GIT_SHA := $(shell git rev-parse --short HEAD 2>/dev/null || echo local)
DATE_STR := $(shell date +%Y%m%d-%H%M%S 2>/dev/null || echo 00000000-000000)
ARTIFACT_DIR ?= .build
ARTIFACT_NAME ?= artifacts-$(ENV)-$(GIT_SHA)-$(DATE_STR)
ARTIFACT_ZIP ?= $(ARTIFACT_DIR)/$(ARTIFACT_NAME).zip

SED_INPLACE = sed -i
ifeq ($(shell uname),Darwin)
  SED_INPLACE = sed -i ''
endif

ifeq ("$(ENV)","prod")
	REDIRECT_URL = https://ratel.foundation
	API_ENDPOINT = https://api.ratel.foundation
	SIGN_DOMAIN = ratel.foundation
endif

ifeq ("$(ENV)","dev")
	REDIRECT_URL = https://dev.ratel.foundation
	API_ENDPOINT = https://api.dev.ratel.foundation
	SIGN_DOMAIN = dev.ratel.foundation
endif

ifeq ("$(ENV)","local")
	REDIRECT_URL = https://dev.ratel.foundation
	API_ENDPOINT = http://$(MAIN_IP):3000
	SIGN_DOMAIN = dev.ratel.foundation
endif

RENDERER ?= canvaskit

$(BUILD_CONFIG_FILE):
	@rm -rf $(BUILD_DIR)/config.json
	@mkdir -p $(BUILD_DIR)
	@cp $(FIXTURE_CONFIG_FILE) $(BUILD_CONFIG_FILE)
	@$(SED_INPLACE) "s/{ENV}/$(ENV)/g" $(BUILD_CONFIG_FILE)
	@$(SED_INPLACE) "s/{LOG_LEVEL}/$(LOG_LEVEL)/g" $(BUILD_CONFIG_FILE)
	@$(SED_INPLACE) "s|{REDIRECT_URL}|$(REDIRECT_URL)|g" $(BUILD_CONFIG_FILE)
	@$(SED_INPLACE) "s|{API_ENDPOINT}|$(API_ENDPOINT)|g" $(BUILD_CONFIG_FILE)
	@$(SED_INPLACE) "s|{SIGN_DOMAIN}|$(SIGN_DOMAIN)|g" $(BUILD_CONFIG_FILE)

.PHONY: all
all: clean run

%.keystore:
	scripts/create-key.sh $@ android$*key $(PASSWORD)

run: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE)

run.android: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d emulator-5554

run.ios: $(BUILD_CONFIG_FILE)
	@if [ -z "$(IOS_SIMULATOR_ID)" ]; then \
		echo "No booted iOS simulator found. Starting one..."; \
		IOS_DEVICE_ID=$$(xcrun simctl list devices available | grep 'iPhone' | head -1 | grep -o '([A-F0-9-]*)' | tr -d '()'); \
		xcrun simctl boot $$IOS_DEVICE_ID; \
		flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d $$IOS_DEVICE_ID; \
	else \
		flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d $(IOS_SIMULATOR_ID); \
	fi

run.web: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d chrome --web-renderer $(RENDERER)

build.ios: $(BUILD_CONFIG_FILE)
	flutter build ios --config-only --dart-define-from-file=$(BUILD_CONFIG_FILE) --release

build.android: $(BUILD_CONFIG_FILE)
	flutter build appbundle --dart-define-from-file=$(BUILD_CONFIG_FILE) --release

release.android: build-config build.android deploy.android
release.ios: deploy.ios

build-config: $(BUILD_CONFIG_FILE)

deploy.android:
	@echo "Deploying to Google Play..."
	@if [ -z "$$GOOGLE_PLAY_KEY_JSON" ]; then \
		echo "Using local JSON key file"; \
		cd android; \
		fastlane deploy; \
	else \
		echo "Using JSON key from ENV"; \
		cd android; \
		fastlane deploy; \
	fi

.PHONY: deploy.ios
deploy.ios: build.ios
	cd ios && fastlane deploy

setup.tools:
	cargo install flutter_rust_bridge_codegen

watch.rust:
	flutter_rust_bridge_codegen generate --watch --

run.emulator:
	flutter emulators --launch $(PROJECT)

create.emulator:
	flutter emulators --create --name $(PROJECT)

setup.sdk:
	sdkmanager "system-images;android-27;google_apis;x86_64"

.PHONY: build
build: clean $(BUILD_CONFIG_FILE)
	flutter build appbundle --dart-define-from-file=$(BUILD_CONFIG_FILE)

clean:
	@rm -rf $(BUILD_CONFIG_FILE) build

.PHONY: build.android.apk
build.android.apk: $(BUILD_CONFIG_FILE)
	flutter build apk --release --split-per-abi --dart-define-from-file=$(BUILD_CONFIG_FILE)
	@ls -lh $(ANDROID_OUT_APK_GLOB) || true

.PHONY: build.android.aab
build.android.aab: $(BUILD_CONFIG_FILE)
	flutter build appbundle --release --dart-define-from-file=$(BUILD_CONFIG_FILE)
	@ls -lh $(ANDROID_OUT_AAB_GLOB) || true

.PHONY: artifact.list
artifact.list:
	@echo "APK:"; ls -1 $(ANDROID_OUT_APK_GLOB) 2>/dev/null || true
	@echo "AAB:"; ls -1 $(ANDROID_OUT_AAB_GLOB) 2>/dev/null || true

.PHONY: artifact.zip
artifact.zip:
	@mkdir -p $(ARTIFACT_DIR)
	@echo "Packaging -> $(ARTIFACT_ZIP)"
	@zip -r $(ARTIFACT_ZIP) $(ANDROID_OUT_APK_DIR) 2>/dev/null || true
	@zip -j $(ARTIFACT_ZIP) $(ANDROID_OUT_AAB_GLOB) 2>/dev/null || true
	@ls -lh $(ARTIFACT_ZIP) || true

.PHONY: ci.android.apk
ci.android.apk: build-config build.android.apk artifact.zip
	@echo "Artifacts ready at $(ARTIFACT_ZIP)"

.PHONY: ci.android.aab
ci.android.aab: build-config build.android.aab artifact.zip
	@echo "Artifacts ready at $(ARTIFACT_ZIP)"
