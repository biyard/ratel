PROJECT ?= ratel
SERVICE ?= main-api
ENV ?= dev
REDIRECT_DOMAIN ?= https://dev.ratel.foundation
AUTH_DOMAIN ?= dev.ratel.foundation
DOMAIN ?= dev.ratel.foundation
BINANCE_WEBHOOK ?= https://20e0ea654b53.ngrok-free.app/v2/binances/webhooks 

WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
RUSTFLAGS ?= -D warnings
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build/api-models
CARGO_TARGET_DIR ?= $(PWD)/target
HASURA_GRAPHQL_AUTH_HOOK ?= https://api.dev.ratel.foundation/v1/auth/hasura

SECRET_TOKEN ?=
RUST_LOG ?= debug
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
MIGRATE ?= true

RDS_RESOURCE_ARN ?= "arn:aws:rds:ap-northeast-2:385474633683:cluster:dev-infra-databaseb269d8bb-2w0xwgte5dr9"
RDS_SECRET_ARN ?= "arn:aws:secretsmanager:ap-northeast-2:385474633683:secret:dev/rds-eWj7pI"
WATERMARK_QUEUE_URL ?= https://sqs.ap-northeast-2.amazonaws.com/385474633683/dev-artwork-image-queue

OPENAPI_KEY ?=
SLACK_CHANNEL_BILL ?=
SLACK_CHANNEL_SPONSOR ?=
SLACK_CHANNEL_ABUSING ?=
SLACK_CHANNEL_MONITOR ?=

BUCKET_NAME ?= metadata.ratel.foundation
BUCKET_EXPIRE ?= 3600
ASSET_DIR ?= metadata

CHIME_BUCKET ?= metadata.ratel.foundation
NOVA_MICRO_MODEL_ID ?= arn:aws:bedrock:ap-northeast-2:385474633683:inference-profile/apac.amazon.nova-micro-v1:0
NOVA_LITE_MODEL_ID ?= arn:aws:bedrock:ap-northeast-2:385474633683:inference-profile/apac.amazon.nova-lite-v1:0
PRIVATE_BUCKET_NAME ?= private.ratel.foundation

REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)

TELEGRAM_TOKEN ?=
BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) NODE=$(NODE) TABLE_NAME=$(TABLE_NAME) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) COMMIT=$(COMMIT) SECRET_TOKEN=$(SECRET_TOKEN) DOMAIN=$(DOMAIN) DATABASE_TYPE=$(DATABASE_TYPE) DATABASE_URL=$(DATABASE_URL) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) BASE_DOMAIN=$(BASE_DOMAIN) RUSTFLAGS="$(RUSTFLAGS)" API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) AUTH_DOMAIN=$(AUTH_DOMAIN) MIGRATE=$(MIGRATE) OPENAPI_KEY=$(OPENAPI_KEY) SLACK_CHANNEL_BILL=$(SLACK_CHANNEL_BILL) SLACK_CHANNEL_SPONSOR=$(SLACK_CHANNEL_SPONSOR) SLACK_CHANNEL_ABUSING=$(SLACK_CHANNEL_ABUSING) SLACK_CHANNEL_MONITOR=$(SLACK_CHANNEL_MONITOR) BUCKET_NAME=$(BUCKET_NAME) ASSET_DIR=$(ASSET_DIR) BUCKET_EXPIRE=$(BUCKET_EXPIRE) CHIME_BUCKET=$(CHIME_BUCKET) NONCELAB_TOKEN=$(NONCELAB_TOKEN) RDS_RESOURCE_ARN=$(RDS_RESOURCE_ARN) RDS_SECRET_ARN=$(RDS_SECRET_ARN) HASURA_GRAPHQL_AUTH_HOOK=$(HASURA_GRAPHQL_AUTH_HOOK) WATERMARK_QUEUE_URL=$(WATERMARK_QUEUE_URL) PRIVATE_BUCKET_NAME=$(PRIVATE_BUCKET_NAME) KAIA_ENDPOINT=$(KAIA_ENDPOINT) KAIA_OWNER_ADDR=$(KAIA_OWNER_ADDR) KAIA_OWNER_KEY=$(KAIA_OWNER_KEY) KAIA_FEEPAYER_ADDR=$(KAIA_FEEPAYER_ADDR) KAIA_FEEPAYER_KEY=$(KAIA_FEEPAYER_KEY) JWT_SECRET_KEY=$(JWT_SECRET_KEY) BBS_BLS_X=$(BBS_BLS_X) BBS_BLS_Y=$(BBS_BLS_Y) BBS_BLS_D=$(BBS_BLS_Z) P256_X=$(P256_X) P256_Y=$(P256_Y) P256_D=$(P256_D) BBS_BLS_CRV=$(BBS_BLS_CRV) P256_CRV=$(P256_CRV) NOVA_LITE_MODEL_ID=$(NOVA_LITE_MODEL_ID) NOVA_MICRO_MODEL_ID=$(NOVA_MICRO_MODEL_ID) NOVA_LITE_MODEL_ID=$(NOVA_LITE_MODEL_ID) PRIVATE_BUCKET_NAME=$(PRIVATE_BUCKET_NAME) TELEGRAM_TOKEN=$(TELEGRAM_TOKEN) BINANCE_API_KEY=$(BINANCE_API_KEY) BINANCE_SECRET_KEY=$(BINANCE_SECRET_KEY) BINANCE_WEBHOOK=$(BINANCE_WEBHOOK) REDIRECT_DOMAIN=$(REDIRECT_DOMAIN)

run:
	$(BUILD_ENV) cargo watch -x run -w src -w $(PWD)/..

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap

test:
	$(BUILD_ENV) RUSTFLAGS="-D warnings" RUST_TEST_THREADS=1 cargo test

docker.build:
	docker run --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) rust:slim-bookworm sh -c 'apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && $(BUILD_ENV) CARGO_TARGET_DIR=/app/packages/$(SERVICE)/target cargo build --release'
	docker build -t $(ECR_NAME):$(COMMIT) --no-cache -f Dockerfile $(PWD)/target/release

docker.push:
	docker push $(ECR_NAME):$(COMMIT)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

mcp-inspector:
	npx @modelcontextprotocol/inspector
