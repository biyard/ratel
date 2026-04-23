import { test, expect } from "@playwright/test";
import { click, fill, goto, getEditor, waitForHydrated } from "../utils";

/**
 * Post tag input — E2E
 *
 * Originally written against the old `SearchInput`-based category UI (testids
 * `category-search-input`, `search-input-field`, `search-input-tag`). The
 * post-edit renewal replaced that component with an inline `.tag-input`
 * container; tags are now added via Enter and removed by per-tag X button.
 *
 * This suite verifies the renewed tag UI via testids exposed by the
 * component: add via Enter, render as styled badges with per-tag remove
 * buttons, and remove via the X button.
 */

test.describe.serial("Post tag input (post-edit renewal)", () => {
  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const postTitle = `Tag Test ${uniqueId}`;
  const categoryA = `CatA${uniqueId.slice(-6)}`;
  const categoryB = `CatB${uniqueId.slice(-6)}`;

  let postEditUrl;

  // Add a tag via the tag input; waits for hydration so the Dioxus
  // `oninput`/`onkeydown` handlers are bound before events are dispatched,
  // and waits for the DOM value to settle so the keydown handler reads the
  // updated signal instead of an empty string.
  async function addTag(page, value) {
    await waitForHydrated(page, "tag-input-field");
    await fill(page, { testId: "tag-input-field" }, value);
    const input = page.getByTestId("tag-input-field");
    await expect(input).toHaveValue(value);
    await input.press("Enter");
  }

  // --- 1. Create a draft post and land on its edit page ---

  test("Create a draft post for tag testing", async ({ page }) => {
    await goto(page, "/");

    await click(page, { testId: "home-btn-create" });
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null,
    );
    postEditUrl = new URL(page.url()).pathname;

    await fill(page, { placeholder: "Title your post…" }, postTitle);

    const editor = await getEditor(page);
    await editor.fill(
      "This is test content for the tag input E2E test. It needs to be long enough to pass server-side validation.",
    );

    await expect(page.getByText("All changes saved")).toBeVisible({
      timeout: 15000,
    });
  });

  // --- 2. Add a tag by typing and pressing Enter ---

  test("Add a tag by typing and pressing Enter", async ({ page }) => {
    await goto(page, postEditUrl);

    await addTag(page, categoryA);

    const tags = page.getByTestId("post-tag");
    await expect(tags).toHaveCount(1);
    await expect(tags.first()).toContainText(categoryA);
  });

  // --- 3. Add a second tag — categories are not autosaved ---

  test("Add a second tag via Enter", async ({ page }) => {
    await goto(page, postEditUrl);

    const tags = page.getByTestId("post-tag");

    // Tags are not autosaved, so re-add the first tag in this session.
    await addTag(page, categoryA);
    await expect(tags).toHaveCount(1);

    await addTag(page, categoryB);
    await expect(tags).toHaveCount(2);
    await expect(tags.filter({ hasText: categoryA })).toHaveCount(1);
    await expect(tags.filter({ hasText: categoryB })).toHaveCount(1);
  });

  // --- 4. Verify each tag has a remove (X) button ---

  test("Tag badges render with remove buttons", async ({ page }) => {
    await goto(page, postEditUrl);

    await addTag(page, categoryA);

    const tags = page.getByTestId("post-tag");
    await expect(tags).toHaveCount(1);

    const removeButtons = page.getByTestId("post-tag-remove");
    await expect(removeButtons).toHaveCount(1);
    await expect(removeButtons.first()).toBeVisible();
  });

  // --- 5. Remove a tag by clicking its X button ---

  test("Remove a tag by clicking its X button", async ({ page }) => {
    await goto(page, postEditUrl);

    await addTag(page, categoryA);

    const tags = page.getByTestId("post-tag");
    await expect(tags).toHaveCount(1);

    await page.getByTestId("post-tag-remove").first().click();

    await expect(tags).toHaveCount(0);
  });
});
