
ENV ?= dev

REGION ?= $(shell aws configure get region)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")

WORKSPACE_ROOT ?= $(PWD)/../..
CARGO_TARGET_DIR ?= $(PWD)/target

RUSTFLAGS ?= -D warnings

RUST_LOG ?= debug
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

BUCKET_NAME ?= metadata.ratel.foundation
BUCKET_EXPIRE ?= 3600
ASSET_DIR ?= metadata

DATABASE_TYPE ?= postgres

BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) COMMIT=$(COMMIT) BUCKET_NAME=$(BUCKET_NAME) ASSET_DIR=$(ASSET_DIR) BUCKET_EXPIRE=$(BUCKET_EXPIRE) DATABASE_TYPE=$(DATABASE_TYPE)

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap

