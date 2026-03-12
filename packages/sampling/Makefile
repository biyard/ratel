DFX_BIN := $(HOME)/Library/Application Support/org.dfinity.dfx/bin
export PATH := $(DFX_BIN):$(PATH)

VERSION ?= $(shell cargo metadata --no-deps --format-version 1 2>/dev/null | jq -r '.packages[] | select(.name=="sampling") | .version')
COMMIT  ?= $(shell git rev-parse --short HEAD 2>/dev/null)
DATE     = $(shell date)
ENV     ?= local
OS      ?= $(shell uname -s | tr A-Z a-z)

ifeq ("$(ENV)","prod")
	NET = ic
	DEPLOY_FLAG = --ic
	SAMPLING_CANISTER_ID ?= $(shell cat canister_ids.json | jq -r .sampling.ic)
endif

ifeq ("$(ENV)","stg")
	NET = stage
	DEPLOY_FLAG = --network stage
	SAMPLING_CANISTER_ID ?= $(shell cat canister_ids.json | jq -r .sampling.stage)
endif

ifeq ("$(ENV)","dev")
	NET = dev
	DEPLOY_FLAG = --network dev
	SAMPLING_CANISTER_ID ?= $(shell cat canister_ids.json | jq -r .sampling.dev)
endif

ifeq ("$(ENV)","test")
	NET = test
	DEPLOY_FLAG = --network test
endif

ifeq ("$(ENV)","local")
	NET = local
	DEPLOY_FLAG =
endif

SAMPLING_CANISTER_ID ?= $(shell cat .dfx/$(NET)/canister_ids.json 2>/dev/null | jq -r '.sampling.$(NET)' 2>/dev/null)
CANISTER_URL ?= https://$(SAMPLING_CANISTER_ID).icp0.io

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

.build/did/sampling.did:
	mkdir -p .build/did
	$(BUILD_ENV) cargo build --release --target wasm32-unknown-unknown
	candid-extractor ../../target/wasm32-unknown-unknown/release/sampling.wasm > .build/did/sampling.did
	@echo "DID exported to .build/did/sampling.did"

did: clean.did .build/did/sampling.did

clean.did:
	rm -rf .build/did

# ─── Build & Deploy ────────────────────────────────────────────────────

build: clean.build did
	$(BUILD_ENV) dfx build $(DEPLOY_FLAG)

deploy: clean.build did
	$(BUILD_ENV) dfx deploy $(DEPLOY_FLAG)

upgrade: build
	dfx canister $(DEPLOY_FLAG) install sampling --mode=upgrade

# ─── Version Management ───────────────────────────────────────────────

check-version:
	@curl -sf $(CANISTER_URL)/version || echo "unreachable"

deployed-version:
	@dfx canister call sampling version $(DEPLOY_FLAG) | tr -d '(")'  | sed 's/-.*//g'

current-version:
	@echo $(VERSION)

should-upgrade:
	$(eval DEPLOYED := $(shell dfx canister call sampling version $(DEPLOY_FLAG) 2>/dev/null | tr -d '(")'  | sed 's/-.*//g'))
	@if [ "$(DEPLOYED)" != "$(VERSION)" ]; then echo "true"; else echo "false"; fi

upgrade-if-needed:
	$(eval DEPLOYED := $(shell dfx canister call sampling version $(DEPLOY_FLAG) 2>/dev/null | tr -d '(")'  | sed 's/-.*//g'))
	@if [ "$(DEPLOYED)" != "$(VERSION)" ]; then \
		echo "Upgrading $(DEPLOYED) -> $(VERSION)..."; \
		$(MAKE) upgrade; \
	else \
		echo "Already at version $(VERSION)"; \
	fi

# ─── Test & Bench ──────────────────────────────────────────────────────

test:
	cargo test

bench:
	dfx canister call sampling run_sampling_with_metrics \
		'(record { id = "bench"; data = vec { \
			record { id = "a1"; bif = vec { 1.0; 1.0; 1.0; 1.0 }; demographics = null }; \
			record { id = "a2"; bif = vec { 1.2; 1.1; 1.3; 1.0 }; demographics = null }; \
			record { id = "a3"; bif = vec { 1.1; 0.9; 1.0; 1.2 }; demographics = null }; \
			record { id = "a4"; bif = vec { 0.9; 1.0; 1.1; 0.8 }; demographics = null }; \
			record { id = "a5"; bif = vec { 1.0; 1.2; 0.9; 1.1 }; demographics = null }; \
			record { id = "b1"; bif = vec { 5.0; 5.0; 5.0; 5.0 }; demographics = null }; \
			record { id = "b2"; bif = vec { 4.8; 5.1; 5.2; 4.9 }; demographics = null }; \
			record { id = "b3"; bif = vec { 5.1; 4.9; 5.0; 5.2 }; demographics = null }; \
			record { id = "b4"; bif = vec { 4.9; 5.0; 4.8; 5.1 }; demographics = null }; \
			record { id = "b5"; bif = vec { 5.2; 5.1; 5.1; 4.8 }; demographics = null } \
		}; min_k = 2 : nat32; max_k = 5 : nat32; variance_threshold = 0.85; max_iterations = 100 : nat32 })'

# ─── Status & Ops ──────────────────────────────────────────────────────

status:
	@echo "=== Version ==="
	@dfx canister call sampling version $(DEPLOY_FLAG)
	@echo "=== Cycles Balance ==="
	@dfx canister call sampling get_cycles_balance '()' $(DEPLOY_FLAG)
	@echo "=== Memory Usage ==="
	@dfx canister call sampling get_memory_usage '()' $(DEPLOY_FLAG)

deposit:
	dfx canister deposit-cycles 10_000_000_000_000 sampling $(DEPLOY_FLAG)

info:
	dfx canister status sampling $(DEPLOY_FLAG)

# ─── Clean ─────────────────────────────────────────────────────────────

clean.build:
	rm -rf .build

clean:
	rm -rf .build .dfx .env
