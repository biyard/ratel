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
RUN_FEATURES ?= local-dev

# Auto-detect the host's LAN IP so Android devices / emulators on the same
# network can reach `dx serve` running on this workstation. Tries Linux
# (`ip route get`) then macOS (`ipconfig getifaddr en0`); falls back to the
# Android emulator loopback alias `10.0.2.2`, which works for emulators but
# not physical phones. Override explicitly when needed:
#   MOBILE_API_URL=http://10.20.30.40:8080 make android
HOST_IP ?= $(shell ip route get 1.1.1.1 2>/dev/null | awk '/src/ {for(i=1;i<=NF;i++) if($$i=="src") {print $$(i+1); exit}}' || ipconfig getifaddr en0 2>/dev/null || echo 10.0.2.2)
MOBILE_API_URL ?= http://$(HOST_IP):$(PORT)

REPOSITORY ?= ratel/app-shell
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY)  --query "repositories[0].repositoryUri" --output text)
QDRANT_PREFIX ?= ratel-$(ENV)
QDRANT_URL ?= http://localhost:6334
QDRANT_API_KEY ?= ""

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

test:
	mkdir -p ./test-build-target/debug/deps/public
	npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
	$(BUILD_ENV) CARGO_TARGET_DIR=./test-build-target cargo test --features server,bypass --tests -- --test-threads=1

run:
	RUSTFLAGS=$(RUSTFLAGS) $(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --web --features $(RUN_FEATURES)

ANDROID_NDK_HOME ?= /opt/android-ndk
ANDROID_NDK_TOOLCHAIN ?= $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64
ANDROID_API_LEVEL ?= 24
ANDROID_TARGET ?= aarch64-linux-android
ANDROID_CC ?= $(ANDROID_NDK_TOOLCHAIN)/bin/$(ANDROID_TARGET)$(ANDROID_API_LEVEL)-clang

# Gradle + Android Gradle Plugin 8.x don't parse class-file major 70 (OpenJDK 26+).
# Force a JDK that Gradle supports; override ANDROID_JAVA_HOME if yours lives elsewhere.
ANDROID_JAVA_HOME ?= /usr/lib/jvm/java-17-openjdk

ANDROID_ENV = \
	JAVA_HOME=$(ANDROID_JAVA_HOME) \
	PATH="$(ANDROID_JAVA_HOME)/bin:$(PATH)" \
	CC_aarch64_linux_android=$(ANDROID_CC) \
	CXX_aarch64_linux_android=$(ANDROID_CC)++ \
	AR_aarch64_linux_android=$(ANDROID_NDK_TOOLCHAIN)/bin/llvm-ar \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(ANDROID_CC) \
	CC_armv7_linux_androideabi=$(ANDROID_NDK_TOOLCHAIN)/bin/armv7a-linux-androideabi$(ANDROID_API_LEVEL)-clang \
	CXX_armv7_linux_androideabi=$(ANDROID_NDK_TOOLCHAIN)/bin/armv7a-linux-androideabi$(ANDROID_API_LEVEL)-clang++ \
	AR_armv7_linux_androideabi=$(ANDROID_NDK_TOOLCHAIN)/bin/llvm-ar \
	CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(ANDROID_NDK_TOOLCHAIN)/bin/armv7a-linux-androideabi$(ANDROID_API_LEVEL)-clang

$(HOME)/.android/debug.keystore:
	keytool -genkey -v -keystore $@ -storepass android -keypass android -alias androiddebugkey -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Korea,O=Biyard,C=KR"

	keytool -list -v -keystore ~/.android/debug.keystore -alias androiddebugkey -storepass android | grep SHA1

$(HOME)/.android/avd/Pixel_6_API_34.avd: $(ANDROID_HOME)/system-images/android-34/google_apis/x86_64
	avdmanager create avd  -n Pixel_6_API_34 -k "system-images;android-34;google_apis;x86_64" -d "pixel_6"

$(ANDROID_HOME)/emulator/emulator:
	sdkmanager --install emulator

$(ANDROID_HOME)/system-images/android-34/google_apis/x86_64:
	sdkmanager --install "system-images;android-34;google_apis;x86_64"

android-emulator: $(HOME)/.android/debug.keystore $(HOME)/.android/avd/Pixel_6_API_34.avd $(ANDROID_HOME)/emulator/emulator
	$(ANDROID_HOME)/emulator/emulator -avd Pixel_6_API_34 -netdelay none -netspeed full &

android: $(HOME)/.android/debug.keystore
	RUSTFLAGS=$(RUSTFLAGS) $(BUILD_ENV) $(ANDROID_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --android --target $(ANDROID_TARGET) --features $(RUN_FEATURES)

ios:
	RUSTFLAGS=$(RUSTFLAGS) $(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --ios --features $(RUN_FEATURES)

build:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features server --platform server

build-testing:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web,bypass --platform web @server --features server,bypass --platform server

build-arm:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features server --platform server --target aarch64-unknown-linux-gnu

build-lambda:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features lambda --platform server

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
