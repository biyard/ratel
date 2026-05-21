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
});
