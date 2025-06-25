ENV ?= local
RUST_LOG ?= debug
TELEGRAM_TOKEN ?=

BUILD_ENVS = ENV=$(ENV) RUST_LOG=$(RUST_LOG) TELEGRAM_TOKEN=$(TELEGRAM_TOKEN)

run:
	$(BUILD_ENVS) cargo watch -x run -w src -w $(PWD)/../dto -w $(PWD)/../../deps/rust-sdk
