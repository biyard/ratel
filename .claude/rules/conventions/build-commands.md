---
globs: ["Makefile", "docker-compose*.yml", "app/ratel/Cargo.toml"]
---

# Build & Verification Commands

## Dioxus App (app/ratel/)

```bash
# MUST run after any code change in app/ratel/
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web

# Dev server (port 8000)
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```

**`DYNAMO_TABLE_PREFIX` is required at compile time** for DynamoEntity. Use `ratel-dev` for dev, `ratel-local` for Docker local.

## Playwright E2E Tests

```bash
cd playwright && npx playwright test <file>
```

## Local Dev (Docker)

```bash
make run          # all services
make infra        # infrastructure only (LocalStack, DynamoDB)
make stop         # stop all
```
