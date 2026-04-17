import { expect, test } from "@playwright/test";
import {
  addPollQuestion,
  click,
  commitAutosave,
  fill,
  fillPollQuestion,
  getEditor,
  goto,
  setReward,
  togglePrerequisite,
  waitPopup,
} from "../utils";

// ─────────────────────────────────────────────────────────────────────────────
// Arena action editor — end-to-end coverage for the HTML-first creator pages
// that replaced the legacy per-action tabs. Exercises poll/quiz/discussion/
// follow editors through the arena dashboard entry point (admin-add-action-
// card → TypePickerModal → auto-Creator-role creator page).
// ─────────────────────────────────────────────────────────────────────────────

test.describe.serial("Arena action editor", () => {
  // beforeAll + 4 creator flows + a delete flow comfortably exceed the
  // default 30s test timeout.
  test.setTimeout(180_000);

  let spaceUrl;

  const postTitle = `Arena Action Editor E2E ${Date.now()}`;
  const postContents =
    "This post drives the arena-action-editor Playwright spec. It is long " +
    "enough to satisfy the minimum content length that the backend enforces " +
    "before a space can be spun up from the draft post.";

  async function hideFab(page) {
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
  }

  /**
   * Open the TypePicker from the arena dashboard and create the requested
   * action type. The arena auto-switches admins into the Creator role on
   * pick and navigates to the creator page — there is no intermediate
   * confirmation.
   */
  async function createAction(page, typeKey, urlRegex) {
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: `type-option-${typeKey}` });
    await page.waitForURL(urlRegex, { waitUntil: "load", timeout: 60000 });
  }

  test.beforeAll(async ({ browser }) => {
    const context = await browser.newContext({ storageState: "user.json" });
    const page = await context.newPage();

    try {
      // Create a draft post and upgrade it into a space via REST so the
      // setup does not depend on the post-edit UI.
      const postRes = await page.request.post("/api/posts", { data: {} });
      expect(postRes.ok(), `create post: ${await postRes.text()}`).toBeTruthy();
      const postPk = (await postRes.json()).post_pk;
      const postId = postPk.includes("#") ? postPk.split("#")[1] : postPk;

      await goto(page, `/posts/${postId}/edit`);
      await fill(page, { placeholder: "Title your post…" }, postTitle);
      const editor = await getEditor(page);
      await editor.fill(postContents);
      await expect(page.getByText("All changes saved")).toBeVisible({
        timeout: 30000,
      });

      const spaceRes = await page.request.post("/api/spaces/create", {
        data: { req: { post_id: postId } },
      });
      expect(
        spaceRes.ok(),
        `create space: ${await spaceRes.text()}`,
      ).toBeTruthy();
      const spaceId = (await spaceRes.json()).space_id;
      spaceUrl = `/spaces/${spaceId}`;
    } finally {
      await context.close();
    }
  });

  test("Poll: create, edit title, add question, toggle prerequisite", async ({
    page,
  }) => {
    await createAction(page, "poll", /\/actions\/polls\//);

    // Topbar — arena pieces specific to the editor shell.
    await expect(page.getByTestId("page-card-content")).toBeVisible();
    await expect(page.getByTestId("page-card-config")).toBeVisible();

    // Title autosave on blur.
    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Arena Poll: Architecture",
    );
    await commitAutosave(page);

    // Question + options (two option inputs by default — no "Add Option").
    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title: "Which layer owns caching?",
      options: ["Application", "Edge"],
    });

    // Prerequisite lives in the ConfigCard tile (no tab switch anymore).
    await togglePrerequisite(page);

    // Reward tile — no-op on a free membership. Verified by the helper not
    // throwing when the toggle is absent.
    await setReward(page, 0);
  });

  test("Quiz: create, edit title, add single question with options", async ({
    page,
  }) => {
    await createAction(page, "quiz", /\/actions\/quizzes\//);

    await expect(page.getByTestId("page-card-content")).toBeVisible();
    await expect(page.getByTestId("page-card-questions")).toBeVisible();
    await expect(page.getByTestId("page-card-config")).toBeVisible();

    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Arena Quiz: Fundamentals",
    );

    // Rich text description via Tiptap.
    const editor = await getEditor(page);
    await editor.fill(
      "Covers the arena UI migration fundamentals — topbar, cards, autosave.",
    );
    await commitAutosave(page);

    // Single SingleChoice question with two options. Each input must blur
    // before the next fill so the onblur autosave commits.
    await click(page, { testId: "quiz-question-add" });
    const q0 = page.getByTestId("quiz-question-0");
    const inputs = q0.locator("input.input");
    const fills = [
      "Which pattern replaced the action editor tabs?",
      "HTML-first cards",
      "Nested tab router",
    ];
    for (let i = 0; i < fills.length; i += 1) {
      await inputs.nth(i).fill(fills[i]);
      await inputs.nth(i).press("Tab");
      await page.waitForLoadState("load");
      await page.waitForTimeout(200);
    }
  });

  test("Discussion: create, edit title, fill Tiptap body", async ({ page }) => {
    await createAction(
      page,
      "discuss",
      /\/actions\/discussions\/[^/]+\/edit/,
    );

    await expect(page.getByTestId("page-card-content")).toBeVisible();
    await expect(page.getByTestId("page-card-config")).toBeVisible();
    // Attachments section is part of the ContentCard for the arena
    // discussion editor (no separate upload tab).
    await expect(page.getByTestId("section-attachments")).toBeVisible();

    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Arena Discussion: Feedback",
    );

    const editor = await getEditor(page);
    await editor.fill(
      "Share your feedback on the arena migration, ergonomics, and pain points.",
    );
    await commitAutosave(page);
  });

  test("Follow: create, edit title in TargetsCard", async ({ page }) => {
    await createAction(page, "follow", /\/actions\/follows\//);

    await expect(page.getByTestId("page-card-targets")).toBeVisible();
    await expect(page.getByTestId("page-card-config")).toBeVisible();

    await fill(
      page,
      { testId: "follow-title" },
      "Arena Follow: Maintainers",
    );
    await commitAutosave(page);
  });

  test("Delete: full flow — click, confirm popup, navigate to actions", async ({
    page,
  }) => {
    // Create a throwaway follow action and delete it from its own page.
    await createAction(page, "follow", /\/actions\/follows\//);

    const deleteBtn = page.getByTestId("page-card-config").getByRole("button", {
      name: "Delete",
    });
    await expect(deleteBtn).toBeVisible();
    await deleteBtn.click();

    // Confirm popup — wait for the backdrop, then click Confirm.
    await waitPopup(page, { visible: true });
    await click(page, { text: "Confirm" });

    // Navigates to the Actions list page after delete completes.
    await page.waitForURL(/\/actions\/?$/, {
      waitUntil: "load",
      timeout: 30000,
    });
  });
});
