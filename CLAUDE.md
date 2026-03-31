# CLAUDE.md

Ratel is a decentralized legislative platform. Monorepo: Dioxus 0.7 fullstack app (`app/ratel/`) + Rust backend packages (`packages/`) + blockchain contracts.

## Build & Verification Commands

```bash
# MUST run after any code change in app/ratel/
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web

# Backend tests
cd packages/main-api && make test

# Playwright e2e tests
cd playwright && npx playwright test <file>

# Local dev (Docker)
make run          # all services
make infra        # infrastructure only (LocalStack, DynamoDB)
make stop         # stop all

# Dioxus app dev (port 8000)
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```

**`DYNAMO_TABLE_PREFIX` is required at compile time** for DynamoEntity. Use `ratel-dev` for dev, `ratel-local` for Docker local.

## Monorepo Structure

| Path | What | Notes |
|------|------|-------|
| `app/ratel/` | Dioxus fullstack app (`app-shell`) | Single package, feature-gated modules |
| `app/ratel/src/common/` | Shared foundation: types, models, components, utils, config | `use crate::common::*;` |
| `app/ratel/src/features/` | Feature modules: auth, posts, spaces, users, teams, membership, admin | Each gated by Cargo feature |
| `packages/main-api/` | REST API (Axum 0.8.1) | v3 controllers at `src/controllers/v3/` |
| `packages/by-macros/` | Proc macros (DynamoEntity derive) | |
| `packages/bdk/` | Biyard Dev Kit | |
| `packages/dioxus-translate/` | i18n framework | `translate!` macro |
| `packages/icons/` | Icon library | |

Feature flags: `full` (default) = `membership` + `users` + `teams` + `spaces_full`. Also: `web`, `server`, `lambda`, `bypass`.

## Critical Rules

These rules prevent silent failures, broken themes, and review rejections.

### 1. Semantic color tokens only — never Tailwind palette colors

```
BAD:  text-neutral-400, bg-slate-800, text-gray-500, bg-green-500
GOOD: text-foreground-muted, bg-card-bg, text-text-primary, bg-background
```

Never use `text-neutral-*`, `bg-gray-*`, `text-zinc-*`, etc. — they bypass the theme system. Tailwind spacing/sizing utilities (`gap-4`, `p-5`, `w-full`) are fine. See `.claude/rules/figma-design-system.md` for token table.

### 2. Always use primitive components — never raw HTML for interactive UI

Use `Button` not `button`, `Input` not `input`, `Select` not `select`, `Card` for card layouts, `Row`/`Col` for flex layout.

Available primitives (via `use crate::common::*;`):
`Accordion`, `AlertDialog`, `Avatar`, `Badge`, `Button`, `Calendar`, `Card`, `Checkbox`, `Col`, `Collapsible`, `ContextMenu`, `DatePicker`, `Dialog`, `DropdownMenu`, `FileUploader`, `Form`, `Input`, `Label`, `Popover`, `Popup`, `Progress`, `RadioGroup`, `Row`, `ScrollArea`, `Select`, `SeoMeta`, `Separator`, `Sheet`, `Sidebar`, `Skeleton`, `Slider`, `Switch`, `Tabs`, `Textarea`, `Toggle`, `ToggleGroup`, `Tooltip`

### 3. All user-facing strings must use `translate!` macro

```rust
translate! {
    MyTranslate;
    title: { en: "Title", ko: "제목" },
}
// In component:
let t = MyTranslate::new(use_locale());
rsx! { "{t.title}" }
```

### 4. Enum values in UI: `.translate()`, never `.to_string()`

```rust
#[derive(Translate)]
pub enum Status {
    #[translate(en = "Active", ko = "활성")]
    Active,
}
// Use: {status.translate(&lang())}
```

### 5. Typed error enums — never `Error::BadRequest(String)`

Define domain-specific error enum with `Translate` derive, register in `common::Error` with `#[from]` + `#[translate(from)]`. See `SpaceRewardError` pattern in `app/ratel/src/common/types/reward/error.rs`.

### 6. `bypass` feature: never bundle into convenience features

`bypass` skips auth verification (accepts `000000`). Must require explicit `--features bypass`, never included in `local-dev` or `full`.

### 7. Guard JS interop with `#[cfg(not(feature = "server"))]`

JS is unavailable during SSR. All `wasm_bindgen` calls must be cfg-gated.

### 8. TailwindCSS bracket syntax for arbitrary values

```
BAD:  z-101 (silently ignored)
GOOD: z-[101], w-[350px], gap-[13px]
```

Standard scale values don't need brackets: `z-10`, `gap-4`.

### 9. Server-side validation for user-configurable numbers

Never use `i32::MAX` / `i64::MAX` as defaults. Define shared constants for upper bounds (e.g., `MAX_TOTAL_ATTEMPTS: i64 = 100`). Validate at write path, clamp at read path.

### 10. SeoMeta on every page view

```rust
rsx! { SeoMeta { title: "Page - Ratel" } }
```

### 11. Status colors must use semantic tokens

Define CSS variables in `tailwind.css` with the space toggle pattern, never use raw `bg-green-500` or `text-red-400`.

## Import Conventions

```rust
// Start with wildcard — brings in common items via re-export chain
use crate::features::<module>::*;
// Only add explicit imports for items NOT in the wildcard chain
use crate::common::hooks::use_infinite_query;
```

Check sibling files to see which imports are standard before adding new ones.

## Feature Module Structure

Each feature in `app/ratel/src/features/<module>/` follows:

```
mod.rs, route.rs, layout.rs
controllers/    - Server functions (#[get], #[post], etc.)
models/         - DynamoDB entities (feature: server)
components/     - Reusable UI
views/          - Page-level views
hooks/          - Dioxus hooks
i18n.rs         - Translations
types/          - Custom types + error enums
```

## Error Handling Pattern

```rust
// 1. Define in features/<module>/types/error.rs
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum MyFeatureError {
    #[error("internal msg")]
    #[translate(en = "User message", ko = "사용자 메시지")]
    SpecificError,
}

// 2. Register in common::Error
#[error("{0}")]
#[translate(from)]  // delegates translation to inner type
MyFeature(#[from] MyFeatureError),

// 3. Use in controllers
return Err(MyFeatureError::SpecificError.into());
```

## JS Interop Pattern (Summary)

Three layers: JS source → register on `window.ratel.<module>` → Rust `#[wasm_bindgen(js_namespace = [...])]`.

```rust
#[wasm_bindgen(js_namespace = ["window", "ratel", "common", "theme"])]
extern "C" {
    pub fn load_theme() -> Option<String>;
}
```

Namespace must match exactly. JS files in `app/ratel/assets/` for `asset!()` macro.

## Accessibility

- `alt` on all `img` elements
- `aria-label` on icon-only buttons
- Use `Link { to: "..." }` for navigation, not `div { onclick: navigator.push() }`

## DynamoDB Single-Table Design

- Partition key (`pk`): `Partition` enum (`USER#<id>`, `SPACE#<id>`, etc.)
- Sort key (`sk`): `EntityType` enum
- GSIs: gsi1 through gsi6+ for alternate access patterns
- `#[dynamo(prefix)]` must be model-specific abbreviation to prevent GSI collisions
- Never call `.to_string()` on `EntityType` for IDs — convert to sub entity type first

## Pagination with `use_infinite_query`

- Prefer over `use_server_future` for any list that may exceed one page
- Always render `{v.more_element()}` at end of list container
- Make `v` mutable: `let mut v = use_infinite_query(...)`
- Filter server-side when possible — client-side filtering after paginated fetch causes edge cases
- Hard-cap server-side DynamoDB scanning loops (`max_pages = 5`)

## Scroll Event Handlers

Never spawn unbounded async tasks from `onscroll`. Use trailing-edge throttle with `scroll_check_pending` signal guard.

## Dioxus Reactivity

- `use_effect` only re-runs when reactive signals are read **inside** the closure
- Event handlers: `onscroll: move |_| { ... }` — no outer brace wrapping needed

## Async Event Handlers

Never call `popup.close()` or navigate away before `.await` points — Dioxus drops the future when the component unmounts. Move unmounting actions after all awaits.

## Documentation Consistency

When updating rules in this file, also update:
- `.github/copilot-instructions.md`
- `docs/playwright-testing.md` (for Playwright rules)

## Reference Docs

| Doc | When to read |
|-----|-------------|
| `docs/dioxus-convention.md` | Writing/reviewing Dioxus components |
| `docs/tailwindcss-convention.md` | Styling rules and token usage |
| `docs/playwright-testing.md` | Writing e2e tests |
| `docs/troubleshooting.md` | Debugging async/component issues |
| `docs/dynamo-prefix-convention.md` | Adding DynamoDB `#[dynamo(prefix)]` |
| `docs/entity-type-id-convention.md` | Converting EntityType to IDs |
| `.claude/rules/figma-design-system.md` | Figma-to-code workflow, design tokens, component props |
