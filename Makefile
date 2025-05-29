ENV ?= dev
BASE_DOMAIN ?=
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

STACK ?= $(PROJECT)-$(SERVICE)-$(ENV)-stack

ifeq ($(ENABLE_DOCKER),true)
	DOCKER_COMMAND_SUFFUIX = -docker
endif

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE) VPC_ID=$(VPC_ID) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) ENABLE_S3=$(ENABLE_S3) ENABLE_DYNAMO=$(ENABLE_DYNAMO) ENABLE_FARGATE=$(ENABLE_FARGATE) ENABLE_LAMBDA=$(ENABLE_LAMBDA) ENABLE_OPENSEARCH=$(ENABLE_OPENSEARCH) BASE_DOMAIN=$(BASE_DOMAIN) PROJECT=$(PROJECT) STACK=$(STACK) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID)

# Basic commands
run:
	cd packages/$(SERVICE) && make run

serve:
	cd packages/$(SERVICE) && make serve

clean:
	rm -rf .build/$(SERVICE)

# Deployment commands
.PHONY: deploy-if-needed
deploy-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(DOMAIN)$(API_PREFIX)/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell toml get packages/$(SERVICE)/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "OLD_VERSION=\"$(DEPLOYED_VERSION)\" NEW_VERSION=\"$(CURRENT_VERSION)\" make deploy"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

.PHONY: deploy-web-if-needed
deploy-web-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(DOMAIN)$(API_PREFIX)/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell toml get packages/$(SERVICE)/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "OLD_VERSION=\"$(DEPLOYED_VERSION)\" NEW_VERSION=\"$(CURRENT_VERSION)\" make deploy-web"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

.PHONY: deploy deploy-web
deploy: build cdk-deploy
deploy-web: build cdk-deploy s3-deploy

.PHONY: build
build: clean
	mkdir -p .build
	cd packages/$(SERVICE) && ENV=$(ENV) ARTIFACT_DIR=$(PWD)/.build/$(SERVICE) make build$(DOCKER_COMMAND_SUFFUIX)

deps/rust-sdk/cdk/node_modules:
	cd deps/rust-sdk/cdk && npm install

cdk-deploy: deps/rust-sdk/cdk/node_modules
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) npm run build
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk synth
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk deploy --require-approval never $(AWS_FLAG) --all

s3-deploy:
	cp -r packages/$(SERVICE)/public .build/$(SERVICE)/public/public
	aws s3 sync .build/$(SERVICE)/public s3://$(DOMAIN) $(AWS_FLAG)
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

node_modules:
	npm i

test: node_modules
	npx playwright test

# Development environment
.PHONY: setup dev start stop clean-dev status

# Setup development environment
setup:
	@echo "🔧 Setting up development environment..."
	@echo "🔍 Checking prerequisites..."
	@command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo not found. Install from https://rustup.rs/"; exit 1; }
	@command -v dx >/dev/null 2>&1 || { echo "📦 Installing dioxus-cli..."; cargo binstall dioxus-cli --no-confirm; }
	@command -v psql >/dev/null 2>&1 || { echo "❌ PostgreSQL not found. Install with: brew install postgresql@14"; exit 1; }
	@command -v /opt/homebrew/opt/llvm/bin/clang >/dev/null 2>&1 || { echo "⚠️  LLVM not found. Install with: brew install llvm"; }
	@command -v cargo-watch >/dev/null 2>&1 || { echo "📦 Installing cargo-watch..."; cargo install cargo-watch; }
	@command -v toml >/dev/null 2>&1 || { echo "📦 Installing toml-cli..."; cargo binstall toml-cli --no-confirm; }
	@command -v firebase >/dev/null 2>&1 || { echo "📦 Installing firebase-tools..."; npm install -g firebase-tools; }
	@command -v tailwindcss >/dev/null 2>&1 || { echo "📦 Installing tailwindcss..."; npm i -g @tailwindcss/cli; }
	@java -version >/dev/null 2>&1 || { echo "❌ Java not found. Install with: brew install openjdk@11"; exit 1; }
	@echo "🗄️  Setting up database..."
	@if [[ "$$OSTYPE" == "darwin"* ]] && command -v brew >/dev/null 2>&1; then brew services start postgresql@14 2>/dev/null || true; fi
	@createdb ratel_dev 2>/dev/null || echo "ℹ️  Database already exists or cannot be created"
	@echo "✅ Development environment setup completed"

# Start development environment (frontend only)
dev: setup
	@echo "🎨 Starting frontend development..."
	@cd packages/$(SERVICE) && \
	ENV=local \
	CC_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/clang \
	AR_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/llvm-ar \
	CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER=rust-lld \
	CARGO_INCREMENTAL=1 \
	CARGO_TARGET_DIR=../../target \
	PATH="/opt/homebrew/opt/llvm/bin:/opt/homebrew/opt/postgresql@14/bin:$$PATH" \
	RUST_LOG=debug \
	DATABASE_URL=postgresql://$(shell whoami)@localhost:5432/ratel_dev \
	JWT_SECRET_KEY=dev-jwt-secret-key-for-local-development-12345 \
	FIREBASE_API_KEY=dev-firebase-api-key \
	FIREBASE_AUTH_DOMAIN=localhost:9099 \
	FIREBASE_PROJECT_ID=ratel-local-dev \
	FIREBASE_STORAGE_BUCKET=ratel-local-dev.appspot.com \
	FIREBASE_MESSAGING_SENDER_ID=123456789 \
	FIREBASE_APP_ID=dev-app-id \
	FIREBASE_MEASUREMENT_ID=dev-measurement-id \
	MAIN_API_ENDPOINT=http://localhost:3000 \
	REDIRECT_URI=http://localhost:8080 \
	make run-fast

# Start full development environment (all services)
start: setup
	@echo "🚀 Starting full development environment..."
	@echo "🔥 Starting Firebase emulator..."
	@mkdir -p logs
	@firebase emulators:start --only auth,firestore,ui --project ratel-local-dev > logs/firebase.log 2>&1 &
	@sleep 5
	@echo "🌐 Starting backend API..."
	@cd packages/main-api && \
	ENV=dev \
	DOMAIN=dev.ratel.foundation \
	AUTH_DOMAIN=dev.ratel.foundation \
	DATABASE_TYPE=postgres \
	OPENAPI_KEY=dev-openapi-key \
	BUCKET_NAME=dev-bucket \
	ASSET_DIR=metadata \
	BUCKET_EXPIRE=3600 \
	SLACK_CHANNEL_SPONSOR=dev-sponsor \
	SLACK_CHANNEL_ABUSING=dev-abusing \
	SLACK_CHANNEL_MONITOR=dev-monitor \
	PATH="/opt/homebrew/opt/llvm/bin:/opt/homebrew/opt/postgresql@14/bin:$$PATH" \
	RUST_LOG=debug \
	DATABASE_URL=postgresql://$(shell whoami)@localhost:5432/ratel_dev \
	JWT_SECRET_KEY=dev-jwt-secret-key-for-local-development-12345 \
	AUTH_TYPE=jwt \
	JWT_EXPIRATION=3600 \
	MAIN_API_ENDPOINT=http://localhost:3000 \
	make run > ../../logs/backend.log 2>&1 &
	@sleep 10
	@echo "🎨 Starting frontend..."
	@cd packages/main-ui && \
	ENV=local \
	CC_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/clang \
	AR_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/llvm-ar \
	CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER=rust-lld \
	CARGO_INCREMENTAL=1 \
	CARGO_TARGET_DIR=../../target \
	PATH="/opt/homebrew/opt/llvm/bin:/opt/homebrew/opt/postgresql@14/bin:$$PATH" \
	RUST_LOG=debug \
	DATABASE_URL=postgresql://$(shell whoami)@localhost:5432/ratel_dev \
	JWT_SECRET_KEY=dev-jwt-secret-key-for-local-development-12345 \
	FIREBASE_API_KEY=dev-firebase-api-key \
	FIREBASE_AUTH_DOMAIN=localhost:9099 \
	FIREBASE_PROJECT_ID=ratel-local-dev \
	FIREBASE_STORAGE_BUCKET=ratel-local-dev.appspot.com \
	FIREBASE_MESSAGING_SENDER_ID=123456789 \
	FIREBASE_APP_ID=dev-app-id \
	FIREBASE_MEASUREMENT_ID=dev-measurement-id \
	MAIN_API_ENDPOINT=http://localhost:3000 \
	REDIRECT_URI=http://localhost:8080 \
	make run-fast > ../../logs/frontend.log 2>&1 &
	@echo ""
	@echo "🎉 Development environment ready!"
	@echo "📍 Access URLs:"
	@echo "   🎨 Frontend:          http://localhost:8080"
	@echo "   🌐 Backend API:       http://localhost:3000"
	@echo "   🔥 Firebase UI:       http://localhost:4000"
	@echo ""
	@echo "🛑 Stop with: make stop"

# Stop all development services
stop:
	@echo "🛑 Stopping development services..."
	@pkill -f "firebase.*emulators" 2>/dev/null || true
	@pkill -f "cargo.*watch" 2>/dev/null || true
	@pkill -f "dx.*serve" 2>/dev/null || true
	@pkill -f "target.*main-api" 2>/dev/null || true
	@echo "✅ All services stopped"

# Clean development files
clean-dev:
	@echo "🧹 Cleaning development files..."
	@rm -rf logs/*.log
	@rm -rf target/
	@rm -rf packages/main-ui/target/
	@rm -rf packages/main-ui/dist
	@rm -rf packages/main-ui/public/tailwind.css
	@rm -rf packages/main-ui/public/dep.js
	@cd packages/main-ui && cargo clean
	@echo "✅ Cleanup completed"

# Show development environment status
status:
	@echo "📊 Development Environment Status:"
	@echo ""
	@echo "🔧 Required Tools:"
	@command -v cargo >/dev/null 2>&1 && echo "   ✅ Rust/Cargo" || echo "   ❌ Rust/Cargo"
	@command -v dx >/dev/null 2>&1 && echo "   ✅ Dioxus CLI" || echo "   ❌ Dioxus CLI"
	@command -v firebase >/dev/null 2>&1 && echo "   ✅ Firebase CLI" || echo "   ❌ Firebase CLI"
	@command -v psql >/dev/null 2>&1 && echo "   ✅ PostgreSQL" || echo "   ❌ PostgreSQL"
	@command -v /opt/homebrew/opt/llvm/bin/clang >/dev/null 2>&1 && echo "   ✅ LLVM (Apple Silicon)" || echo "   ❌ LLVM (Apple Silicon)"
	@echo ""
	@echo "🚀 Running Services:"
	@pgrep -f "firebase.*emulators" >/dev/null 2>&1 && echo "   🔥 Firebase Emulator" || echo "   ⭕ Firebase Emulator (stopped)"
	@pgrep -f "target.*main-api" >/dev/null 2>&1 && echo "   🌐 Backend API" || echo "   ⭕ Backend API (stopped)"
	@pgrep -f "dx.*serve" >/dev/null 2>&1 && echo "   🎨 Frontend" || echo "   ⭕ Frontend (stopped)"
	@echo ""
	@echo "📋 Available Commands:"
	@echo "   🔧 make setup     - Setup development environment"
	@echo "   🎨 make dev       - Start frontend only"
	@echo "   🚀 make start     - Start full stack (all services)"
	@echo "   🛑 make stop      - Stop all services"
	@echo "   🧹 make clean-dev - Clean development files"
	@echo "   📊 make status    - Show this status"
