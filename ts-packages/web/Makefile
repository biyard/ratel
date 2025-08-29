PORT ?= 8080
VERSION ?= $(shell git rev-parse --short HEAD)

BUILD_ENV ?= VERSION=$(VERSION)

.PHONY: run node_module
node_module:
	pnpm i

run: node_module
	VERSION=$(VERSION) PORT=$(PORT) pnpm dev

build: node_module
	$(BUILD_ENV) pnpm build
