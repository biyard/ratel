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

#### Core Services

- **main-api** (`packages/main-api/`) - Primary REST API built with Axum
- **fetcher** (`packages/fetcher/`) - Data fetching service for legislative information
- **survey-worker** (`packages/survey-worker/`) - AWS Lambda worker for survey operations
- **image-worker** (`packages/image-worker/`) - Image processing service
- **telegram-bot** (`packages/telegram-bot/`) - Telegram integration
- **web** (`ts-packages/web/`) - Vite frontend with React 19

#### Support Packages

- **by-macros** (`packages/by-macros/`) - Procedural macros including DynamoEntity derive (v0.6.*)
- **by-axum** (`packages/by-axum/`) - Axum framework extensions (v0.2.*)
- **by-types** (`packages/by-types/`) - Shared type definitions (v0.3.*)
- **bdk** (`packages/bdk/`) - Blockchain development kit with Ethereum support
- **btracing** (`packages/btracing/`) - Tracing and observability utilities
- **dto** (`packages/dto/`) - Shared data transfer objects
- **dioxus-translate** (`packages/dioxus-translate/`) - i18n translation framework
- **rest-api** (`packages/rest-api/`) - REST API utilities with test support
- **migrator** (`packages/migrator/`) - Database migration utilities

### Frontend

#### Package Management & Runtime
- **Package Manager:** pnpm 10.18.2
- **Node Version:** 22.14
- **React:** 19.2.0
- **Vite:** 7.1.9
- **TypeScript:** 5.9.3
- **TailwindCSS:** v4.1.14

#### Key Dependencies
- **State Management:** Zustand 5.0.8
- **Data Fetching:** TanStack React Query 5.90.2, Axios 1.12.2
- **Blockchain:** Ethers.js 6.15.0
- **Rich Text Editor:** Tiptap 2.26+ (with tables, collaboration support)
- **UI Components:** Radix UI, Heroicons, Lucide React
- **Charts:** Recharts 3.3.0
- **Forms & Validation:** Zod 4.1.12, React Hook Form
- **Utilities:** dayjs, date-fns, i18next, DOMPurify

#### Testing Infrastructure
- **Framework:** Playwright 1.56.1
- **Test Projects:** `anonymous`, `authenticated`, `admin`, `e2e-web`
- **Test Patterns:** `*.anon.spec.ts`, `*.auth.spec.ts`, `*.admin.spec.ts`
- **Configuration:** `/playwright.config.ts`

#### Page Structure
- All page implementations are placed in `ts-packages/web/src/app` as similar tree of routes.
- Each page directory will contains below
  - `{name}-page.tsx` is a main component for the page.
  - `use-{name}-page-controller.tsx` implement controller class to manage and handle events on the page. And also it manage `use-{name}-data.tsx`.
  - `use-{name}-data.tsx` fetches all remote data needed by the page.
  - `{name}-page-i18n.tsx` defines `ko` and `en` languages.
  - `{name}-page.anon.spec.tsx` is Playwright tests for anonymous users for the page.
  - `{name}-page.auth.spec.tsx` is Playwright tests for authenticated users for the page.
  - `{name}-page.stories.tsx` is for Storybook file for the page.

#### Feature Modules
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

### Root Makefile Commands
```bash
# Start all services with Docker Compose
make run

# Stop all services
make stop

# Build main-api and web together
make build-with-web

# Deploy to AWS via CDK
make deploy

# Run specific service locally (without Docker)
make serve SERVICE=main-api
make serve SERVICE=fetcher

# Run tests
make test
```

### Service-Specific Development
```bash
# Main API
cd packages/main-api && make run      # Dev with cargo-watch
cd packages/main-api && make build    # Build release binary
cd packages/main-api && make test     # Run Rust tests

# Web frontend
cd ts-packages/web && make run        # Dev server on port 8080
cd ts-packages/web && make build      # Production build
cd ts-packages/web && make test       # Playwright tests
cd ts-packages/web && make storybook  # Storybook on port 6006

# Fetcher
cd packages/fetcher && make run
cd packages/fetcher && make build

# Build for different environments
ENV=dev make build SERVICE=main-api
```

### Linting/Formatting
```bash
# Vite/React linting
cd ts-packages/web && npm run lint
```

## Key Technologies

- **Backend**: Rust 2024, Axum 0.8.1, DynamoDB, Tokio, Askama (SSR)
- **Frontend**: Vite 7, React 19, TailwindCSS v4.1, Zustand, TanStack Query
- **Testing**: Playwright 1.56, Tokio test framework, custom HTTP test macros
- **Infrastructure**: AWS (Lambda, S3, RDS, SQS, Bedrock), Docker, LocalStack
- **Blockchain**: Ethers.js 6.15, Kaia network (Ethereum-compatible)
- **Authentication**: Firebase, JWT, DID/Verifiable Credentials
- **Payments**: Portone gateway, Binance API
- **AI**: AWS Bedrock agents
- **Messaging**: Telegram Bot API, FCM notifications
- **API Spec**: Aide (OpenAPI/Swagger documentation)

## Docker Services

The docker-compose.yaml provides:

- **main-api** - REST API (port 3000)
- **fetcher** - Legislative data fetching (port 3001)
- **survey-worker** - Survey worker service (port 3002)
- **image-worker** - Image processing service
- **telegram-bot** - Telegram bot (optional, requires TELEGRAM_TOKEN)
- **web** - Vite/React frontend (port 8080)
- **storybook** - Component documentation (port 6006)
- **localstack** - AWS services emulation (port 4566)
- **localstack-init** - DynamoDB table initialization
- **dynamodb-admin** - DynamoDB web UI (port 8081)

Access points:
- Web Application: http://localhost:8080
- Main API: http://localhost:3000
- Fetcher API: http://localhost:3001
- Survey Worker: http://localhost:3002
- Storybook: http://localhost:6006
- DynamoDB Admin: http://localhost:8081
- LocalStack: http://localhost:4566

## Environment Configuration

Copy `.env.example` to `.env` and configure:

### Backend Environment Variables
- `ENV` - Environment (dev, staging, prod)
- `PORT` - Service port
- `RUST_LOG` - Logging level
- `WEB_BUILD` - Web build configuration
- `DYNAMO_ENDPOINT` - DynamoDB endpoint (http://localstack:4566 for local)
- `DYNAMO_TABLE_PREFIX` - Table prefix (ratel-local for dev)
- `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` - AWS credentials
- `TELEGRAM_TOKEN` - Telegram bot authentication token
- `FIREBASE_PROJECT_ID` - Firebase project identifier
- `PORTONE_*` - Payment gateway credentials
- `BEDROCK_AGENT_*` - AWS Bedrock AI agent configuration
- `BIYARD_*` - External Biyard API integration

### Frontend Environment Variables (VITE_*)
- `VITE_API_URL` - Backend API base URL
- `VITE_LOG_LEVEL` - Frontend logging level
- `VITE_RPC_URL` - Blockchain RPC endpoint
- `VITE_BLOCK_EXPLORER_URL` - Blockchain explorer URL (Kaia network)
- `VITE_FIREBASE_*` - Firebase authentication config
- `VITE_PORTONE_*` - Payment UI configuration
- `VITE_OPERATOR_ADDRESS` - Blockchain operator address

## Development Notes

- The web frontend requires environment setup via `ts-packages/web/setup-env.sh`
- Services use environment variables for configuration
- AWS integration for cloud deployments (disabled in local Docker setup)
- Telegram SDK integration for bot functionality
- Real-time features using WebSockets and collaborative editing
- Database migrations run automatically on startup when MIGRATE=true

## Advanced Features

### Notable Platform Capabilities
- **MCP Integration** - Model Context Protocol server for LLM interactions
- **Decentralized Identity** - DID (Decentralized Identifiers) & Verifiable Credentials
- **Collaborative Editing** - Real-time document collaboration using Tiptap + Yjs
- **Real-time Notifications** - Multi-channel delivery via WebSockets, Telegram, and FCM
- **Reward System** - Complex points and membership tier management
- **AI Assistance** - AWS Bedrock agent integration for legislative drafting
- **Document Processing** - PDF generation and manipulation
- **Workspace Management** - Multi-space organization with granular permissions
- **DAO Governance** - Blockchain-based voting and proposal systems

## CI/CD Workflows

### GitHub Actions
- **dev-workflow.yml** - Automated development branch builds and tests
- **pr-workflow.yml** - Pull request validation and checks
- **prod-workflow.yml** - Production deployment automation

### Deployment Infrastructure
- **AWS CDK** - Infrastructure as Code (TypeScript)
- **ECR** - Container registry for Docker images
- **Lambda** - Serverless function deployments
- **CloudFormation** - Stack management and orchestration

## DynamoDB Configuration

### Local Development Setup
- **Endpoint:** `http://localstack:4566`
- **Table Prefix:** `ratel-local` (development environment)
- **Main Table:** `ratel-local-main` (unified table with multiple GSIs)
- **Global Secondary Indexes:** Email-based, username-based, timestamp-based queries
- **Admin UI:** Available at http://localhost:8081

### Table Initialization
- Automatic schema creation via `localstack-init` container
- Admin user seeded on startup
- Migration support via `migrator` package

## Main API
Main Api package is the main backend APIs for Ratel written by Rust.
- location: `packages/main-api`
- Language: Rust

### API Architecture

#### v3 API Controllers
Located at `packages/main-api/src/controllers/v3/`:
- **auth/** - Authentication endpoints (login, signup, token refresh)
- **users/** - User management (profiles, settings, credentials)
- **posts/** - Post CRUD operations (create, read, update, delete)
- **spaces/** - Space management (creation, membership, settings)
- **teams/** - Team operations (creation, invitations, roles)
- **payments/** - Payment processing (Portone integration)
- **rewards/** - Rewards system (points, tiers, distributions)
- **notifications/** - Notification delivery (push, email, in-app)
- **assets/** - Asset management (uploads, downloads)
- **reports/** - Reporting features (moderation, analytics)

#### Feature Modules
Located at `packages/main-api/src/features/`:
- **spaces/** - Space domain logic and business rules
- **teams/** - Team functionality and permissions
- **membership/** - Membership tier and access control
- **payment/** - Payment gateway integration and processing
- **notification/** - Multi-channel notification system
- **did/** - Decentralized Identity implementation
- **migration/** - Data migration utilities
- **telegrams/** - Telegram bot integration logic

#### v3 Endpoints
- `v3` endpoints are implemented based on Axum native convention
- `v3` endpoints use DynamoDB models implemented in `packages/main-api/src/models/dynamo_tables/main`
- API documentation available via Aide (OpenAPI/Swagger)

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
- you should register i18n to i18n/config.tsx
- you should add page route path to router.tsx