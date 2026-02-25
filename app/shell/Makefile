ENV ?= dev

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
DYNAMO_ENDPOINT ?= none

RUST_LOG ?= debug

PORT ?= 8000

BUILD_ENV ?= AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
					AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
					AWS_REGION=$(AWS_REGION) \
					AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) \
					DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
					DYNAMODB_ENDPOINT=$(DYNAMO_ENDPOINT) \
					RUST_LOG=$(RUST_LOG) \
					ENV=$(ENV) \
					COMMIT=$(COMMIT) \
					RUST_LOG=$(RUST_LOG) \
					PORT=$(PORT) \
					PORTONE_STORE_ID=$(PORTONE_STORE_ID) \
					PORTONE_INICIS_CHANNEL_KEY=$(PORTONE_INICIS_CHANNEL_KEY) \
					PORTONE_API_SECRET=$(PORTONE_API_SECRET) \
					FIREBASE_API_KEY=$(FIREBASE_API_KEY) \
					FIREBASE_AUTH_DOMAIN=$(FIREBASE_AUTH_DOMAIN) \
					FIREBASE_PROJECT_ID=$(FIREBASE_PROJECT_ID) \
					FIREBASE_STORAGE_BUCKET=$(FIREBASE_STORAGE_BUCKET) \
					FIREBASE_MESSAGING_SENDER_ID=$(FIREBASE_MESSAGING_SENDER_ID) \
					FIREBASE_APP_ID=$(FIREBASE_APP_ID) \
					FIREBASE_MEASUREMENT_ID=$(FIREBASE_MEASUREMENT_ID) \
					UPSTREAM_URL=$(UPSTREAM_URL)

RUST_FLAG ?=

run:
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --web

build:
	$(BUILD_ENV) dx build --web --release

docker:
	docker build -t ratel:$(COMMIT) -f Dockerfile .
