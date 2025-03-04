PROJECT ?= ratel
SERVICE ?= main-api
ENV ?= dev
AUTH_DOMAIN ?= dev.ratel.foundation
DOMAIN ?= $(ENV).$(BASE_DOMAIN)

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

BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) NODE=$(NODE) TABLE_NAME=$(TABLE_NAME) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) COMMIT=$(COMMIT) SECRET_TOKEN=$(SECRET_TOKEN) DOMAIN=$(DOMAIN) DATABASE_TYPE=$(DATABASE_TYPE) DATABASE_URL=$(DATABASE_URL) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) BASE_DOMAIN=$(BASE_DOMAIN) RUSTFLAGS="$(RUSTFLAGS)" API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) AUTH_DOMAIN=$(AUTH_DOMAIN)

run:
	$(BUILD_ENV) cargo watch -x run -w src

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap

test:
	$(BUILD_ENV) cargo test --all