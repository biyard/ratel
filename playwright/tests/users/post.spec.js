import { test } from "@playwright/test";
import { waitPopup, click, fill, goto, getLocator, getEditor } from "../utils";

test("Create a post", async ({ page }) => {
  await goto(page, "/");

  await click(page, { label: "Create Post" });
  await fill(page, { placeholder: "Title" }, "My Playwright Post");

  const editor = await getEditor(page);
  await editor.fill("This is a post created using Playwright.");

  await click(page, { text: "Publish" });
  await getLocator(page, { label: "Create a Space" });
});
