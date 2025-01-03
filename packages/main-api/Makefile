ENV ?= dev
BASE_DOMAIN ?= democrasee.me
DOMAIN ?= $(ENV).$(BASE_DOMAIN)

WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
REGION ?= $(shell aws configure get region)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
	TABLE_NAME = "dagit-prod"
endif

ifeq ("$(ENV)","dev")
	LOG_LEVEL ?= debug
endif

SECRET_TOKEN ?=
TABLE_NAME ?= "democrasee-dev"
LOG_LEVEL ?= debug
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

BUILD_ENV ?= ENV=$(ENV) LOG_LEVEL=$(LOG_LEVEL) NODE=$(NODE) TABLE_NAME=$(TABLE_NAME) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) COMMIT=$(COMMIT) SECRET_TOKEN=$(SECRET_TOKEN) DOMAIN=$(DOMAIN)

run:
	$(BUILD_ENV) cargo watch -x run -w src

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(WORKSPACE_ROOT)/target/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap
