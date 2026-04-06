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

DFX_JSON ?= ../../packages/ratel-canister/dfx.json
CANISTER_IDS_JSON ?= ../../packages/ratel-canister/canister_ids.json

ifeq ($(ENV),prod)
IC_NETWORK ?= ic
IC_URL ?= https://ic0.app
ICP_IDENTITY_PEM ?=
else
IC_NETWORK ?= dev
IC_URL ?= $(shell jq -r '.networks.$(IC_NETWORK).providers[0] // empty' $(DFX_JSON) 2>/dev/null)
ICP_IDENTITY_PEM ?= $(shell cat ~/.config/dfx/identity/default/identity.pem 2>/dev/null | tr '\n' '|' | sed 's/|/\\n/g')
endif
RATEL_CANISTER_ID ?= $(shell jq -r '.["ratel-canister"].$(IC_NETWORK) // empty' $(CANISTER_IDS_JSON) 2>/dev/null)
VOTER_TAG_SECRET ?= test
# cargo run -p attr-voting --bin attr-voting-keygen
ATTR_VOTING_AUTHORITY_JSON ?=
RUST_LOG ?= debug

PORT ?= 8080
RUN_FEATURES ?= local-dev

REPOSITORY ?= ratel/app-shell
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY)  --query "repositories[0].repositoryUri" --output text)
QDRANT_PREFIX ?= ratel-$(ENV)

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
					IC_URL=$(IC_URL) \
					RATEL_CANISTER_ID=$(RATEL_CANISTER_ID) \
					ICP_IDENTITY_PEM='$(ICP_IDENTITY_PEM)' \
					VOTER_TAG_SECRET='$(VOTER_TAG_SECRET)' \
					ATTR_VOTING_AUTHORITY_JSON='$(ATTR_VOTING_AUTHORITY_JSON)' \
					QDRANT_URL=$(QDRANT_URL) \
					QDRANT_API_KEY=$(QDRANT_API_KEY) \
					QDRANT_PREFIX=$(QDRANT_PREFIX) \

test:
	mkdir -p ./test-build-target/debug/deps/public
	npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
	$(BUILD_ENV) CARGO_TARGET_DIR=./test-build-target cargo test --features server,bypass --tests -- --test-threads=1

run:
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --web --features $(RUN_FEATURES)


build:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features server --platform server

build-testing:
	$(BUILD_ENV) dx build --release --debug-symbols false @client --features web --platform web @server --features server,bypass --platform server

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
