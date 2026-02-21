# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ratel is a decentralized legislative platform built with Rust (Dioxus fullstack) and DynamoDB, designed to bridge the gap between crypto users and policymakers. The primary web application is built using Dioxus 0.7 with fullstack rendering (SSR + client-side hydration), with DynamoDB as the database.

## Architecture

This is a monorepo with a Cargo workspace structure:

- **app/** - Dioxus fullstack application modules (auth, posts, spaces, socials, shell)
- **packages/** - Rust workspace packages (APIs, workers, shared libraries, macros)
- **ts-packages/** - TypeScript packages (legacy Vite/React web frontend)
- **kaia/** - Blockchain contracts (Hardhat)
- **contracts/** - Primary blockchain contracts (Hardhat)

### Dioxus App Modules (`app/`)

The main web application is composed of modular Dioxus packages under `app/`. Each module follows a consistent structure with feature flags for multi-platform support (`web`, `server`, `desktop`, `mobile`).

#### App Shell (`app/shell/`)

Main application entry point that orchestrates all sub-modules via Dioxus Router.

- **Package:** `app-shell`
- **Entry:** `src/main.rs` with `Dioxus.toml` configuration
- **Routing:** `src/route.rs` defines top-level routes using `ChildRouter`
- **Layout:** `src/layout.rs` wraps all pages in `AppLayout`
- **Port:** 8000 (via `dx serve`)

**Routes:**
- `/` - Index/home page
- `/auth/:..rest` - Authentication (delegates to `ratel-auth`)
- `/posts/:..rest` - Post feed (delegates to `ratel-post`)
- `/:username/:..rest` - User profile (delegates to `ratel-user-shell`)
- `/teams/:teamname/:..rest` - Team pages (delegates to `ratel-team-shell`)
- `/spaces/:..rest` - Space pages (delegates to `space-shell`)

#### Common (`app/common/`)

Foundational shared library used by all other app modules.

- **Package:** `common`
- **Provides:** Types, models, components, configuration, utilities, AWS integration
- **Key types:**
  - `Partition` enum - DynamoDB partition key variants (~90 entity types)
  - `EntityType` enum - DynamoDB sort key variants (~130 entity types)
  - `User` model - DynamoEntity with 6 GSIs (email, username, phone, user_type, etc.)
- **Config:** `src/config/server/` (DynamoDB, S3, SES, SNS), `src/config/web/` (Firebase)
- **Components:** Badge, Button, Layover, Popup, ThemeSwitcher
- **Tailwind:** `tailwind.css` is the shared TailwindCSS v4 entrypoint for all Dioxus apps
- **Utils:** `src/utils/aws/` (DynamoDB, S3, SES, SNS helpers), password, sha256, time

#### Auth (`app/auth/`)

- **Package:** `ratel-auth`
- **Handles:** Login, signup, email/phone verification (SES/SNS), password reset, OAuth, logout
- **Models:** User, EmailVerification, PhoneVerification, UserOAuth, UserRefreshToken, UserTelegram, UserEVMAddress
- **Components:** AuthProvider, LoginModal, SignUpModal
- **Routes:** `/auth` (login), `/auth/forgot-password` (reset)

#### Posts (`app/posts/`)

- **Package:** `ratel-post`
- **Handles:** Post CRUD, comments, likes, shares, visibility
- **Models:** Post, PostComment, PostLike, PostCommentLike, PostRepost, PostArtwork
- **Types:** PostType (Post, Article, Poll), PostStatus (Draft, Published, Archived), Visibility
- **Components:** FeedCard, FeedList

#### Spaces (`app/spaces/`)

Governance/legislative spaces with panels, voting, and proposals.

```
app/spaces/
  shell/          - space-shell (main container, navigation, layout)
  pages/
    overview/     - space-page-overview
    dashboard/    - space-page-dashboard
    actions/      - space-page-actions
    apps/         - space-page-apps
    report/       - space-page-report
  actions/
    poll/         - space-action-poll (voting)
```

- **Shell models:** Space, SpacePanelParticipant, SpacePanelQuota, VerifiedAttributes
- **Components:** SpaceNav (sidebar), SpaceTop (header with title, participate button)

#### Socials - Users (`app/socials/users/`)

User profile and dashboard.

```
app/socials/users/
  shell/          - ratel-user-shell (layout, routing)
  pages/
    post/         - ratel-user-post
    reward/       - ratel-user-reward
    setting/      - ratel-user-setting
    membership/   - ratel-user-membership
    draft/        - ratel-user-draft
    credential/   - ratel-user-credential (DID)
    space/        - ratel-user-space
```

#### Socials - Teams (`app/socials/teams/`)

Team management and organization.

```
app/socials/teams/
  shell/          - ratel-team-shell (layout, routing)
  pages/
    home/         - ratel-team-home
    draft/        - ratel-team-draft
    group/        - ratel-team-group
    member/       - ratel-team-member
    dao/          - ratel-team-dao (governance)
    reward/       - ratel-team-reward
    setting/      - ratel-team-setting
```

#### Interops (`app/interops/web-components/`)

- **Package:** `dioxus-components`
- Reusable Dioxus components library for web integration

### Dioxus App Module Structure

Each app module follows a consistent internal structure:

```
src/
  lib.rs          - Public API exports
  main.rs         - Entry point
  route.rs        - Dioxus Router routes
  layout.rs       - Layout wrapper component
  config.rs       - Configuration
  provider.rs     - State provider
  constants.rs    - Constants
  controllers/    - Server-side request handlers (feature: server)
  models/         - DynamoDB entities with DynamoEntity derive (feature: server)
  components/     - Dioxus UI components
  views/          - Page-level view components
  hooks/          - Dioxus hooks for state management
  interop/        - JavaScript interoperability (feature: web)
  server/         - Server-only functionality (feature: server)
  web/            - Web-only functionality (feature: web)
  utils/          - Helper functions
  dto/            - Data transfer objects
  types/          - Custom type definitions
```

### Feature Flags (All App Modules)

All Dioxus app packages use consistent feature flags:

- `web` - Dioxus web renderer, browser UI components
- `server` - Dioxus server renderer, DynamoDB, AWS SDK, backend handlers
- `desktop` - Dioxus desktop (Tauri) target
- `mobile` - Dioxus mobile (Android/iOS) target
- `bypass` - Skip authentication for testing (auth module only)

### JavaScript Interop Pattern

Dioxus app modules interact with JavaScript via `wasm_bindgen` FFI. Each module registers its JS functions under a namespaced global object `window.ratel.<module>`.

#### How It Works

There are 3 layers:

1. **JavaScript source** (`<module>/js/src/` or `<module>/assets/`) - Plain JS functions
2. **JS bundle entry** (`index.js`) - Registers exports onto `window.ratel.<namespace>`
3. **Rust FFI binding** (`src/interop/mod.rs`) - Declares `extern "C"` functions via `wasm_bindgen`

#### Step 1: Write JavaScript Functions

Create JS source files with exported functions:

```js
// app/common/js/src/theme.js
const STORAGE_KEY = "ratel-common-theme";

export function load_theme() {
  return window.localStorage.getItem(STORAGE_KEY);
}

export function save_theme(theme) {
  window.localStorage.setItem(STORAGE_KEY, theme);
}

export function apply_theme(theme) {
  if (theme === "system") {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      theme = "dark";
    } else {
      theme = "light";
    }
  }
  window.document.documentElement.setAttribute("data-theme", theme);
}
```

#### Step 2: Register on `window.ratel` Namespace

Create an entry point that mounts exports onto the global namespace:

```js
// app/common/js/src/index.js
import * as theme from "./theme";

if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.common = {
    theme,
  };
}
```

The namespace convention is `window.ratel.<module_name>` with nested sub-modules as needed. Examples across the codebase:

| Module | JS Namespace |
| :--- | :--- |
| common/theme | `window.ratel.common.theme` |
| app-shell | `window.ratel.app_shell` |
| ratel-auth (firebase) | `window.ratel.auth.firebase` |
| ratel-post | `window.ratel.post` |
| ratel-user-shell | `window.ratel.ratel_user_shell` |
| ratel-team-shell | `window.ratel.ratel_team_shell` |

#### Step 3: Build the JS Bundle

For modules with complex JS (e.g., `app/common/js/`), use webpack to bundle:

```json
// package.json
{
  "scripts": {
    "build": "npx webpack --mode production -d false",
    "build-dev": "npx webpack --mode development"
  }
}
```

The output goes to `dist/main.js`. For simple modules, a hand-written minified `.js` file in `assets/` is sufficient (e.g., `app/shell/assets/ratel-app-shell.js`).

#### Step 4: Declare Rust FFI Bindings

Use `#[wasm_bindgen]` with `js_namespace` matching the global path:

```rust
// app/common/src/components/theme_switcher/interop_theme.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "ratel", "common", "theme"])]
extern "C" {
    pub fn load_theme() -> Option<String>;
    pub fn save_theme(theme: &str);
    pub fn apply_theme(theme: &str);
}
```

The `js_namespace` array maps directly to the nested object path on `window`.

#### Step 5: Load the JS Bundle in Dioxus

Include the JS file as a Dioxus `Asset` and load it via `document::Script`:

```rust
// app/shell/src/main.rs
pub const MAIN_JS: Asset = asset!("/assets/ratel-app-shell.js");

#[component]
fn App() -> Element {
    rsx! {
        document::Script { src: MAIN_JS }
        // ...
    }
}
```

#### Step 6: Call from Rust Components

Use the imported functions directly in Dioxus components/services. Gate calls with `#[cfg(not(feature = "server"))]` since JS is unavailable during SSR:

```rust
// app/common/src/components/theme_switcher/theme_service.rs
impl ThemeService {
    pub fn init() {
        #[cfg(not(feature = "server"))]
        let saved = load_theme().unwrap_or_default().parse().unwrap_or_default();
        #[cfg(feature = "server")]
        let saved = Theme::default();

        let svc = Self {
            theme: use_signal(move || saved),
        };
        #[cfg(not(feature = "server"))]
        apply_theme(saved.to_string().as_str());

        use_context_provider(move || svc);
    }

    pub fn set(&mut self, theme: Theme) {
        self.theme.set(theme);
        let theme = theme.to_string();
        save_theme(&theme);   // calls JS via wasm_bindgen
        apply_theme(&theme);  // calls JS via wasm_bindgen
    }
}
```

#### Key Rules

- **Always guard JS calls** with `#[cfg(not(feature = "server"))]` - JS is only available in the browser
- **Namespace must match exactly** between `index.js` registration and `js_namespace` array in Rust
- **Function names must match** between JS exports and `extern "C"` declarations (use `#[wasm_bindgen(js_name = ...)]` if renaming)
- **JS files go in `assets/`** for Dioxus `asset!()` macro access, or in `js/dist/` if webpack-bundled

### Support Packages (`packages/`)

#### Framework Libraries

- **by-macros** (`packages/by-macros/`) - Procedural macros including DynamoEntity derive (v0.6.\*)
- **by-axum** (`packages/by-axum/`) - Axum framework extensions (v0.2.\*)
- **by-types** (`packages/by-types/`) - Shared type definitions (v0.3.\*)
- **bdk** (`packages/bdk/`) - Biyard Development Kit with Ethereum support
- **btracing** (`packages/btracing/`) - Tracing and observability utilities
- **rest-api** (`packages/rest-api/`) - REST API utilities with test support

#### Data & Translation

- **dto** (`packages/dto/`) - Shared data transfer objects
- **dioxus-translate** (`packages/dioxus-translate/`) - i18n translation framework for Dioxus
- **icons** (`packages/icons/`) - Icon component library

#### Backend Services

- **main-api** (`packages/main-api/`) - Primary REST API built with Axum (v3 endpoints)
- **fetcher** (`packages/fetcher/`) - Data fetching service for legislative information
- **survey-worker** (`packages/survey-worker/`) - AWS Lambda worker for survey operations
- **space-stream-worker** (`packages/space-stream-worker/`) - DynamoDB Streams Lambda worker
- **image-worker** (`packages/image-worker/`) - Image processing Lambda
- **migrator** (`packages/migrator/`) - DynamoDB migration utilities

## Build System

### Rust Workspace

- Rust edition 2024, resolver v2
- Workspace managed by Cargo with members from `app/*` and `packages/*`
- Dioxus 0.7.1 with fullstack + router features
- Common dependencies defined in workspace `Cargo.toml`
- Custom build profiles: `wasm-dev` (opt-level=1), `server-dev`, `android-dev`
- `DYNAMO_TABLE_PREFIX` environment variable required at compile time for DynamoEntity

### Dioxus App

- Built and served with `dx serve` (Dioxus CLI)
- TailwindCSS v4 configured via `app/common/tailwind.css`
- Source scanning: `@source "../**/*/src/**/*.{rs,css}"`
- Theme support: dark (default) and light via `data-theme` attribute
- Dioxus.toml per app module for configuration

## Development Commands

### Docker (Recommended for Local Development)

```bash
# Start all services with Docker Compose
docker compose --profile development up -d

# Start infrastructure only (LocalStack, DynamoDB)
make infra

# View logs
docker compose logs -f ratel-main-api-1

# Stop all services
make stop
```

### Root Makefile Commands

```bash
make run          # Start all services with Docker Compose (profile: development)
make stop         # Stop all services
make infra        # Start infrastructure only (LocalStack, DynamoDB Admin)
make build SERVICE=main-api  # Build specific service
make deploy       # Deploy to AWS via CDK
make serve SERVICE=main-api  # Run specific service locally
```

### Dioxus App Development

```bash
# Run Dioxus app shell (port 8000)
cd app/shell && make run

# Equivalent to:
DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```

### Service-Specific Development

```bash
# Main API
cd packages/main-api && make run      # Dev with cargo-watch
cd packages/main-api && make build    # Build release binary
cd packages/main-api && make test     # Run Rust tests

# Fetcher
cd packages/fetcher && make run
cd packages/fetcher && make build
```

## Key Technologies

- **Frontend/Fullstack**: Dioxus 0.7.1 (fullstack SSR + WASM hydration), TailwindCSS v4
- **Backend**: Rust 2024, Axum 0.8.1, Tokio
- **Database**: DynamoDB (via aws-sdk-dynamodb + serde_dynamo + DynamoEntity derive macro)
- **AWS Services**: Lambda, S3, SES (email), SNS (SMS), Bedrock (AI), DynamoDB Streams
- **Blockchain**: Ethers 2.0.14 (Rust) / Ethers.js 6.15 (TS), Kaia network
- **Authentication**: Firebase, tower-sessions, DID/Verifiable Credentials
- **Payments**: Portone gateway, Binance API
- **AI**: AWS Bedrock agents, MCP (Model Context Protocol)
- **Messaging**: Telegram Bot API
- **API Spec**: Aide (OpenAPI/Swagger) for main-api v3 endpoints
- **Infrastructure**: Docker, LocalStack, AWS CDK

## Docker Services

The docker-compose.yaml provides:

- **main-api** - REST API (port 3000)
- **space-shell** - Dioxus app shell (port 8000)
- **fetcher** - Legislative data fetching (port 3001)
- **survey-worker** - Survey worker service (port 3002)
- **web** - Legacy Vite/React frontend (port 8080)
- **storybook** - Component documentation (port 6006)
- **localstack** - AWS services emulation (port 4566)
- **localstack-init** - DynamoDB table initialization
- **dynamodb-admin** - DynamoDB web UI (port 8081)

Access points:

- Dioxus App: http://localhost:8000
- Main API: http://localhost:3000
- DynamoDB Admin: http://localhost:8081
- LocalStack: http://localhost:4566

## Environment Configuration

### Backend Environment Variables

- `ENV` - Environment (dev, staging, prod)
- `PORT` - Service port
- `RUST_LOG` - Logging level
- `DYNAMO_TABLE_PREFIX` - Table prefix (ratel-local for local dev, ratel-dev for dev) **Required at compile time**
- `DYNAMO_ENDPOINT` - DynamoDB endpoint (http://localstack:4566 for local, `none` for AWS)
- `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` - AWS credentials
- `UPSTREAM_URL` - Upstream URL for proxy (Dioxus app)
- `COMMIT` - Git commit hash
- `TELEGRAM_TOKEN` - Telegram bot authentication token
- `FIREBASE_PROJECT_ID` - Firebase project identifier
- `PORTONE_*` - Payment gateway credentials
- `BEDROCK_AGENT_*` - AWS Bedrock AI agent configuration
- `BIYARD_*` - External Biyard API integration

## DynamoDB Configuration

### Local Development Setup

- **Endpoint:** `http://localstack:4566`
- **Table Prefix:** `ratel-local` (local development)
- **Main Table:** `ratel-local-main` (unified single-table design with multiple GSIs)
- **Global Secondary Indexes:** gsi1 through gsi6+ for email, username, phone, status, visibility queries
- **Admin UI:** Available at http://localhost:8081

### Single-Table Design

The project uses DynamoDB single-table design with composite keys:

- **Partition Key (pk):** Uses `Partition` enum variants (e.g., `USER#<id>`, `FEED#<id>`, `SPACE#<id>`)
- **Sort Key (sk):** Uses `EntityType` enum variants (e.g., `User`, `Post`, `SpaceCommon`)
- **GSIs:** Multiple global secondary indexes for alternative access patterns

### Table Initialization

- Automatic schema creation via `localstack-init` container
- Admin user seeded on startup
- Migration support via `migrator` package

## Main API

Main API package is the primary backend REST API for Ratel written in Rust.

- Location: `packages/main-api`
- Language: Rust (Axum 0.8.1)

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

## by_macro

`by_macro` package provides macros to simplify the code.

### DynamoEntity derive

`DynamoEntity` generates CRUD utility functions for interaction with DynamoDB.

#### Structure attribute

- `DYNAMO_TABLE_PREFIX` is required for composing full table name.
  - For example, if `DYNAMO_TABLE_PREFIX` is set to `ratel-local` when building it, the table name of the entity will be set to `ratel-local-main` as default.
  - If `table` attribute is set to `users`, the full table name will be `ratel-local-users`.

| Attribute  | Description                                  | Default             |
| :--------- | -------------------------------------------- | :------------------ |
| table      | table name except for prefix                 | main                |
| result_ty  | Result type                                  | std::result::Result |
| error_ctor | Error type                                   | create::Error2      |
| pk_name    | Partition key name                           | pk                  |
| sk_name    | (optional) Sort key name (none for removing) | sk                  |

#### Field attribute

| Attribute | Description                          |
| :-------- | ------------------------------------ |
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
```

## CI/CD Workflows

### GitHub Actions

- **dev-workflow.yml** - Automated development branch builds and tests
- **pr-workflow.yml** - Pull request validation and checks
- **prod-workflow.yml** - Production deployment automation

### Deployment Infrastructure

- **AWS CDK** - Infrastructure as Code (TypeScript)
- **ECR** - Container registry for Docker images
- **Lambda** - Serverless function deployments (main-api, fetcher, survey-worker, space-stream-worker)
- **CloudFormation** - Stack management and orchestration
