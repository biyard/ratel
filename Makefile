ENV ?= dev
BASE_DOMAIN ?= ratel.foundation
DOMAIN ?= $(ENV).$(BASE_DOMAIN)
COMMIT ?= $(shell git rev-parse --short HEAD)

HOSTED_ZONE_ID ?= $(shell aws route53 list-hosted-zones-by-name --dns-name $(BASE_DOMAIN) --query "HostedZones[0].Id" --output text | cut -d'/' -f3)
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
WORKSPACE_ROOT ?= $(PWD)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
VPC_ID ?= $(shell aws ec2 describe-vpcs --query "Vpcs[0].VpcId" --output json | tr -d \")

STACK ?= ratel-$(ENV)-stack

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) WORKSPACE_ROOT=$(WORKSPACE_ROOT) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) BASE_DOMAIN=$(BASE_DOMAIN) STACK=$(STACK)


.build/evm-keys:
	mkdir -p .build
	docker run --rm ghcr.io/foundry-rs/foundry:latest "cast wallet new --json" > .build/evm-keys.json

preview:
	dx server -p preview

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

cdk/.build/%/bootstrap:
	mkdir -p cdk/.build/$*
	cp .build/$*/bootstrap cdk/.build/$*/bootstrap

cdk-deploy-v2:
	cd cdk && npm i
	cd cdk && $(BUILD_CDK_ENV) npm run build
	cd cdk && $(BUILD_CDK_ENV) cdk synth
	cd cdk && $(BUILD_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG) --all --concurrency 3

DEPLOY_AGENT_ENV ?= AGENT_NAME=$(AGENT_NAME) AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION)
# FIXME: Use cdk-deploy-v2 with flag
cdk-deploy-ai-stack:
	cd cdk && npm i
	cd cdk && $(DEPLOY_AGENT_ENV) npm run build
	cd cdk && $(DEPLOY_AGENT_ENV) cdk synth --app "npx ts-node bin/cdk-ai.ts"

node_modules:
	pnpm i

infra: .build/evm-keys
	docker compose --profile infra up -d --remove-orphans

testing: .build/evm-keys
	COMMIT=$(COMMIT) RESET_DB=true docker compose --profile testing up -d --remove-orphans

clean-infra:
	docker compose --profile infra down
