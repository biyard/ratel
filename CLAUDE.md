# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ratel is a decentralized legislative platform built with Rust and TypeScript, designed to bridge the gap between crypto users and policymakers. The project consists of multiple Rust services and a Next.js web frontend.

## Architecture

This is a monorepo with a workspace structure:
- **packages/** - Rust workspace packages (APIs, workers, shared DTOs)
- **ts-packages/** - TypeScript packages (Next.js web frontend)
- **deps/** - Shared Rust SDK dependencies
- **benchmark/** - Performance testing tools
- **kaia/** - Blockchain contracts
- **tests/** - Playwright integration tests

### Key Components

- **main-api** (`packages/main-api/`) - Primary REST API built with Axum
- **fetcher** (`packages/fetcher/`) - Data fetching service for legislative information
- **image-worker** (`packages/image-worker/`) - Image processing service
- **telegram-bot** (`packages/telegram-bot/`) - Telegram integration
- **dto** (`packages/dto/`) - Shared data transfer objects
- **web** (`ts-packages/web/`) - Next.js frontend with React 19

## Build System

### Rust Services
- Rust workspace managed by Cargo
- Each service has its own Makefile
- Uses custom build profiles (wasm-dev, server-dev, android-dev)
- Common dependencies defined in workspace Cargo.toml

### Frontend
- Next.js with TypeScript
- Package manager: pnpm
- Uses Tailwind CSS v4

## Development Commands

### Docker (Recommended for Local Development)
```bash
# Start all services with Docker Compose
docker-compose --profile development up -d

# Start specific services
docker-compose up -d postgres redis hasura main-api web

# Include telegram bot (requires TELEGRAM_TOKEN)
docker-compose --profile telegram up -d

# View logs
docker-compose logs -f ratel-main-api-1
docker-compose logs -f ratel-fetcher-1
docker-compose logs -f ratel-web-1

# Stop all services
docker-compose down

# Code changes will be reflected automatically to each docker
```

### Manual Service Development
```bash
# Run main API
cd packages/main-api && make run

# Run web frontend  
cd ts-packages/web && make run

# Run any service via root Makefile
make run SERVICE=main-api
make serve SERVICE=main-api
```

### Building
```bash
# For main-api
cd packages/main-api && make build

# For fetcher
cd packages/fetcher && make build

# Build for different environments
ENV=dev make build SERVICE=main-api
```

### Testing
```bash
# Run Rust tests for main-api
cd packages/main-api && make test

# Run Playwright tests (from root)
make test
# or
npx playwright test
```

### Linting/Formatting
```bash
# Next.js linting
cd ts-packages/web && npm run lint
```

## Key Technologies

- **Backend**: Rust, Axum, DynamoDB (with deprecated SQLx (PostgreSQL)), Tokio
- **Frontend**: Next.js 15, React 19, TailwindCSS v4, Apollo GraphQL
- **Testing**: Playwright for E2E tests
- **Infrastructure**: AWS (Lambda, S3, RDS), Docker
- **Blockchain**: Ethereum-compatible contracts

## Testing

### Playwright Tests

Playwright tests are located in the `tests/` directory and follow a specific structure that mirrors the app router structure:

#### Directory Structure
Tests should be organized to match the Next.js app router directory structure:
- `tests/(social)/` - Tests for pages in `ts-packages/web/src/app/(social)/`
- `tests/teams/` - Tests for pages in `ts-packages/web/src/app/teams/`
- etc.

#### Naming Convention
Test files should follow this naming pattern:
- `{test-name}.auth.spec.ts` - Tests for authenticated users
- `{test-name}.anon.spec.ts` - Tests for anonymous/guest users

#### Examples
- Tests for `ts-packages/web/src/app/(social)/page.tsx` → `tests/(social)/homepage.auth.spec.ts` and `tests/(social)/homepage.anon.spec.ts`
- Tests for `ts-packages/web/src/app/teams/[username]/page.tsx` → `tests/teams/[username]/team-page.auth.spec.ts`

#### Test Structure
All test files should use the standard Playwright test structure with describe blocks:

```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature Name', () => {
  test('should perform specific action', async ({ page }) => {
    // Test implementation
  });
});
```

## Database

- Now, SQLx has been deprecated.
- Under development of migration to DynamoDB.

## Docker Services

The docker-compose.yaml provides:

- **postgres** - PostgreSQL database (port 5432)
- **hasura** - GraphQL API (port 28080)
- **main-api** - REST API (port 3000)
- **fetcher** - Legislative data fetching (port 3001)
- **image-worker** - Image processing service
- **telegram-bot** - Telegram bot (optional, requires TELEGRAM_TOKEN)
- **web** - Next.js frontend (port 8080)
- **localstack** - DynamoDB (port 4566)

Access points:
- Web Application: http://localhost:3002
- Main API: http://localhost:3000
- Hasura Console: http://localhost:28080
- Fetcher API: http://localhost:3001

## Environment Configuration

Copy `.env.example` to `.env` and configure:
- `TELEGRAM_TOKEN` - Required for telegram bot functionality
- AWS credentials - Leave empty for local development
- Other optional integrations (Slack, OpenAPI)

## Development Notes

- The web frontend requires environment setup via `ts-packages/web/setup-env.sh`
- Services use environment variables for configuration
- AWS integration for cloud deployments (disabled in local Docker setup)
- Telegram SDK integration for bot functionality
- Real-time features using WebSockets and collaborative editing
- Database migrations run automatically on startup when MIGRATE=true

## Main API
Main Api package is the main backend APIs for Ratel written by Rust.
- location: `packages/main-api`
- Language: Rust

### `v1` endpoints
- `v1` has been implemented based on RPC endpoints convention.
- `v1` endpoints has used postgres database models implemented in `dto` packages, which is deprecated.

### `v2` endpoints
- `v2` has been implemented based on Axum native convention instead of RPC.
- `v1` endpoints has used postgres database models implemented in `packages/main-api/src/models`

### `v3` endpoints
- `v3` will be implemented based on Axum native convention.
- `v3` endpoints will use DynamoDB models implemented in `packages/main-api/src/models/dynamo_tables/main`


## DTO
DTO package is deprecated.
- location: `packages/dto`

## by_macro
`by_macro` package provides macros to simplify the code.

### DynamoEntity derive
`DynamoEntity` generates CRUD utility functions for interaction with DynamoDB.

#### Structure attribute

- `DYNAMO_TABLE_PREFIX` is required for composing full table name.
  - For example, if `DYNAMO_TABLE_PREFIX` is set to `ratel-local` when building it, the table name of the entity will be set to `ratel-local-main` as default.
  - If `table` attribute is set to `users`, the full table name will be `ratel-local-users`.

| Attribute  | Description                                  | Default             |
|:-----------|----------------------------------------------|:--------------------|
| table      | table name except for prefix                 | main                |
| result_ty  | Result type                                  | std::result::Result |
| error_ctor | Error type                                   | create::Error2      |
| pk_name    | Partition key name                           | pk                  |
| sk_name    | (optional) Sort key name (none for removing) | sk                  |


#### Field attribute
| Attribute | Description                          |
|:----------|--------------------------------------|
| prefix    | Prefix of indexed value              |
| index     | Index name                           |
| pk        | Partition key of index               |
| sk        | sort key of index                    |
| name      | Function name for querying the index |

#### Usage
The below code is an example of using DynamoEntity
- If `DYNAMO_TABLE_PREFIX` environment is set to `ratel-local` and `table` is set to `main`, the practical table name will be `ratel-local-main`.
- For the first `gsi1-index`, it can be queried by calling `EmailVerification::find_by_email_and_code`.
  - `email` field will be indexedm to `gsi1_pk` field.
    - the value of `gsi1_pk` field will be `EMAIL#a@example.com` if `email` is `a@example.com`.
  - `value` field will be indexed to `gsi1_sk` field.
    - Because no prefix is set, `gsi1_sk` will be same to `value`.
- For the second `gsi2-index`, we can query by calling `EmailVerification::find_by_code`.
  - `gsi2_pk` will be set to naive value of `value`.
  - `gsi2_sk` will be set to `created_at` with `TS` prefix such as `TS#{created_at}`

```rust
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct EmailVerification {
    pub pk: String,
    pub sk: String,
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "EMAIL", name = "find_by_email_and_code", index = "gsi1", pk)]
    pub email: String,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", name = "find_by_code", pk)]
    pub value: String,
    pub expired_at: i64,
    pub attemp_count: i32,
}

impl EmailVerification {
    pub fn new(email: String, value: String, expired_at: i64) -> Self {
        let pk = format!("EMAIL#{}", email);
        let sk = format!("VERIFICATION#{}", value);
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            email,
            created_at,
            value,
            expired_at,
            attemp_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[tokio::test]
    async fn test_email_verification_new() {
        let conf = aws_sdk_dynamodb::Config::builder()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                "test", "test", None, None, "dynamo",
            ))
            .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
            .endpoint_url("http://localhost:4566")
            .behavior_version_latest()
            .build();

        let cli = aws_sdk_dynamodb::Client::from_conf(conf);
        let now = chrono::Utc::now().timestamp();
        let expired_at = now + 3600; // 1 hour later
        let email = format!("a+{}@example.com", now);

        let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);

        assert_eq!(EmailVerification::table_name(), "ratel-local-main");
        assert_eq!(EmailVerification::pk_field(), "pk");
        assert_eq!(EmailVerification::sk_field(), Some("sk"));

        assert!(
            ev.create(&cli).await.is_ok(),
            "failed to create email verification"
        );

        let fetched_ev = EmailVerification::get(&cli, ev.pk.clone(), Some(ev.sk.clone())).await;

        assert!(fetched_ev.is_ok(), "failed to fetch email verification");
        let fetched_ev = fetched_ev.unwrap();
        assert!(fetched_ev.is_some(), "email verification not found");
        let fetched_ev = fetched_ev.unwrap();
        assert_eq!(fetched_ev.email, ev.email);
        assert_eq!(fetched_ev.value, ev.value);
        assert_eq!(fetched_ev.expired_at, ev.expired_at);
    }

    #[tokio::test]
    async fn test_email_verification_delete() {
        let conf = aws_sdk_dynamodb::Config::builder()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                "test", "test", None, None, "dynamo",
            ))
            .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
            .endpoint_url("http://localhost:4566")
            .behavior_version_latest()
            .build();

        let cli = aws_sdk_dynamodb::Client::from_conf(conf);
        let now = chrono::Utc::now().timestamp();
        let expired_at = now + 3600; // 1 hour later
        let email = format!("d+{}@example.com", now);
        let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);
        assert!(
            ev.create(&cli).await.is_ok(),
            "failed to create email verification"
        );
        let fetched_ev = EmailVerification::get(&cli, ev.pk.clone(), Some(ev.sk.clone())).await;
        assert!(fetched_ev.is_ok(), "failed to fetch email verification");
        let fetched_ev = fetched_ev.unwrap();
        assert!(fetched_ev.is_some(), "email verification not found");
        let fetched_ev = fetched_ev.unwrap();
        assert_eq!(fetched_ev.email, ev.email);
        assert_eq!(fetched_ev.value, ev.value);
        assert_eq!(fetched_ev.expired_at, ev.expired_at);
        assert!(
            EmailVerification::delete(&cli, ev.pk.clone(), Some(ev.sk.clone()))
                .await
                .is_ok(),
            "failed to delete email verification"
        );
        let fetched_ev = EmailVerification::get(&cli, ev.pk.clone(), Some(ev.sk.clone())).await;
        assert!(fetched_ev.is_ok(), "failed to fetch email verification");
        let fetched_ev = fetched_ev.unwrap();
        assert!(fetched_ev.is_none(), "email verification should be deleted");
    }

    #[tokio::test]
    async fn test_email_verification_find_by_email_and_code() {
        let conf = aws_sdk_dynamodb::Config::builder()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                "test", "test", None, None, "dynamo",
            ))
            .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
            .endpoint_url("http://localhost:4566")
            .behavior_version_latest()
            .build();

        let cli = aws_sdk_dynamodb::Client::from_conf(conf);
        let now = chrono::Utc::now().timestamp();
        let expired_at = now + 3600; // 1 hour later
        for i in 0..5 {
            let email = format!("l+{now}-{i}@example.com");

            let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);
            assert!(
                ev.create(&cli).await.is_ok(),
                "failed to create email verification"
            );
        }

        let fetched_evs = EmailVerification::find_by_email_and_code(
            &cli,
            format!("EMAIL#l+{now}-0@example.com"),
            EmailVerificationQueryOption::builder()
                .limit(10)
                .sk("a".to_string()),
        )
        .await;
        assert!(fetched_evs.is_ok(), "failed to find email verification");
        let (fetched_evs, last_evaluated_key) = fetched_evs.unwrap();
        assert!(
            last_evaluated_key.is_none(),
            "last_evaluated_key should be empty"
        );
        assert_eq!(fetched_evs.len(), 1, "should find one email verification");
        assert_eq!(fetched_evs[0].email, format!("l+{now}-0@example.com"));
    }

    #[tokio::test]
    async fn test_email_verification_find_by_code() {
        let conf = aws_sdk_dynamodb::Config::builder()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                "test", "test", None, None, "dynamo",
            ))
            .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
            .endpoint_url("http://localhost:4566")
            .behavior_version_latest()
            .build();

        let cli = aws_sdk_dynamodb::Client::from_conf(conf);
        let now = chrono::Utc::now().timestamp();
        let expired_at = now + 3600; // 1 hour later
        for i in 0..5 {
            let email = format!("c+{now}-{i}@example.com");

            let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);
            assert!(
                ev.create(&cli).await.is_ok(),
                "failed to create email verification"
            );
        }

        sleep(Duration::from_millis(500));

        let fetched_evs = EmailVerification::find_by_code(
            &cli,
            format!("aaaa"),
            EmailVerificationQueryOption::builder()
                .limit(4)
                .sk("TS".to_string()),
        )
        .await;
        assert!(fetched_evs.is_ok(), "failed to find email verification");
        let (fetched_evs, last_evaluated_key) = fetched_evs.unwrap();

        println!("fetched_evs: {:?}", fetched_evs.len());
        assert!(
            last_evaluated_key.is_some(),
            "last_evaluated_key should not be empty"
        );
        assert_eq!(fetched_evs.len(), 4, "should find one email verification");
        assert_eq!(fetched_evs[0].email, format!("c+{now}-4@example.com"));
        assert_eq!(fetched_evs[0].email, format!("c+{now}-3@example.com"));
        assert_eq!(fetched_evs[0].email, format!("c+{now}-2@example.com"));
        assert_eq!(fetched_evs[0].email, format!("c+{now}-1@example.com"));
    }
}
```
- Please make sure that your playwright code is alway success by executing `make test` yourself.