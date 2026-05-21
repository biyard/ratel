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

  test("Setup: create a draft post via REST", async ({ page }) => {
    // page.request inherits the session cookies from storageState
    // (configured in playwright.config.js). The unauthenticated `request`
    // fixture would 401 against /api/posts.
    const res = await page.request.post("/api/posts", { data: {} });
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
    const start = await editor.locator("ol").getAttribute("start");
    expect(start).toBeNull();
  });

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
    // "After" should NOT live inside the <ul>; it should appear in the
    // editor at large (i.e. in the paragraph that the Enter-exit produced).
    await expect(editor.locator("ul")).not.toContainText("After");
    await expect(editor).toContainText("After");
  });

  test("> → <blockquote>", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("> Quoted");
    await expect(editor.locator("blockquote")).toContainText("Quoted");
  });

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

  test("--- + space → <hr> followed by an empty paragraph", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("--- ");
    await page.keyboard.type("after");
    await expect(editor.locator("hr")).toHaveCount(1);
    const html = await editor.evaluate((el) => el.innerHTML);
    expect(html).toMatch(/<hr[^>]*>\s*<p[^>]*>after<\/p>/);
  });

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

  test("Typing any character after conversion disarms the revert", async ({ page }) => {
    const editor = await openEditor(page);
    await page.keyboard.type("# ");
    await page.keyboard.type("a");
    await expect(editor.locator("h1")).toHaveText("a");
    await page.keyboard.press("Backspace");
    // Backspace should just delete the "a", leaving an empty H1.
    await expect(editor.locator("h1")).toBeVisible();
    await expect(editor.locator("h1")).toHaveText("");
  });

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
});
