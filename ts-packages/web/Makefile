PORT ?= 8080

.PHONY: run node_module
node_module:
	pnpm i

run: node_module
	PORT=$(PORT) pnpm dev

build: node_module
	pnpm build
