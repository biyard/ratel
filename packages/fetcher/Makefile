PROJECT ?= ratel
SERVICE ?= fetcher
ENV ?= local
AUTH_DOMAIN ?= dev.ratel.foundation
DOMAIN ?= dev.ratel.foundation

WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
RUSTFLAGS ?= -D warnings
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build/api-models
CARGO_TARGET_DIR ?= $(PWD)/target

SECRET_TOKEN ?=
RUST_LOG ?= debug
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
MIGRATE ?= true
REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)
OPENAPI_KEY ?=
SLACK_CHANNEL_BILL ?=
SERVER_KEY ?=
US_CONGRESS_KEY ?=
RPC_URL ?= https://public-en-kairos.node.kaia.io
TELEGRAM_TOKEN ?=
BUILD_ENV ?= ENV=$(ENV) \
                                RUST_LOG=$(RUST_LOG) \
                                NODE=$(NODE) \
                                TABLE_NAME=$(TABLE_NAME) \
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
                                OPENAPI_KEY=$(OPENAPI_KEY) \
                                SLACK_CHANNEL_BILL=$(SLACK_CHANNEL_BILL) \
                                SERVER_KEY=$(SERVER_KEY) \
                                US_CONGRESS_KEY=$(US_CONGRESS_KEY) \
                                RPC_ENDPOINT=$(RPC_ENDPOINT) \
                                DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
                                TELEGRAM_TOKEN=$(TELEGRAM_TOKEN) \
                                RPC_URL=$(RPC_URL) \
                                DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
								DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX)

run:
	$(BUILD_ENV) cargo watch -x run -w src -w $(PWD)/..

build: docker.login docker.build docker.push

bin.build:
	$(BUILD_ENV) cargo build --release

docker.push:
	docker push $(ECR_NAME):$(COMMIT)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

docker.build:
	docker run --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) rust:slim-bookworm sh -c '\
		apt-get update && apt-get install -y \
			libssl-dev \
			pkg-config \
			ca-certificates \
			g++ \
			libfreetype6-dev \
			libfontconfig1-dev \
		&& $(BUILD_ENV) WEB_BUILD=false CARGO_TARGET_DIR=/app/packages/$(SERVICE)/target cargo build --release'
	docker build -t $(ECR_NAME):$(COMMIT) --no-cache -f Dockerfile $(PWD)/target/release
    
test:
	$(BUILD_ENV) RUSTFLAGS="-D warnings" RUST_TEST_THREADS=1 cargo test
