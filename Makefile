ENV ?= dev
BASE_DOMAIN ?= ratel.foundation
DOMAIN ?= $(ENV).$(BASE_DOMAIN)

HOSTED_ZONE_ID ?= $(shell aws route53 list-hosted-zones-by-name --dns-name $(BASE_DOMAIN) --query "HostedZones[0].Id" --output text | cut -d'/' -f3)
PROJECT ?= $(shell basename `git rev-parse --show-toplevel`)
SERVICE ?= main-ui
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
WORKSPACE_ROOT ?= $(PWD)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
VPC_ID ?= $(shell aws ec2 describe-vpcs --query "Vpcs[0].VpcId" --output json | tr -d \")
API_PREFIX ?=

STACK ?= ratel-dev-stack

WEB_REPO_NAME ?= ratel/web
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(WEB_REPO_NAME)  --query "repositories[0].repositoryUri" --output text)

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) BASE_DOMAIN=$(BASE_DOMAIN) PROJECT=$(PROJECT) STACK=$(STACK)


.build/evm-keys:
	mkdir -p .build
	docker run --rm ghcr.io/foundry-rs/foundry:latest "cast wallet new --json" > .build/evm-keys.json

run: .build/evm-keys
	COMMIT=${COMMIT} docker compose --profile development up -d --remove-orphans

stop:
	docker compose --profile development down --remove-orphans

serve:
	cd packages/$(SERVICE) && make serve

clean:
	rm -rf .build/$(SERVICE)

.PHONY: deploy
deploy: build cdk-deploy

.PHONY: build
build: clean
	mkdir -p .build
	cd packages/$(SERVICE) && ENV=$(ENV) ARTIFACT_DIR=$(PWD)/.build/$(SERVICE) make build$(DOCKER_COMMAND_SUFFUIX)

.PHONY: build-with-web
build-with-web: clean
	mkdir -p .build
	@echo "Building main-api..."
	cd packages/main-api && ENV=$(ENV) ARTIFACT_DIR=$(PWD)/.build/main-api make build
	@echo "Building web..."
	cd ts-packages/web && ENV=$(ENV) make build

cdk/.next:
	docker create --name web-container $(ECR_NAME):$(COMMIT)
	docker cp web-container:/app/ts-packages/web/.next cdk/.next
	docker rm -f web-container

cdk/public:
	cp -r ts-packages/web/public cdk/public

cdk/.build/%/bootstrap:
	mkdir -p cdk/.build/$*
	cp .build/$*/bootstrap cdk/.build/$*/bootstrap

cdk-deploy-v2:
	cd cdk && npm i
	cd cdk && $(BUILD_CDK_ENV) npm run build
	cd cdk && $(BUILD_CDK_ENV) cdk synth
	cd cdk && $(BUILD_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG) --all --concurrency 3

cdk-deploy: deps/rust-sdk/cdk/node_modules
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) npm run build
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk synth
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk deploy --require-approval never $(AWS_FLAG) --all

DEPLOY_AGENT_ENV ?= AGENT_NAME=$(AGENT_NAME) AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION)
# FIXME: Use cdk-deploy-v2 with flag
cdk-deploy-ai-stack:
	cd cdk && npm i
	cd cdk && $(DEPLOY_AGENT_ENV) npm run build
	cd cdk && $(DEPLOY_AGENT_ENV) cdk synth
	cd cdk && $(DEPLOY_AGENT_ENV) cdk deploy --require-approval never $(AWS_FLAG) --all --concurrency 3 --app "npx ts-node bin/cdk-ai.ts" 

node_modules:
	pnpm i

infra: .build/evm-keys
	docker compose --profile infra up -d --remove-orphans
