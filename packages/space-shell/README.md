# Development

## Adding Space Page

This guide walks through adding a new page to the space shell. We use `dashboard` as a reference example.

### 1. Create the page package

Create a new package `packages/space-page-{name}/` with the following structure:

```
packages/space-page-{name}/
├── Cargo.toml
├── assets/
│   ├── favicon.ico
│   ├── main.css
│   └── tailwind.css
└── src/
    ├── app.rs
    ├── assets.rs
    ├── lib.rs
    ├── main.rs
    ├── menu.rs
    ├── route.rs
    └── views/
        ├── mod.rs
        ├── creator_page.rs
        ├── participant_page.rs
        ├── candidate_page.rs
        └── viewer_page.rs
```

### 2. Define Cargo.toml

```toml
[package]
name = "space-page-{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = { workspace = true }
common = { workspace = true }

[features]
default = []
web = ["dioxus/web", "common/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server", "common/server"]
```

### 3. Create source files

**`src/lib.rs`** - Module declarations and public exports:

```rust
mod app;
mod assets;
mod menu;
mod route;
mod views;

pub use assets::*;
use dioxus::prelude::*;

pub use app::App;
pub use menu::get_nav_item;

use common::*;
use route::Route;
```

**`src/assets.rs`** - Asset references:

```rust
use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
```

**`src/app.rs`** - Entry component routed from the shell:

```rust
use crate::*;

#[component]
pub fn App(space_id: SpacePartition, rest: Vec<String>) -> Element {
    if !rest.is_empty() {
        return rsx! {
            h2 { "Rest: {rest:?}" }
        };
    }

    rsx! {
        Router::<Route> {}
    }
}
```

**`src/route.rs`** - Internal page routes:

```rust
use crate::*;
use crate::views::HomePage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/spaces/:space_id/{name}")]
    HomePage { space_id: SpacePartition },
}
```

**`src/menu.rs`** - Navigation item for the sidebar. Returns `None` to hide from users without permission:

```rust
use crate::*;

pub fn get_nav_item(
    space_id: SpacePartition,
    _role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    Some((
        icon(),
        SpacePage::{Name},       // Must match the SpacePage enum variant
        Route::HomePage { space_id }.into(),
    ))
}

#[component]
pub fn icon() -> Element {
    rsx! {
        svg {
            // SVG icon content
        }
    }
}
```

**`src/views/mod.rs`** - Role-based page dispatch:

```rust
use crate::*;

mod creator_page;
mod viewer_page;
// ... other role pages

use creator_page::*;
use viewer_page::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_server_future(move || async move { SpaceUserRole::Viewer })?.value();

    match role().unwrap_or_default() {
        SpaceUserRole::Creator => rsx! { CreatorPage { space_id } },
        SpaceUserRole::Viewer => rsx! { ViewerPage { space_id } },
        // ... other roles
    }
}
```

**`src/views/creator_page.rs`** (and other role pages):

```rust
use super::*;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    rsx! {
        "Creator page"
    }
}
```

**`src/main.rs`** - Standalone dev entry point:

```rust
use dioxus::prelude::*;
use space_page_{name}::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
```

### 4. Add SpacePage variant

Add a new variant to `SpacePage` enum in `packages/common/src/types/space_page.rs`:

```rust
pub enum SpacePage {
    // ... existing variants
    #[translate(ko = "한글 이름")]
    {Name},
}
```

### 5. Register in space-shell

**`packages/space-shell/Cargo.toml`** - Add the dependency and feature flags:

```toml
[dependencies]
space-page-{name} = { path = "../space-page-{name}/", optional = true }

[features]
web = ["...", "space-page-{name}/web"]
desktop = ["...", "space-page-{name}/desktop"]
mobile = ["...", "space-page-{name}/mobile"]
server = ["...", "space-page-{name}/server"]
```

**`packages/space-shell/src/lib.rs`** - Import the page:

```rust
use space_page_{name} as {name};
```

**`packages/space-shell/src/route.rs`** - Add the route variant:

```rust
use {name}::App as {Name}App;

pub enum Route {
    // ... existing routes
    #[route("/{name}/:..rest")]
    {Name}App { space_id: SpacePartition, rest: Vec<String> },
}
```

**`packages/space-shell/src/layout.rs`** - Add to the navigation menu:

```rust
let menus = vec![
    // ... existing items
    {name}::get_nav_item(space_id.clone(), role),
];
```

