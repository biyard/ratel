# HTML-First Component Pattern

Build Dioxus components by designing in HTML/CSS/JS first, then converting to RSX.

## File Structure

Pages and their sub-components each own their assets. The page `mod.rs` is the entry point; sub-components live in named subdirectories with the same structure:

```
<page_dir>/
├── mod.rs              # Page module: registers sub-components, re-exports
├── component.rs        # Page-level Dioxus component
├── style.css           # Page-level styles (arena, portal, HUD, etc.)
├── script.js           # Page-level JS helpers (optional)
├── i18n.rs             # Translations shared across the page
├── page.html           # Source HTML (kept as reference, not compiled)
├── <sub_component>/
│   ├── mod.rs          # Sub-component module
│   ├── component.rs    # Sub-component Dioxus component
│   └── style.css       # Sub-component styles (loaded via own asset!())
├── <another_component>/
│   ├── mod.rs
│   ├── component.rs
│   └── style.css
└── ...
```

### Key rules
- Each sub-component loads its own `style.css` via `document::Link { rel: "stylesheet", href: asset!("./style.css") }` inside its component
- CSS for a sub-component lives in that sub-component's directory, not in the parent
- The page `mod.rs` declares sub-component modules and re-exports them for use in `component.rs`
- i18n is shared at the page level — sub-components import from the parent via `use crate::features::<module>::pages::<page>::*`
- Extract a sub-component when it is self-contained (own panel, modal, card) and > ~50 lines of RSX

### Page `mod.rs` pattern

```rust
mod component;
mod i18n;
mod overview_panel;    // sub-component
mod settings_panel;    // sub-component

pub use component::*;
use i18n::*;
use overview_panel::*;
use settings_panel::*;

use crate::features::<module>::*;
```

### Sub-component `mod.rs` pattern

```rust
mod component;
pub use component::*;
```

### Sub-component `component.rs` pattern

```rust
use crate::features::<module>::pages::<page>::*;  // access parent i18n + shared types

#[component]
pub fn MySubComponent(open: bool, on_close: EventHandler<()>) -> Element {
    let tr: PageTranslate = use_translate();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        // ... RSX
    }
}
```

## Reference implementation

`app/ratel/src/features/spaces/pages/index/` — Space viewer arena page with:
- `overview_panel/` sub-component (slide-in overview panel)
- `settings_panel/` sub-component (slide-in theme/language settings)

## Conversion Flow

```
1. Write HTML mockup (docs/design/<name>.html)
2. Approve design with user
3. Split into page.html + style.css + script.js
4. Run: dx translate -f page.html
5. Paste RSX output into component.rs
6. Replace static IDs/text with signals + translate!
7. Add event handlers
8. Extract large sections into sub-component directories
```

## Asset Loading

```rust
rsx! {
    document::Link { rel: "stylesheet", href: asset!("./style.css") }
    document::Script { defer: true, src: asset!("./script.js") }
    // ... component RSX
}
```

**Important:** Always use `defer: true` on `document::Script`. Without it, the script runs before the component's DOM elements exist, causing `getElementById` to return null.

## CSS Dark/Light Theme Colors

Use the space toggle pattern (`var(--dark, ...) var(--light, ...)`) for all colors that differ between themes. This leverages the `--dark` / `--light` custom properties set on `html[data-theme]` in `dx-components-theme.css`.

Define component-scoped CSS variables at the root element of each component:

```css
.my-component {
  --comp-bg: var(--dark, #0c0c1a) var(--light, #ffffff);
  --comp-text: var(--dark, #f0f0f5) var(--light, #12121a);
  --comp-text-muted: var(--dark, #8888a8) var(--light, #6b6b80);
  --comp-border: var(--dark, rgba(255,255,255,0.06)) var(--light, rgba(0,0,0,0.08));
  --comp-glass: var(--dark, rgba(12,12,26,0.65)) var(--light, rgba(255,255,255,0.72));
  --comp-shadow: var(--dark, rgba(0,0,0,0.4)) var(--light, rgba(0,0,0,0.08));

  background: var(--comp-bg);
  color: var(--comp-text);
  border: 1px solid var(--comp-border);
}
```

### Rules
- **Never hardcode colors** — always use `var(--dark, ...) var(--light, ...)` for anything that should change between themes
- Define variables on the component's root element, then reference them in child selectors
- Accent colors that stay the same in both themes (e.g. `#fcb300` gold) can be used directly
- Reference: `app/ratel/assets/dx-components-theme.css` for the global pattern

## CSS State Management

Use data attributes and ARIA attributes for CSS-driven state instead of conditional class strings:

```css
/* CSS */
.panel[data-open="true"] { transform: translateX(0); }
.portal[data-dimmed="true"] { opacity: 0.15; filter: blur(6px); }
.settings-opt[aria-selected="true"] { color: #fcb300; }
.hud-btn[aria-pressed="true"] { background: rgba(252,179,0,0.12); }
```

```rust
// RSX — Dioxus sets attributes reactively
div { class: "panel", "data-open": is_open, ... }
div { class: "settings-opt", "aria-selected": is_active, onclick: move |_| { ... }, ... }
```

## JS Namespace Convention

Register JS helpers under `window.ratel.<module>`:

```js
(function () {
  window.ratel = window.ratel || {};
  window.ratel.myModule = {
    doSomething: function () { /* ... */ }
  };
})();
```

Keep JS minimal — prefer Dioxus signals and event handlers for state management. Use JS only for DOM operations that RSX cannot express.

## When NOT to Use This Pattern

- Simple components that are mostly Tailwind utility classes — use direct RSX
- Components composed entirely from existing primitives (`Button`, `Card`, `Row`, etc.)
- Server-side-only code (controllers, models)
