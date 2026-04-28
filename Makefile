ENV ?= dev
BASE_DOMAIN ?= ratel.foundation
DOMAIN ?= $(ENV).$(BASE_DOMAIN)
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
WORKSPACE_ROOT ?= $(PWD)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

STACK ?= ratel-$(ENV)-stack

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) WORKSPACE_ROOT=$(WORKSPACE_ROOT) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) BASE_DOMAIN=$(BASE_DOMAIN) STACK=$(STACK) BIYARD_PUBLIC_API_URL=$(BIYARD_PUBLIC_API_URL) BIYARD_PRIVATE_API_URL=$(BIYARD_PRIVATE_API_URL)

.build/evm-keys:
	mkdir -p .build
	docker run --rm ghcr.io/foundry-rs/foundry:latest "cast wallet new --json" > .build/evm-keys.json

preview:
	dx server -p preview

run: .build/evm-keys
	COMMIT=${COMMIT} docker compose --profile development up -d --remove-orphans

stop:
	docker compose --profile development down --remove-orphans

serve:
	cd packages/$(SERVICE) && make serve

clean:
	rm -rf .build/$(SERVICE)

.PHONY: deploy
deploy: build cdk-deploy

cdk/.build/%/bootstrap:
	mkdir -p cdk/.build/$*
	cp .build/$*/bootstrap cdk/.build/$*/bootstrap

cdk-deploy-v2:
	cd cdk && npm i
	cd cdk && $(BUILD_CDK_ENV) npm run build
	cd cdk && $(BUILD_CDK_ENV) cdk synth
	cd cdk && $(BUILD_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG) --all --concurrency 3

node_modules:
	pnpm i

infra: .build/evm-keys
	docker compose --profile infra up -d --remove-orphans

# ── Local Lambda-handler test drivers ─────────────────────────────
# Bypass EventBridge — call the analysis handlers directly against
# the AWS DynamoDB table whose prefix matches `ratel-$(ENV)`.
# Usage:
#   ENV=victor SPACE=<uuid> REPORT=<uuid> make test-analyze-report
#   ENV=victor SPACE=<uuid> REPORT=<uuid> DISCUSSION=<discussion_id>#<request_uuid> make test-analyze-discussion
#
# `option_env!` in `aws_config.rs` captures credentials at compile time,
# so any change to ENV / region / keys requires re-touching the
# config file to force a rebuild — handled below.
TEST_ANALYZE_ENV = \
	DYNAMO_TABLE_PREFIX=ratel-$(ENV) \
	AWS_REGION=$(REGION) \
	AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) \
	AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) \
	RUST_LOG=$${RUST_LOG:-info}

.PHONY: test-analyze-report test-analyze-discussion run-pending-discussions

# Drain InProgress discussion-analysis rows for a space. Same effect
# as the discussion Lambda waking up. Optionally narrow by REPORT.
#
# Usage:
#   ENV=victor SPACE=<uuid> make run-pending-discussions
#   ENV=victor SPACE=<uuid> REPORT=<report_id> make run-pending-discussions
run-pending-discussions:
	@test -n "$(SPACE)" || (echo "SPACE=<space_uuid> required"; exit 1)
	@touch app/ratel/src/common/config/server/aws_config.rs
	cd app/ratel && $(TEST_ANALYZE_ENV) \
		cargo run --bin run_pending_discussions --features server -- \
		$(SPACE) $(if $(REPORT),--report $(REPORT))

test-analyze-report:
	@test -n "$(SPACE)" || (echo "SPACE=<space_uuid> required"; exit 1)
	@test -n "$(REPORT)" || (echo "REPORT=<report_id> required"; exit 1)
	@touch app/ratel/src/common/config/server/aws_config.rs
	cd app/ratel && $(TEST_ANALYZE_ENV) \
		cargo run --bin test_analyze_report --features server -- \
		$(SPACE) $(REPORT)

test-analyze-discussion:
	@test -n "$(SPACE)" || (echo "SPACE=<space_uuid> required"; exit 1)
	@test -n "$(REPORT)" || (echo "REPORT=<report_id> required"; exit 1)
	@test -n "$(DISCUSSION)" || (echo 'DISCUSSION=<discussion_id>#<request_uuid> required'; exit 1)
	@touch app/ratel/src/common/config/server/aws_config.rs
	cd app/ratel && $(TEST_ANALYZE_ENV) \
		cargo run --bin test_analyze_discussion --features server -- \
		$(SPACE) $(REPORT) $(DISCUSSION)

testing: .build/evm-keys
	COMMIT=$(COMMIT) RESET_DB=true docker compose --profile testing up -d --remove-orphans

clean-infra:
	docker compose --profile infra down

pr-testing:
	scripts/pr-testing-playwright.sh
