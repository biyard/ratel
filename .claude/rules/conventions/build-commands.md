---
globs: ["Makefile", "docker-compose*.yml", "app/ratel/Cargo.toml"]
---

# Build & Verification Commands

## Checking compliation errors (app/ratel/)

```bash
```

## Dioxus App (app/ratel/)

### Lint check
```bash
# MUST run after nay code change in app/ratel
# This verifies compliation error for server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server

# MUST run after nay code change in app/ratel
# This verifies compliation error for web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web

# MUST run after nay code change in app/ratel
# This verifies compliation error for web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features mobile

# MUST run after any code change in app/ratel/
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web

# MUST run after any code change in app/ratel/
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --mobile

# Dev server (port 8000)
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```


**`DYNAMO_TABLE_PREFIX` is required at compile time** for DynamoEntity. Use `ratel-dev` for dev, `ratel-local` for Docker local.

**`RUSTFLAGS='-D warnings'` is required** for all build checks. Warnings are treated as errors — code must compile clean with zero warnings.

## Playwright E2E Tests

```bash
cd playwright && npx playwright test <file>
```

## Local Dev (Docker)

```bash
make run          # all services
AWS_REGION=ap-northeast-2 AWS_DEFAULT_REGION=ap-northeast-2 make infra        # infrastructure only (LocalStack, DynamoDB)
make stop         # stop all

docker start ratel-localstack-init-1 # Reset database
```

