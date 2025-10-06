import { test } from "@playwright/test";
import { click, fill } from "../utils";

test.describe("Create Post - Authenticated User", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await page.waitForLoadState("networkidle");
  });

  test("should create a general post successfully", async ({ page }) => {
    const testTitle = "Automated Post Creation - E2E";
    const testContent =
      "This is an automated post content created by Playwright E2E. " +
      "The purpose of this is to verify that the post creation functionality " +
      "works correctly from end to end, including title input, content editing, " +
      "auto-save, and final publication. This content is intentionally long to " +
      "meet the minimum character requirements for post publishing.";

    await click(page, { text: "Create Post" });
    await fill(page, { placeholder: "Write a title..." }, testTitle);
    await fill(page, { label: "general-post-editor" }, testContent);

    await click(page, { label: "Publish" });

    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });
  });
});
