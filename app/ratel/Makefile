ENV ?= dev

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
DYNAMO_ENDPOINT ?= none
KAIA_ENDPOINT ?= https://public-en-kairos.node.kaia.io
BLOCK_EXPLORER_URL ?= https://kairos.kaiascan.io
WALLETCONNECT_PROJECT_ID ?= "WALLETCONNECT_PROJECT_ID"

DFX_JSON ?= ../../packages/ratel-canister/dfx.json
CANISTER_IDS_JSON ?= ../../packages/ratel-canister/canister_ids.json

ifeq ($(ENV),prod)
IC_NETWORK ?= ic
IC_URL ?= https://ic0.app
ICP_IDENTITY_PEM ?=
else
IC_NETWORK ?= dev
IC_URL ?= $(shell jq -r '.networks.$(IC_NETWORK).providers[0] // empty' $(DFX_JSON) 2>/dev/null)
ICP_IDENTITY_PEM ?= $(shell cat ~/.config/dfx/identity/default/identity.pem 2>/dev/null | tr '\n' '|' | sed 's/|/\\n/g')
endif
RATEL_CANISTER_ID ?= $(shell jq -r '.["ratel-canister"].$(IC_NETWORK) // empty' $(CANISTER_IDS_JSON) 2>/dev/null)
VOTER_TAG_SECRET ?= test
# cargo run -p attr-voting --bin attr-voting-keygen
ATTR_VOTING_AUTHORITY_JSON ?=
RUST_LOG ?= debug
RUSTFLAGS ?= "-D warnings"

PORT ?= 8080
RUN_FEATURES ?= local-dev,bypass

# Auto-detect the host's LAN IP so Android devices / emulators on the same
# network can reach `dx serve` running on this workstation. OS branch is
# split so each side's failure doesn't silently zero the other (a piped
# `awk` exits 0 on empty input, which broke the previous `||` chain on
# macOS). Falls back to the Android emulator loopback alias `10.0.2.2`,
# which works for emulators but not physical phones. Override explicitly
# when needed:
#   MOBILE_API_URL=http://10.20.30.40:8080 make android
ifeq ($(shell uname -s),Darwin)
  HOST_IP ?= $(shell iface=$$(route -n get default 2>/dev/null | awk '/interface:/ {print $$2}'); \
                     [ -n "$$iface" ] && ipconfig getifaddr "$$iface" || echo 10.0.2.2)
	ARCH ?= arm64-v8a
else
  HOST_IP ?= $(shell ip route get 1.1.1.1 2>/dev/null | awk '/src/ {for(i=1;i<=NF;i++) if($$i=="src") {print $$(i+1); exit}}')
  ifeq ($(strip $(HOST_IP)),)
    HOST_IP := 10.0.2.2
  endif

	ARCH ?= $(shell uname -m)
endif

MOBILE_API_URL ?= http://$(HOST_IP):$(PORT)

REPOSITORY ?= ratel/app-shell
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY)  --query "repositories[0].repositoryUri" --output text)
QDRANT_PREFIX ?= ratel-$(ENV)
QDRANT_URL ?= http://localhost:6334
QDRANT_API_KEY ?= ""
AVD_NAME ?= Pixel6_$(ARCH)
ANDROID_EMULATOR_ID ?= $(shell adb devices | grep emulator | cut -f1)
DISABLE_ANSI ?= false
BUCKET_NAME ?= metadata.ratel.foundation
HOST_UID ?= $(shell id -u)
HOST_GID ?= $(shell id -g)

ifneq ($(ANDROID_EMULATOR_ID),)
	DEVICE_FLAG ?= --device $(ANDROID_EMULATOR_ID)
else
	FIRST_ANDROID_DEVICE := $(shell adb devices | awk 'NR>1 {print $$1; exit}')
	ifneq ($(FIRST_ANDROID_DEVICE),)
		DEVICE_FLAG ?= --device $(FIRST_ANDROID_DEVICE)
		ANDROID_TARGET ?= aarch64-linux-android
	endif
endif

HOST_OS := $(shell uname -s | tr '[:upper:]' '[:lower:]')
ifeq ($(HOST_OS),darwin)
  ANDROID_NDK_HOME ?= /opt/homebrew/share/android-ndk
  ANDROID_JAVA_HOME ?= /Library/Java/JavaVirtualMachines/temurin-21.jdk/Contents/Home
	ANDROID_TARGET ?= aarch64-linux-android
else
  ANDROID_NDK_HOME ?= /opt/android-ndk
  # Gradle + Android Gradle Plugin 8.x don't parse class-file major 70
  # (OpenJDK 26+). Force a JDK that Gradle supports.
  ANDROID_JAVA_HOME ?= /usr/lib/jvm/java-21-openjdk
	ANDROID_TARGET ?= x86_64-linux-android
endif

MODE ?= debug

ifneq ($(CARGO_TARGET_DIR),)
	ANDROID_DX_RES_DIR ?= $(CARGO_TARGET_DIR)/dx/app-shell/$(MODE)/android/app/app/src/main
else
	ANDROID_DX_RES_DIR ?= target/dx/app-shell/$(MODE)/android/app/app/src/main
endif

ANDROID_MIPMAP_DENSITIES ?= mdpi hdpi xhdpi xxhdpi xxxhdpi

ANDROID_ENV = \
	JAVA_HOME=$(ANDROID_JAVA_HOME) \
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME)


BUILD_ENV ?= AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
					AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
					AWS_REGION=$(AWS_REGION) \
					AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) \
					DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
					DYNAMODB_ENDPOINT=$(DYNAMO_ENDPOINT) \
					RUST_LOG=$(RUST_LOG) \
					ENV=$(ENV) \
					COMMIT=$(COMMIT) \
					PORT=$(PORT) \
					PORTONE_STORE_ID=$(PORTONE_STORE_ID) \
					PORTONE_KPN_CHANNEL_KEY=$(PORTONE_KPN_CHANNEL_KEY) \
					PORTONE_INICIS_CHANNEL_KEY=$(PORTONE_INICIS_CHANNEL_KEY) \
					PORTONE_API_SECRET=$(PORTONE_API_SECRET) \
					KAIA_ENDPOINT=$(KAIA_ENDPOINT) \
					BLOCK_EXPLORER_URL=$(BLOCK_EXPLORER_URL) \
					FIREBASE_API_KEY=$(FIREBASE_API_KEY) \
					FIREBASE_AUTH_DOMAIN=$(FIREBASE_AUTH_DOMAIN) \
					FIREBASE_PROJECT_ID=$(FIREBASE_PROJECT_ID) \
					FIREBASE_STORAGE_BUCKET=$(FIREBASE_STORAGE_BUCKET) \
					FIREBASE_MESSAGING_SENDER_ID=$(FIREBASE_MESSAGING_SENDER_ID) \
					FIREBASE_APP_ID=$(FIREBASE_APP_ID) \
					FIREBASE_MEASUREMENT_ID=$(FIREBASE_MEASUREMENT_ID) \
					IC_URL=$(IC_URL) \
					RATEL_CANISTER_ID=$(RATEL_CANISTER_ID) \
					ICP_IDENTITY_PEM='$(ICP_IDENTITY_PEM)' \
					VOTER_TAG_SECRET='$(VOTER_TAG_SECRET)' \
					ATTR_VOTING_AUTHORITY_JSON='$(ATTR_VOTING_AUTHORITY_JSON)' \
					QDRANT_URL=$(QDRANT_URL) \
					QDRANT_API_KEY=$(QDRANT_API_KEY) \
					QDRANT_PREFIX=$(QDRANT_PREFIX) \
					MOBILE_API_URL=$(MOBILE_API_URL) \
					DISABLE_ANSI=$(DISABLE_ANSI) \
					BUCKET_NAME=$(BUCKET_NAME) \
					SLACK_HOOK=$(SLACK_HOOK) \

.PHONY: env.sh test run android android-emulator ios build build-testing build-arm build-lambda docker
env.sh:
	@echo "export AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID)" > $@
	@echo "export AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY)" >> $@
	@echo "export AWS_REGION=$(AWS_REGION)" >> $@
	@echo "export AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID)" >> $@
	@echo "export REGION=$(AWS_REGION)" >> $@
	@echo "export ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID)" >> $@
	@echo "export SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY)" >> $@
	@echo "export ACCOUNT_ID=$(ACCOUNT_ID)" >> $@
	@echo "export DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX)" >> $@
	@echo "export DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT)" >> $@
	@echo "export RUST_LOG=$(RUST_LOG)" >> $@
	@echo "export ENV=$(ENV)" >> $@
	@echo "export COMMIT=$(COMMIT)" >> $@
	@echo "export PORT=$(PORT)" >> $@
	@echo "export SERVICE=$(SERVICE)" >> $@
	@echo "export DOMAIN=$(DOMAIN)" >> $@
	@echo "export REDIRECT_DOMAIN=$(REDIRECT_DOMAIN)" >> $@
	@echo "export BUCKET_NAME=$(BUCKET_NAME)" >> $@
	@echo "export SLACK_HOOK=$(SLACK_HOOK)" >> $@
	@echo "export DISABLE_ANSI=$(DISABLE_ANSI)" >> $@
	@echo "export ECR=$(ECR)" >> $@
	@echo "export MOBILE_API_URL=$(MOBILE_API_URL)" >> $@
	@echo "export PORTONE_STORE_ID=$(PORTONE_STORE_ID)" >> $@
	@echo "export PORTONE_KPN_CHANNEL_KEY=$(PORTONE_KPN_CHANNEL_KEY)" >> $@
	@echo "export PORTONE_INICIS_CHANNEL_KEY=$(PORTONE_INICIS_CHANNEL_KEY)" >> $@
	@echo "export PORTONE_API_SECRET='$(PORTONE_API_SECRET)'" >> $@
	@echo "export KAIA_ENDPOINT=$(KAIA_ENDPOINT)" >> $@
	@echo "export KAIA_ARCHIVE_ENDPOINT=$(KAIA_ARCHIVE_ENDPOINT)" >> $@
	@echo "export BLOCK_EXPLORER_URL=$(BLOCK_EXPLORER_URL)" >> $@
	@echo "export KAIA_FEEPAYER_KEY='$(KAIA_FEEPAYER_KEY)'" >> $@
	@echo "export KAIA_FEEPAYER_ADDR=$(KAIA_FEEPAYER_ADDR)" >> $@
	@echo "export KAIA_OWNER_KEY='$(KAIA_OWNER_KEY)'" >> $@
	@echo "export KAIA_OWNER_ADDR=$(KAIA_OWNER_ADDR)" >> $@
	@echo "export BINANCE_API_KEY='$(BINANCE_API_KEY)'" >> $@
	@echo "export BINANCE_SECRET_KEY='$(BINANCE_SECRET_KEY)'" >> $@
	@echo "export BINANCE_WEBHOOK=$(BINANCE_WEBHOOK)" >> $@
	@echo "export TELEGRAM_TOKEN='$(TELEGRAM_TOKEN)'" >> $@
	@echo "export BBS_BLS_X='$(BBS_BLS_X)'" >> $@
	@echo "export BBS_BLS_Y='$(BBS_BLS_Y)'" >> $@
	@echo "export BBS_BLS_D='$(BBS_BLS_D)'" >> $@
	@echo "export BBS_BLS_CRV='$(BBS_BLS_CRV)'" >> $@
	@echo "export P256_X='$(P256_X)'" >> $@
	@echo "export P256_Y='$(P256_Y)'" >> $@
	@echo "export P256_D='$(P256_D)'" >> $@
	@echo "export P256_CRV='$(P256_CRV)'" >> $@
	@echo "export FCM_ENABLED=$(FCM_ENABLED)" >> $@
	@echo "export RATEL_PROJECT_ID=$(RATEL_PROJECT_ID)" >> $@
	@echo "export REWARD=$(REWARD)" >> $@
	@echo "export BEDROCK_AGENT_ID='$(BEDROCK_AGENT_ID)'" >> $@
	@echo "export BEDROCK_AGENT_ALIAS_ID='$(BEDROCK_AGENT_ALIAS_ID)'" >> $@
	@echo "export BIYARD_API_KEY='$(BIYARD_API_KEY)'" >> $@
	@echo "export BIYARD_PROJECT_ID='$(BIYARD_PROJECT_ID)'" >> $@
	@echo "export FIREBASE_API_KEY='$(FIREBASE_API_KEY)'" >> $@
	@echo "export FIREBASE_AUTH_DOMAIN=$(FIREBASE_AUTH_DOMAIN)" >> $@
	@echo "export FIREBASE_PROJECT_ID=$(FIREBASE_PROJECT_ID)" >> $@
	@echo "export FIREBASE_STORAGE_BUCKET=$(FIREBASE_STORAGE_BUCKET)" >> $@
	@echo "export FIREBASE_MESSAGING_SENDER_ID='$(FIREBASE_MESSAGING_SENDER_ID)'" >> $@
	@echo "export FIREBASE_APP_ID='$(FIREBASE_APP_ID)'" >> $@
	@echo "export FIREBASE_MEASUREMENT_ID=$(FIREBASE_MEASUREMENT_ID)" >> $@
	@echo "export QDRANT_URL=$(QDRANT_URL)" >> $@
	@echo "export QDRANT_API_KEY='$(QDRANT_API_KEY)'" >> $@
	@echo "export QDRANT_PREFIX=$(QDRANT_PREFIX)" >> $@
	@echo "export WALLETCONNECT_PROJECT_ID='$(WALLETCONNECT_PROJECT_ID)'" >> $@
	@echo "export RATEL_CANISTER_ID=$(RATEL_CANISTER_ID)" >> $@
	@echo "export IC_URL=$(IC_URL)" >> $@
	@bash -c 'printf "export ICP_IDENTITY_PEM=%q\n" "$$ICP_IDENTITY_PEM"' >> $@
	@bash -c 'printf "export VOTER_TAG_SECRET=%q\n" "$$VOTER_TAG_SECRET"' >> $@
	@bash -c 'printf "export ATTR_VOTING_AUTHORITY_JSON=%q\n" "$$ATTR_VOTING_AUTHORITY_JSON"' >> $@
	@echo "export HOST_UID='$(HOST_UID)'" >> $@
	@echo "export HOST_GID='$(HOST_GID)'" >> $@
	@echo "export BIYARD_API_URL='$(BIYARD_API_URL)'" >> $@
	@echo "export CROSS_POSTING_DATA_KEY='$(CROSS_POSTING_DATA_KEY)'" >> $@
	@echo "export CROSS_POSTING_DATA_KEY_PREVIOUS='$(CROSS_POSTING_DATA_KEY_PREVIOUS)'" >> $@


test:
	mkdir -p ./test-build-target/debug/deps/public
	npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
	$(BUILD_ENV) CARGO_TARGET_DIR=./test-build-target cargo test --features server,bypass --tests -- --test-threads=1

js/node_modules:
	npm i

../../node_modules:
	cd ../.. && npm i

run: ../../node_modules js/node_modules
	RUSTFLAGS=$(RUSTFLAGS) $(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --web --features $(RUN_FEATURES) --no-default-features

$(HOME)/.android/debug.keystore:
	keytool -genkey -v -keystore $@ -storepass android -keypass android -alias androiddebugkey -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Korea,O=Biyard,C=KR"

	keytool -list -v -keystore ~/.android/debug.keystore -alias androiddebugkey -storepass android | grep SHA1

$(HOME)/.android/avd/$(AVD_NAME).avd: $(ANDROID_HOME)/system-images/android-34/google_apis/$(ARCH)
	avdmanager create avd  -n $(AVD_NAME) -k "system-images;android-34;google_apis;$(ARCH)" -d "pixel_6"

$(ANDROID_HOME)/emulator/emulator:
	sdkmanager --install emulator

$(ANDROID_HOME)/system-images/android-34/google_apis/$(ARCH):
	sdkmanager --install "system-images;android-34;google_apis;$(ARCH)"

android-emulator: $(HOME)/.android/debug.keystore $(HOME)/.android/avd/$(AVD_NAME).avd $(ANDROID_HOME)/emulator/emulator
	$(ANDROID_HOME)/emulator/emulator -avd $(AVD_NAME) -netdelay none -netspeed full &

.PHONY: $(ANDROID_DX_RES_DIR)
$(ANDROID_DX_RES_DIR):
	mkdir -p $(ANDROID_DX_RES_DIR)
	cp -rf android/res $(ANDROID_DX_RES_DIR)/

android: $(HOME)/.android/debug.keystore $(ANDROID_DX_RES_DIR)
	RUSTFLAGS=$(RUSTFLAGS) $(BUILD_ENV) $(ANDROID_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --features $(RUN_FEATURES) --no-default-features @client --android --target $(ANDROID_TARGET) $(DEVICE_FLAG)

ios:
	RUSTFLAGS=$(RUSTFLAGS) $(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --ios --features $(RUN_FEATURES)

build:
	$(BUILD_ENV) dx build --release --no-default-features --debug-symbols false @client --features web --platform web @server --features server --platform server

build-testing:
	$(BUILD_ENV) dx build --release --no-default-features --debug-symbols false @client --features web,bypass,beta --platform web @server --features server,bypass,local-dev --platform server

build-arm:
	$(BUILD_ENV) dx build --release --no-default-features --debug-symbols false @client --features web --platform web @server --features server --platform server --target aarch64-unknown-linux-gnu

build-lambda: env.sh
	docker run -v $(PWD)/../..:/app -v $(HOME)/.cargo/bin:/root/.cargo/bin -w /app public.ecr.aws/sam/build-provided.al2023 /bin/bash -c "scripts/building-lambda-bootstrap.sh"

docker:
	docker build -t $(ECR):$(COMMIT) -f Dockerfile ../../target/dx/app-shell/release/web

docker.push:
	docker tag $(ECR):$(COMMIT) $(ECR):$(ENV)
	docker push $(ECR):$(COMMIT)
	docker push $(ECR):$(ENV)

docker.run:
	docker run -d --name ratel-app -e "IP=0.0.0.0" -p 8080:8080 $(ECR):$(COMMIT)

docker.rm:
	docker rm -f ratel-app

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

android.log:
	adb logcat -c && adb logcat -s RatelRust
