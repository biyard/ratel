DFX_BIN := $(HOME)/Library/Application Support/org.dfinity.dfx/bin
export PATH := $(DFX_BIN):$(PATH)

VERSION ?= $(shell cargo metadata --no-deps --format-version 1 2>/dev/null | jq -r '.packages[] | select(.name=="ratel-canister") | .version')
COMMIT  ?= $(shell git rev-parse --short HEAD 2>/dev/null)
DATE     = $(shell date)
ENV     ?= local
OS      ?= $(shell uname -s | tr A-Z a-z)

ifeq ("$(ENV)","prod")
	NET = ic
	DEPLOY_FLAG = --ic
	RATEL_CANISTER_ID ?= $(shell cat canister_ids.json | jq -r '."ratel-canister".ic')
endif

ifeq ("$(ENV)","stg")
	NET = stage
	DEPLOY_FLAG = --network stage
	RATEL_CANISTER_ID ?= $(shell cat canister_ids.json | jq -r '."ratel-canister".stage')
endif

ifeq ("$(ENV)","dev")
	NET = dev
	DEPLOY_FLAG = --network dev
	RATEL_CANISTER_ID ?= $(shell cat canister_ids.json | jq -r '."ratel-canister".dev')
endif

ifeq ("$(ENV)","test")
	NET = test
	DEPLOY_FLAG = --network test
endif

ifeq ("$(ENV)","local")
	NET = local
	DEPLOY_FLAG =
endif

RATEL_CANISTER_ID ?= $(shell cat .dfx/$(NET)/canister_ids.json 2>/dev/null | jq -r '."ratel-canister"."$(NET)"' 2>/dev/null)
CANISTER_URL ?= https://$(RATEL_CANISTER_ID).icp0.io

BUILD_ENV ?= DATE="$(DATE)" VERSION="$(VERSION)" COMMIT="$(COMMIT)"

.PHONY: tools start stop did build deploy upgrade test bench \
        status check-version deployed-version current-version \
        should-upgrade upgrade-if-needed deposit info clean

# ─── Tools ──────────────────────────────────────────────────────────────

tools:
	@cargo install candid-extractor
	@rustup target add wasm32-unknown-unknown

# ─── Local Replica ─────────────────────────────────────────────────────

start:
	dfx start --background --clean

stop:
	dfx stop

# ─── DID Generation ────────────────────────────────────────────────────

.build/did/ratel-canister.did:
	mkdir -p .build/did
	$(BUILD_ENV) cargo build --release --target wasm32-unknown-unknown
	candid-extractor ../../target/wasm32-unknown-unknown/release/ratel_canister.wasm > .build/did/ratel-canister.did
	@echo "DID exported to .build/did/ratel-canister.did"

did: clean.did .build/did/ratel-canister.did

clean.did:
	rm -rf .build/did

# ─── Build & Deploy ────────────────────────────────────────────────────

build: clean.build did
	$(BUILD_ENV) dfx build $(DEPLOY_FLAG)

deploy: clean.build did
	$(BUILD_ENV) dfx deploy $(DEPLOY_FLAG)

upgrade: build
	dfx canister $(DEPLOY_FLAG) install ratel-canister --mode=upgrade

# ─── Version Management ───────────────────────────────────────────────

check-version:
	@curl -sf $(CANISTER_URL)/version || echo "unreachable"

deployed-version:
	@dfx canister call ratel-canister version $(DEPLOY_FLAG) | tr -d '(")'  | sed 's/-.*//g'

current-version:
	@echo $(VERSION)

should-upgrade:
	$(eval DEPLOYED := $(shell dfx canister call ratel-canister version $(DEPLOY_FLAG) 2>/dev/null | tr -d '(")'  | sed 's/-.*//g'))
	@if [ "$(DEPLOYED)" != "$(VERSION)" ]; then echo "true"; else echo "false"; fi

upgrade-if-needed:
	$(eval DEPLOYED := $(shell dfx canister call ratel-canister version $(DEPLOY_FLAG) 2>/dev/null | tr -d '(")'  | sed 's/-.*//g'))
	@if [ "$(DEPLOYED)" != "$(VERSION)" ]; then \
		echo "Upgrading $(DEPLOYED) -> $(VERSION)..."; \
		$(MAKE) upgrade; \
	else \
		echo "Already at version $(VERSION)"; \
	fi

# ─── Test & Bench ──────────────────────────────────────────────────────

test:
	cargo test

# ─── Status & Ops ──────────────────────────────────────────────────────

status:
	@echo "=== Version ==="
	@dfx canister call ratel-canister version $(DEPLOY_FLAG)
	@echo "=== Cycles Balance ==="
	@dfx canister call ratel-canister get_cycles_balance '()' $(DEPLOY_FLAG)
	@echo "=== Memory Usage ==="
	@dfx canister call ratel-canister get_memory_usage '()' $(DEPLOY_FLAG)

deposit:
	dfx canister deposit-cycles 10_000_000_000_000 ratel-canister $(DEPLOY_FLAG)

info:
	dfx canister status ratel-canister $(DEPLOY_FLAG)

# ─── Clean ─────────────────────────────────────────────────────────────

clean.build:
	rm -rf .build

clean:
	rm -rf .build .dfx .env
