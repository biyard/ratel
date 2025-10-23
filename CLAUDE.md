# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ratel is a decentralized legislative platform built with Rust and TypeScript, designed to bridge the gap between crypto users and policymakers. The project consists of multiple Rust services and a Vite/React web frontend.

## Architecture

This is a monorepo with a workspace structure:
- **packages/** - Rust workspace packages (APIs, workers, shared DTOs)
- **ts-packages/** - TypeScript packages (Vite/React web frontend)
- **kaia/** - Blockchain contracts
- **tests/** - Playwright integration tests

### Key Components

- **main-api** (`packages/main-api/`) - Primary REST API built with Axum
- **fetcher** (`packages/fetcher/`) - Data fetching service for legislative information
- **image-worker** (`packages/image-worker/`) - Image processing service
- **telegram-bot** (`packages/telegram-bot/`) - Telegram integration
- **dto** (`packages/dto/`) - Shared data transfer objects
- **web** (`ts-packages/web/`) - Vite frontend with React 19

### Frontend
- All page implementations are placed in `ts-packages/web/src/app` as similar tree of routes.
- Each page directory will contains below
  - `{name}-page.tsx` is a main component for the page.
  - `use-{name}-page-controller.tsx` implement controller class to manage and handle events on the page. And also it manage `use-{name}-data.tsx`.
  - `use-{name}-data.tsx` fetches all remote data needed by the page.
  - `{name}-page-i18n.tsx` defines `ko` and `en` languages.
  - `{name}-page.anon.spec.tsx` is Playwright tests for anonymous users for the page.
  - `{name}-page.auth.spec.tsx` is Playwright tests for authenticated users for the page.
  - `{name}-page.stories.tsx` is for Storybook file for the page.
- `ts-packages/web/src/features` defines `feature`-based modules.
  - `features/{name}/components` implements feature components and their storybook files
  - `features/{name}/hooks` implements hooks for the feature
  - `features/{name}/utils` provides utility functions for the feature

## Build System

### Rust Services
- Rust workspace managed by Cargo
- Each service has its own Makefile
- Uses custom build profiles (wasm-dev, server-dev, android-dev)
- Common dependencies defined in workspace Cargo.toml

### Frontend
- Vite/React19 with TypeScript
- Package manager: pnpm
- Uses Tailwind CSS v4

## Development Commands

### Docker (Recommended for Local Development)
```bash
# Start all services with Docker Compose
docker-compose --profile development up -d

# Start specific services
docker-compose up -d redis hasura main-api web

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
cd ts-packages/web && make test
# or
cd ts-packages/web && npx playwright test
```

### Linting/Formatting
```bash
# Vite/React linting
cd ts-packages/web && npm run lint
```

## Key Technologies

- **Backend**: Rust, Axum, DynamoDB, Tokio, Askama(for SSR)
- **Frontend**: Vite, React 19, TailwindCSS v4, Apollo GraphQL
- **Testing**: Playwright for E2E tests
- **Infrastructure**: AWS (Lambda, S3, RDS), Docker
- **Blockchain**: Ethereum-compatible contracts

## Docker Services

The docker-compose.yaml provides:

- **main-api** - REST API (port 3000)
- **fetcher** - Legislative data fetching (port 3001)
- **image-worker** - Image processing service
- **telegram-bot** - Telegram bot (optional, requires TELEGRAM_TOKEN)
- **web** - Vite/React frontend (port 8080)
- **localstack** - DynamoDB (port 4566)

Access points:
- Web Application: http://localhost:3002
- Main API: http://localhost:3000
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

### `v3` endpoints
- `v3` will be implemented based on Axum native convention.
- `v3` endpoints will use DynamoDB models implemented in `packages/main-api/src/models/dynamo_tables/main`

### Testing Backend APIs
The main-api uses custom HTTP request macros for testing API endpoints. Tests are located in `tests.rs` files within controller modules.

#### Test File Structure
- Test files are named `tests.rs` and placed within the corresponding controller module directory
- Example: `/packages/main-api/src/controllers/v3/posts/tests.rs` contains tests for all post-related endpoints
- Tests use the `#[tokio::test]` attribute for async test functions

#### HTTP Request Macros
The codebase provides convenient macros for making HTTP requests in tests, defined in `/packages/main-api/src/tests/macros.rs`:

**Available Macros:**
- `get!` - For GET requests
- `post!` - For POST requests
- `patch!` - For PATCH requests
- `put!` - For PUT requests
- `delete!` - For DELETE requests

**Macro Parameters:**
The macros accept the following parameters (in order):
1. `app:` - The application instance from TestContextV3
2. `path:` - The API endpoint path (e.g., "/v3/posts/{id}")
3. `headers:` (optional) - HTTP headers (e.g., authentication headers)
4. `body:` (optional) - Request body as JSON object literal
5. `response_type:` (optional) - Expected response type (defaults to `serde_json::Value`)

**Return Value:**
All macros return a tuple: `(StatusCode, HeaderMap, ResponseBody)`

#### Example Test Patterns

**Basic GET request:**
```rust
let (status, _headers, body) = get! {
    app: app,
    path: format!("/v3/posts/{}", post_pk),
    headers: test_user.1.clone(),
    response_type: PostDetailResponse
};
assert_eq!(status, 200);
```

**POST request with body:**
```rust
let (status, _headers, body) = post! {
    app: app,
    path: "/v3/posts",
    headers: test_user.1.clone(),
    body: {
        "title": "Test Post",
        "content": "<p>Test Content</p>"
    },
    response_type: CreatePostResponse
};
assert_eq!(status, 200);
```

**Request without authentication:**
```rust
let (status, _headers, body) = get! {
    app: app,
    path: format!("/v3/posts/{}", post_pk),
    response_type: PostDetailResponse
};
```

#### Test Context Setup
Use `TestContextV3::setup()` to get test context with:
- `app` - The application instance
- `test_user` - A tuple of `(User, HeaderMap)` for authenticated requests
- `now` - Current timestamp for unique test data
- `ddb` - DynamoDB client for direct database operations

**Example:**
```rust
#[tokio::test]
async fn test_get_post() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;
    // Your test code here
}
```

#### Best Practices for Writing Tests
1. **Test each handler function comprehensively** - Cover success cases, error cases, and edge cases
2. **Use descriptive test names** - Name tests clearly (e.g., `test_get_post_when_authenticated`)
3. **Test permissions** - Verify that authentication/authorization works correctly
4. **Test with and without auth** - Test endpoints both as authenticated users and guests
5. **Verify response structure** - Check status codes, response bodies, and headers
6. **Test error cases** - Verify correct error codes and messages for invalid requests
7. **Always run tests before committing** - Execute `cd packages/main-api && make test` to ensure all tests pass

#### Common Test Scenarios
For a typical API handler, write tests for:
- ✅ Successful request with valid data
- ✅ Request with authentication vs without authentication
- ✅ Request with invalid/missing parameters
- ✅ Request for non-existent resources (404)
- ✅ Unauthorized access (401/403)
- ✅ Related data fetching (e.g., with comments, likes, etc.)
- ✅ Permission-based data filtering (e.g., hiding space info)

#### Example Test Suite Structure
```rust
// Test successful operations
#[tokio::test]
async fn test_get_post_when_authenticated() { /* ... */ }

// Test guest access
#[tokio::test]
async fn test_get_post_when_not_authenticated() { /* ... */ }

// Test with related data
#[tokio::test]
async fn test_get_post_with_comments() { /* ... */ }

// Test error cases
#[tokio::test]
async fn test_get_nonexistent_post() { /* ... */ }

// Test permissions
#[tokio::test]
async fn test_get_post_permissions() { /* ... */ }
```


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
