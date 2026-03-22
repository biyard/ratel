# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ratel is a decentralized legislative platform built with Rust (Dioxus fullstack) and DynamoDB, designed to bridge the gap between crypto users and policymakers. The primary web application is built using Dioxus 0.7 with fullstack rendering (SSR + client-side hydration), with DynamoDB as the database.

## Architecture

This is a monorepo with a Cargo workspace structure:

- **app/ratel/** - Dioxus fullstack application (single package with feature-gated modules: auth, posts, spaces, users, teams, membership, admin)
- **app/interops/** - Reusable web components library (`dioxus-components`)
- **packages/** - Rust workspace packages (APIs, workers, shared libraries, macros)
- **kaia/** - Blockchain contracts (Hardhat)
- **contracts/** - Primary blockchain contracts (Hardhat)

### Dioxus App (`app/ratel/`)

The main web application is a single Dioxus fullstack package (`app-shell`) with feature-gated modules. Previously split across many packages, it is now consolidated into `app/ratel/`.

- **Package:** `app-shell`
- **Entry:** `src/main.rs` with `Dioxus.toml` configuration
- **Routing:** `src/route.rs` defines top-level routes using `ChildRouter`
- **Layout:** `src/layout.rs` wraps all pages in `AppLayout`
- **Port:** 8000 (via `dx serve`)

**Routes:**
- `/` - Index/home page
- `/auth/:..rest` - Authentication (ChildRouter to auth feature)
- `/posts/:..rest` - Post feed (ChildRouter to posts feature)
- `/my-follower/:..rest` - Follower feed
- `/admin/:..rest` - Admin panel
- `/:username/:..rest` - User profile (feature: `users`)
- `/teams/:teamname/:..rest` - Team pages (feature: `teams`)
- `/spaces/:space_id/...` - Space pages (feature: `spaces`)

#### Project Structure

```
app/ratel/src/
  main.rs              - Entry point
  route.rs             - Top-level Dioxus Router enum
  layout.rs            - AppLayout wrapper
  config.rs            - App configuration
  app.rs               - App component
  assets.rs            - Static asset declarations
  common/              - Shared foundation (see below)
  components/          - App-level shell components (app_menu, mobile_menu, profile_dropdown, etc.)
  contexts/            - App-level context providers
  features/            - Feature modules (see below)
  interop/             - JS FFI bindings (wasm_bindgen)
  views/               - Top-level page views (Index, etc.)
  tests/               - Test utilities and macros
```

#### Common (`src/common/`)

Foundational shared code used by all feature modules.

- **Types:** `Partition` enum (DynamoDB partition keys), `EntityType` enum (sort keys), custom types
- **Models:** `User`, `SpaceCommon`, `SpaceParticipant`, etc. (DynamoEntity derive)
- **Components:** Badge, Button, Card, Input, Switch, Popup, SeoMeta, ThemeSwitcher, LoadingIndicator, Textarea, Sidemenu, SpaceCard
- **Config:** `config/server/` (DynamoDB, S3, SES, SNS), `config/web/` (Firebase, WalletConnect)
- **Utils:** `utils/aws/` (DynamoDB, S3, SES, SNS helpers), password, sha256, time
- **Hooks:** `use_scroll_lock`, etc.
- **Services:** Biyard API integration
- **Tailwind:** `app/ratel/tailwind.css` is the TailwindCSS v4 entrypoint

##### Primitive Component Library (`src/common/components/`)

**IMPORTANT:** When building any UI in `app/ratel/src/`, always check and prefer primitive components from `src/common/components/` before creating custom elements. These are the project's design system primitives and MUST be used for visual consistency.

Available primitive components:
`Accordion`, `AlertDialog`, `AspectRatio`, `Avatar`, `Badge`, `Button`, `Calendar`, `Card`, `Checkbox`, `Col`, `Collapsible`, `ContextMenu`, `DatePicker`, `DateAndTimePicker`, `TimePicker`, `TimezonePicker`, `Dialog`, `DragAndDropList`, `DropdownMenu`, `FileUploader`, `Form`, `HoverCard`, `Input`, `Label`, `Layover`, `LoadingIndicator`, `Menubar`, `Navbar`, `Pagination`, `Popover`, `Popup`, `Progress`, `RadioGroup`, `Row`, `ScrollArea`, `Select`, `SeoMeta`, `Separator`, `Sheet`, `Sidebar`, `Sidemenu`, `Skeleton`, `Slider`, `SpaceCard`, `SuspenseBoundary`, `Switch`, `Tabs`, `TeamCreationForm`, `TeamSelector`, `Textarea`, `ThemeSwitcher`, `Toggle`, `ToggleGroup`, `Toolbar`, `Tooltip`

Rules:
- **Always use these primitives** instead of raw HTML elements for interactive UI (e.g., use `Button` not `button`, `Input` not `input`, `Select` not `select`, `Card` for card layouts)
- **Check component props** before implementation — many support variants, sizes, and styling props (e.g., `Card` has `direction`, `main_axis_align`, `cross_axis_align`)
- **Compose from primitives** — build complex UI by composing these components rather than writing raw divs with Tailwind
- **Only create new primitives** in `src/common/components/` if no existing component fits the need

#### Features (`src/features/`)

Each feature module is gated by a Cargo feature flag and follows a consistent structure:

| Feature | Path | Feature Flag | Description |
|---------|------|-------------|-------------|
| Auth | `features/auth/` | (always enabled) | Login, signup, verification, OAuth, password reset |
| Posts | `features/posts/` | (always enabled) | Post CRUD, comments, likes, shares, visibility |
| Spaces | `features/spaces/` | `spaces` | Governance spaces, panels, voting, proposals |
| Users | `features/users/` | `users` | User profile, settings, credentials, rewards |
| Teams | `features/teams/` | `teams` | Team management, members, groups, DAOs |
| Admin | `features/admin/` | (always enabled) | Admin panel |
| Membership | `features/membership/` | `membership` | Membership tiers and access |
| My Follower | `features/my_follower/` | (always enabled) | Follower feed |

**Auth Context & Membership Integration:**

The `UserContext` (`features/auth/context/user_context.rs`) stores the logged-in user and, when the `membership` feature is enabled, their `UserMembershipResponse`. All membership-related fields use `#[cfg(feature = "membership")]` gating.

- **`use_user_membership()` hook** (`features/auth/hooks/use_user_membership.rs`) — Returns `Option<UserMembershipResponse>`. Lazy-loads from server via `use_resource` when user is logged in but membership is `None` (e.g., after login/signup). Subsequent reads come from context.
- **`UserMembershipResponse`** (`features/membership/models/membership.rs`) — Contains `tier: MembershipPartition`, `remaining_credits`, `max_credits_per_space`, etc. Has `is_paid()` helper that checks `!tier.0.contains("Free")`.
- **`MembershipPartition`** — Newtype `pub MembershipPartition(pub String)` containing e.g. `"MEMBERSHIP#Free"`. Convert to `Partition` via `let pk: Partition = membership_pk.clone().into();` (explicit type annotation needed to avoid inference ambiguity).
- **Membership tiers:** `Free`, `Pro`, `Max`, `Vip`, `Enterprise(String)` — Only paid tiers (non-Free) can access premium features like reward boost.
- **Membership page route:** `/{username}/memberships` (`UserRoute::UserMemberships`)

**Spaces sub-pages** (`features/spaces/pages/`):

```
spaces/
  space_common/     - Shared space models, hooks, components (SpaceNav, SpaceTop)
  pages/
    overview/       - Space overview page
    dashboard/      - Space dashboard page
    actions/        - Space actions (polls, quizzes)
    apps/           - Space app settings
      apps/
        general/    - General settings (visibility, anonymous, invite, admin)
        file/       - File management
        incentive_pool/ - Incentive pool settings
        rewards/    - Rewards settings
    report/         - Space report page
```

**Users sub-pages** (`features/users/pages/`):

```
users/
  pages/
    post/           - User post feed
    reward/         - User rewards
    setting/        - User settings
    membership/     - User membership
    draft/          - User drafts
    credentials/    - DID / Verifiable Credentials
    space/          - User spaces
```

**Teams sub-pages** (`features/teams/pages/`):

```
teams/
  pages/
    home/           - Team home
    draft/          - Team drafts
    group/          - Team groups
    member/         - Team members
    dao/            - Team DAO governance
    reward/         - Team rewards
    setting/        - Team settings
```

#### Feature Module Structure

Each feature module follows a consistent internal structure:

```
features/<module>/
  mod.rs            - Module exports, re-exports
  route.rs          - Feature-level Dioxus Router routes
  layout.rs         - Feature layout wrapper
  config.rs         - Feature configuration
  provider.rs       - State provider
  constants.rs      - Constants
  controllers/      - Server functions (#[get], #[post], #[patch], etc.)
  models/           - DynamoDB entities (DynamoEntity derive, feature: server)
  components/       - Reusable UI components
  views/            - Page-level view components
  hooks/            - Dioxus hooks for state management
  interop/          - JavaScript FFI bindings (wasm_bindgen)
  server/           - Server-only functionality (feature: server)
  web/              - Web-only functionality (feature: web)
  utils/            - Helper functions
  dto/              - Data transfer objects
  types/            - Custom type definitions
  i18n.rs           - Translation strings (translate! macro)
  services/         - Domain service logic
```

Not every module has all directories — only what is needed.

#### Interops (`app/interops/web-components/`)

- **Package:** `dioxus-components`
- Reusable Dioxus components library for web integration

### Feature Flags

The `app-shell` package uses these feature flags:

| Flag | Description |
|------|-------------|
| `full` | Enables `membership`, `users`, `teams`, `spaces_full` (default) |
| `web` | Dioxus web renderer, browser UI components |
| `server` | Dioxus server renderer, DynamoDB, AWS SDK, backend handlers |
| `desktop` | Dioxus desktop (Tauri) target |
| `mobile` | Dioxus mobile (Android/iOS) target |
| `lambda` | AWS Lambda deployment (includes `server`) |
| `spaces` | Space feature modules |
| `spaces_full` | Full spaces with all sub-features |
| `users` | User profile feature modules |
| `teams` | Team management feature modules |
| `membership` | Membership feature module |
| `bypass` | Skip authentication for testing |

### JavaScript Interop Pattern

The app interacts with JavaScript via `wasm_bindgen` FFI. Each module registers its JS functions under a namespaced global object `window.ratel.<module>`.

#### How It Works

There are 3 layers:

1. **JavaScript source** (`app/ratel/js/src/` or `app/ratel/assets/`) - Plain JS functions
2. **JS bundle entry** (`index.js`) - Registers exports onto `window.ratel.<namespace>`
3. **Rust FFI binding** (`src/interop/mod.rs` or `src/features/<module>/interop/`) - Declares `extern "C"` functions via `wasm_bindgen`

#### Step 1: Write JavaScript Functions

Create JS source files with exported functions:

```js
// app/ratel/js/src/theme.js
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
// app/ratel/js/src/index.js
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
| features/auth (firebase) | `window.ratel.auth.firebase` |
| features/posts | `window.ratel.post` |
| features/users | `window.ratel.ratel_user_shell` |
| features/teams | `window.ratel.ratel_team_shell` |

#### Step 3: Build the JS Bundle

For modules with complex JS (e.g., `app/ratel/js/`), use webpack to bundle:

```json
// package.json
{
  "scripts": {
    "build": "npx webpack --mode production -d false",
    "build-dev": "npx webpack --mode development"
  }
}
```

The output goes to `dist/main.js`. For simple modules, a hand-written minified `.js` file in `assets/` is sufficient (e.g., `app/ratel/assets/ratel-app-shell.js`).

#### Step 4: Declare Rust FFI Bindings

Use `#[wasm_bindgen]` with `js_namespace` matching the global path:

```rust
// app/ratel/src/common/components/theme_switcher/interop_theme.rs
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
// app/ratel/src/main.rs
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
// app/ratel/src/common/components/theme_switcher/theme_service.rs
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

### SeoMeta Component

The `SeoMeta` component (`app/ratel/src/common/components/seo_meta/`) renders SEO meta tags into the document head. Every page-level view should include it for proper Google SEO, Open Graph, and Twitter Card support.

#### Props

| Prop | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `title` | `String` | Yes | — | Page title (`<title>` + og/twitter title) |
| `description` | `String` | No | `""` | Meta description for search result snippets |
| `image` | `String` | No | `""` | Preview image URL for social sharing |
| `url` | `String` | No | `""` | Page URL for Open Graph |
| `og_type` | `String` | No | `"website"` | Open Graph type (`website`, `article`, etc.) |
| `keywords` | `Vec<String>` | No | `[]` | SEO keywords (joined with `", "`) |
| `canonical` | `String` | No | `""` | Canonical URL (`<link rel="canonical">`) |
| `robots` | `Robots` | No | `IndexFollow` | Robots directive enum |

#### Robots Enum

| Variant | Output |
|---------|--------|
| `Robots::IndexFollow` (default) | `index, follow` |
| `Robots::NoindexFollow` | `noindex, follow` |
| `Robots::IndexNofollow` | `index, nofollow` |
| `Robots::NoindexNofollow` | `noindex, nofollow` |

#### Usage

```rust
use common::components::SeoMeta;
use common::components::Robots;

// Minimal - only title required
rsx! {
    SeoMeta { title: "Home - Ratel" }
}

// Full usage
rsx! {
    SeoMeta {
        title: "Space Overview - Ratel",
        description: "Explore governance spaces on Ratel.",
        image: "https://ratel.foundation/og-image.png",
        url: "https://ratel.foundation/spaces",
        og_type: "article",
        keywords: vec!["governance".into(), "crypto".into(), "policy".into()],
        canonical: "https://ratel.foundation/spaces",
        robots: Robots::IndexFollow,
    }
}

// Draft/private page - prevent indexing
rsx! {
    SeoMeta {
        title: "Draft Post",
        robots: Robots::NoindexNofollow,
    }
}
```

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

## Enum Display in UI

When displaying an Enum type value in the UI, always use `.translate()` (e.g., `{value.translate(&lang())}`) instead of `Display` / `.to_string()`. If the Enum type does not yet derive `Translate`, implement it by adding:

```rust
#[derive(Translate)]
pub enum MyEnum {
    #[translate(en = "English Label", ko = "한국어 라벨")]
    Variant,
}
```

Use `let lang = use_language();` in the component, then `{value.translate(&lang())}` in RSX.

## Dioxus UI Development Guidelines

### Accessibility

- **Always add `alt` attributes** to `img` elements (use descriptive text or `alt: ""` for decorative images)
- **Always add `aria-label`** to icon-only buttons so screen readers can announce their purpose
- **Use semantic elements** for clickable navigation: use `Link { to: "..." }` (renders `<a>`) instead of `div { onclick: ... }` with `use_navigator().push()`. This provides native keyboard accessibility (tab focus, Enter activation) and correct link semantics without manual `role`, `tabindex`, or keyboard handlers

### Import Conventions

- **Use wildcard re-exports**: Start with `use crate::features::<module>::*;` which brings in common items through the re-export chain (e.g., `crate::common::*` -> `dioxus_translate::*`, etc.)
- **Only add explicit imports** for items NOT available through the wildcard chain (e.g., `use_infinite_query` from `crate::common::hooks`, `time_ago` from `crate::common::utils::time`, cross-feature handlers)
- **Do NOT duplicate imports** that are already available via wildcards — check sibling files in the same directory to see which imports are standard

### Scroll Event Handlers

- **Never spawn unbounded async tasks** from `onscroll` — inertial scrolling fires many events rapidly
- **Implement trailing-edge throttle**: use a `scroll_check_pending` signal guard to skip events while a check is in-flight, plus a `scroll_dirty` flag so one final check runs after the current one completes. This ensures the final scroll position is always reflected

### Paginated Lists with `use_infinite_query`

- **Prefer `use_infinite_query`** over `use_server_future` for any list that may exceed a single page
- **Always render `{v.more_element()}`** at the end of the list container — this provides the IntersectionObserver sentinel that triggers loading additional pages
- **Make `v` mutable** (`let mut v = use_infinite_query(...)`) so `more_element()` can be called

### Dioxus Reactivity in `use_effect`

- `use_effect` only re-runs when it reads reactive signals **inside the closure body**
- Capturing a plain `usize` or other non-reactive value outside the closure will NOT trigger re-runs
- To react to item count changes: call `v.items()` inside the effect (which reads from the underlying `Signal`), not `let count = items.len()` captured from outside

### Event Handler Syntax

- In Dioxus RSX, event handlers do NOT need outer brace wrapping. Use `onscroll: move |_| { ... },` directly, not `onscroll: { move |_| { ... } },`

### Server-Side Pagination & Filtering

- **Filter server-side** when possible — client-side filtering after paginated fetch can cause edge cases (e.g., first page contains only filtered-out items, hiding the section even though later pages have valid items)
- **Add query parameters** (e.g., `active_only: Option<bool>`) instead of changing existing handler semantics, to avoid breaking other callers
- **Hard-cap server-side loops**: when scanning multiple DynamoDB pages to collect filtered results, add a `max_pages` cap (e.g., 5) to bound reads per request
- **Preserve bookmark on cap**: when hitting the hard cap, set `bookmark = next_bookmark` (not `None`) so clients can continue scanning from where they left off in subsequent requests
- **Do NOT use `.take(remaining)` in filtered collection**: when filtering results across DynamoDB pages (e.g., `active_only`), collect all matching items from each page without `.take()` — otherwise valid items from a page may be silently dropped and can never be re-fetched since the bookmark advances past them. Use post-loop `truncate()` only if a hard limit is needed

### Performance Patterns

- **Use `HashMap` for O(1) lookups** instead of linear scans when mapping between collections (e.g., post titles by key)
- **Avoid redundant `.to_string()` calls** in hot paths — store the result in a local variable when the same conversion is used multiple times (e.g., HashMap key lookup)

### TailwindCSS Syntax

- **Always use bracket syntax for arbitrary values**: write `z-[101]`, not `z-101`. Non-standard values without brackets are silently ignored by TailwindCSS and produce no CSS output
- This applies to all arbitrary values: `z-[101]`, `w-[350px]`, `gap-[13px]`, etc.
- Standard Tailwind scale values don't need brackets: `z-10`, `z-50`, `w-full`, `gap-4`

### TailwindCSS Color Classes

- **Never use Tailwind's built-in color palette classes** (`text-neutral-400`, `text-gray-500`, `bg-slate-800`, `text-zinc-300`, etc.) — these bypass the project's theme system and break dark/light mode consistency
- **Always use semantic token classes** instead: `text-foreground-muted`, `text-text-primary`, `bg-card-bg`, `bg-background`, `border-border`, etc. (see Design Tokens section in `.claude/rules/figma-design-system.md`)
- **Do not combine `light:` variant with palette colors** (e.g., `light:text-neutral-600`) — use a single semantic token class that handles both themes automatically
- Tailwind spacing, sizing, and layout utilities (`gap-4`, `p-5`, `rounded-lg`, `w-full`) are fine to use directly

## Error Handling Convention

### Avoid `Error::BadRequest(String)` -- Use Typed Error Variants

**Do NOT use `Error::BadRequest(String)` for domain-specific errors.** Instead, define a specific error enum with `Translate` derive for user-friendly error handling in the frontend.

**Pattern:** Follow the `SpaceRewardError` / `SpaceActionQuizError` pattern:

1. **Define a domain-specific error enum** in the feature's `types/error.rs`:

```rust
use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum MyFeatureError {
    #[error("Descriptive internal message")]
    #[translate(en = "User-facing English", ko = "사용자용 한국어")]
    SpecificErrorVariant,
}
```

2. **Implement server traits** (`status_code()`, `IntoResponse`, `AsStatusCode`) following the `SpaceRewardError` pattern.

3. **Register in `common::Error`** by adding a `#[from]` variant with `#[translate(from)]`:

```rust
#[error("{0}")]
#[translate(from)]
MyFeature(#[from] MyFeatureError),
```

The `#[translate(from)]` attribute tells the `Translate` derive macro to delegate translation to the inner type's `translate()` method instead of using a static string. This ensures `toast.error()` shows the specific inner-error translation (e.g., "No remaining attempts") instead of a generic outer message.

4. **Add to status code match arms** in both `IntoResponse` and `AsStatusCode` impls.

5. **Use in controllers** via `.into()`:

```rust
return Err(MyFeatureError::SpecificErrorVariant.into());
```

**Why:** Typed errors enable per-variant i18n translation in the frontend via `toast.error()`, provide better error categorization, and eliminate ambiguous string-based error messages. The `#[translate(from)]` attribute on wrapper variants delegates to the inner error's `Translate` impl so that each variant's translation is surfaced correctly.

**When refactoring existing code:** If you encounter `Error::BadRequest(String)` in code you are modifying, refactor it to use a typed error variant in the same change.

## Server-Side Validation for User-Configurable Numeric Values

**Always enforce reasonable server-side upper bounds for user-configurable numeric values** (e.g., `retry_count`, attempt limits, page sizes). Do not rely on frontend-only validation.

**Key rules:**

1. **Define a shared constant** for the upper bound in the feature module (e.g., `pub const MAX_TOTAL_ATTEMPTS: i64 = 100;`) so all controllers use the same limit.
2. **Validate at the write path** (e.g., `update_quiz`) — reject values exceeding the bound with a typed error variant.
3. **Clamp at the read path** (e.g., `get_quiz`, `respond_quiz`) — use `.min(MAX_CONSTANT)` instead of `unwrap_or(i32::MAX)` or `unwrap_or(i64::MAX)` as fallbacks. This provides defense-in-depth for legacy data that may have been written before the validation was added.
4. **Never use `i32::MAX` or `i64::MAX` as default limits** for DynamoDB queries or similar operations — these create unbounded read costs and abuse vectors.

**Why:** User-controlled values can be set to extreme values (e.g., `i64::MAX`), causing integer overflow, unbounded database queries, and excessive read costs. Server-side bounds prevent this regardless of client behavior.

## Agent Workflow Rules

### PR Comment Resolver

When the `pr-comment-resolver` agent resolves PR review comments, it must save the review feedback as project-scoped memories so that Claude Code and the `github-issue-resolver` agent can reference and apply those learnings in future work. This prevents the same review feedback from being repeated across PRs.

## Playwright Test Guidelines

### Navigation & Hydration

- **Use `page.goto(url, { waitUntil: "load" })` directly** — do NOT follow with a separate `waitForLoadState("load")` call, as it is redundant
- **Do NOT wait on specific WASM response status codes** (e.g., `response.status() === 200`) — responses may be cached as 304 or served differently across environments
- **Do NOT use `Promise.all` with `page.goto()` and `page.waitForResponse()`** — this pattern is flaky because responses may arrive as 304 (cached) and the status check fails silently. Use `page.goto(url, { waitUntil: "load" })` instead
- **Use deterministic hydration detection** instead of fixed `waitForTimeout()` sleeps:
  ```js
  await page.waitForFunction(
    () => window.dioxus && typeof window.dioxus.send === "function",
    { timeout: 10000 }
  );
  ```
  This checks that the Dioxus WASM app has fully hydrated before interacting with the page

**Anti-pattern (do NOT use):**
```js
// ❌ Flaky: status may be 304, waitForLoadState is redundant after goto
await Promise.all([
  page.waitForResponse((r) => r.url().includes(".wasm") && r.status() === 200),
  page.goto(url),
]);
await page.waitForLoadState("load");
await page.waitForTimeout(500);

// ✅ Correct: deterministic navigation + hydration check
await page.goto(url, { waitUntil: "load" });
await page.waitForFunction(
  () => window.dioxus && typeof window.dioxus.send === "function",
  { timeout: 10000 }
);
```

### Configuration Conventions

- **Retries must be CI-only**: Always use `retries: process.env.CI ? 2 : 0` — never hardcode retries to a non-zero value. Hardcoded retries on local dev runs mask flaky tests and slow feedback loops
- **Keep implementation and docs aligned**: When changing test infrastructure behavior (e.g., `goto()` wait strategy, retry policy), update `docs/playwright-testing.md` in the same PR to avoid implementation/documentation drift

### Placeholder/Empty State Styling

- When a UI element has both a normal and a placeholder state (e.g., "untitled" vs actual title), always apply visually distinct styling to the placeholder case (e.g., `text-foreground-muted italic`) rather than using the same primary text style for both

### Semantic Tokens for Status Colors

- Status-indicating colors (pass/fail, success/error, active/inactive) **must use semantic tokens**, not raw Tailwind palette colors (e.g., `bg-green-500`, `text-red-400`)
- Define them in `tailwind.css` with the CSS space toggle pattern (`var(--dark, ...) var(--light, ...)`) and use them in component classes
- Group them by feature namespace (e.g., `--color-sp-act-quiz-pass-bg`, `--color-sp-act-quiz-fail-text`)
- **When adding new status badges or indicators**, always define CSS variables first in `tailwind.css`, then use them as Tailwind classes — never use raw palette classes like `bg-green-500/10` or `text-red-400` even with opacity modifiers

## Feature Flag Security

- **Never bundle security-bypass features into general-purpose feature groups**: The `bypass` feature (which disables auth verification, e.g., accepting `000000`) must remain a separate, explicit opt-in flag. Do not include it in composite features like `local-dev` — developers using `--features local-dev` for unrelated dev behavior could accidentally ship builds with auth bypass enabled
- **Security-sensitive features** (`bypass`, test-only auth overrides, mock credentials) should always require explicit `--features <flag>` activation, never be transitively enabled through other feature groups
- **When adding new dev-convenience features**, review whether they carry security implications before adding them to composite feature groups
