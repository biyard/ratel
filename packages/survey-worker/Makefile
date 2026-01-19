PROJECT ?= ratel
SERVICE ?= survey-worker
ENV ?= local
DOMAIN ?= dev.ratel.foundation

RUST_FLAG ?=
WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
RUSTFLAGS ?= -D warnings
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build
CARGO_TARGET_DIR ?= $(PWD)/target

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)

DYNAMO_ENDPOINT ?= http://localstack:4566
DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
BUCKET_NAME ?= metadata.ratel.foundation
ASSET_DIR ?= metadata
BUCKET_EXPIRE ?= 3600
S3_REGION ?= ap-northeast-2

BUILD_ENV ?= ENV=$(ENV) \
								AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
								AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
								AWS_REGION=$(AWS_REGION) \
								VERSION=$(VERSION) \
								COMMIT=$(COMMIT) \
								DOMAIN=$(DOMAIN) \
								AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) \
								RUSTFLAGS="$(RUSTFLAGS)" \
								API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) \
								CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
								DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
								DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
								BUCKET_NAME=$(BUCKET_NAME) \
								ASSET_DIR=$(ASSET_DIR) \
								BUCKET_EXPIRE=$(BUCKET_EXPIRE) \
								S3_REGION=$(S3_REGION)

run:
	$(BUILD_ENV) cargo run --features local-run

watch:
	$(BUILD_ENV) cargo watch -x 'run --features local-run' -w src -w $(PWD)/..

build: docker.login docker.build docker.push

bin.build:
	$(BUILD_ENV) cargo build $(RUST_FLAG)

docker.push:
	docker push $(ECR_NAME):$(COMMIT)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

docker.build:
	docker run --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) rust:slim-bookworm sh -c 'apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && $(BUILD_ENV) WEB_BUILD=false CARGO_TARGET_DIR=/app/packages/$(SERVICE)/target cargo build --release $(RUST_FLAG)'
	docker build -t $(ECR_NAME):$(COMMIT) --no-cache -f Dockerfile $(PWD)/target/release
	
test:
	$(BUILD_ENV) RUSTFLAGS="-D warnings" RUST_TEST_THREADS=1 cargo test
