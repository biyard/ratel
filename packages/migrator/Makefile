AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

DYNAMO_ENDPOINT ?= http://localstack:4566
DYNAMO_TABLE_PREFIX ?= ratel-local

RUST_LOG ?= debug
RUST_FLAG ?= "--timings"

RUN_ENV ?= \
	RUST_LOG=$(RUST_LOG) \
	AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
	AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
	AWS_REGION=$(AWS_REGION) \
	DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX)

run:
	$(RUN_ENV) cargo run $(RUST_FLAG)
