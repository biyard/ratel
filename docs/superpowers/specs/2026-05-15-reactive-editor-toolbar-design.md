# Reactive Editor Bubble Toolbar — Design

**Branch**: `feature/reactive-editor-toolbar`
**Author / Date**: hackartist · 2026-05-15
**Scope**: `app/ratel/src/common/components/editor/`

## Summary

Add a selection-triggered "bubble" toolbar to the existing contenteditable editor. When the user drags to select text inside `.re-content`, a compact floating toolbar appears immediately below the selection (auto-flips above if no space). It exposes the subset of commands that act on a selection: inline formatting (Bold / Italic / Underline / Strikethrough / Inline code / Link) plus a block-format dropdown (Paragraph / H1 / H2 / H3 / Quote / Code block). The existing static toolbar at the top of the editor is **unchanged** and remains the home of all other commands (Undo/Redo, alignment, lists, indent, image/youtube/table/hr, clear-format).

## Goals

1. Reduce visual distance between the user's cursor and the format buttons they most often want during writing.
2. Preserve every existing behavior — no regression to IME composition, no change to read-only rendering, no change to the static toolbar.
3. Implementation surface stays inside the editor module (three files): `component.rs`, `script.js`, `assets/main.css`.

## Non-goals

- Replacing or hiding the static toolbar.
- Mobile/touch support — the bubble is desktop-only. Touch devices keep the OS native selection popup + static toolbar.
- A pointer/arrow tail on the bubble — visually simple v1, no caret/triangle.
- Smooth follow-on-scroll — scrolling hides the bubble; the user re-selects to bring it back.
- Multi-range selections (collapses to the first range).
- Adding any new commands beyond the 6 inline + block dropdown listed above.

## Architecture

The editor is a contenteditable + `document.execCommand` widget where all interactive state lives in plain JS and the Rust/Dioxus layer only renders the DOM scaffolding and bridges content via a hidden `<input>` element. This is intentional: re-rendering the editor through Dioxus during selection changes re-applies `dangerous_inner_html` and destroys the caret position — most visibly during Korean (IME) composition. See the existing comment at `app/ratel/src/common/components/editor/component.rs:19`.

The bubble follows the same split:

```
component.rs           main.css                          script.js
─────────────          ────────                          ─────────
<div class="re-bubble" .ratel-editor .re-bubble { ... }   listen to selectionchange/mouseup/keyup
     data-visible="false">                                position bubble, toggle data-visible
   .re-block                                              hide on collapse/blur/ESC/scroll
   .re-toolbar__group                                     skip wiring entirely on (pointer: coarse)
     6× .re-tb-btn (existing class)
   </>
```

State (visibility, position) lives entirely in JS — Dioxus signals are not involved.

## Components

### 3.1 RSX (`component.rs`)

Add a sibling element after `.re-content`, gated by `is_editable` (no bubble in read-only mode):

```rust
if is_editable {
    div { class: "re-bubble", "data-visible": "false",
        div { class: "re-bubble__inner",
            // Block-format dropdown — reuses .re-block markup
            div { class: "re-block", "data-open": "false",
                button { class: "re-block__btn", ... }
                div { class: "re-block__menu", role: "listbox",
                    button { class: "re-block__item", "data-block": "P",         ... }
                    button { class: "re-block__item re-block__item--h1",  "data-block": "H1",         ... }
                    button { class: "re-block__item re-block__item--h2",  "data-block": "H2",         ... }
                    button { class: "re-block__item re-block__item--h3",  "data-block": "H3",         ... }
                    button { class: "re-block__item re-block__item--quote","data-block": "BLOCKQUOTE",... }
                    button { class: "re-block__item re-block__item--code", "data-block": "PRE",        ... }
                }
            }
            // Inline buttons — reuse .re-tb-btn / data-cmd codes from the static toolbar
            div { class: "re-toolbar__group",
                button { class: "re-tb-btn", "data-cmd": "bold",          ... } // <svg>...</svg>
                button { class: "re-tb-btn", "data-cmd": "italic",        ... }
                button { class: "re-tb-btn", "data-cmd": "underline",     ... }
                button { class: "re-tb-btn", "data-cmd": "strikeThrough", ... }
                button { class: "re-tb-btn", "data-cmd": "code-inline",   ... }
                button { class: "re-tb-btn", "data-cmd": "link",          ... }
            }
        }
    }
}
```

Because the buttons reuse the existing `.re-tb-btn` / `.re-block` classes and `data-cmd` values, they pick up the existing JS click handlers, modal flow (`link` → existing modal), and `aria-pressed` active-state sync for free.

### 3.2 CSS (`app/ratel/assets/main.css`)

Append to the existing editor section (per `conventions/styling.md` — no per-component stylesheet):

```css
/* === src/common/components/editor — bubble toolbar ===================== */
.ratel-editor { position: relative; }       /* if not already set */
.ratel-editor .re-bubble {
  position: absolute;
  z-index: 50;
  pointer-events: none;
  opacity: 0;
  transform: translateY(4px) scale(0.96);
  transition: opacity 120ms ease, transform 120ms ease;
}
.ratel-editor .re-bubble[data-visible="true"] {
  pointer-events: auto;
  opacity: 1;
  transform: translateY(0) scale(1);
}
.ratel-editor .re-bubble__inner {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px;
  background: var(--re-toolbar-bg);          /* existing token */
  border: 1px solid var(--re-border-subtle); /* existing token */
  border-radius: 10px;
  backdrop-filter: blur(20px);               /* parity with .re-toolbar */
  box-shadow: 0 6px 24px rgba(0,0,0,0.18);
  white-space: nowrap;
}
```

Token reuse keeps the bubble visually consistent with the static toolbar across dark/light themes without redefinition.

### 3.3 JS (`script.js`)

Two distinct changes:

**A. Tiny refactor — handle multiple `.re-block` dropdowns.**

Today: `var blockDropdown = root.querySelector(".re-block")` picks one element. After this change there will be two `.re-block` instances (static + bubble). Replace the single-element wiring with a per-instance helper that runs over `root.querySelectorAll(".re-block")` and binds open/close, click-outside, item-click, and label-sync independently. `syncBlockLabel()` also walks every `.re-block__label` and every `[data-block]` item.

**B. New bubble controller.**

```js
// Hard gate: never activate on touch / coarse-pointer devices.
if (window.matchMedia("(pointer: coarse)").matches) { /* skip bubble wiring */ }

var bubble = root.querySelector(".re-bubble");
if (bubble) {
  // Critical: pressing the bubble must NOT move focus / collapse the editor's
  // selection. Without this, click → focus shifts to <button> → selection
  // collapses → execCommand has nothing to operate on.
  bubble.addEventListener("mousedown", function (e) { e.preventDefault(); });

  function showBubble(range) {
    var sr = range.getBoundingClientRect();
    var er = root.getBoundingClientRect();
    var bw = bubble.offsetWidth;
    var bh = bubble.offsetHeight;
    var top = sr.bottom - er.top + 8;                         // default: below selection
    if (sr.bottom + bh + 16 > window.innerHeight) {           // flip when out of viewport
      top = sr.top - er.top - bh - 8;
    }
    var left = sr.left + sr.width / 2 - er.left - bw / 2;     // center horizontally
    left = Math.max(8, Math.min(left, er.width - bw - 8));    // clamp
    bubble.style.top  = top + "px";
    bubble.style.left = left + "px";
    bubble.dataset.visible = "true";
  }
  function hideBubble() { bubble.dataset.visible = "false"; }

  function updateBubble() {
    if (document.activeElement !== editor) return hideBubble();
    var sel = window.getSelection();
    if (!sel || sel.rangeCount === 0 || sel.isCollapsed) return hideBubble();
    var range = sel.getRangeAt(0);
    if (!editor.contains(range.commonAncestorContainer)) return hideBubble();
    showBubble(range);
  }

  // Trigger sources:
  //   - selectionchange : already wired; add updateBubble() into the existing handler
  //   - mouseup on editor : selection only stabilizes on mouseup during drag
  //   - keyup on editor : Shift+arrow / Home / End / PageUp / PageDown keyboard selection
  //   - scroll / resize : hide (simpler than recomputing)
  //   - Escape : hide and return focus to editor
}
```

The `selectionchange` listener already exists for `syncToolbarState()`/`syncBlockLabel()`; we add a `updateBubble()` call into the same handler — no second listener.

## Data flow

1. User drags / Shift+arrow → browser fires `selectionchange` (and `mouseup` / `keyup`).
2. `updateBubble()` reads `window.getSelection()`, confirms the range is inside `editor` and non-collapsed.
3. `getBoundingClientRect()` of the range + editor root → bubble's `top`/`left` (computed in editor-relative coords).
4. `data-visible="true"` toggles CSS to fade/scale in.
5. User clicks a bubble button: `mousedown` is prevented (selection preserved) → click → existing `.re-tb-btn` handler → `applyCmd` / modal flow → `document.execCommand` operates on the still-live selection.
6. User clicks elsewhere / presses Esc / scrolls / blurs → `hideBubble()`.

Dioxus is not involved at any step; no signal is read or written. The hidden-input bridge that already syncs editor HTML back to Rust continues to fire on `input` events from `execCommand`.

## Behavior matrix

| Trigger | Result |
|---|---|
| Mouse drag selects text | bubble appears at mouseup, below selection (or flipped above) |
| Shift + Arrow / Home / End | bubble appears on keyup |
| Double-click / triple-click | bubble appears on selectionchange |
| Selection collapses (caret only) | bubble hides |
| Click outside editor (focus leaves) | bubble hides — note: clicking the bubble itself does NOT blur because of `mousedown` preventDefault |
| Escape pressed while bubble visible | bubble hides, focus returns to editor |
| Window scroll or resize | bubble hides (user re-selects to bring it back) |
| Touch / coarse pointer | bubble never wires up |
| `is_editable == false` | bubble not rendered |

## Error handling / edge cases

- **`getBoundingClientRect()` returns a zero-width rect on certain selections** (e.g. inside an empty `<br>`-only paragraph). Treat zero-area rect as "no useful anchor" → hide.
- **Selection spans across nodes outside the editor** (rare, e.g. user selected from outside into the editor). `editor.contains(range.commonAncestorContainer)` filters this — hide.
- **Bubble wider than editor** at very narrow viewports — `clamp(8, …, er.width - bw - 8)` may produce `left = 8` and overflow on the right. Acceptable for v1 (the toolbar is ~250px; the editor is unlikely to be that narrow on desktop). If it becomes an issue we wrap the buttons to a second row via flex-wrap, no JS change needed.
- **Existing `.re-block` JS code assumes a single dropdown.** This is the explicit refactor in §3.3.A. We must test the static toolbar's block dropdown still works after the refactor.

## Test plan

### Manual / regression
- Drag-select prose → bubble appears below; click Bold → text becomes bold; selection remains highlighted.
- Drag-select at the bottom of a long scrollable editor → bubble auto-flips above.
- Type Korean Hangul ("안녕하세요") continuously — caret must not jump to start of editor. (Same IME guarantee as today; this is the regression smoke test for the no-Dioxus-rerender contract.)
- Static toolbar's block dropdown still opens, selects, closes — verifies the `querySelector` → `querySelectorAll` refactor.
- Touch device (or DevTools touch emulation with `pointer: coarse`) — bubble never appears; static toolbar unaffected.
- Read-only mode (`editable=false`) — bubble not in DOM.

### Playwright
Extend the editor-touching spec (likely the post-creation or discussion-editor spec in `playwright/tests/web/`) with a new `test()` block in the existing `describe.serial`:

```js
test("editor bubble appears on selection", async ({ page }) => {
  // (assumes editor is already mounted and focused from a previous step)
  await fill(editorLocator, "Hello world");
  // select "Hello" — Playwright keyboard selection (Shift+End-of-word) keeps it deterministic
  await page.keyboard.press("Home");
  await page.keyboard.press("Shift+End");
  await expect(page.locator(".re-bubble[data-visible='true']")).toBeVisible();
  await page.locator(".re-bubble .re-tb-btn[data-cmd='bold']").click();
  await expect(editorLocator).toContainText("Hello world"); // structurally bolded
  // Optional: read editor HTML and assert <b>/<strong> wrapping if the editor exposes it.
});
```

### Build verification (project convention)
- `rustywind --custom-regex 'class: "(.*)"' --write` on `component.rs`
- `dx fmt -f component.rs`
- `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web`
- `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web`
- `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`

## Files changed

| File | Nature of change |
|---|---|
| `app/ratel/src/common/components/editor/component.rs` | +~80 lines: new `.re-bubble` subtree gated on `is_editable` |
| `app/ratel/src/common/components/editor/script.js` | refactor `.re-block` wiring to per-instance loop; +~50 lines: bubble controller (gate + show/hide/position + event listeners) |
| `app/ratel/assets/main.css` | +~30 lines: `.re-bubble` styles under the editor section, plus `position: relative` on `.ratel-editor` if missing |
| `playwright/tests/web/<existing-editor-spec>.spec.js` | +1 `test()` step for the bubble selection flow |

No changes to: any feature module under `src/features/`, any controller, any DynamoDB entity, any DTO, any Cargo dependency, any CDK code.

## Open questions / risks

- **`syncToolbarState()` performance with two toolbars.** It already runs on every `selectionchange`. Doubling the `.re-tb-btn` count (+6) is negligible.
- **Bubble flicker during drag.** `mouseup` is the primary trigger to avoid showing/hiding on every micro-selection during a drag. `selectionchange` still calls `updateBubble()`, but a quick `requestAnimationFrame` throttle can be added inside the existing scheduler (`scheduleUpdate`) if flicker shows up in QA. Plan: ship without throttle, add only if observed.
- **Existing `.re-block` `mousedown` saves selection.** When we duplicate `.re-block`, both instances will run `savedRange = saveSelection()`. This is the same `savedRange` variable in the same closure — last-write-wins, which is fine because the user can only interact with one dropdown at a time.
