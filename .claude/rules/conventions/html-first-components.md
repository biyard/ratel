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
- Each sub-component loads its own `style.css` via `document::Stylesheet { href: asset!("./style.css") }` inside its component
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
        document::Stylesheet { href: asset!("./style.css") }
        // ... RSX
    }
}
```

## Reference Implementations

| Page | Path | Description |
|------|------|-------------|
| Space viewer (portal) | `features/spaces/pages/index/` | Viewer splash, participation flow, arena aesthetic |
| Action dashboard | `features/spaces/pages/index/action_dashboard/` | Carousel quest board for participants |

HTML mockups in `docs/design/`:
- `space-viewer.html` — viewer portal with participate/signin/verification cards
- `space-participant.html` — participant action carousel with quest cards

## Conversion Flow

```
1. Write HTML mockup (docs/design/<name>.html)
2. Approve design with user — iterate in browser
3. Split into page.html + style.css + script.js in component directory
4. Run: dx translate -f page.html
5. Paste RSX output into component.rs
6. Replace static IDs/text with signals + translate!
7. Add event handlers
8. Extract large sections into sub-component directories
```

## Asset Loading

```rust
rsx! {
    document::Stylesheet { href: asset!("./style.css") }
    document::Script { defer: true, src: asset!("./script.js") }
    // ... component RSX
}
```

**Important:** Always use `defer: true` AND wrap the script in a MutationObserver pattern. `defer` handles SSR (initial page load), but CSR (client-side navigation) renders components asynchronously — the script may run before DOM elements exist. The MutationObserver catches both cases:

```js
(function () {
  function init() {
    var el = document.getElementById("my-element");
    if (!el || el.dataset.bound) return;
    el.dataset.bound = "true";
    // ... setup logic
  }
  init(); // SSR
  new MutationObserver(function () { init(); })
    .observe(document.body, { childList: true, subtree: true }); // CSR
})();
```

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

Use data attributes and ARIA attributes for Dioxus-controlled state, and CSS classes for JS-controlled state:

### Dioxus-controlled state → `data-*` / `aria-*` attributes

```css
.panel[data-open="true"] { transform: translateX(0); }
.portal[data-dimmed="true"] { opacity: 0.15; filter: blur(6px); }
.settings-opt[aria-selected="true"] { color: #fcb300; }
.hud-btn[aria-pressed="true"] { background: rgba(252,179,0,0.12); }
```

```rust
// RSX — Dioxus sets attributes reactively
div { class: "panel", "data-open": is_open, ... }
div { class: "settings-opt", "aria-selected": is_active, ... }
```

### JS-controlled state → CSS classes (`.active`, `.open`)

When JS needs to toggle state on scroll/interaction (e.g., carousel active card), use CSS classes — **not** `data-*` attributes. Dioxus re-renders reset attributes it manages, but won't touch classes added by JS.

```css
.quest-card { opacity: 0.25; filter: blur(6px); transform: scale(0.75); }
.quest-card.active { opacity: 1; filter: blur(0); transform: scale(1.05); }

.carousel-dot { width: 8px; }
.carousel-dot.active { width: 28px; }
```

```js
// JS toggles .active class on scroll
cards.forEach((c, i) => c.classList.toggle('active', i === closest));
```

**Rule:** If Dioxus owns the state (signals), use `data-*` attributes. If JS owns the state (scroll position, animation), use CSS classes.

## Class Name Consistency

**Critical:** When converting an HTML mockup to a Dioxus component, keep the **exact same CSS class names and element IDs** as the mockup. The JS file queries elements by these names. Renaming classes (e.g., `carousel-track` → `action-carousel__track`) breaks JS selectors silently.

Checklist:
- CSS class names in `style.css` match the HTML mockup exactly
- Element IDs in RSX match what `script.js` queries via `getElementById`
- JS selectors (`.querySelector`, `.querySelectorAll`) match the CSS classes
- If the mockup uses `.quest-card`, the RSX uses `class: "quest-card"`, not `class: "action-card"`

## JS Patterns for Scroll-Based UI

For carousels and scroll-snap UIs where the active element depends on scroll position:

1. **Let JS own the active state** — don't try to sync scroll position into a Dioxus signal
2. **Use `classList.toggle`** — not `setAttribute('data-active', ...)` which Dioxus would overwrite
3. **Use `defer: true`** on the script tag — the DOM must exist before JS queries it
4. **Use `scroll-snap-type: x mandatory`** on the track and `scroll-snap-align: center` on cards
5. **Calculate closest-to-center card** on each scroll event using `getBoundingClientRect`

Reference implementation: `features/spaces/pages/index/action_dashboard/script.js`

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

Keep JS minimal — prefer Dioxus signals and event handlers for state management. Use JS only for DOM operations that RSX cannot express (scroll-based active detection, fly-to-archive animations, etc.).

## When NOT to Use This Pattern

- Simple components that are mostly Tailwind utility classes — use direct RSX
- Components composed entirely from existing primitives (`Button`, `Card`, `Row`, etc.)
- Server-side-only code (controllers, models)
