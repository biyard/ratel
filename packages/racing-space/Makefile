CARGO_TARGET_DIR ?= $(PWD)/target
# CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER ?= wasm-server-runner

BUILD_ENV ?= CARGO_TARGET_DIR=$(CARGO_TARGET_DIR)

.PHONY: run build
run:
	$(BUILD_ENV) trunk serve --watch src -p 8080 -v

desktop-run:
	$(BUILD_ENV) cargo watch -x run -w src -w $(PWD)/../dto -w $(PWD)/../../deps/rust-sdk

build:
	$(BUILD_ENV) trunk build --release
	wasm-opt -Oz -o ./dist/optimized.wasm ./dist/*.wasm
