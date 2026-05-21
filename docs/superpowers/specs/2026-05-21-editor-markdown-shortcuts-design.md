# Editor Markdown Shortcuts — Design

**Author / Date**: hackartist · 2026-05-21
**Target editor**: `app/ratel/src/common/components/editor/` (contentEditable + execCommand based custom editor — the one used by posts, sub-team docs, spaces overview, quizzes, polls, discussions, reports, fact-or-fold)
**Out of scope**: the separate Tiptap-based web component at `app/interops/web-components/assets/tiptap-editor.js`

## Summary

Add Notion-style markdown shortcuts to the custom rich-text editor so that typing markdown markers at the start of a line converts the surrounding block to the corresponding HTML structure. Includes list nesting via Tab / Shift+Tab and a one-step revert affordance. Implementation is JS-only — the Dioxus `Editor` component is unchanged.

## 1. Scope

### Block conversions (marker + space → block type)

| Input typed | Result | Notes |
|---|---|---|
| `# ` | `<h1>` | |
| `## ` | `<h2>` | |
| `### ` | `<h3>` | |
| `* `, `- `, `+ ` | `<ul><li>` | all three accepted |
| `1. ` (or any digits + `. `) | `<ol><li>` | leading number is discarded; list always starts at the natural index |
| `> ` | `<blockquote>` | |
| ` ``` ` (third backtick) | `<pre>` followed by an empty `<p>` | fires on the third backtick at line start; Enter on a `<p>```</p>` line is kept as a fallback. The implementation emits a bare `<pre>` (not `<pre><code>`) because the editor's read/write surface treats `<pre>` itself as the code block. |
| `--- ` or `*** ` | `<hr>` followed by an empty paragraph | caret lands in the new paragraph |

### List behaviors (key-based)

- Inside any `<li>`: `Tab` → `document.execCommand("indent")` (nests deeper)
- Inside any `<li>`: `Shift+Tab` → `document.execCommand("outdent")` (one level up; outdents out of the list entirely when already at depth 1)
- On `Enter` inside an **empty** `<li>` → exit the list to a new paragraph (Notion/Slack standard)

## 2. Trigger conditions

- Marker must be typed at the **start of the current block** — caret position 0 of the block's first text node. Markers typed mid-line never trigger.
- **The block does NOT have to be empty.** If the user types `# ` at the start of an existing paragraph `Hello world`, the result is `<h1>Hello world</h1>` — only the marker (`# `) is stripped, the rest of the line is preserved.
- Trigger timing:
  - **Space-triggered shortcuts** (`# `, `- `, `> `, etc.) fire on the `input` event whose `inputType === "insertText"` and `data === " "`, evaluated *after* the space has been inserted. Implementation reads the text content before the caret to match against the marker regex.
  - **Enter-triggered shortcut** (` ``` `) fires on `keydown` for `Enter` when the block's text content is exactly ` ``` `.
- After conversion:
  - The marker characters are removed
  - The block type is replaced (uses `document.execCommand("formatBlock", ...)` for headings/quotes/code, `insertUnorderedList` / `insertOrderedList` for lists, `insertHorizontalRule` + inserted `<p>` for hr)
  - Caret is repositioned to the start of the surviving line content inside the new block

## 3. IME / Korean composition safety

The editor already tracks `composing` via `compositionstart` / `compositionend` events (see `script.js:42-51`). The markdown-shortcut module follows the same guard:

- During composition (`isComposing === true` on the input event, OR the editor's `composing` flag is set) **no shortcut may fire**.
- The `compositionend` handler does not retroactively trigger a shortcut for a space typed mid-composition — the user typing `안녕 ` should never produce a heading.

## 4. Undo / escape hatch

The single most important UX detail. Users frequently want to type `# ` literally and not get a heading. The escape hatch:

- **Immediately after a conversion**, pressing `Backspace` reverts it.
- "Immediately after" means: the next user input after the conversion is the revert key. Any non-Backspace, non-modifier-only key (typing a character, arrow keys) **disarms** the escape hatch — subsequent Backspace is treated as a normal delete.
- Revert restores the literal marker text that the user typed (`# `, `- `, ` ``` `, etc.) and the block type that the conversion replaced. Caret returns to its position right after the marker (mirroring what the user originally had).
- Implementation: a small per-editor `lastConversion` object holding `{ markerText, snapshot }`. Cleared on any non-revert input.

**Out of scope for this initial cut (tracked as follow-ups):**

- `Cmd+Z` / `Ctrl+Z` revert. The browser's native undo currently runs after a conversion, but we don't intercept it to do a single-step revert the way Backspace does. If a future change wants this, the implementation would mirror the Backspace branch in the `keydown` listener.
- Disarming on `selectionchange` from a mouse click that moves the caret away from the conversion site. Arrow-key navigation already disarms via the keydown disarm path; mouse clicks don't fire keydown, so a click + Backspace can still revert a conversion that's no longer relevant. Listening to `selectionchange` and clearing `lastConversion` when the caret leaves the conversion block would close this gap.

## 5. Architecture

All changes are in `app/ratel/src/common/components/editor/script.js`. No Rust changes. No new files.

```
script.js
├── init(root)                          (existing)
├── IME guard / debounced emit          (existing)
├── toolbar + block dropdown wiring     (existing)
├── modals (link/image/youtube/table)   (existing)
├── selection bubble toolbar            (existing)
└── ─────────────────────────────────────
    markdown-shortcuts module (NEW)
    ├── handleBeforeInput(e)            ← may not be needed; we use `input` + `keydown`
    ├── handleInput(e)                  ← space-triggered conversions
    ├── handleKeydown(e)                ← Tab/Shift+Tab, Enter on empty <li>, Enter on ```, Backspace revert
    ├── tryConvertBlock(marker, blockEl)
    ├── exitListIfEmpty()
    ├── undoLastConversion()
    └── armRevert(snapshot) / disarmRevert()
```

### Why no `beforeinput`?

`beforeinput` would let us cancel the space before it's inserted, which is theoretically cleaner. But:
- Safari WebKit's `beforeinput` for IME composition is inconsistent.
- The existing editor reads state from the DOM after `input` fires, so reusing that pattern keeps the data flow uniform.
- Cancelling and re-inserting via `beforeinput` makes the undo stack lose the marker chars, which we *want* to keep around for the Backspace revert.

### Reuse of existing helpers

- `applyCmd(cmd, value)` — already wraps `editor.focus()` + `execCommand` + `scheduleUpdate`. Block conversions call it directly.
- `scheduleUpdate()` — every conversion ends here so the bridge `<input>` fires and Dioxus sees the new HTML.
- `composing` flag — already correct; we just read it.
- `savedRange` / `saveSelection` / `restoreSelection` — already exist for the block dropdown; not directly reused but the same Range-restoration pattern applies.

## 6. Edge cases

- **Typing `# ` inside a heading or code block**: do nothing (the markers belong to the user's content there). Detection: walk up from caret; if the closest block ancestor is already `H1/H2/H3/PRE/BLOCKQUOTE`, skip.
- **Typing `- ` inside an existing `<li>`**: do nothing (avoids confusing nested-list creation when the user is just typing).
- **Numbered list with custom start** (`5. `): we discard the number and start the new `<ol>` at 1. Notion behaves this way; supporting custom `start=` requires either a wrapping `<ol start="N">` or DOM rewriting, neither of which is worth the complexity here.
- **Pasted content containing markdown markers**: not converted. Conversion only fires on direct user typing.
- **Multiple paragraphs selected when marker typed**: the leading marker on a multi-block selection isn't a real Notion pattern — treat as a normal text insertion (no conversion).

## 7. Test plan

New spec: `playwright/tests/web/editor-markdown-shortcuts.spec.js`. Use the existing post/discussion compose page that already mounts the `Editor` component (e.g., the post creation flow used in `arena-action-editor.spec.js`).

| # | Scenario | Expected |
|---|---|---|
| 1 | Type `# Title` in empty paragraph | `<h1>Title</h1>` |
| 2 | Type `## ` then `Heading2` | `<h2>Heading2</h2>` |
| 3 | Type `### Sub` | `<h3>Sub</h3>` |
| 4 | Type `- Apple` then Enter, then `Banana` | `<ul><li>Apple</li><li>Banana</li></ul>` |
| 5 | Type `* item` (asterisk variant) | `<ul><li>item</li></ul>` |
| 6 | Type `1. first` then Enter, `second` | `<ol><li>first</li><li>second</li></ol>` |
| 7 | In an `<li>`, press Tab | new nested `<ul><li>` at depth 2 |
| 8 | After Tab, press Shift+Tab | back to depth 1 |
| 9 | Press Enter on an empty `<li>` | list ends, new `<p>` appears |
| 10 | Type `> Quoted` | `<blockquote><p>Quoted</p></blockquote>` (whatever execCommand produces) |
| 11 | Type ` ``` ` then Enter | `<pre>` block, caret inside |
| 12 | Type `---` then space | `<hr>` followed by empty `<p>`, caret in `<p>` |
| 13 | Type `# Title`, then immediately Backspace | reverts to literal `<p># Title</p>` with caret after the space |
| 14 | Type `# `, then type a letter, then Backspace | normal Backspace (escape hatch already disarmed) — letter is deleted |
| 15 | **Block has content first**: type `Hello`, move caret to start, type `# ` | `<h1>Hello</h1>` (marker stripped, text preserved) |
| 16 | Korean composition: type `안녕 ` (with Korean IME) | no conversion, text remains as plain paragraph |
| 17 | Mid-line `# `: type `abc# `  | no conversion (marker not at block start) |
| 18 | Inside existing `<h1>`, type `# ` at start | no conversion (already a heading; don't double-process) |
| 19 | Inside a `<pre>` code block, type `- ` at start | no conversion (code content is literal) |

All tests follow the existing `playwright/tests/utils.js` helpers (`goto`, `click`, `fill`, `getEditor`).

## 8. Non-goals (explicit)

- Inline markdown marks: `**bold**`, `*italic*`, `~~strike~~`, `` `code` `` — separate work
- Markdown link shortcut: `[text](url)` — separate work
- Full markdown → HTML parser for paste handling — separate work
- Table / image / YouTube markdown shortcuts — existing modal flow stays
- Tiptap-based web component (`app/interops/web-components/assets/tiptap-editor.js`) — out of scope; that editor has its own input-rules system

## 9. Risks / open considerations

- `document.execCommand` is officially deprecated by WHATWG but every shipping browser still implements it; the rest of this editor already depends on it heavily, so no new risk introduced.
- Safari's `execCommand("outdent")` behavior when already at the outermost list level differs from Chrome — the Shift+Tab path needs the test to be cross-browser checked. Local fix path: detect "no further outdent possible" via DOM and do a manual `formatBlock <p>` instead.
- The Backspace-revert mechanism requires snapshotting innerHTML at conversion time. For very large editor contents this is O(N) per conversion — fine for our usage (posts, descriptions), but worth noting if the editor is ever used for long-form documents.
