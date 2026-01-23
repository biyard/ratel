PROJECT ?= ratel
SERVICE ?= main-api
ENV ?= local
REDIRECT_DOMAIN ?= https://dev.ratel.foundation
AUTH_DOMAIN ?= dev.ratel.foundation
DOMAIN ?= dev.ratel.foundation
BINANCE_WEBHOOK ?= https://dev.ratel.foundation/v2/binances/webhooks 

WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
RUSTFLAGS ?= -D warnings
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build
CARGO_TARGET_DIR ?= $(PWD)/target

RUST_FLAG ?= "--timings"

SECRET_TOKEN ?=
RUST_LOG ?= debug
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
MIGRATE ?= false

WATERMARK_QUEUE_URL ?= https://sqs.ap-northeast-2.amazonaws.com/385474633683/dev-artwork-image-queue

ACCOUNT_ID ?=

BUCKET_NAME ?= metadata.ratel.foundation
BUCKET_EXPIRE ?= 3600
ASSET_DIR ?= metadata
S3_REGION ?= ap-northeast-2

CHIME_BUCKET ?= metadata.ratel.foundation
NOVA_MICRO_MODEL_ID ?= dummy
NOVA_LITE_MODEL_ID ?= dummy
PRIVATE_BUCKET_NAME ?= private.ratel.foundation

BEDROCK_AGENT_ID ?= dummy
BEDROCK_AGENT_ALIAS_ID ?= dummy

REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)

DATABASE_TYPE ?= postgres
DATABASE_URL ?= "postgresql://postgres:postgres@postgres:5432/ratel"
DYNAMO_ENDPOINT ?= http://localstack:4566
DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
KAIA_ENDPOINT ?= https://public-en-kairos.node.kaia.io
KAIA_OWNER_ADDR ?= $(strip $(shell ./key.sh | awk '{print $$1}'))
KAIA_OWNER_KEY ?= $(strip $(shell ./key.sh | awk '{print $$2}'))
KAIA_FEEPAYER_ADDR ?= $(strip $(shell ./key.sh | awk '{print $$1}'))
KAIA_FEEPAYER_KEY ?= $(strip $(shell ./key.sh | awk '{print $$2}'))
TELEGRAM_TOKEN ?=
FIREBASE_PROJECT_ID ?=
PORTONE_API_SECRET ?=

BIYARD_API_URL ?= https://api.dev.biyard.co
BIYARD_API_KEY ?= biyard_default_api_key
BIYARD_PROJECT_ID ?= ratel

RATEL_PROJECT_ID ?= 
FCM_ENABLED ?= false

REWARD ?= true


BUILD_ENV ?= ENV=$(ENV) \
								RUST_LOG=$(RUST_LOG) \
								AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
								AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
								AWS_REGION=$(AWS_REGION) \
								VERSION=$(VERSION) \
								COMMIT=$(COMMIT) \
								SECRET_TOKEN=$(SECRET_TOKEN) \
								DOMAIN=$(DOMAIN) \
								DATABASE_TYPE=$(DATABASE_TYPE) \
								DATABASE_URL=$(DATABASE_URL) \
								AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) \
								BASE_DOMAIN=$(BASE_DOMAIN) \
								RUSTFLAGS="$(RUSTFLAGS)" \
								API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) \
								CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
								AUTH_DOMAIN=$(AUTH_DOMAIN) \
								MIGRATE=$(MIGRATE) \
								SLACK_CHANNEL_BILL=$(SLACK_CHANNEL_BILL) \
								SLACK_CHANNEL_SPONSOR=$(SLACK_CHANNEL_SPONSOR) \
								SLACK_CHANNEL_ABUSING=$(SLACK_CHANNEL_ABUSING) \
								SLACK_CHANNEL_MONITOR=$(SLACK_CHANNEL_MONITOR) \
								BUCKET_NAME=$(BUCKET_NAME) \
								ASSET_DIR=$(ASSET_DIR) \
								BUCKET_EXPIRE=$(BUCKET_EXPIRE) \
								S3_REGION=$(S3_REGION) \
								CHIME_BUCKET=$(CHIME_BUCKET) \
								NONCELAB_TOKEN=$(NONCELAB_TOKEN) \
								WATERMARK_QUEUE_URL=$(WATERMARK_QUEUE_URL) \
								PRIVATE_BUCKET_NAME=$(PRIVATE_BUCKET_NAME) \
								KAIA_ENDPOINT=$(KAIA_ENDPOINT) \
								KAIA_OWNER_ADDR=$(KAIA_OWNER_ADDR) \
								KAIA_OWNER_KEY=$(KAIA_OWNER_KEY) \
								KAIA_FEEPAYER_ADDR=$(KAIA_FEEPAYER_ADDR) \
								KAIA_FEEPAYER_KEY=$(KAIA_FEEPAYER_KEY) \
								JWT_SECRET_KEY=$(JWT_SECRET_KEY) \
								BBS_BLS_X=$(BBS_BLS_X) \
								BBS_BLS_Y=$(BBS_BLS_Y) \
								BBS_BLS_D=$(BBS_BLS_Z) \
								P256_X=$(P256_X) \
								P256_Y=$(P256_Y) \
								P256_D=$(P256_D) \
								BBS_BLS_CRV=$(BBS_BLS_CRV) \
								P256_CRV=$(P256_CRV) \
								NOVA_LITE_MODEL_ID=$(NOVA_LITE_MODEL_ID) \
								NOVA_MICRO_MODEL_ID=$(NOVA_MICRO_MODEL_ID) \
								NOVA_LITE_MODEL_ID=$(NOVA_LITE_MODEL_ID) \
								PRIVATE_BUCKET_NAME=$(PRIVATE_BUCKET_NAME) \
								TELEGRAM_TOKEN=$(TELEGRAM_TOKEN) \
								DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
								DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
								BINANCE_API_KEY=$(BINANCE_API_KEY) \
								BINANCE_SECRET_KEY=$(BINANCE_SECRET_KEY) \
								BINANCE_WEBHOOK=$(BINANCE_WEBHOOK) \
								REDIRECT_DOMAIN=$(REDIRECT_DOMAIN) \
								FIREBASE_PROJECT_ID=$(FIREBASE_PROJECT_ID) \
								PORTONE_API_SECRET=$(PORTONE_API_SECRET) \
								PORTONE_KPN_CHANNEL_KEY=$(PORTONE_KPN_CHANNEL_KEY) \
								PORTONE_STORE_ID=$(PORTONE_STORE_ID) \
								ACCOUNT_ID=$(ACCOUNT_ID) \
								RATEL_PROJECT_ID=$(RATEL_PROJECT_ID) \
								FCM_ENABLED=$(FCM_ENABLED) \
								REWARD=$(REWARD) \
								BIYARD_API_URL=$(BIYARD_API_URL) \
								BIYARD_API_KEY=$(BIYARD_API_KEY) \
								BIYARD_PROJECT_ID=$(BIYARD_PROJECT_ID) \
								BEDROCK_AGENT_ID=$(BEDROCK_AGENT_ID) \
								BEDROCK_AGENT_ALIAS_ID=$(BEDROCK_AGENT_ALIAS_ID)

.build/evm-keys:
	docker run --rm ghcr.io/foundry-rs/foundry:latest "cast wallet new --json" > .build/evm-keys.json

dist:
	cd ../../ts-packages/web && make build
	cp -r ../../ts-packages/web/dist ./

WATCH_TARGET = -w src -w $(PWD)/.. 
# if web build is true, then also watch web files
ifeq ($(WEB_BUILD),"true")
	WATCH_TARGET += -w $(PWD)/../../ts-packages/web
endif

run:
	$(BUILD_ENV) cargo watch -x "run $(RUST_FLAG)" $(WATCH_TARGET)

build:
	$(eval WEB_INDEX_JS=$(shell basename $(shell ls dist/assets/index-*.js)))
	$(eval WEB_INDEX_CSS=$(shell basename $(shell ls dist/assets/index-*.css)))

	$(BUILD_ENV) cargo build $(RUST_FLAG)

build-only:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap

build-lambda:
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) amazonlinux/2023 bash -c 'source ~/.cargo/env && $(BUILD_ENV) cargo build $(RUST_FLAG)'


TEST_TARGET ?= controllers::v3 models::dynamo_tables features

test:
	$(BUILD_ENV) WEB_BUILD="false" RUSTFLAGS="-A warnings" cargo test $(RUST_FLAG) --lib -j 1 --  --test-threads=1 $(TEST_TARGET) --show-output

docker.build:
	$(eval WEB_INDEX_JS=$(shell basename $(shell ls dist/assets/index-*.js)))
	$(eval WEB_INDEX_CSS=$(shell basename $(shell ls dist/assets/index-*.css)))

	docker run --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) rust:slim-bookworm sh -c '\
		apt-get update && apt-get install -y \
			libssl-dev \
			ca-certificates \
			pkg-config \
			g++ \
			libfontconfig1-dev \
			libfreetype6-dev \
		&& $(BUILD_ENV) WEB_BUILD=false WEB_INDEX_JS=$(WEB_INDEX_JS) WEB_INDEX_CSS=$(WEB_INDEX_CSS) CARGO_TARGET_DIR=/app/packages/$(SERVICE)/target cargo build --release $(RUST_FLAG)'

	docker run --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) rust:slim-bookworm sh -c 'cp -rf dist target/release/'
	docker build -t $(ECR_NAME):$(COMMIT) --no-cache -f Dockerfile $(PWD)/target/release
docker.push:
	docker push $(ECR_NAME):$(COMMIT)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

mcp-inspector:
	npx @modelcontextprotocol/inspector