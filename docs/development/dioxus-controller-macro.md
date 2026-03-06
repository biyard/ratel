# DioxusController Derive Macro Guide

The `DioxusController` derive macro generates getter methods for Dioxus reactive fields on context provider structs. It eliminates boilerplate accessor code by inspecting each field's type and generating a method that unwraps the reactive wrapper.

## Table of Contents

- [Overview](#overview)
- [Supported Types](#supported-types)
- [Usage](#usage)
  - [Basic Example](#basic-example)
  - [Context Provider Pattern](#context-provider-pattern)
  - [Consuming the Context](#consuming-the-context)
- [When to Use](#when-to-use)
- [When NOT to Use](#when-not-to-use)
- [Migration Guide](#migration-guide)

## Overview

Context provider structs in Dioxus often hold reactive fields (`Signal`, `Loader`, `Memo`, etc.) that consumers need to read. Without `DioxusController`, you write manual getters or use verbose syntax like `(self.field)()` or `self.field.read().clone()` at every call site.

`DioxusController` auto-generates a getter method per field that returns the unwrapped inner value.

## Supported Types

| Field Type | Generated Signature | Implementation |
|---|---|---|
| `Signal<T>` | `fn field(&self) -> T` | `(self.field)()` |
| `ReadOnlySignal<T>` | `fn field(&self) -> T` | `(self.field)()` |
| `Memo<T>` | `fn field(&self) -> T` | `(self.field)()` |
| `Loader<T>` | `fn field(&self) -> T` | `(self.field)()` |
| `Resource<T>` | `fn field(&self) -> Result<T, RenderError>` | `self.field.suspend()?()` |
| Other types | *(skipped — no method generated)* | — |

**Note:** `Resource<T>` getters integrate with Dioxus suspense — they return `Result` and suspend the component if the resource is still loading.

## Usage

### Basic Example

```rust
use by_macros::DioxusController;

#[derive(Clone, Copy, DioxusController)]
pub struct MyController {
    pub name: Signal<String>,
    pub count: Signal<i32>,
    pub items: Loader<Vec<Item>>,
}
```

**Generated code (conceptual):**

```rust
impl MyController {
    pub fn name(&self) -> String { (self.name)() }
    pub fn count(&self) -> i32 { (self.count)() }
    pub fn items(&self) -> Vec<Item> { (self.items)() }
}
```

### Context Provider Pattern

Use `DioxusController` on context provider structs that hold reactive state and are shared via `use_context_provider` / `use_context`.

```rust
use dioxus::fullstack::{Loader, Loading};

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

### Consuming the Context

**At the init site (layout)** — use the returned value directly:

```rust
#[component]
pub fn SpaceLayout(space_id: SpacePartition) -> Element {
    let ctx = SpaceContextProvider::init(&space_id)?;

    let role = ctx.current_role();  // Generated getter
    let space = ctx.space();        // Generated getter
    // ...
}
```

**In child components** — retrieve via `use_context`:

```rust
#[component]
pub fn DashboardPage(space_id: SpacePartition) -> Element {
    let ctx = use_space_context();
    let role = ctx.current_role();

    match role {
        SpaceUserRole::Creator => rsx! { CreatorPage { space_id } },
        _ => rsx! { ViewerPage { space_id } },
    }
}
```

Or use a dedicated hook that wraps the context access:

```rust
pub fn use_space_role() -> ReadSignal<SpaceUserRole> {
    let ctx = use_space_context();
    ctx.current_role.into()
}

// In component:
let role = use_space_role()();
```

## When to Use

Apply `DioxusController` to structs that are **all** of the following:

1. **Context providers** — registered via `use_context_provider`, consumed via `use_context`
2. **Hold reactive fields** — `Signal`, `Loader`, `Memo`, `ReadOnlySignal`, or `Resource`
3. **Are `Clone + Copy`** — required for Dioxus context sharing

Typical candidates:
- `SpaceContextProvider` (role, space data)
- `AuthContext` (user session)
- `TeamContext` (team list, selection)

## When NOT to Use

Do **not** attach `DioxusController` when:

- **Custom getter logic is needed** — e.g., `selected_team()` in `TeamContext` indexes into a `Vec` using another signal. The macro only generates `(self.field)()`.
- **Fields have non-reactive types** — plain `String`, `i32`, etc. are skipped (no method generated), so the derive is useless if all fields are plain types.
- **The struct is a service with mutation methods** — e.g., `PopupService`, `ToastService`, `LayoverService`. These have rich `&mut self` APIs (`open`, `close`, `push`). Adding generated getters would be redundant or confusing alongside hand-written methods.
- **Fields are private** — generated getters are `pub fn`, but they access `self.field` directly. If the field itself is private, consumers can't construct the struct, but the getters still work. However, the convention is `pub` fields for `DioxusController` structs.

## Migration Guide

### Before (manual access)

```rust
#[derive(Clone, Copy)]
pub struct SpaceContextProvider {
    pub role: Loader<SpaceUserRole>,
    pub space: Loader<SpaceResponse>,
    pub current_role: Signal<SpaceUserRole>,
}

// Call site — verbose
let role_loader = use_user_role(&space_id)?;
let role = role_loader.read().clone();

let space_loader = use_space_query(&space_id)?;
let space = space_loader.read().clone();
```

### After (with DioxusController)

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct SpaceContextProvider {
    pub role: Loader<SpaceUserRole>,
    pub space: Loader<SpaceResponse>,
    pub current_role: Signal<SpaceUserRole>,
}

// Call site — clean
let ctx = SpaceContextProvider::init(&space_id)?;
let role = ctx.current_role();
let space = ctx.space();

// Or in child components:
let ctx = use_space_context();
let role = ctx.current_role();
```

### Key changes when migrating

1. Add `DioxusController` to the derive list
2. Remove manual getter methods that simply unwrap reactive fields
3. Replace `use_user_role(&space_id)?` + `.read().clone()` with `ctx.current_role()`
4. Remove redundant `use_*` hooks that duplicated data already in the context provider
5. Keep custom methods (e.g., `toggle_role`, `switch_role`) — the macro doesn't touch `impl` blocks
