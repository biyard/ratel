# Dioxus Convention Guide

This document defines the coding conventions for the Ratel Dioxus fullstack application. All code reviews should verify compliance with these rules.

---

## 1. Module Structure

Every app module under `app/` must follow this directory layout:

```
src/
  lib.rs          # Public API exports, re-exports, type aliases
  main.rs         # Entry point (only in runnable crates)
  route.rs        # Dioxus Router enum
  layout.rs       # Layout wrapper component
  config.rs       # Config struct + singleton accessor
  provider.rs     # Script/style loading component
  constants.rs    # Constants
  controllers/    # Server functions (#[get], #[post], etc.)
  models/         # DynamoDB entities (DynamoEntity derive)
  components/     # Reusable UI components
  views/          # Page-level view components
  hooks/          # Custom Dioxus hooks (use_*)
  interop/        # JS FFI bindings (wasm_bindgen)
  types/          # Custom type definitions
  dto/            # Data transfer objects
  utils/          # Helper functions
  server/         # Server-only module (gated: feature = "server")
  web/            # Web-only module (gated: feature = "web" or not(feature = "server"))
```

**Rules:**
- Modules containing server-only code must be gated with `#[cfg(feature = "server")]`.
- Modules containing web-only code must be gated with `#[cfg(not(feature = "server"))]` or `#[cfg(feature = "web")]`.
- Not every directory is required — only include what the module needs.

---

### 1.1 Views Structure
View is a page component.
- Each view should be implemented in `{view_name}/mod.rs`
- Component dedicated to the view should be placed in `{view_name}` directory.

```
views/
  mod.rs
  {view_name}/
    mod.rs              # View page component
    i18n.rs             # i18n for this view. If small, it can be merged into mod.rs.
    {small_component_name}.rs # Each component used by this view. It can include its own i18n too.
    {large_component_name}/
      mod.rs
      i18n.rs
```

### 1.2 Components Structure
Components is shared components by two or more views.

```
components/
  mod.rs
  {small_component_name}.rs # Each component used by this view. It can include its own i18n too.
  {large_component_name}/
    mod.rs
    i18n.rs
```

## 2. Imports (`lib.rs` and `use crate::*`)

### lib.rs Pattern

Each module's `lib.rs` re-exports its public surface. Internal files use `use crate::*;` to pull in these exports.

```rust
#![allow(unused_imports, dead_code)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod models;
mod provider;
mod route;
pub mod types;
pub mod views;

pub use provider::Provider;
pub use route::Route;

// Re-export common types needed by models (available via `use crate::*;`)
pub use common::macros::dynamo_entity::DynamoEntity;
pub use common::models::*;
pub use common::types::*;
pub use common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};
pub use serde::{Deserialize, Serialize};

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
```

### File-Level Imports

Every source file begins with:

```rust
use crate::*;
```

Controllers additionally import models:

```rust
use crate::*;
use crate::models::*;
```

**Rules:**
- Always use `use crate::*;` — do not scatter individual imports from sibling modules.
- Define `type Result<T> = common::Result<T>;` in `lib.rs` for module-local error handling.

---

## 3. Components

### Definition

```rust
#[component]
pub fn FeedCard(
    post: PostResponse,
    on_like: Option<EventHandler<bool>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2.5",
            ..attributes,
            {children}
        }
    }
}
```

### Props Rules

| Pattern | Usage |
|---------|-------|
| `prop: T` | Required prop |
| `prop: Option<T>` | Optional prop |
| `#[props(default)]` | Prop with default value |
| `#[props(extends = GlobalAttributes)]` | HTML attribute passthrough |
| `children: Element` | Slot content |
| `EventHandler<MouseEvent>` | Event handler (wrap in `Option` if optional) |

### Naming

| Type | Convention | Example |
|------|-----------|---------|
| Components | PascalCase | `FeedCard`, `SpaceNav` |
| Pages/Views | PascalCase, optionally suffixed with `Page` | `AdminPage`, `Index` |
| Modals | Suffixed with `Modal` | `LoginModal`, `SignUpModal` |
| Services | Suffixed with `Service` | `PopupService`, `ThemeService` |

### rsx! Patterns

```rust
rsx! {
    // Tailwind classes
    div { class: "flex flex-col gap-2.5 bg-bg text-primary",

        // Conditional rendering
        if user.is_some() {
            div { "Logged in" }
        }

        // Loops with keys
        for post in posts {
            FeedCard { key: "post-{post.pk}", post: post.clone() }
        }

        // Text interpolation
        span { "{username}" }

        // Raw HTML
        div { dangerous_inner_html: "{html_string}" }

        // Children slot
        {children}

        // Spread HTML attributes
        ..attributes,
    }
}
```

**Rules:**
- Always use Tailwind CSS utility classes for styling.
- Always provide `key` when rendering lists.
- Never use inline styles — use Tailwind classes or `data-*` attributes.

---

## 4. Routes

### Route Enum

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Index { },

        #[route("/auth/:..rest")]
        Auth { rest: Vec<String> },

        #[nest("/:space_id")]
            #[layout(SpaceProvider)]
                #[layout(SpaceLayout)]
                    #[route("/dashboard/:..rest")]
                    Dashboard { space_id: SpacePartition, rest: Vec<String> },
                #[end_layout]
            #[end_layout]
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
```

**Rules:**
- Always apply `#[rustfmt::skip]` to preserve route hierarchy indentation.
- Always include a `PageNotFound` catch-all route.
- Use `#[layout(...)]` / `#[end_layout]` for wrapping routes in layout components.
- Use `#[nest(...)]` / `#[end_nest]` for path prefix grouping.
- Use `:..rest` (Vec<String>) for wildcard sub-route delegation.
- Use typed path parameters (`SpacePartition`, `TeamPartition`) instead of raw `String`.

### ChildRouter Delegation

When a route delegates to another crate's router, use the `define_app_wrapper!` macro:

```rust
macro_rules! define_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(rest: Vec<String>) -> Element {
            let router = use_context::<dioxus::router::RouterContext>();
            let route: $route_module = router.current();
            rsx! {
                ChildRouter::<$route_module> {
                    route,
                    format_route_as_root_route: |r: $route_module| r.to_string(),
                    parse_route_from_root_route: |url: &str| {
                        <$route_module as std::str::FromStr>::from_str(url).ok()
                    },
                }
            }
        }
    };
}

define_app_wrapper!(Auth, AuthRoute);
define_app_wrapper!(Space, SpaceRoute);
```

**Rules:**
- Never nest `Router` components directly — always use `ChildRouter`.
- Use the macro for standard delegation; write manual wrappers only when authorization guards are needed.

---

## 5. Layouts

Layouts wrap child routes via `Outlet::<Route> {}`.

```rust
#[component]
pub fn SpaceLayout(space_id: SpacePartition) -> Element {
    let space_loader = use_space_query(&space_id)?;
    let space = space_loader.read().clone();

    rsx! {
        div { class: "grid grid-cols-7",
            SpaceNav { space: space.clone() }
            div { class: "col-span-6",
                Outlet::<Route> {}
            }
        }
    }
}
```

**Rules:**
- Layouts receive route parameters as props.
- Use `Outlet::<Route> {}` exactly once to render child content.
- Separate **Provider** (context injection) from **Layout** (UI chrome) as distinct `#[layout(...)]` layers in the route definition.

---

## 6. Controllers (Server Functions)

Controllers are server functions that act as API endpoints.

### Declaration

```rust
use crate::*;
use crate::models::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreatePostResponse {
    pub post_pk: Partition,
}

#[post("/api/posts/create", user: User)]
pub async fn create_post_handler(team_id: Option<TeamPartition>) -> Result<CreatePostResponse> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let post = Post::draft(user.into());
    post.create(cli).await?;

    Ok(CreatePostResponse { post_pk: post.pk })
}
```

### HTTP Method Macros

| Macro | Usage |
|-------|-------|
| `#[get("/path", injections...)]` | GET endpoint |
| `#[post("/path", injections...)]` | POST endpoint |
| `#[patch("/path", injections...)]` | PATCH endpoint |
| `#[put("/path", injections...)]` | PUT endpoint |
| `#[delete("/path", injections...)]` | DELETE endpoint |

### Injected Parameters

Declared in the macro attribute, available as local variables in the function body:

| Injection | Type | Description |
|-----------|------|-------------|
| `user: User` | Required auth | Rejects unauthenticated requests |
| `user: OptionalUser` | Optional auth | `None` for guests |
| `session: Extension<tower_sessions::Session>` | Session | For login/logout |
| `role: SpaceUserRole` | Role | Space-specific role |

### Request/Response DTOs

Define DTOs in the same controller file:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LoginResponse {
    #[serde(flatten)]
    pub user: User,
}
```

### Type Injection via `FromRequestParts`

The controller macros (`#[get]`, `#[post]`, etc.) support **type injection** — types declared in the macro attribute are automatically extracted from the HTTP request using Axum's `FromRequestParts` trait. This eliminates redundant boilerplate in every controller.

**Prefer type injection over manual extraction.** If you find yourself writing session/user/space extraction logic in a controller body, implement `FromRequestParts` on a dedicated type instead.

#### Bad: Manual extraction (redundant boilerplate)

```rust
// DON'T DO THIS — duplicates extraction logic in every handler
#[post("/api/teams/create", session: Extension<tower_sessions::Session>)]
pub async fn create_team_handler(body: CreateTeamRequest) -> Result<CreateTeamResponse> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await
        .map_err(|e| Error::Unauthorized(e.to_string()))?
        .ok_or(Error::Unauthorized("no session".to_string()))?;

    let cli = crate::config::get().dynamodb();
    let user_pk: Partition = user_pk.parse().unwrap_or_default();
    let user = User::get(cli, user_pk, Some(EntityType::User))
        .await?
        .ok_or(Error::Unauthorized("User not found".to_string()))?;
    // ... use user
}
```

#### Good: Type injection (clean, reusable)

```rust
// DO THIS — User is automatically extracted via FromRequestParts
#[post("/api/teams/create", user: User)]
pub async fn create_team_handler(body: CreateTeamRequest) -> Result<CreateTeamResponse> {
    let cli = crate::config::get().dynamodb();
    // `user` is already available as a local variable
    // ... use user directly
}
```

#### Implementing `FromRequestParts`

When you need a new injectable type, implement the trait on it:

```rust
#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for MyType
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        // 1. Check if already extracted (cache in extensions)
        if let Some(cached) = parts.extensions.get::<MyType>() {
            return Ok(cached.clone());
        }

        // 2. Extract dependencies (can chain other FromRequestParts types)
        let user = User::from_request_parts(parts, state).await?;

        // 3. Perform lookup / computation
        let conf = ServerConfig::default();
        let cli = conf.dynamodb();
        let my_data = MyType::get(cli, &user.pk).await?
            .ok_or(Error::NotFound("not found".into()))?;

        // 4. Cache in extensions for subsequent extractors
        parts.extensions.insert(my_data.clone());

        Ok(my_data)
    }
}
```

#### Existing Injectable Types

| Type | Injection | Behavior |
|------|-----------|----------|
| `User` | `user: User` | Extracts authenticated user from session; rejects if not logged in |
| `OptionalUser` | `user: OptionalUser` | Same but returns `None` for guests instead of rejecting |
| `SpaceCommon` | `space: SpaceCommon` | Extracts space from `space_id` path parameter |
| `SpaceParticipant` | `participant: SpaceParticipant` | Extracts participant (requires User + SpaceCommon) |
| `SpaceUserRole` | `role: SpaceUserRole` | Determines user's role in a space (Viewer/Participant/Creator) |
| `Extension<Session>` | `session: Extension<tower_sessions::Session>` | Raw session access (only for login/logout) |

#### Key Implementation Rules

1. **Cache in `parts.extensions`**: Always check `parts.extensions.get::<T>()` first and insert after extraction. This prevents redundant DB queries when multiple extractors depend on the same data.
2. **Chain extractors**: Compose complex types from simpler ones (e.g., `SpaceUserRole` calls `SpaceCommon::from_request_parts` then `User::from_request_parts`).
3. **Gate with `#[cfg(feature = "server")]`**: `FromRequestParts` is only available on the server.
4. **Use `Error` as `Rejection`**: All extractors use `type Rejection = Error;` for consistent error handling.

### Controller Module Organization

```rust
// controllers/mod.rs
pub mod create_post;
pub mod list_posts;
pub mod get_post;

pub use create_post::*;
pub use list_posts::*;
pub use get_post::*;
```

**Rules:**
- Always use `#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]` on DTOs.
- Return `Result<T>` (not `Json<T>`).
- Access DynamoDB via `crate::config::get().dynamodb()`.
- Gate helper functions with `#[cfg(feature = "server")]`.
- One handler function per file; co-locate its DTOs and helpers.
- Name handler functions with `_handler` suffix (e.g., `create_post_handler`).
- **Never manually extract session/user/space in controller bodies** — use type injection via `FromRequestParts` instead.

### Calling Controllers from Components

```rust
onclick: move |_| async move {
    match create_post_handler(None).await {
        Ok(resp) => { /* success */ }
        Err(e) => { error_message.set(Some(format!("{e}"))); }
    }
},
```

---

## 7. Models (DynamoEntity)

### Definition

```rust
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Post {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,

    #[dynamo(index = "gsi2", order = 1, sk)]
    pub visibility: Option<Visibility>,

    pub title: String,
    pub html_contents: String,
}
```

### Key Fields

- `pk: Partition` — always the partition key.
- `sk: EntityType` — always the sort key.

### `#[dynamo(...)]` Attributes

| Attribute | Description |
|-----------|-------------|
| `index = "gsiN"` | Target GSI (gsi1–gsi6) |
| `pk` | Field is the GSI partition key |
| `sk` | Field is the GSI sort key |
| `prefix = "PREFIX"` | Prepends `PREFIX#` to the stored value |
| `name = "find_by_x"` | Name of the generated query method |
| `order = N` | Ordering for composite sort keys |

A field can have **multiple** `#[dynamo(...)]` annotations for different GSIs.

### Constructors

Gate constructors and business logic with `#[cfg(feature = "server")]`:

```rust
#[cfg(feature = "server")]
impl Post {
    pub fn new(title: &str, author: Author) -> Self {
        let uid = uuid::Uuid::now_v7().to_string();
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            pk: Partition::Feed(uid),
            sk: EntityType::Post,
            created_at: now,
            title: title.to_string(),
            ..Default::default()
        }
    }
}
```

### Transactional Writes

```rust
transact_write!(
    cli,
    post.create_transact_write_item(),
    post_like.create_transact_write_item()
)?;
```

**Rules:**
- Always derive `Default, Debug, Clone, Serialize, Deserialize, DynamoEntity, PartialEq`.
- Always use `Partition` for pk and `EntityType` for sk.
- Use timestamps as `i64` (milliseconds via `chrono::Utc::now().timestamp_millis()`).
- Use `uuid::Uuid::now_v7()` for generating unique IDs.
- Gate all `impl` blocks with `#[cfg(feature = "server")]`.

---

## 8. Hooks

### Simple Context Accessor

```rust
pub fn use_user_context() -> Store<UserContext> {
    use_context::<Context>().user_context
}

pub fn use_popup() -> PopupService {
    use_context::<PopupService>()
}
```

### Query Hook (Fallible)

```rust
pub fn use_space_query(
    space_id: &SpacePartition,
) -> dioxus::prelude::Result<Loader<SpaceResponse>, Loading> {
    let key = space_key(space_id);
    use_query(&key, {
        let space_id = space_id.clone();
        move || get_space(space_id.clone())
    })
}

// Usage in component — note the `?` operator:
let space_loader = use_space_query(&space_id)?;
let space = space_loader.read().clone();
```

### Infinite Query Hook

```rust
let mut v = use_infinite_query(list_posts_handler)?;
// v.items()        → Vec<I>
// v.has_more()     → bool
// v.more_element() → Element (sentinel for infinite scroll)
```

### Action Hook (Mutations)

```rust
let mut participate = use_action(participate_space);
// Later:
participate.call(space_id).await;
invalidate_query(&space_detail_key);
```

**Rules:**
- Name all hooks with the `use_` prefix.
- Hooks returning loaded data must return `Result<Loader<T>, Loading>` — callers use `?` to suspend.
- After mutations, call `invalidate_query(&key)` to refresh dependent queries.

---

## 9. Providers and Context

### Script/Style Provider

```rust
#[component]
pub fn Provider() -> Element {
    rsx! {
        document::Script { src: asset!("/assets/ratel-post.js") }
    }
}
```

### State Context Provider (init pattern)

Context provider structs that hold reactive fields (`Signal`, `Loader`, `Memo`, `ReadOnlySignal`, `Resource`) should derive `DioxusController` to auto-generate getter methods. See [DioxusController Macro Guide](development/dioxus-controller-macro.md) for full details.

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct SpaceContextProvider {
    pub role: Loader<SpaceUserRole>,
    pub space: Loader<SpaceResponse>,
    pub current_role: Signal<SpaceUserRole>,
}

impl SpaceContextProvider {
    pub fn init(space_id: &SpacePartition) -> Result<Self, Loading> {
        let role = use_loader(move || get_user_role(space_id.clone()))?;
        let space = use_space_query(space_id)?;
        let current_role = use_signal(|| role());

        let srv = Self { role, space, current_role };
        use_context_provider(move || srv);
        Ok(srv)
    }
}

pub fn use_space_context() -> SpaceContextProvider {
    use_context()
}
```

**Consuming the context:**

```rust
// At the init site (layout) — use returned value directly:
let ctx = SpaceContextProvider::init(&space_id)?;
let role = ctx.current_role();  // Generated by DioxusController
let space = ctx.space();        // Generated by DioxusController

// In child components — retrieve via use_context:
let ctx = use_space_context();
let role = ctx.current_role();
```

**When NOT to use DioxusController:**
- Services with rich mutation APIs (e.g., `PopupService`, `ToastService`) — hand-written methods are more appropriate.
- Structs with custom getter logic (e.g., `TeamContext.selected_team()` indexes into a `Vec`).
- Structs where all fields are plain (non-reactive) types.

### Service init pattern

```rust
impl ThemeService {
    pub fn init() {
        #[cfg(not(feature = "server"))]
        let saved = load_theme().unwrap_or_default().parse().unwrap_or_default();
        #[cfg(feature = "server")]
        let saved = Theme::default();

        let svc = Self { theme: use_signal(move || saved) };
        use_context_provider(move || svc);
    }
}
```

### App Root Setup Order

```rust
#[component]
fn App() -> Element {
    // 1. Register services
    use_context_provider(|| PopupService::new());
    ToastService::init();
    ThemeService::init();

    // 2. Initialize auth context (may suspend)
    let _ = ratel_auth::Context::init()?;

    // 3. Initialize other contexts
    TeamContext::init();
    common::query::query_provider();

    rsx! {
        // 4. Global assets
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: MAIN_JS }

        // 5. Module providers
        common::Provider {}
        AuthProvider {}
        ratel_post::Provider {}

        // 6. Router
        Router::<Route> {}

        // 7. Global overlays
        ToastProvider {}
    }
}
```

**Rules:**
- Services use `init()` method → calls `use_context_provider` internally.
- Contexts loading server data return `Result<Self, Loading>` — callers use `?` to suspend.
- Provider components only load assets (scripts, styles) — no UI.
- Initialize services before auth context; initialize query provider after contexts.
- Context provider structs with reactive fields (`Signal`, `Loader`, `Memo`, `ReadOnlySignal`, `Resource`) should derive `DioxusController` to auto-generate getter methods. Do NOT derive it on service structs with mutation APIs (`PopupService`, `ToastService`, etc.) or structs with custom getter logic.

---

## 10. Feature Flags

| Flag | Purpose | Gate Pattern |
|------|---------|-------------|
| `server` | Server-side rendering, DynamoDB, AWS SDK | `#[cfg(feature = "server")]` |
| `web` | Browser WASM, client-side UI | `#[cfg(feature = "web")]` |
| `desktop` | Tauri desktop target | `#[cfg(feature = "desktop")]` |
| `mobile` | Android/iOS target | `#[cfg(feature = "mobile")]` |
| `bypass` | Skip auth verification in tests | `#[cfg(feature = "bypass")]` |

### Common Patterns

```rust
// Server-only module
#[cfg(feature = "server")]
pub mod server;

// Client-only code
#[cfg(not(feature = "server"))]
apply_theme(theme.to_string().as_str());

// Conditional derives
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]

// Server-only impl block
#[cfg(feature = "server")]
impl Post {
    pub fn new(...) -> Self { ... }
}

// Bypass for testing
#[cfg(feature = "bypass")]
if code.eq("000000") { /* skip verification */ }
```

**Rules:**
- All JS interop calls must be guarded with `#[cfg(not(feature = "server"))]`.
- All DynamoDB operations must be inside `#[cfg(feature = "server")]` blocks.
- Conditional derives use `#[cfg_attr(...)]`, not separate `impl` blocks.

---

## 11. Translation (i18n)

### Defining Translations

```rust
translate! {
    LoginModalTranslate;

    new_user: {
        en: "New user?",
        ko: "새 사용자?",
    },
    email_placeholder: {
        en: "Enter your email address",
        ko: "이메일 주소를 입력하세요",
    },
}
```

### Using Translations

```rust
let tr: LoginModalTranslate = use_translate();

rsx! {
    span { {tr.new_user} }
    input { placeholder: tr.email_placeholder }
}
```

### File Organization

- **Small components**: `translate!` block at the bottom of the component file.
- **View pages**: dedicated `i18n.rs` file in the same directory.
- **Layouts**: `translate!` block at the bottom of `layout.rs`.

**Rules:**
- Always provide both `en` and `ko` translations.
- Struct name must be suffixed with `Translate` (e.g., `LoginModalTranslate`).
- Use `use_translate()` to consume — never construct the struct manually.

---

## 12. Error Handling and Toast Notifications

### Error Enum

The project uses `common::Error` with `#[derive(Translate)]` for i18n error messages:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate)]
pub enum Error {
    #[error("Not found: {0}")]
    #[translate(en = "Not found", ko = "찾을 수 없습니다.")]
    NotFound(String),

    #[error("No session found")]
    #[translate(en = "Please sign in first", ko = "먼저 로그인 해주세요.")]
    NoSessionFound,

    #[cfg(feature = "server")]
    #[serde(skip)]
    #[error("AWS error: {0}")]
    Aws(#[from] crate::utils::aws::error::AwsError),
}
```

### Server-Side Usage

```rust
// Early return with context
let user = User::get(cli, &pk, Some(EntityType::User))
    .await?
    .ok_or(Error::NotFound("User not found".into()))?;
```

### Client-Side Error Display (Inline)

```rust
let mut error_message: Signal<Option<String>> = use_signal(|| None);

match login_handler(req).await {
    Ok(resp) => { /* success */ }
    Err(e) => { error_message.set(Some(format!("{e}"))); }
}

if let Some(err) = error_message() {
    div { class: "text-sm text-red-500", "{err}" }
}
```

### Toast Notifications

`ToastService` provides three methods for displaying toast notifications:

| Method | Parameter | Description |
|--------|-----------|-------------|
| `toast.info(msg)` | `impl Into<String>` | Informational message (plain string) |
| `toast.warn(msg)` | `impl Into<String>` | Warning message (plain string) |
| `toast.error(err)` | `common::Error` | Error toast (**must be a typed Error variant**) |

`toast.error()` accepts `common::Error` (not a string). It automatically translates the error message based on the user's current language via the `Translate` derive.

#### Correct Usage

```rust
let mut toast = use_toast();

// Info and warn accept strings
toast.info("Operation completed successfully");
toast.warn("This action cannot be undone");

// Error MUST use a typed Error variant — never pass a raw string
toast.error(common::Error::InsufficientAdmins);
toast.error(common::Error::WalletError(format!("Connection failed: {}", details)));

// When a controller/server function returns common::Error, pass it directly
match some_handler(req).await {
    Ok(resp) => { /* success */ }
    Err(e) => { toast.error(e); }
}
```

#### Anti-Patterns (Do NOT Do)

```rust
// BAD — toast.error() does not accept strings
toast.error("Something went wrong");
toast.error(format!("Failed: {}", err));
toast.error(err.to_string());

// BAD — manual translation is redundant; toast.error() translates automatically
toast.error(err.translate(&lang));
```

#### Adding New Error Variants for Toast

When you need a new user-facing error message for `toast.error()`, add a variant to `common::Error` in `app/common/src/types/error.rs`:

```rust
#[error("Descriptive error message")]
#[translate(en = "User-facing English message", ko = "사용자용 한국어 메시지")]
MyNewError,
```

Then use it: `toast.error(common::Error::MyNewError);`

**Rules:**
- Server-only error variants must be gated with `#[cfg(feature = "server")]` and `#[serde(skip)]`.
- Use `?` for propagation; use `.ok_or(Error::Variant(...))` for `Option` → `Result` conversion.
- Always provide `#[translate(...)]` on new error variants.
- **Always use typed `common::Error` variants with `toast.error()`** — never pass raw strings.
- **Never manually call `.translate()` before passing to `toast.error()`** — translation is automatic.
- For generic/unexpected errors, use `common::Error::Unknown(message)`.

---

## 13. Config

### Pattern

```rust
#[derive(Debug, Default)]
pub struct Config {
    pub common: CommonConfig,
    // Module-specific config fields
}

#[cfg(feature = "server")]
impl Config {
    pub fn dynamodb(&self) -> &aws_sdk_dynamodb::Client {
        self.common.dynamodb()
    }
    pub fn s3(&self) -> &common::utils::aws::S3Client { self.common.s3() }
    pub fn sns(&self) -> &common::utils::aws::SnsClient { self.common.sns() }
    pub fn ses(&self) -> &common::utils::aws::SesClient { self.common.ses() }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
```

**Rules:**
- Every module with controllers must have a `config.rs` with this pattern.
- Always delegate AWS client access through `CommonConfig`.
- Access config via `crate::config::get()` in controllers.

---

## 14. JavaScript Interop

### Step 1: Write JS Functions

```js
// app/common/js/src/theme.js
export function load_theme() {
    return window.localStorage.getItem("ratel-common-theme");
}
export function save_theme(theme) {
    window.localStorage.setItem("ratel-common-theme", theme);
}
```

### Step 2: Register on `window.ratel` Namespace

```js
// app/common/js/src/index.js
import * as theme from "./theme";

if (typeof window !== "undefined") {
    if (typeof window.ratel === "undefined") {
        window.ratel = {};
    }
    window.ratel.common = { theme };
}
```

### Step 3: Declare Rust FFI Bindings

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "ratel", "common", "theme"])]
extern "C" {
    pub fn load_theme() -> Option<String>;
    pub fn save_theme(theme: &str);
}
```

### Step 4: Load Bundle in Dioxus

```rust
pub const MAIN_JS: Asset = asset!("/assets/ratel-common.js");

rsx! {
    document::Script { src: MAIN_JS }
}
```

### Namespace Convention

| Module | JS Namespace |
|--------|-------------|
| common/theme | `window.ratel.common.theme` |
| app-shell | `window.ratel.app_shell` |
| ratel-auth/firebase | `window.ratel.auth.firebase` |
| ratel-post | `window.ratel.post` |

**Rules:**
- Always guard JS calls with `#[cfg(not(feature = "server"))]`.
- `js_namespace` array must match exactly the JS global path.
- Function names must match between JS exports and `extern "C"` declarations.
- Use `#[wasm_bindgen(js_name = "...")]` if renaming is needed.

---

## 15. Query and Data Fetching

### Query Keys

Define query key functions in the hooks module:

```rust
fn space_key(space_id: &SpacePartition) -> Vec<String> {
    vec!["spaces".to_string(), space_id.to_string()]
}
```

### use_query

```rust
let loader = use_query(&key, {
    let id = id.clone();
    move || get_handler(id.clone())
})?;
let data = loader.read().clone();
```

### use_infinite_query

```rust
let mut feed = use_infinite_query(list_posts_handler)?;

rsx! {
    for post in feed.items() {
        FeedCard { key: "post-{post.pk}", post }
    }
    {feed.more_element()}
}
```

### Invalidation

```rust
// After mutation
invalidate_query(&["spaces", &space_id.to_string()]);
```

**Rules:**
- Query keys are hierarchical `Vec<String>` — invalidating a prefix invalidates all descendants.
- Always call `invalidate_query` after mutations that affect cached data.
- Use `use_query` for single-resource fetches; use `use_infinite_query` for paginated lists.

---

## 16. Views

### Role-Based Dispatch

```rust
#[component]
pub fn DashboardPage(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

    match role {
        SpaceUserRole::Creator => rsx! { CreatorPage { space_id } },
        SpaceUserRole::Participant => rsx! { ParticipantPage { space_id } },
        SpaceUserRole::Viewer => rsx! { ViewerPage { space_id } },
        _ => rsx! { ViewerPage { space_id } },
    }
}
```

### View File Organization

```
src/views/
  mod.rs              # Dispatcher (role-based, feature-based)
  admin_page.rs       # Admin view
  viewer_page.rs      # Read-only view
  participant_page.rs # Participant view
  i18n.rs             # Translations for this page
```

### Manual Refresh Pattern

```rust
let mut refresh = use_signal(|| 0u64);

let resource = use_server_future(move || {
    let _ = refresh();  // creates dependency
    async move { fetch_data().await }
})?;

// After mutation:
refresh.set(refresh() + 1);
```

**Rules:**
- Use role-based dispatch when different users see different UIs.
- Use `?` on all fallible hooks to suspend while loading.
- Prefer `use_query` over `use_server_future` when data needs cache invalidation.

---

## 17. SeoMeta Component

The `SeoMeta` component (`app/common/src/components/seo_meta/`) renders SEO meta tags into the document head. Every page-level view should include it for proper Google SEO, Open Graph, and Twitter Card support.

### Props

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

### Robots Enum

| Variant | Output |
|---------|--------|
| `Robots::IndexFollow` (default) | `index, follow` |
| `Robots::NoindexFollow` | `noindex, follow` |
| `Robots::IndexNofollow` | `index, nofollow` |
| `Robots::NoindexNofollow` | `noindex, nofollow` |

### Usage

```rust
use common::components::SeoMeta;
use common::components::Robots;

// Minimal — only title required
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

// Draft/private page — prevent indexing
rsx! {
    SeoMeta {
        title: "Draft Post",
        robots: Robots::NoindexNofollow,
    }
}
```

**Rules:**
- Every page-level view must include `SeoMeta` with at least a `title`.
- Use `Robots::NoindexNofollow` for draft, private, or admin-only pages.
- Provide `description` for all public-facing pages (Google uses it for search snippets).
- Set `canonical` when the same content is accessible via multiple URLs.
- Set `og_type` to `"article"` for post/article pages; default `"website"` is fine for other pages.

---

## 18. Button Component (Loading Pattern)

The `Button` component (`common::components::Button`) supports an optional `loading` prop for async click handlers. The parent component owns the loading state and passes it as a `ReadOnlySignal<bool>`.

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `String` | `""` | Additional CSS classes |
| `size` | `ButtonSize` | `Medium` | Size variant |
| `style` | `ButtonStyle` | `Primary` | Style variant (`Primary`, `Secondary`, `Outline`, `Text`) |
| `shape` | `ButtonShape` | `Rounded` | Shape variant (`Rounded`, `Square`) |
| `disabled` | `bool` | `false` | Disable the button |
| `loading` | `ReadOnlySignal<bool>` | `false` | Show loading indicator and disable button |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | Click handler |

**Loading pattern for async handlers:**

The `Button` cannot auto-detect async handlers because `EventHandler::call()` returns `()` (Dioxus spawns async futures internally via `SpawnIfAsync`). Instead, the parent manages a `loading` signal and passes it to the button.

```rust
use crate::common::*;

#[component]
pub fn MyView() -> Element {
    let mut loading = use_signal(|| false);

    rsx! {
        Button {
            loading: loading(),
            onclick: move |_| async move {
                loading.set(true);
                let result = do_something_async().await;
                loading.set(false);
                match result {
                    Ok(_) => { /* success */ }
                    Err(err) => { /* handle error */ }
                }
            },
            "Submit"
        }
    }
}
```

**Rules:**
- Always use `use_signal(|| false)` for loading state in the parent component.
- Set `loading.set(true)` at the start and `loading.set(false)` after the async operation completes (in both success and error paths).
- Guard against double-clicks by checking `if loading() { return; }` at the top of the async handler.
- When `loading` is `true`, the button automatically disables itself and shows a `LoadingIndicator` instead of children.
- For synchronous handlers, omit the `loading` prop entirely.

---

## Quick Reference

| Concern | Convention |
|---------|-----------|
| Component | `#[component] pub fn Name(...) -> Element` |
| Route enum | `#[derive(Routable)]` + `#[rustfmt::skip]` |
| Layout | `Outlet::<Route> {}` inside layout |
| Server function | `#[get/post/patch/put/delete("/path", injections)]` |
| Type injection | Implement `FromRequestParts`, use `user: User` in macro attr |
| Model | `#[derive(DynamoEntity)]` with `pk: Partition, sk: EntityType` |
| GSI | `#[dynamo(prefix="X", index="gsiN", pk/sk, name="fn")]` |
| Hook | `use_*` prefix, return `Result<Loader<T>, Loading>` for data |
| Translation | `translate! { Struct; key: { en: "...", ko: "..." } }` |
| Error | `common::Error` enum with `#[translate(...)]` |
| Toast error | `toast.error(common::Error::Variant)` — typed Error, never raw string |
| Config | `crate::config::get().dynamodb()` singleton |
| Server-only | `#[cfg(feature = "server")]` |
| Client-only | `#[cfg(not(feature = "server"))]` |
| Imports | `use crate::*;` at file top |
| JS interop | `#[wasm_bindgen(js_namespace = [...])]` + guard with `cfg` |
| SEO | `SeoMeta { title, description, ... }` in every page view |
| Button loading | Parent owns `use_signal(\|\| false)`, passes `loading: loading()`, sets in async handler |
