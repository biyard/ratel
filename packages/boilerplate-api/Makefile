ENV ?= local

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
DYNAMO_ENDPOINT ?= http://localhost:4566

RUST_LOG ?= debug

PORT ?= 3000

BUILD_ENV ?= AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
					AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
					AWS_REGION=$(AWS_REGION) \
					AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) \
					DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
					DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
					RUST_LOG=$(RUST_LOG) \
					ENV=$(ENV) \
					COMMIT=$(COMMIT) \
					RUST_LOG=$(RUST_LOG) \
					PORT=$(PORT)

RUST_FLAG ?=
WATCH_TARGET = -w src -w $(PWD)/

run:
	$(BUILD_ENV) cargo watch -x "run $(RUST_FLAG)" $(WATCH_TARGET)
