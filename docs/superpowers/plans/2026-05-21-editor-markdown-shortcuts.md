# Editor Markdown Shortcuts Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Notion-style markdown shortcuts to the custom contentEditable editor so that typing `# `, `## `, `### `, `* `/`- `/`+ `, `1. `, `> `, ` ``` `, `--- `, `*** ` at the start of a block converts the block to the matching HTML structure; `Tab`/`Shift+Tab` nest/un-nest list items; `Enter` on an empty `<li>` exits the list; and one-step `Backspace`/`Cmd+Z` reverts a just-fired conversion.

**Architecture:** JS-only change. All logic appended to the existing per-editor `init(root)` closure in `app/ratel/src/common/components/editor/script.js` so it shares the `composing`, `editor`, `scheduleUpdate`, `applyCmd` bindings. No Rust changes. Reuses `document.execCommand` for the actual DOM mutation so output matches what the existing toolbar produces. Backspace revert is implemented as a snapshot of `editor.innerHTML` with a `﻿`-wrapped sentinel marking caret position, restored on revert.

**Tech Stack:** Vanilla browser JS in `script.js`, executed in the page that mounts the Dioxus `Editor` component. Playwright (JavaScript, `tests/web/`) for e2e tests.

**Spec:** [docs/superpowers/specs/2026-05-21-editor-markdown-shortcuts-design.md](../specs/2026-05-21-editor-markdown-shortcuts-design.md)

---

## File Structure

| File | Action | Purpose |
|---|---|---|
| `app/ratel/src/common/components/editor/script.js` | **Modify** — append ~180 lines inside `init(root)` just before the final `// Initial paint` block | All markdown-shortcut behavior (input handler, keydown handler, marker matcher, conversion dispatchers, revert state) |
| `playwright/tests/web/editor-markdown-shortcuts.spec.js` | **Create** | E2E coverage of all 19 scenarios from spec §7 |

No new Rust files. No new JS files. The existing `script.js` is ~510 lines; adding ~180 lines keeps it under the 700-line range typical for this codebase's `init(root)` style.

---

## Prerequisites

- Local dev server running before any Playwright task: from repo root run `make infra` (LocalStack + DynamoDB), then in another shell `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local ENV=local dx serve --port 8000 --web`.
- `playwright/` workspace deps already installed (`npm i` in `playwright/`).
- Login session for the Playwright test: handled by `goto()` + existing storage state in `playwright/playwright.config.js`. The new spec follows the same auth setup as `cross-posting.spec.js`.

---

### Task 1: Scaffold the Playwright spec with a smoke test

**Files:**
- Create: `playwright/tests/web/editor-markdown-shortcuts.spec.js`

- [ ] **Step 1: Create the spec file with shared setup + a smoke test that confirms the editor mounts**

```js
// playwright/tests/web/editor-markdown-shortcuts.spec.js
import { test, expect } from "@playwright/test";
import { goto, getEditor } from "../utils";

/**
 * Markdown shortcuts in the custom contentEditable editor.
 *
 * Spec: docs/superpowers/specs/2026-05-21-editor-markdown-shortcuts-design.md
 *
 * All tests share one draft post — markdown conversions are scoped to the
 * editor's local DOM, so we don't need a fresh post per test. Each test
 * resets the editor body via evaluate() before driving keystrokes.
 */
test.describe.serial("Editor markdown shortcuts", () => {
  let postId;

  test.beforeAll(async ({ request }) => {
    const res = await request.post("/api/posts", { data: {} });
    expect(res.ok(), `create draft post: ${await res.text()}`).toBeTruthy();
    const pk = (await res.json()).post_pk;
    postId = pk.includes("#") ? pk.split("#")[1] : pk;
    expect(postId).toBeTruthy();
  });

  async function openEditor(page) {
    await goto(page, `/posts/${postId}/edit`);
    const editor = await getEditor(page);
    // Reset to a clean empty paragraph so every test starts from the same DOM.
    await page.evaluate(() => {
      const ed = document.querySelector(".re-content");
      ed.innerHTML = "<p><br></p>";
      const r = document.createRange();
      r.setStart(ed.firstChild, 0);
      r.collapse(true);
      const sel = window.getSelection();
      sel.removeAllRanges();
      sel.addRange(r);
      ed.focus();
    });
    return editor;
  }

  test("Smoke: editor mounts and accepts input", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("hello");
    await expect(editor).toContainText("hello");
  });
});
```

- [ ] **Step 2: Run the smoke test and verify it PASSES**

Run from `playwright/`:
```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js --headed
```

Expected: 1 passed. Smoke test confirms the spec scaffolding and `openEditor` helper work against the running dev server. If it fails, fix the scaffolding before moving on — no implementation tasks should start until the harness boots.

- [ ] **Step 3: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "test(editor): scaffold markdown shortcuts playwright spec

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 2: Add the markdown-shortcuts module skeleton to script.js

This task adds the helper functions and the new `input` + `keydown` listeners with **no-op bodies** so we have a stable place to plug each feature into. Validates that adding these listeners doesn't break any existing editor behavior.

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js` — insert a new section just before line 495 (the `// Initial paint` comment block at the end of `init(root)`)

- [ ] **Step 1: Open script.js and locate the insertion point**

Find this block near the end of `init(root)`:

```js
    // Initial paint so word/char counts and toolbar state are correct.
    emitChange();
  }
```

The new code is appended **before** `emitChange();` so all listeners are wired before the first paint.

- [ ] **Step 2: Insert the markdown-shortcuts skeleton**

In `app/ratel/src/common/components/editor/script.js`, just above the `// Initial paint` comment inside `init(root)`, add:

```js
    // ── Markdown shortcuts ─────────────────────────────────
    // Notion-style block conversions triggered by typing a marker + space at
    // the start of a block. Sibling lifecycle: lives inside init(root) so it
    // shares `composing`, `editor`, `scheduleUpdate` with the rest of the
    // editor. See docs/superpowers/specs/2026-05-21-editor-markdown-shortcuts-design.md.

    var BLOCK_TAGS = ["P", "DIV", "H1", "H2", "H3", "H4", "H5", "H6", "BLOCKQUOTE", "PRE", "LI"];
    var SKIP_BLOCK_TAGS = ["H1", "H2", "H3", "H4", "H5", "H6", "PRE", "BLOCKQUOTE"];
    // U+FEFF (BOM/zero-width no-break space) is invisible and wouldn't be
    // typed by a user, so wrapping the unique tag with it guarantees we can
    // round-trip it through innerHTML restoration without colliding with
    // real content.
    var REVERT_SENTINEL = "﻿__RATEL_REVERT_SENTINEL__﻿";

    var lastConversion = null;

    function mdGetCaretBlock() {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var node = sel.getRangeAt(0).startContainer;
      if (node.nodeType === 3) node = node.parentNode;
      while (node && node !== editor) {
        if (BLOCK_TAGS.indexOf(node.nodeName) >= 0) return node;
        node = node.parentNode;
      }
      return editor;
    }

    function mdAncestorTag(tag) {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var node = sel.getRangeAt(0).startContainer;
      if (node.nodeType === 3) node = node.parentNode;
      while (node && node !== editor) {
        if (node.nodeName === tag) return node;
        node = node.parentNode;
      }
      return null;
    }

    function mdHasAncestorTag(tag) {
      return mdAncestorTag(tag) !== null;
    }

    function mdTextBeforeCaretInBlock(blockEl) {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return "";
      var range = sel.getRangeAt(0);
      var clone = range.cloneRange();
      clone.selectNodeContents(blockEl);
      clone.setEnd(range.startContainer, range.startOffset);
      return clone.toString();
    }

    function mdMatchBlockMarker(text) {
      // Longest patterns first so /^### / wins over /^# /.
      if (/^### $/.test(text)) return { kind: "heading", level: 3, markerLen: 4 };
      if (/^## $/.test(text))  return { kind: "heading", level: 2, markerLen: 3 };
      if (/^# $/.test(text))   return { kind: "heading", level: 1, markerLen: 2 };
      if (/^[\*\-\+] $/.test(text)) return { kind: "ulist", markerLen: 2 };
      var m = text.match(/^(\d+)\. $/);
      if (m) return { kind: "olist", markerLen: m[0].length };
      if (/^> $/.test(text)) return { kind: "blockquote", markerLen: 2 };
      if (/^(---|\*\*\*) $/.test(text)) return { kind: "hr", markerLen: 4 };
      return null;
    }

    function mdDeleteFirstChars(blockEl, count) {
      // Walk forward from the start of blockEl, removing `count` characters of
      // visible text. Returns true on success.
      var walker = document.createTreeWalker(blockEl, NodeFilter.SHOW_TEXT, null);
      var node;
      var remaining = count;
      while ((node = walker.nextNode())) {
        if (node.length >= remaining) {
          node.deleteData(0, remaining);
          return true;
        }
        remaining -= node.length;
        var dead = node;
        // advance walker first, then drop the consumed node
        var next = walker.nextNode();
        dead.parentNode.removeChild(dead);
        if (!next) break;
        node = next;
        if (node.length >= remaining) {
          node.deleteData(0, remaining);
          return true;
        }
        remaining -= node.length;
      }
      return false;
    }

    function mdPlaceCaretAtBlockStart(blockEl) {
      var walker = document.createTreeWalker(blockEl, NodeFilter.SHOW_TEXT, null);
      var firstText = walker.nextNode();
      var sel = window.getSelection();
      sel.removeAllRanges();
      var caret = document.createRange();
      if (firstText) {
        caret.setStart(firstText, 0);
      } else {
        caret.setStart(blockEl, 0);
      }
      caret.collapse(true);
      sel.addRange(caret);
    }

    function mdSnapshotForRevert(markerText) {
      // Insert a sentinel text node at the current caret so we can locate
      // and remove it after restoring innerHTML.
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var range = sel.getRangeAt(0);
      var sentinel = document.createTextNode(REVERT_SENTINEL);
      range.insertNode(sentinel);
      var html = editor.innerHTML;
      sentinel.parentNode.removeChild(sentinel);
      return { markerText: markerText, snapshot: html };
    }

    function mdRevert(c) {
      editor.innerHTML = c.snapshot;
      var walker = document.createTreeWalker(editor, NodeFilter.SHOW_TEXT, null);
      var node;
      while ((node = walker.nextNode())) {
        var idx = node.nodeValue.indexOf(REVERT_SENTINEL);
        if (idx >= 0) {
          var before = node.nodeValue.slice(0, idx);
          var after = node.nodeValue.slice(idx + REVERT_SENTINEL.length);
          node.nodeValue = before + after;
          var sel = window.getSelection();
          sel.removeAllRanges();
          var caret = document.createRange();
          caret.setStart(node, idx);
          caret.collapse(true);
          sel.addRange(caret);
          return;
        }
      }
      editor.focus();
    }

    function mdIsLiEmpty(li) {
      // Empty if no visible text and no nested list. A leftover <br> is fine.
      if (li.textContent.replace(/​/g, "").trim() !== "") return false;
      if (li.querySelector("ul, ol")) return false;
      return true;
    }

    function mdTryConvert(inputEvent) {
      // Filled in by Tasks 3–8.
      return false;
    }

    editor.addEventListener("input", function (e) {
      if (composing) return;
      if (e.inputType === "insertText" && e.data === " ") {
        if (mdTryConvert(e)) return;
      }
      // Any input that wasn't a successful conversion disarms revert.
      lastConversion = null;
    });

    editor.addEventListener("keydown", function (e) {
      // Filled in by Tasks 5, 7, 9.
    });
```

- [ ] **Step 3: Run the smoke test from Task 1 to confirm nothing broke**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js --headed
```

Expected: 1 passed (the smoke test still passes — the new listeners are no-ops aside from clearing `lastConversion`).

- [ ] **Step 4: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): scaffold markdown-shortcuts module

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 3: Heading conversions (#, ##, ###)

Covers spec tests 1, 2, 3, 15 (preserves existing line content), 17 (mid-line marker not triggered), 18 (in-heading marker not double-applied).

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js` — flesh out `mdTryConvert`
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js` — add tests

- [ ] **Step 1: Write failing Playwright tests for headings**

Append inside the `test.describe.serial(...)` block, after the smoke test:

```js
  test("# → H1 (empty paragraph)", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("# ");
    await page.keyboard.type("Title");
    await expect(editor.locator("h1")).toHaveText("Title");
  });

  test("## → H2", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("## Heading2");
    await expect(editor.locator("h2")).toHaveText("Heading2");
  });

  test("### → H3", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("### Sub");
    await expect(editor.locator("h3")).toHaveText("Sub");
  });

  test("# prepended to existing text preserves the rest of the line", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("Hello world");
    await page.keyboard.press("Home");
    await page.keyboard.type("# ");
    await expect(editor.locator("h1")).toHaveText("Hello world");
  });

  test("Mid-line # is not converted", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("abc# x");
    await expect(editor.locator("h1")).toHaveCount(0);
    await expect(editor).toContainText("abc# x");
  });

  test("# inside an existing H1 is typed literally", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("# Title");
    await expect(editor.locator("h1")).toHaveText("Title");
    await page.keyboard.press("Home");
    await page.keyboard.type("# ");
    await expect(editor.locator("h1")).toHaveText("# Title");
    await expect(editor.locator("h1 h1")).toHaveCount(0);
  });
```

- [ ] **Step 2: Run the new tests — they should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "H1|H2|H3|prepended|Mid-line|inside an existing H1"
```

Expected: 6 failed (markers are still typed literally because `mdTryConvert` is a no-op).

- [ ] **Step 3: Implement heading branch in `mdTryConvert`**

Replace the stub body of `mdTryConvert` in `script.js` with:

```js
    function mdTryConvert(inputEvent) {
      var block = mdGetCaretBlock();
      if (!block || block === editor) return false;
      // Already a special block? Skip.
      if (SKIP_BLOCK_TAGS.indexOf(block.nodeName) >= 0) return false;
      // Inside a <pre> ancestor? Skip.
      if (mdHasAncestorTag("PRE")) return false;
      // Inside an existing <li>? Skip (avoid auto-converting bullets typed
      // inside a list as nested lists; users can use Tab for that).
      if (mdHasAncestorTag("LI")) return false;

      var before = mdTextBeforeCaretInBlock(block);
      var info = mdMatchBlockMarker(before);
      if (!info) return false;

      var snap = mdSnapshotForRevert(before);

      // Strip the marker chars from the start of the block.
      if (!mdDeleteFirstChars(block, info.markerLen)) {
        // If anything went wrong, abort without applying the format.
        return false;
      }
      // Re-anchor selection at the start of the (shortened) block.
      mdPlaceCaretAtBlockStart(block);

      editor.focus();
      switch (info.kind) {
        case "heading":
          document.execCommand("formatBlock", false, "<H" + info.level + ">");
          break;
        // Other kinds added in later tasks.
        default:
          return false;
      }

      lastConversion = snap;
      scheduleUpdate();
      return true;
    }
```

- [ ] **Step 4: Re-run the heading tests — they should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "H1|H2|H3|prepended|Mid-line|inside an existing H1"
```

Expected: 6 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): markdown heading shortcuts (#, ##, ###)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 4: Bullet and ordered list conversions (-, *, +, 1.)

Covers spec tests 4, 5, 6.

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js` — extend `mdTryConvert` switch
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js` — add list tests

- [ ] **Step 1: Write failing tests for list conversions**

Append after the heading tests:

```js
  test("- → <ul>", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("- Apple");
    await page.keyboard.press("Enter");
    await page.keyboard.type("Banana");
    await expect(editor.locator("ul > li")).toHaveCount(2);
    await expect(editor.locator("ul > li").nth(0)).toHaveText("Apple");
    await expect(editor.locator("ul > li").nth(1)).toHaveText("Banana");
  });

  test("* → <ul> (asterisk variant)", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("* item");
    await expect(editor.locator("ul > li")).toHaveText("item");
  });

  test("+ → <ul> (plus variant)", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("+ item");
    await expect(editor.locator("ul > li")).toHaveText("item");
  });

  test("1. → <ol>", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("1. first");
    await page.keyboard.press("Enter");
    await page.keyboard.type("second");
    await expect(editor.locator("ol > li")).toHaveCount(2);
    await expect(editor.locator("ol > li").nth(0)).toHaveText("first");
    await expect(editor.locator("ol > li").nth(1)).toHaveText("second");
  });

  test("7. → <ol> (custom number is discarded, list starts at 1)", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("7. seven");
    await expect(editor.locator("ol > li")).toHaveText("seven");
    // No `start=` attribute support — verify the ol does not carry a start
    // attribute that would visually number it differently.
    const start = await editor.locator("ol").getAttribute("start");
    expect(start).toBeNull();
  });
```

- [ ] **Step 2: Run new tests — they should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<ul>|<ol>"
```

Expected: 5 failed.

- [ ] **Step 3: Extend the `mdTryConvert` switch with list cases**

In `script.js`, expand the `switch (info.kind)` block so the full switch reads:

```js
      switch (info.kind) {
        case "heading":
          document.execCommand("formatBlock", false, "<H" + info.level + ">");
          break;
        case "ulist":
          document.execCommand("insertUnorderedList", false);
          break;
        case "olist":
          document.execCommand("insertOrderedList", false);
          break;
        default:
          return false;
      }
```

- [ ] **Step 4: Re-run the list tests — they should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<ul>|<ol>"
```

Expected: 5 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): markdown list shortcuts (-, *, +, 1.)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 5: Tab/Shift+Tab list nesting + Enter on empty `<li>` exits list

Covers spec tests 7, 8, 9.

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js` — flesh out the empty `keydown` handler

- [ ] **Step 1: Write failing tests**

Append after the list tests:

```js
  test("Tab inside <li> nests deeper", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("- Outer");
    await page.keyboard.press("Enter");
    await page.keyboard.press("Tab");
    await page.keyboard.type("Inner");
    await expect(editor.locator("ul > li > ul > li")).toHaveText("Inner");
  });

  test("Shift+Tab un-nests back one level", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("- Outer");
    await page.keyboard.press("Enter");
    await page.keyboard.press("Tab");
    await page.keyboard.type("Inner");
    await page.keyboard.press("Enter");
    await page.keyboard.down("Shift");
    await page.keyboard.press("Tab");
    await page.keyboard.up("Shift");
    await page.keyboard.type("Back");
    // After Shift+Tab we should be back at depth 1 (sibling of "Outer").
    await expect(editor.locator("ul > li")).toContainText(["Outer", "Back"]);
  });

  test("Enter on an empty <li> exits the list", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("- Item");
    await page.keyboard.press("Enter");
    // Now in a fresh empty <li>; pressing Enter again should exit.
    await page.keyboard.press("Enter");
    await page.keyboard.type("After");
    await expect(editor.locator("ul > li")).toHaveCount(1);
    await expect(editor.locator("ul > li")).toHaveText("Item");
    // "After" lives in a <p> (or whatever the browser picks) outside the list.
    const afterInUl = await editor.locator("ul").innerText();
    expect(afterInUl).not.toContain("After");
    await expect(editor).toContainText("After");
  });
```

- [ ] **Step 2: Run the new tests — they should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "Tab inside|un-nests|empty <li> exits"
```

Expected: 3 failed.

- [ ] **Step 3: Implement Tab / Shift+Tab / Enter-exit-empty-li in the keydown handler**

Replace the empty keydown handler at the bottom of the markdown-shortcuts section with:

```js
    editor.addEventListener("keydown", function (e) {
      // Tab / Shift+Tab inside a list item: nest deeper / un-nest.
      if (e.key === "Tab" && !e.metaKey && !e.ctrlKey && !e.altKey) {
        if (mdAncestorTag("LI")) {
          e.preventDefault();
          if (e.shiftKey) document.execCommand("outdent", false);
          else            document.execCommand("indent", false);
          scheduleUpdate();
          return;
        }
      }

      // Enter on an empty <li>: outdent (Notion-style exit-list).
      if (e.key === "Enter" && !e.shiftKey && !e.metaKey && !e.ctrlKey && !e.altKey) {
        var li = mdAncestorTag("LI");
        if (li && mdIsLiEmpty(li)) {
          e.preventDefault();
          document.execCommand("outdent", false);
          // If still inside a list (some browsers leave nested wrappers),
          // force a fresh paragraph.
          if (mdAncestorTag("LI")) {
            document.execCommand("formatBlock", false, "<P>");
          }
          scheduleUpdate();
          return;
        }
      }
    });
```

- [ ] **Step 4: Re-run the Tab / Shift+Tab / Enter-exit tests — they should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "Tab inside|un-nests|empty <li> exits"
```

Expected: 3 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): list nesting via Tab/Shift+Tab + Enter-to-exit empty li

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 6: Blockquote (`>`)

Covers spec test 10.

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js`
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js`

- [ ] **Step 1: Write failing test**

Append:

```js
  test("> → <blockquote>", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("> Quoted");
    await expect(editor.locator("blockquote")).toContainText("Quoted");
  });
```

- [ ] **Step 2: Run — should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<blockquote>"
```

Expected: 1 failed.

- [ ] **Step 3: Add blockquote case to the switch**

Extend `mdTryConvert`'s switch:

```js
      switch (info.kind) {
        case "heading":
          document.execCommand("formatBlock", false, "<H" + info.level + ">");
          break;
        case "ulist":
          document.execCommand("insertUnorderedList", false);
          break;
        case "olist":
          document.execCommand("insertOrderedList", false);
          break;
        case "blockquote":
          document.execCommand("formatBlock", false, "<BLOCKQUOTE>");
          break;
        default:
          return false;
      }
```

- [ ] **Step 4: Re-run — should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<blockquote>"
```

Expected: 1 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): markdown blockquote shortcut (>)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 7: Code block (` ``` ` + Enter)

Covers spec tests 11, 19. The code block uses **Enter** as the trigger (space is part of code content).

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js`
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js`

- [ ] **Step 1: Write failing tests**

Append:

```js
  test("``` + Enter → <pre>", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("```");
    await page.keyboard.press("Enter");
    await page.keyboard.type("console.log(1)");
    await expect(editor.locator("pre")).toContainText("console.log(1)");
  });

  test("- inside <pre> is literal, not a list", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("```");
    await page.keyboard.press("Enter");
    await page.keyboard.type("- not a list");
    await expect(editor.locator("pre")).toContainText("- not a list");
    await expect(editor.locator("ul")).toHaveCount(0);
  });
```

- [ ] **Step 2: Run — should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<pre>|inside <pre>"
```

Expected: 2 failed.

- [ ] **Step 3: Add the Enter-on-``` branch to the keydown handler**

Inside the `keydown` listener, append a new branch **after** the empty-li exit branch but still inside the same listener function:

```js
      // ``` + Enter → <pre>
      if (e.key === "Enter" && !e.shiftKey && !e.metaKey && !e.ctrlKey && !e.altKey) {
        var block = mdGetCaretBlock();
        if (block && block !== editor && block.textContent === "```") {
          e.preventDefault();
          // Snapshot for revert covers the to-be-emptied marker block.
          var snap = mdSnapshotForRevert("```");
          // Clear the block's text, then format it as PRE.
          while (block.firstChild) block.removeChild(block.firstChild);
          block.appendChild(document.createTextNode(""));
          mdPlaceCaretAtBlockStart(block);
          document.execCommand("formatBlock", false, "<PRE>");
          lastConversion = snap;
          scheduleUpdate();
          return;
        }
      }
```

- [ ] **Step 4: Re-run — should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<pre>|inside <pre>"
```

Expected: 2 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): markdown code-block shortcut (\\\`\\\`\\\` + Enter)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 8: Horizontal rule (`---` / `***`)

Covers spec test 12.

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js`
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js`

- [ ] **Step 1: Write failing test**

Append:

```js
  test("--- + space → <hr> followed by an empty paragraph", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("--- ");
    await page.keyboard.type("after");
    await expect(editor.locator("hr")).toHaveCount(1);
    // Typing "after" should land in a <p> AFTER the <hr>.
    const html = await editor.evaluate((el) => el.innerHTML);
    expect(html).toMatch(/<hr[^>]*>\s*<p[^>]*>after<\/p>/);
  });
```

- [ ] **Step 2: Run — should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<hr>"
```

Expected: 1 failed.

- [ ] **Step 3: Add the hr case to the switch**

Replace the `default:` case in `mdTryConvert`'s switch with the `hr` branch before falling through to default:

```js
      switch (info.kind) {
        case "heading":
          document.execCommand("formatBlock", false, "<H" + info.level + ">");
          break;
        case "ulist":
          document.execCommand("insertUnorderedList", false);
          break;
        case "olist":
          document.execCommand("insertOrderedList", false);
          break;
        case "blockquote":
          document.execCommand("formatBlock", false, "<BLOCKQUOTE>");
          break;
        case "hr":
          // Block is now empty (marker was stripped). Replace it with an <hr>
          // and ensure a typeable paragraph follows it.
          document.execCommand("insertHorizontalRule", false);
          var sel2 = window.getSelection();
          if (sel2 && sel2.rangeCount > 0) {
            var ctr = sel2.getRangeAt(0).startContainer;
            // If caret is loose in the editor or inside an empty block, drop
            // a fresh <p> after the <hr> and move caret into it.
            var hr = editor.querySelector("hr:last-of-type");
            if (hr) {
              var nextBlock = hr.nextElementSibling;
              if (!nextBlock || (nextBlock.nodeName !== "P" && nextBlock.nodeName !== "DIV")) {
                var p = document.createElement("p");
                p.appendChild(document.createElement("br"));
                hr.parentNode.insertBefore(p, hr.nextSibling);
                nextBlock = p;
              }
              var caret = document.createRange();
              caret.setStart(nextBlock, 0);
              caret.collapse(true);
              sel2.removeAllRanges();
              sel2.addRange(caret);
            }
          }
          break;
        default:
          return false;
      }
```

- [ ] **Step 4: Re-run — should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "<hr>"
```

Expected: 1 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): markdown hr shortcut (--- / ***)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 9: Backspace revert + disarm

Covers spec tests 13, 14. The revert state (`lastConversion`) is already maintained by `mdTryConvert` and disarmed by the existing `input` listener; this task wires the Backspace branch and ensures any non-Backspace keypress disarms the revert window before the user's next keystroke proceeds.

**Files:**
- Modify: `app/ratel/src/common/components/editor/script.js`
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js`

- [ ] **Step 1: Write failing tests**

Append:

```js
  test("Backspace immediately after conversion reverts to literal marker", async ({ page }) => {
    const editor = await openEditor(page);
    // Type the marker + space; conversion fires immediately and the empty H1
    // gains focus with caret at position 0.
    await page.keyboard.type("# ");
    // One Backspace within the revert window restores the literal "# ".
    await page.keyboard.press("Backspace");
    await expect(editor.locator("h1")).toHaveCount(0);
    await expect(editor).toContainText("# ");
  });

  test("Typing any character after conversion disarms the revert (Backspace then = normal delete)", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("# ");
    await page.keyboard.type("a");
    await expect(editor.locator("h1")).toHaveText("a");
    await page.keyboard.press("Backspace");
    // Backspace should just delete the "a", leaving an empty H1.
    await expect(editor.locator("h1")).toBeVisible();
    await expect(editor.locator("h1")).toHaveText("");
  });
```

- [ ] **Step 2: Run — should FAIL**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "Backspace immediately|disarms the revert"
```

Expected: 2 failed (Backspace deletes a char instead of reverting; or revert may not fire at all).

- [ ] **Step 3: Add the Backspace-revert branch at the TOP of the keydown handler**

Edit the keydown listener so it starts with the revert branch:

```js
    editor.addEventListener("keydown", function (e) {
      // Backspace immediately after a conversion → revert to the literal marker.
      if (
        e.key === "Backspace" &&
        lastConversion &&
        !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey
      ) {
        e.preventDefault();
        mdRevert(lastConversion);
        lastConversion = null;
        scheduleUpdate();
        return;
      }

      // Any non-modifier key that isn't Backspace disarms the revert window.
      // (Backspace itself is handled above; modifier-only events shouldn't
      // collapse a pending revert.)
      var isModifierOnly =
        e.key === "Shift" || e.key === "Control" ||
        e.key === "Alt"   || e.key === "Meta"    ||
        e.key === "Process";  // IME composition placeholder
      if (e.key !== "Backspace" && !isModifierOnly) {
        lastConversion = null;
      }

      // (Tab / Shift+Tab / Enter-on-empty-li / ```+Enter branches as before)
      // ... existing branches stay below ...
    });
```

Keep the previously added Tab/Enter branches **below** this new revert/disarm prelude.

- [ ] **Step 4: Re-run — should PASS**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "Backspace immediately|disarms the revert"
```

Expected: 2 passed.

- [ ] **Step 5: Commit**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add app/ratel/src/common/components/editor/script.js playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "feat(editor): one-step Backspace revert for markdown conversions

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 10: IME / Korean composition safety

Covers spec test 16. This is a guard test — `composing` flag already gates `mdTryConvert`; we just need to verify it.

**Files:**
- Modify: `playwright/tests/web/editor-markdown-shortcuts.spec.js`

- [ ] **Step 1: Write failing-or-passing IME test**

Append:

```js
  test("No conversion fires while IME composition is active", async ({ page }) => {
    await openEditor(page);
    // Simulate the start of an IME composition session by dispatching
    // compositionstart directly on the editor — the script listens on
    // .re-content and flips `composing = true`.
    await page.evaluate(() => {
      const ed = document.querySelector(".re-content");
      ed.dispatchEvent(new CompositionEvent("compositionstart"));
    });
    // Now type "# " — the input event fires, but `composing` is true so no
    // conversion should happen.
    await page.keyboard.type("# ");
    await expect(page.locator(".re-content h1")).toHaveCount(0);
    await expect(page.locator(".re-content")).toContainText("# ");
    // End the composition session so the editor returns to a clean state.
    await page.evaluate(() => {
      const ed = document.querySelector(".re-content");
      ed.dispatchEvent(new CompositionEvent("compositionend"));
    });
  });
```

- [ ] **Step 2: Run — should PASS already (the input listener guards on `composing`)**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js -g "IME composition"
```

Expected: 1 passed. If it fails, the guard at the top of the input listener is wrong — fix `if (composing) return;` to live before any conversion attempt.

- [ ] **Step 3: Commit (test-only)**

```bash
/usr/bin/git -C "$(git rev-parse --show-toplevel)" add playwright/tests/web/editor-markdown-shortcuts.spec.js
/usr/bin/git -C "$(git rev-parse --show-toplevel)" commit -m "test(editor): regression test for IME composition safety

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

### Task 11: Build verification + full Playwright sweep

The repo has no prettier configuration, so there is no JS-formatting step — match the existing 2-space indent / single-quote-free / function-statement style already used in `script.js` while writing the code in earlier tasks.

**Files:** none modified — verification only.

- [ ] **Step 1: Verify the Dioxus build still passes (script.js is referenced via `asset!()`, but a syntax error inside it can still break the dev/server pipeline)**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

Expected: build passes with no warnings. If a Rust file complains, the change is unrelated to script.js — fix it before proceeding.

- [ ] **Step 2: Run the full markdown-shortcuts spec end-to-end**

```bash
cd playwright && npx playwright test tests/web/editor-markdown-shortcuts.spec.js
```

Expected: all tests pass (smoke + 6 heading + 5 list + 3 nest/exit + 1 blockquote + 2 code + 1 hr + 2 revert + 1 IME = **21 passing**).

- [ ] **Step 3: Run the rest of the Playwright suite to catch regressions in other editor consumers**

```bash
cd playwright && CI=true npx playwright test
```

Expected: every spec passes (in particular `arena-action-editor.spec.js`, `cross-posting.spec.js`, `discussion-comment-deep-link.spec.js`, `post-category-search-input.spec.js`, `space-publish-invitation.spec.js`, `space-scenario.spec.js`, `sub-team.spec.js`, `team-draft-timeline.spec.js`, `team-space-full-lifecycle.spec.js` all touch the editor).

If any spec regresses, the most likely culprits are:
- The new `keydown` Backspace branch swallowing a Backspace that another test expected to delete a char → check `lastConversion` is null at test entry
- The new `input` listener clearing state another test relied on → check it only clears `lastConversion` (no other vars)
- Tab being preventDefaulted when no `<li>` is the ancestor → confirm the `if (mdAncestorTag("LI"))` guard wraps the preventDefault

- [ ] **Step 4: No additional commit needed (no file changes in this task).**

---

## Spec coverage matrix

| Spec § / test # | Task |
|---|---|
| §1 Block conversions: `#` `##` `###` | 3 |
| §1 Block conversions: `* - +` | 4 |
| §1 Block conversions: `1.` | 4 |
| §1 Block conversions: `>` | 6 |
| §1 Block conversions: ` ``` ` | 7 |
| §1 Block conversions: `--- ` / `*** ` | 8 |
| §1 List behaviors: Tab / Shift+Tab | 5 |
| §1 List behaviors: Enter on empty `<li>` | 5 |
| §2 Trigger only at block start | 3 (test "Mid-line # is not converted") |
| §2 Works on a block with existing content | 3 (test "# prepended to existing text…") |
| §3 IME safety | 10 |
| §4 Backspace revert + disarm | 9 |
| §5 Architecture (all logic in `init(root)` closure of script.js) | 2 (scaffolding) — preserved by 3-9 |
| §6 Edge case: marker inside heading/code | 3 (H1 test), 7 (inside `<pre>` test) |
| §6 Edge case: marker inside existing `<li>` | 4 (the `if (mdHasAncestorTag("LI")) return false;` guard in `mdTryConvert`) |
| §6 Edge case: custom number `7.` discarded | 4 |
| §6 Edge case: pasted markdown not converted | implicit — `input` listener only triggers on `insertText` + ` `, paste fires `insertFromPaste` |
| §7 All 19 scenarios | covered across tasks 1-10; final sweep in 11 |

Note: spec §6 mentions "Multiple paragraphs selected when marker typed" — typing a character while a non-collapsed selection is active replaces the selection in a single `input` event with `inputType === "insertReplacementText"` (or `insertText` after deletion); the `mdMatchBlockMarker` regex won't match against arbitrary multi-block selection text, so no special branch is needed. If a future user report shows this firing incorrectly, add an `if (sel.isCollapsed === false) return false;` guard in `mdTryConvert`.
