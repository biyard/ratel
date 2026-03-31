import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor } from "../utils";

/**
 * Post Category SearchInput — E2E
 *
 * Verifies the SearchInput component used for category selection in the post
 * edit page (issue #1342). Categories should appear as styled badges with X
 * buttons inside the input box, not as unstyled text above it.
 *
 * Test flow:
 *   1. Create a draft post
 *   2. Type a category name and press Enter to add it as a tag
 *   3. Verify the tag appears as a badge inside the SearchInput
 *   4. Add a second category via comma separator
 *   5. Verify both tags are displayed
 *   6. Remove a tag by clicking its X button
 *   7. Verify the tag is removed
 *
 * Test IDs used:
 *   - data-testid="category-search-input"    — SearchInput outer container
 *   - data-testid="search-input-field"       — the text input inside SearchInput
 *   - data-testid="search-input-tag"         — each tag badge wrapper
 *   - data-testid="search-input-tag-remove"  — the X button on each tag
 */

test.describe.serial("Post category SearchInput", () => {
  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const postTitle = `Category Test ${uniqueId}`;
  const categoryA = `CatA${uniqueId.slice(-6)}`;
  const categoryB = `CatB${uniqueId.slice(-6)}`;

  let postEditUrl;

  // --- 1. Create a draft post and navigate to its edit page ---

  test("Create a draft post for category testing", async ({ page }) => {
    await goto(page, "/");

    // Click Create Post to start a new post
    await click(page, { text: "Create Post" });

    // Wait for post edit page
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });
    postEditUrl = new URL(page.url()).pathname;

    // Fill in the title
    await fill(page, { placeholder: "Title" }, postTitle);

    // Fill in content so autosave triggers
    const editor = await getEditor(page);
    await editor.fill(
      "This is test content for the category SearchInput E2E test. It needs to be long enough to pass validation."
    );

    // Wait for autosave
    await expect(page.getByText("All changes saved")).toBeVisible({
      timeout: 15000,
    });
  });

  // --- 2. Add a category by typing and pressing Enter ---

  test("Add a category tag by typing and pressing Enter", async ({ page }) => {
    await goto(page, postEditUrl);

    // Locate the SearchInput container
    const searchInput = await getLocator(page, {
      testId: "category-search-input",
    });
    await expect(searchInput).toBeVisible();

    // Type a category name in the input field
    const inputField = searchInput.getByTestId("search-input-field");
    await inputField.fill(categoryA);

    // Press Enter to add the tag
    await inputField.press("Enter");

    // Verify the tag appears as a badge inside the SearchInput
    const tags = searchInput.getByTestId("search-input-tag");
    await expect(tags).toHaveCount(1);
    await expect(tags.first()).toContainText(categoryA);
  });

  // --- 3. Add a second category using comma separator ---

  test("Add a second category tag using comma", async ({ page }) => {
    await goto(page, postEditUrl);

    const searchInput = await getLocator(page, {
      testId: "category-search-input",
    });

    const inputField = searchInput.getByTestId("search-input-field");

    // Categories are not auto-saved (only title/content auto-save),
    // so re-add the first category in this session before adding the second.
    await inputField.fill(categoryA);
    await inputField.press("Enter");
    const tags = searchInput.getByTestId("search-input-tag");
    await expect(tags).toHaveCount(1);

    // Type a second category with trailing comma
    await inputField.fill(categoryB + ",");

    // Wait for both tags to appear
    await expect(tags).toHaveCount(2);
    await expect(searchInput).toContainText(categoryA);
    await expect(searchInput).toContainText(categoryB);
  });

  // --- 4. Verify tags are styled as badges (not plain text) ---

  test("Category tags are rendered as badges with remove buttons", async ({
    page,
  }) => {
    await goto(page, postEditUrl);

    const searchInput = await getLocator(page, {
      testId: "category-search-input",
    });

    // Categories are not auto-saved, so add one in this session
    const inputField = searchInput.getByTestId("search-input-field");
    await inputField.fill(categoryA);
    await inputField.press("Enter");

    const tags = searchInput.getByTestId("search-input-tag");
    await expect(tags).toHaveCount(1);

    // Each tag should have an X (remove) button
    const count = await tags.count();
    for (let i = 0; i < count; i++) {
      const removeBtn = tags.nth(i).getByTestId("search-input-tag-remove");
      await expect(removeBtn).toBeVisible();
    }
  });

  // --- 5. Remove a category by clicking its X button ---

  test("Remove a category tag by clicking the X button", async ({ page }) => {
    await goto(page, postEditUrl);

    const searchInput = await getLocator(page, {
      testId: "category-search-input",
    });

    // Categories are not auto-saved, so add one in this session
    const inputField = searchInput.getByTestId("search-input-field");
    await inputField.fill(categoryA);
    await inputField.press("Enter");

    const tags = searchInput.getByTestId("search-input-tag");
    await expect(tags).toHaveCount(1);

    // Click the X button on the first tag
    const firstRemoveBtn = tags.first().getByTestId("search-input-tag-remove");
    await firstRemoveBtn.click();

    // Verify the tag is removed
    await expect(tags).toHaveCount(0);
  });
});
