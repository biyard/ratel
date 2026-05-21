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
