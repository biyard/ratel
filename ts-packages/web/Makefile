# For deploy
PROJECT ?= ratel
SERVICE ?= web
VERSION ?= $(shell git rev-parse --short HEAD)
COMMIT ?= $(shell git rev-parse --short HEAD)

REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
AWS_REGION ?= $(shell aws configure get region)

# For build
PORT ?= 8080

# BUILD_ENVS
VITE_ENV ?= dev
VITE_LOG_LEVEL ?= info
VITE_FIREBASE_API_KEY ?= dummy
VITE_FIREBASE_AUTH_DOMAIN ?= dummy
VITE_FIREBASE_PROJECT_ID ?= dummy
VITE_FIREBASE_STORAGE_BUCKET ?= dummy
VITE_FIREBASE_MESSAGING_SENDER_ID ?= dummy
VITE_FIREBASE_APP_ID ?= dummy
VITE_FIREBASE_MEASUREMENT_ID ?= dummy
VITE_SIGN_DOMAIN ?= dev.ratel.foundation
VITE_API_URL ?= http://localhost:3000
VITE_EXPERIMENT ?= false
VITE_TELEGRAM_BOTNAME ?= dummy
VITE_OPERATOR_ADDRESS ?= dummy
VITE_RPC_URL ?= https://public-en-kairos.node.kaia.io
VITE_USDT_ADDRESS ?= 0xd077a400968890eacc75cdc901f0356c943e4fdb
VITE_BLOCK_EXPLORER_URL ?= https://kairos.scope.kaia.io

ENV ?= dev

STACK ?= ratel-$(ENV)-stack

WEB_CDN_ID=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(STACK) \
  --query "Stacks[0].Outputs[?OutputKey=='BucketName'].OutputValue" \
  --output text)

WEB_BUCKET=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(STACK) \
  --query "Stacks[0].Outputs[?OutputKey=='DistributionId'].OutputValue" \
  --output text)

BUILD_ENV ?= VERSION=$(VERSION) \
							VITE_ENV=$(VITE_ENV) \
							VITE_LOG_LEVEL=$(VITE_LOG_LEVEL) \
							VITE_FIREBASE_API_KEY=$(VITE_FIREBASE_API_KEY) \
							VITE_FIREBASE_AUTH_DOMAIN=$(VITE_FIREBASE_AUTH_DOMAIN) \
							VITE_FIREBASE_PROJECT_ID=$(VITE_FIREBASE_PROJECT_ID) \
							VITE_FIREBASE_STORAGE_BUCKET=$(VITE_FIREBASE_STORAGE_BUCKET) \
							VITE_FIREBASE_MESSAGING_SENDER_ID=$(VITE_FIREBASE_MESSAGING_SENDER_ID) \
							VITE_FIREBASE_APP_ID=$(VITE_FIREBASE_APP_ID) \
							VITE_FIREBASE_MEASUREMENT_ID=$(VITE_FIREBASE_MEASUREMENT_ID) \
							VITE_SIGN_DOMAIN=$(VITE_SIGN_DOMAIN) \
							VITE_API_URL=$(VITE_API_URL) \
							VITE_EXPERIMENT=$(VITE_EXPERIMENT) \
							VITE_TELEGRAM_BOTNAME=$(VITE_TELEGRAM_BOTNAME) \
							VITE_OPERATOR_ADDRESS=$(VITE_OPERATOR_ADDRESS) \
							VITE_RPC_URL=$(VITE_RPC_URL) \
							VITE_BLOCK_EXPLORER_URL=$(VITE_BLOCK_EXPLORER_URL) \
							VITE_USDT_ADDRESS=$(VITE_USDT_ADDRESS)
DOCKER_FLAGS ?= --build-arg VITE_ENV=$(VITE_ENV) \
									--build-arg VITE_LOG_LEVEL=$(VITE_LOG_LEVEL) \
									--build-arg VITE_FIREBASE_API_KEY=$(VITE_FIREBASE_API_KEY) \
									--build-arg VITE_FIREBASE_AUTH_DOMAIN=$(VITE_FIREBASE_AUTH_DOMAIN) \
									--build-arg VITE_FIREBASE_PROJECT_ID=$(VITE_FIREBASE_PROJECT_ID) \
									--build-arg VITE_FIREBASE_STORAGE_BUCKET=$(VITE_FIREBASE_STORAGE_BUCKET) \
									--build-arg VITE_FIREBASE_MESSAGING_SENDER_ID=$(VITE_FIREBASE_MESSAGING_SENDER_ID) \
									--build-arg VITE_FIREBASE_APP_ID=$(VITE_FIREBASE_APP_ID) \
									--build-arg VITE_FIREBASE_MEASUREMENT_ID=$(VITE_FIREBASE_MEASUREMENT_ID) \
									--build-arg VITE_SIGN_DOMAIN=$(VITE_SIGN_DOMAIN) \
									--build-arg VITE_API_URL=$(VITE_API_URL) \
									--build-arg VITE_EXPERIMENT=$(VITE_EXPERIMENT) \
									--build-arg VITE_OPERATOR_ADDRESS=$(VITE_OPERATOR_ADDRESS) \
									--build-arg VITE_RPC_URL=$(VITE_RPC_URL) \
									--build-arg VITE_BLOCK_EXPLORER_URL=$(VITE_BLOCK_EXPLORER_URL) \
									--build-arg VITE_USDT_ADDRESS=$(VITE_USDT_ADDRESS) \
                  --build-arg VITE_VERSION=$(VERSION) \
									--build-arg VITE_TELEGRAM_BOTNAME=$(VITE_TELEGRAM_BOTNAME)

RATEL_TEST_PLAYWRIGHT_URL ?= http://localhost:8080
RATEL_TEST_PLAYWRIGHT_ID ?= $(shell date +%s)
PLAYWRIGHT_ARGS ?=
PLAYWRIGHT_ENV ?= RATEL_TEST_PLAYWRIGHT_URL=$(RATEL_TEST_PLAYWRIGHT_URL) \
								RATEL_TEST_PLAYWRIGHT_ID=$(RATEL_TEST_PLAYWRIGHT_ID)

.PHONY: run node_module
node_module:
	pnpm i -p

run: node_module
	VERSION=$(VERSION) PORT=$(PORT) pnpm dev

build: node_module
	$(BUILD_ENV) pnpm build

dist: build

sync: dist
	aws s3 sync ./dist s3://$(WEB_BUCKET)
	aws cloudfront create-invalidation --distribution-id $(WEB_CDN_ID) --paths "/*"

test: node_modules
	$(PLAYWRIGHT_ENV) npx playwright test $(PLAYWRIGHT_ARGS)
