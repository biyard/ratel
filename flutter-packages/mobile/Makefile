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
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d CEA92FFC-F43F-49A9-84EA-B1AEC28FC8F1

run.web: $(BUILD_CONFIG_FILE)
	flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d chrome --web-renderer $(RENDERER)

build.ios: $(BUILD_CONFIG_FILE)
	flutter build ios --dart-define-from-file=$(BUILD_CONFIG_FILE) --release --no-codesign

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