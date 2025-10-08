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
							VITE_TELEGRAM_BOTNAME=$(VITE_TELEGRAM_BOTNAME)

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
                  --build-arg VITE_VERSION=$(VERSION) \
									--build-arg VITE_TELEGRAM_BOTNAME=$(VITE_TELEGRAM_BOTNAME)


.PHONY: run node_module
node_module:
	pnpm i -p

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
