SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
	DOMAIN ?= 
	REDIRECT_URI = https://dagit.club
endif

ifeq ("$(ENV)","dev")
	LOG_LEVEL ?= debug
	DOMAIN ?= 
	REDIRECT_URI = 
endif

REDIRECT_URI ?= http://localhost:3000

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL) ENV=$(ENV) DOMAIN=${DOMAIN} AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} SERVICE=$(SERVICE) WORKSPACE_ROOT=$(WORKSPACE_ROOT) BASE_URL=$(BASE_URL) API_ENDPOINT=$(API_ENDPOINT) REDIRECT_URI=$(REDIRECT_URI)

setup.tool:
	cargo install dioxus-cli --version 0.6.0
	cargo install toml-cli
	npm i -g tailwindcss

run: public/tailwind.css
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --platform web

build-lambda: public/tailwind.css
	$(BUILD_ENV) dx build --release --fullstack --client-features web-release --server-features lambda

public/tailwind.css:
	tailwindcss -i ./public/input.css -o ./public/tailwind.css --minify
