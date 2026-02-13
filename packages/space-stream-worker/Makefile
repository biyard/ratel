PROJECT ?= ratel
SERVICE ?= space-stream-worker
ENV ?= local
DOMAIN ?= dev.ratel.foundation

RUST_FLAG ?=
WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
RUSTFLAGS ?= -D warnings
CARGO_TARGET_DIR ?= $(PWD)/target

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

DYNAMO_ENDPOINT ?= http://localstack:4566
DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
PRIVATE_BUCKET_NAME ?= metadata.ratel.foundation
CHIME_BUCKET ?= ratel-$(ENV)-chime

BUILD_ENV ?= ENV=$(ENV) \
								AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
								AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
								AWS_REGION=$(AWS_REGION) \
								VERSION=$(VERSION) \
								COMMIT=$(COMMIT) \
								DOMAIN=$(DOMAIN) \
								AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) \
								RUSTFLAGS="$(RUSTFLAGS)" \
								CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
								DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
								DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
								PRIVATE_BUCKET_NAME=$(PRIVATE_BUCKET_NAME) \
								CHIME_BUCKET=$(CHIME_BUCKET)

run:
	$(BUILD_ENV) cargo run --features local-run $(RUST_FLAG)

watch:
	$(BUILD_ENV) cargo watch -x 'run --features local-run $(RUST_FLAG)' -w src -w $(PWD)/..

bin.build:
	$(BUILD_ENV) cargo build $(RUST_FLAG)

test:
	$(BUILD_ENV) RUSTFLAGS="-D warnings" RUST_TEST_THREADS=1 cargo test
