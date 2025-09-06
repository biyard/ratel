# For deploy
PROJECT ?= ratel
SERVICE ?= web

REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
AWS_REGION ?= $(shell aws configure get region)

# For build
PORT ?= 8080
VERSION ?= $(shell git rev-parse --short HEAD)

BUILD_ENV ?= VERSION=$(VERSION)
DOCKER_FLAGS ?= --build-arg NEXT_PUBLIC_LOG_LEVEL=$(NEXT_PUBLIC_LOG_LEVEL) --build-arg NEXT_PUBLIC_FIREBASE_API_KEY=$(NEXT_PUBLIC_FIREBASE_API_KEY) --build-arg NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=$(NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN) --build-arg NEXT_PUBLIC_FIREBASE_PROJECT_ID=$(NEXT_PUBLIC_FIREBASE_PROJECT_ID) --build-arg NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=$(NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET) --build-arg NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=$(NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID) --build-arg NEXT_PUBLIC_FIREBASE_APP_ID=$(NEXT_PUBLIC_FIREBASE_APP_ID) --build-arg NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=$(NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID) --build-arg NEXT_PUBLIC_ENV=$(NEXT_PUBLIC_ENV) --build-arg NEXT_PUBLIC_SIGN_DOMAIN=$(NEXT_PUBLIC_SIGN_DOMAIN) --build-arg NEXT_PUBLIC_API_URL=$(NEXT_PUBLIC_API_URL) --build-arg NEXT_PUBLIC_EXPERIMENT=$(NEXT_PUBLIC_EXPERIMENT) --build-arg NEXT_PUBLIC_GRAPHQL_URL=$(NEXT_PUBLIC_GRAPHQL_URL)

.PHONY: run node_module
node_module:
	pnpm i

run: node_module
	VERSION=$(VERSION) PORT=$(PORT) pnpm dev

build: node_module
	$(BUILD_ENV) pnpm build

docker.build:
	docker build -t $(ECR_NAME):$(COMMIT) --no-cache $(DOCKER_FLAGS) -f Dockerfile ../..

docker.push:
	docker push $(ECR_NAME):$(COMMIT)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com
