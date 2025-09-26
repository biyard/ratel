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

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE) VPC_ID=$(VPC_ID) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) ENABLE_S3=$(ENABLE_S3) ENABLE_DYNAMO=$(ENABLE_DYNAMO) ENABLE_FARGATE=$(ENABLE_FARGATE) ENABLE_LAMBDA=$(ENABLE_LAMBDA) ENABLE_OPENSEARCH=$(ENABLE_OPENSEARCH) BASE_DOMAIN=$(BASE_DOMAIN) PROJECT=$(PROJECT) STACK=$(STACK) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID)


# Playwright test envs
RATEL_TEST_PLAYWRIGHT_URL ?= http://localhost:8080
PLAYWRIGHT_ENV ?= RATEL_TEST_PLAYWRIGHT_URL=$(RATEL_TEST_PLAYWRIGHT_URL)


.build/evm-keys:
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

node_modules:
	pnpm i

test: node_modules
	$(PLAYWRIGHT_ENV ) npx playwright test
