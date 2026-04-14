# HTML to Dioxus Implementation

Workflow for converting existing HTML/CSS/JS into Dioxus RSX components. Use when you already have approved HTML and need to implement it as a Dioxus component.

## When to Use

- You have an existing HTML file (mockup, design, or reference) ready for conversion
- Migrating a standalone HTML page into the Dioxus app
- Converting HTML received from a designer or external tool into RSX
- The design phase is complete — this is purely implementation

## Step 1: Analyze the HTML

- Read the HTML file and identify its structure (sections, components, interactive elements)
- Identify CSS: inline styles, `<style>` blocks, or external CSS files
- Identify JS: inline scripts, `<script>` blocks, or external JS files
- List all class names and element IDs — these must be preserved exactly
- Identify interactive states (hover, active, open/close, scroll-based)
- Note any external dependencies (fonts, icons, images)

## Step 2: Plan Component Decomposition

- Identify the page-level component and potential sub-components
- Extract a sub-component when a section is self-contained (panel, modal, card) and > ~50 lines of RSX
- Map HTML sections to the file structure:

```
app/ratel/src/features/<module>/pages/<page>/
├── mod.rs              # Page module: registers sub-components, re-exports
├── component.rs        # Page-level Dioxus component
├── style.css           # Page-level styles
├── script.js           # JS helpers (optional)
├── i18n.rs             # Translations (EN + KO)
├── page.html           # Original HTML (kept as reference)
├── <sub_component>/
│   ├── mod.rs
│   ├── component.rs
│   └── style.css
```

- **References**: conventions/html-first-components.md, conventions/feature-module-structure.md

## Step 3: Separate HTML, CSS, JS

Split the HTML file into three concerns in the component directory:

### CSS (`style.css`)
- Extract all styles from `<style>` blocks and inline styles
- Use CSS classes with BEM-like naming (`.block__element--modifier`)
- Use `data-*` / `aria-*` attribute selectors for Dioxus-controlled state (`[data-open="true"]`)
- Use CSS classes (`.active`, `.open`) for JS-controlled state (scroll, animations)
- Avoid Tailwind utilities in CSS files — Tailwind is for RSX `class:` attributes only
- Use space toggle for dark/light theme colors:
  ```css
  .my-component {
    --comp-bg: var(--dark, #0c0c1a) var(--light, #ffffff);
    --comp-text: var(--dark, #f0f0f5) var(--light, #12121a);
  }
  ```

### JS (`script.js`) — only if needed
- Register helpers on `window.ratel.<module>` namespace
- Keep JS minimal — prefer Dioxus signals/events for state management
- JS is for DOM manipulation that RSX cannot express (scroll-based detection, animations)
- Use `classList.toggle` for JS state, not `setAttribute('data-*')` (Dioxus re-renders overwrite attributes)
- Wrap in MutationObserver pattern for CSR compatibility:
  ```js
  (function () {
    function init() {
      var el = document.getElementById("my-element");
      if (!el || el.dataset.bound) return;
      el.dataset.bound = "true";
      // ... setup logic
    }
    init();
    new MutationObserver(function () { init(); })
      .observe(document.body, { childList: true, subtree: true });
  })();
  ```

### HTML (`page.html`)
- Keep the structural HTML as a reference file (not compiled)
- Strip `<style>` and `<script>` blocks — those are now separate files

- **References**: conventions/html-first-components.md, conventions/styling.md

## Step 4: Convert HTML to RSX

```bash
dx translate -f <path>/page.html
```

- Copy the RSX output into `component.rs`
- **Keep all class names and IDs identical** to the HTML — JS and CSS depend on them
- Fix any conversion artifacts (unclosed tags, attribute formatting)

- **Skills**: dioxus-knowledge-patch

## Step 5: Replace Static Content

- Replace hardcoded text with `translate!` macro references
- Replace static data with values from hooks/props (`use_space()`, `use_loader()`, etc.)
- Replace hardcoded images with `asset!()` references
- Replace raw HTML elements with project primitives:
  - `<div class="flex ...">` → `Row` or `Col`
  - `<button>` → `Button`
  - `<input>` → `Input`
  - `<select>` → `Select`
- Replace hardcoded colors with semantic token classes (never `text-neutral-400`, always `text-foreground-muted`)

- **References**: conventions/dioxus-app.md, conventions/styling.md, conventions/anti-patterns.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 6: Wire State & Event Handlers

- Add Dioxus signals for UI state (panel open/close, active tab, selected item)
- Wire event handlers (`onclick`, `onchange`, `oninput`) to signals and server calls
- Dioxus-controlled state → `data-*` / `aria-*` attributes in RSX
- JS-controlled state (scroll, animations) → CSS classes via `classList.toggle` in JS
- Use existing patterns: `use_popup()` for modals, `use_navigator()` with `Route` enum for navigation
- Place navigation **after** all `.await` points in async handlers

- **References**: conventions/dioxus-app.md, conventions/anti-patterns.md

## Step 7: Load Assets in Component

```rust
rsx! {
    document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
    document::Script { defer: true, src: asset!("./script.js") }  // only if JS exists
    // ... component RSX
}
```

- Each sub-component loads its own `style.css` via its own `document::Link`
- Always use `defer: true` on `document::Script`

## Step 8: Create i18n Translations

- Define `translate!` block in `i18n.rs` with EN and KO strings
- All user-facing text must use the `translate!` macro

```rust
translate! {
    PageTranslate;
    title: { en: "Title", ko: "제목" },
    description: { en: "Description", ko: "설명" },
}
```

- **References**: conventions/i18n.md

## Step 9: Register Module

- Add `pub mod <page>;` in the parent `mod.rs`
- Wire into route if this is a routable page
- Sub-components: parent `mod.rs` declares and re-exports sub-component modules

## Step 10: Lint & Format

```bash
rustywind --custom-regex "class: \"(.*)\"" --write <file>.rs
dx fmt -f <file>.rs
```

- Apply to every `.rs` file created or modified
- **References**: conventions/lint-and-format.md

## Step 11: Verify Build

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Checklist

- [ ] Original HTML preserved as `page.html` reference
- [ ] CSS extracted to `style.css` with space toggle for dark/light
- [ ] JS (if any) extracted to `script.js` with MutationObserver pattern
- [ ] Class names and IDs match HTML exactly
- [ ] All text uses `translate!` macro
- [ ] Raw HTML elements replaced with project primitives (`Button`, `Input`, `Row`, `Col`)
- [ ] Semantic color tokens used (no raw Tailwind palette colors)
- [ ] `defer: true` on all `document::Script` tags
- [ ] Sub-components each load their own `style.css`
- [ ] Module registered in parent `mod.rs`
- [ ] `rustywind` + `dx fmt` applied
- [ ] `dx check --features web` passes
- [ ] `cargo check --features web` passes
- [ ] `cargo check --features server` passes
