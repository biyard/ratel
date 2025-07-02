PROJECT ?= ratel
EMULATOR ?= $(PROJECT)
ENV ?= local
LOG_LEVEL ?= debug
FIXTURE_CONFIG_FILE=fixtures/config.json
BUILD_DIR=.build
BUILD_CONFIG_FILE=$(BUILD_DIR)/config.json
NET ?= local
MOBILE ?= android
PORT ?= 5000

ifeq ("$(ENV)","prod")
	REDIRECT_URL = https://ratel.foundation
endif

ifeq ("$(ENV)","dev")
	REDIRECT_URL = https://dev.ratel.foundation
endif

ifeq ("$(ENV)","local")
	REDIRECT_URL = https://dev.ratel.foundation
endif

RENDERER ?= canvaskit

$(BUILD_CONFIG_FILE):
	@rm -rf $(BUILD_DIR)/config.json
	@mkdir -p $(BUILD_DIR)
	@cp $(FIXTURE_CONFIG_FILE) $(BUILD_CONFIG_FILE)
	@sed -i '' "s/{ENV}/$(ENV)/g" $(BUILD_CONFIG_FILE)
	@sed -i '' "s/{LOG_LEVEL}/$(LOG_LEVEL)/g" $(BUILD_CONFIG_FILE)
	@sed -i '' "s|{REDIRECT_URL}|$(REDIRECT_URL)|g" $(BUILD_CONFIG_FILE)

.PHONY: all
all: clean run

%.keystore:
	scripts/create-key.sh $@ android$*key $(PASSWORD)

run.android: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d emulator-5554

run.ios: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d 947A4545-75D7-4264-AC8C-EFFE2C728C87

run.web: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d chrome --web-renderer $(RENDERER)

build.ios: $(BUILD_CONFIG_FILE)
	flutter build ios --dart-define-from-file=$(BUILD_CONFIG_FILE) --release --no-codesign

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