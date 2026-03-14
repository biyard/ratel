ENV ?= dev

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
DYNAMO_ENDPOINT ?= none
KAIA_ENDPOINT ?= https://public-en-kairos.node.kaia.io
BLOCK_EXPLORER_URL ?= https://kairos.kaiascan.io
WALLETCONNECT_PROJECT_ID ?= "WALLETCONNECT_PROJECT_ID"
RUST_LOG ?= debug

PORT ?= 8080

REPOSITORY ?= ratel/app-shell
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY)  --query "repositories[0].repositoryUri" --output text)

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
					PORTONE_KPN_CHANNEL_KEY=$(PORTONE_KPN_CHANNEL_KEY) \
					PORTONE_INICIS_CHANNEL_KEY=$(PORTONE_INICIS_CHANNEL_KEY) \
					PORTONE_API_SECRET=$(PORTONE_API_SECRET) \
					KAIA_ENDPOINT=$(KAIA_ENDPOINT) \
					BLOCK_EXPLORER_URL=$(BLOCK_EXPLORER_URL) \
					FIREBASE_API_KEY=$(FIREBASE_API_KEY) \
					FIREBASE_AUTH_DOMAIN=$(FIREBASE_AUTH_DOMAIN) \
					FIREBASE_PROJECT_ID=$(FIREBASE_PROJECT_ID) \
					FIREBASE_STORAGE_BUCKET=$(FIREBASE_STORAGE_BUCKET) \
					FIREBASE_MESSAGING_SENDER_ID=$(FIREBASE_MESSAGING_SENDER_ID) \
					FIREBASE_APP_ID=$(FIREBASE_APP_ID) \
					FIREBASE_MEASUREMENT_ID=$(FIREBASE_MEASUREMENT_ID) \

RUST_FLAG ?=

test:
	mkdir -p ./test-build-target/debug/deps/public
	npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
	$(BUILD_ENV) CARGO_TARGET_DIR=./test-build-target cargo test --features server,bypass --tests

run:
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --web

build:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features server --platform server

build-arm:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features server --platform server --target aarch64-unknown-linux-gnu

build-lambda:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features lambda --platform server

docker:
	docker build -t $(ECR):$(COMMIT) -f Dockerfile ../../target/dx/app-shell/release/web

docker.push:
	docker tag $(ECR):$(COMMIT) $(ECR):$(ENV)
	docker push $(ECR):$(COMMIT)
	docker push $(ECR):$(ENV)

docker.run:
	docker run -d --name ratel-app -e "IP=0.0.0.0" -p 8080:8080 $(ECR):$(COMMIT)

docker.rm:
	docker rm -f ratel-app

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com
