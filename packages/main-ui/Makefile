SERVICE ?= main-ui
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= dev

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

DOMAIN ?= dev.ratel.foundation
RUST_LOG ?= debug
REDIRECT_URI ?= http://localhost:3000
WORKSPACE_ROOT ?= ../..
ARTIFACT_DIR ?= $(WORKSPACE_ROOT)/.build/$(SERVICE)
MAIN_API_ENDPOINT ?= https://api.dev.ratel.foundation
RUSTFLAGS ?= -D warnings
DXFLAGS ?=

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) ENV=$(ENV) DOMAIN=${DOMAIN} AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} SERVICE=$(SERVICE) WORKSPACE_ROOT=$(WORKSPACE_ROOT) BASE_URL=$(BASE_URL) MAIN_API_ENDPOINT=$(MAIN_API_ENDPOINT) REDIRECT_URI=$(REDIRECT_URI) RUSTFLAGS="$(RUSTFLAGS)"

setup.tool:
	cargo binstall dioxus-cli
	cargo binstall toml-cli
	npm i -g @tailwindcss/cli
	npm i -g webpack-cli

run: clean public/tailwind.css
	CARGO_TARGET_DIR=$(PWD)/target $(BUILD_ENV) dx serve --fullstack $(DXFLAGS)

run.android: clean public/tailwind.css
	CARGO_TARGET_DIR=$(PWD)/target $(BUILD_ENV) dx serve --platform android --features mobile $(DXFLAGS)

run.ios: clean public/tailwind.css
	cargo build --target aarch64-apple-ios-sim
	CARGO_TARGET_DIR=$(PWD)/target $(BUILD_ENV) dx serve --platform ios --features mobile --verbose

build.android:
	$(BUILD_ENV) RUST_LOG=trace dx build --release --platform android --features mobile --verbose

build.ios:
	$(BUILD_ENV) dx build --release --platform ios --features mobile --verbose

build: clean public/tailwind.css
	$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)

	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

public/dep.js:
	cd $(WORKSPACE_ROOT)/deps/js && make build && cp ./dist/main.js $(PWD)/public/dep.js

public/tailwind.css:
	tailwindcss -i ./public/input.css -o ./public/tailwind.css

clean:
	rm -rf public/tailwind.css public/dep.js

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'

s3-deploy:
	cp -r packages/$(SERVICE)/public/* .build/$(SERVICE)/public
	aws s3 sync .build/$(SERVICE)/public s3://$(DOMAIN) $(AWS_FLAG)
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null
