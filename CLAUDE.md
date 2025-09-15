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

| Attribute  | Description                               | Default                 |
|:-----------|-------------------------------------------|:------------------------|
| table      | table name except for prefix              | main                    |
| prefix_env | Environment variable getting table prefix | DYNAMO_TABLE_PREFIX     |
| result     | Result type                               | std::result::Result     |
| error_ctor | Error type                                | aws_sdk_dynamodb::Error |


#### Field attribute
| Attribute | Description             |
|:----------|-------------------------|
| prefix    | Prefix of indexed value |
| index     | Index name              |
| pk        | Partition key of index  |
| sk        | sort key of index       |

#### Usage
The below code is an example of using DynamoEntity
- If `DYNAMO_TABLE_PREFIX` environment is set to `ratel-local` and `table` is set to `main`, the practical table name will be `ratel-local-main`.
- `email` field will be indexedm to `gsi1_pk` field.
   - the value of `gsi1_pk` field will be `EMAIL#a@example.com` if `email` is `a@example.com`.
- `value` field will be indexed to `gsi1_sk` field.
   - Because no prefix is set, `gsi_sk` will be same to `value`.

```rust
#[derive(DynamoEntity)]
#[dynamo(table = "main", prefix_env = "DYNAMO_TABLE_PREFIX", result = "crate::Result", error_ctor = "crate::Error::DynamoDbError")]
pub struct Model {
    pub pk: String,

    #[dynamo(prefix = "EMAIL", index = "gsi1", pk)]
    pub email: String,

    #[dynamo(index = "gsi1", sk)]
    pub value: String,

    pub expired_at: i64,
}

fn main() -> crate::Result<()> {
    // Creation
    let model = Model::new();            // = Default::default()
    model.create(&cli).await?;

    // Updating (PK-only 예시)
    Model::builder()
        .with_email("user@example.com".to_string())
        .with_expired_at(11)
        .execute(&cli, "PK#123")
        .await?;

    // Delete
    Model::delete(&cli, "PK#123").await?;

    let got = Model::get(&cli, "PK#123").execute().await?;

    let (models, bookmark) = Model::query()
        .on_gsi("EMAIL#user@example.com".to_string())
        .with_sk_prefix("{code}".to_string())
        .with_bookmark(bookmark)
        .execute(&cli)
        .await?;

    Ok(())
}
```
