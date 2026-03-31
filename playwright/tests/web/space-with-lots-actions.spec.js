import { test } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor } from "../utils";

test.describe.serial("Space with lots actions", () => {
  let spaceUrl;
  const postTitle = "Why Personal Knowledge Should Become AI-Readable Memory";
  const postContents =
    "Introduction\n\nFor centuries, knowledge has been stored in books, articles, and databases. However, most of this information is written for humans, not for machines.\n\nAs AI agents become increasingly integrated into everyday work, a new gap becomes clear: human knowledge is abundant, but machine-readable knowledge is scarce.\n\nRatel proposes a new concept called Essence — a unit of knowledge designed to be both human-understandable and AI-retrievable.\n\nThe Problem: Knowledge That AI Cannot Use\n\nMost knowledge platforms today are optimized for publishing content rather than structuring it for reasoning and retrieval.\n\nTypical formats include blogs, research papers, and long articles. While these formats are readable, they are not ideal for AI systems.\n\nSeveral issues commonly appear:\n\nImportant insights are buried deep inside long paragraphs\n\nArticles mix multiple ideas within a single document\n\nRelationships between concepts are not explicitly defined\n\nKnowledge is difficult to reuse in smaller units\n\nBecause of this, AI systems must rely heavily on approximate embeddings and summarization, which can lead to shallow or inaccurate responses.\n\nThe Concept of Knowledge Essence\n\nAn Essence is a minimal unit of knowledge containing one clear idea, explanation, or insight.\n\nInstead of storing knowledge only as long documents, information can be broken down into atomic units that are easier for both humans and machines to understand.\n\nAn Essence typically has three important properties:\n\nAtomic\nEach essence focuses on a single concept or claim.\n\nStructured\nInformation follows a predictable structure so it can be interpreted consistently.\n\nComposable\nMultiple essences can connect together to form larger knowledge networks.\n\nThis model treats knowledge more like a neural system than a static document.\n\nFrom Documents to Neural Knowledge\n\nTraditional knowledge systems follow a document-centric structure.\n\nA document contains sections, and sections contain paragraphs. Meaning is spread across the entire text.\n\nIn contrast, the Ratel model treats knowledge as a network of connected ideas.\n\nEach essence becomes a node in a knowledge graph. AI agents can retrieve these nodes individually and combine them to produce more precise answers.\n\nThis improves:\n\nretrieval accuracy\n\nknowledge reuse\n\nexplainability of AI responses\n\nAI Agents and the Future of Personal Knowledge\n\nIn the near future, individuals will work alongside personal AI agents that help with research, writing, and decision making.\n\nFor these agents to be truly useful, they must have access to high-quality human knowledge.\n\nThis includes:\n\npersonal insights and experiences\n\nprofessional expertise\n\nverified evidence\n\ncommunity opinions\n\nRatel allows users to publish their knowledge in a format that AI agents can directly retrieve and use.\n\nInstead of searching through unstructured internet content, AI agents can query curated networks of human knowledge.\n\nToward a Collective Intelligence Network\n\nWhen many people contribute essences, a new form of knowledge infrastructure emerges.\n\nThis system can include several types of essences:\n\nKnowledge Essence\nExpert knowledge, deep explanations, and professional insights.\n\nResponse Essence\nCommunity opinions, surveys, and collective judgments.\n\nEvidence Essence\nVerified facts, references, and supporting data.\n\nTogether, these layers form a collaborative knowledge network where experts contribute ideas, communities evaluate them, and AI agents synthesize the results.\n\nConclusion\n\nThe future of knowledge is not only about publishing information. It is about structuring knowledge so that both humans and AI can reason with it.\n\nRatel introduces a new paradigm: knowledge as AI-readable memory.\n\nBy turning ideas into essences, we can build a global network of structured knowledge that powers the next generation of intelligent systems.";

  async function createSpaceFromPost(page) {
    await goto(page, "/");

    await click(page, { label: "Create Post" });
    await fill(page, { placeholder: "Title" }, postTitle);
    await click(page, { testId: "skip-space-checkbox" });

    const editor = await getEditor(page);
    await editor.fill(postContents);

    await click(page, { text: "Go to Space" });
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
      waitUntil: "networkidle",
    });
    await getLocator(page, { text: "Dashboard" });

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/dashboard$/, "");
  }

  test.beforeAll(async ({ browser }) => {
    const context = await browser.newContext({ storageState: "user.json" });
    const page = await context.newPage();

    await createSpaceFromPost(page);

    await context.close();
  });

  test("Create a post", async ({ page }) => {
    await goto(page, `${spaceUrl}/dashboard`);
    await getLocator(page, { text: "Dashboard" });
  });

  test("Create a discussion in space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // open create action modal
    await click(page, { text: "Select Action Type" });
    // select Discussion
    await click(page, { testId: "action-type-discussion" });
    // hide FAB that overlaps modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
    // confirm creation
    await click(page, { text: "Create" });

    // wait for discussion page (creator sees inline editor directly)
    await page.waitForURL(/\/actions\/discussions\//, {
      waitUntil: "networkidle",
    });

    // fill discussion fields on CreatorMain (inline editing)
    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Playwright Discussion Topic"
    );
    await fill(
      page,
      { placeholder: "Enter category (optional)..." },
      "Testing"
    );

    // fill rich text content via TiptapEditor
    const editor = await getEditor(page);
    await editor.fill(
      "This is a test discussion created by Playwright to verify the discussion creation flow within a space."
    );

    await click(page, { text: "Save" });
  });

  test("Create a poll action in space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // open create action modal
    await click(page, { text: "Select Action Type" });
    // select Poll (Quiz is default, so click Poll)
    await click(page, { testId: "action-type-poll" });
    // hide FAB that overlaps modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
    // confirm creation
    await click(page, { text: "Create" });

    // wait for poll creator page
    await page.waitForURL(/\/actions\/polls\//, {
      waitUntil: "networkidle",
    });

    // fill poll title (saves on blur)
    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Playwright Poll Question"
    );

    // trigger blur to save title
    await page.keyboard.press("Tab");
    await page.waitForLoadState("networkidle");
  });

  test("Create a quiz action in space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // open create action modal
    await click(page, { text: "Select Action Type" });
    // Quiz is selected by default, no need to click
    // hide FAB that overlaps modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
    // confirm creation
    await click(page, { text: "Create" });

    // wait for quiz creator page
    await page.waitForURL(/\/actions\/quizzes\//, {
      waitUntil: "networkidle",
    });

    // fill quiz title on Overview tab (default tab)
    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Playwright Quiz Challenge"
    );

    // fill rich text description via TiptapEditor
    const editor = await getEditor(page);
    await editor.fill(
      "This is a test quiz created by Playwright to verify the quiz creation flow."
    );

    await click(page, { text: "Save" });
  });

  test("Create a follow action in space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // open create action modal
    await click(page, { text: "Select Action Type" });
    // select Follow (Quiz is default, so click Follow)
    await click(page, { testId: "action-type-follow" });
    // hide FAB that overlaps modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
    // confirm creation
    await click(page, { text: "Create" });

    // wait for follow creator page
    await page.waitForURL(/\/actions\/follows\//, {
      waitUntil: "networkidle",
    });

    // verify creator sees the General tab with follower settings
    await getLocator(page, { text: "General" });
  });

  test("Publish space public", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");

    await click(page, { text: "Publish" });
    await click(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
  });
});

// import { expect, test } from "@playwright/test";
// import { CONFIGS } from "../config";
// import { click, fill, goto, getLocator, getEditor, waitPopup } from "../utils";

// test.describe.serial("Space with lots actions", () => {
//   let spaceUrl;
//   let discussionUrl;
//   let pollUrl;
//   let quizUrl;
//   let followUrl;
//   const postTitle = "Why Personal Knowledge Should Become AI-Readable Memory";
//   const postContents =
//     "Introduction\n\nFor centuries, knowledge has been stored in books, articles, and databases. However, most of this information is written for humans, not for machines.\n\nAs AI agents become increasingly integrated into everyday work, a new gap becomes clear: human knowledge is abundant, but machine-readable knowledge is scarce.\n\nRatel proposes a new concept called Essence — a unit of knowledge designed to be both human-understandable and AI-retrievable.\n\nThe Problem: Knowledge That AI Cannot Use\n\nMost knowledge platforms today are optimized for publishing content rather than structuring it for reasoning and retrieval.\n\nTypical formats include blogs, research papers, and long articles. While these formats are readable, they are not ideal for AI systems.\n\nSeveral issues commonly appear:\n\nImportant insights are buried deep inside long paragraphs\n\nArticles mix multiple ideas within a single document\n\nRelationships between concepts are not explicitly defined\n\nKnowledge is difficult to reuse in smaller units\n\nBecause of this, AI systems must rely heavily on approximate embeddings and summarization, which can lead to shallow or inaccurate responses.\n\nThe Concept of Knowledge Essence\n\nAn Essence is a minimal unit of knowledge containing one clear idea, explanation, or insight.\n\nInstead of storing knowledge only as long documents, information can be broken down into atomic units that are easier for both humans and machines to understand.\n\nAn Essence typically has three important properties:\n\nAtomic\nEach essence focuses on a single concept or claim.\n\nStructured\nInformation follows a predictable structure so it can be interpreted consistently.\n\nComposable\nMultiple essences can connect together to form larger knowledge networks.\n\nThis model treats knowledge more like a neural system than a static document.\n\nFrom Documents to Neural Knowledge\n\nTraditional knowledge systems follow a document-centric structure.\n\nA document contains sections, and sections contain paragraphs. Meaning is spread across the entire text.\n\nIn contrast, the Ratel model treats knowledge as a network of connected ideas.\n\nEach essence becomes a node in a knowledge graph. AI agents can retrieve these nodes individually and combine them to produce more precise answers.\n\nThis improves:\n\nretrieval accuracy\n\nknowledge reuse\n\nexplainability of AI responses\n\nAI Agents and the Future of Personal Knowledge\n\nIn the near future, individuals will work alongside personal AI agents that help with research, writing, and decision making.\n\nFor these agents to be truly useful, they must have access to high-quality human knowledge.\n\nThis includes:\n\npersonal insights and experiences\n\nprofessional expertise\n\nverified evidence\n\ncommunity opinions\n\nRatel allows users to publish their knowledge in a format that AI agents can directly retrieve and use.\n\nInstead of searching through unstructured internet content, AI agents can query curated networks of human knowledge.\n\nToward a Collective Intelligence Network\n\nWhen many people contribute essences, a new form of knowledge infrastructure emerges.\n\nThis system can include several types of essences:\n\nKnowledge Essence\nExpert knowledge, deep explanations, and professional insights.\n\nResponse Essence\nCommunity opinions, surveys, and collective judgments.\n\nEvidence Essence\nVerified facts, references, and supporting data.\n\nTogether, these layers form a collaborative knowledge network where experts contribute ideas, communities evaluate them, and AI agents synthesize the results.\n\nConclusion\n\nThe future of knowledge is not only about publishing information. It is about structuring knowledge so that both humans and AI can reason with it.\n\nRatel introduces a new paradigm: knowledge as AI-readable memory.\n\nBy turning ideas into essences, we can build a global network of structured knowledge that powers the next generation of intelligent systems.";

//   async function createSpaceFromPost(page) {
//     await goto(page, "/");

//     await click(page, { label: "Create Post" });
//     await fill(page, { placeholder: "Title" }, postTitle);
//     await click(page, { testId: "skip-space-checkbox" });

//     const editor = await getEditor(page);
//     await editor.fill(postContents);

//     await click(page, { text: "Go to Space" });
//     await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
//       waitUntil: "networkidle",
//     });
//     await getLocator(page, { text: "Dashboard" });

//     const url = new URL(page.url());
//     spaceUrl = url.pathname.replace(/\/dashboard$/, "");
//   }

//   async function signInAsViewer(browser) {
//     const context = await browser.newContext({
//       storageState: {
//         cookies: [],
//         origins: [],
//       },
//     });
//     const page = await context.newPage();

//     await goto(page, "/");
//     await click(page, { role: "button", text: /sign in/i });
//     await waitPopup(page, { visible: true });
//     await fill(
//       page,
//       { placeholder: "Enter your email address" },
//       "hi+user2@biyard.co"
//     );
//     await click(page, { text: "Continue" });
//     await fill(page, { placeholder: "Enter your password" }, "admin!234");
//     await click(page, { text: "Continue" });
//     await waitPopup(page, { visible: false });

//     return { context, page };
//   }

//   async function gotoActions(page) {
//     await page.goto(`${CONFIGS.BASE_URL}${spaceUrl}/actions`);
//     await page.waitForLoadState("networkidle");
//     await expect(
//       page.getByText("Select Action Type", { exact: true })
//     ).toBeVisible();
//   }

//   test.beforeAll(async ({ browser }) => {
//     const context = await browser.newContext({ storageState: "user.json" });
//     const page = await context.newPage();

//     await createSpaceFromPost(page);

//     await context.close();
//   });

//   test("Create a post", async ({ page }) => {
//     await goto(page, `${spaceUrl}/dashboard`);
//     await getLocator(page, { text: "Dashboard" });
//   });

//   test("Create a discussion in space", async ({ page }) => {
//     await gotoActions(page);

//     // open create action modal
//     await click(page, { text: "Select Action Type" });
//     // select Discussion
//     await click(page, { testId: "action-type-discussion" });
//     // hide FAB that overlaps modal buttons
//     await page.evaluate(() => {
//       const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
//       if (fab) fab.style.display = "none";
//     });
//     // confirm creation
//     await click(page, { text: "Create" });

//     // wait for discussion page (creator sees inline editor directly)
//     await page.waitForURL(/\/actions\/discussions\//, {
//       waitUntil: "networkidle",
//     });
//     discussionUrl = new URL(page.url()).pathname;

//     // fill discussion fields on CreatorMain (inline editing)
//     await fill(
//       page,
//       { placeholder: "Enter discussion title..." },
//       "Playwright Discussion Topic"
//     );
//     await fill(
//       page,
//       { placeholder: "Enter category (optional)..." },
//       "Testing"
//     );

//     // fill rich text content via TiptapEditor
//     const editor = await getEditor(page);
//     await editor.fill(
//       "This is a test discussion created by Playwright to verify the discussion creation flow within a space."
//     );

//     await click(page, { text: "Save" });
//   });

//   test("Create a poll action in space", async ({ page }) => {
//     await gotoActions(page);

//     // open create action modal
//     await click(page, { text: "Select Action Type" });
//     // select Poll (Quiz is default, so click Poll)
//     await click(page, { testId: "action-type-poll" });
//     // hide FAB that overlaps modal buttons
//     await page.evaluate(() => {
//       const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
//       if (fab) fab.style.display = "none";
//     });
//     // confirm creation
//     await click(page, { text: "Create" });

//     // wait for poll creator page
//     await page.waitForURL(/\/actions\/polls\//, {
//       waitUntil: "networkidle",
//     });
//     pollUrl = new URL(page.url()).pathname;

//     // fill poll title (saves on blur)
//     await fill(
//       page,
//       { placeholder: "Enter poll title..." },
//       "Playwright Poll Question"
//     );

//     // trigger save explicitly, then blur
//     await page.keyboard.press("Enter");
//     await page.keyboard.press("Tab");
//     await page.waitForLoadState("networkidle");

//     // add one poll question so viewer page has actual content
//     await click(page, { testId: "poll-add-question" });
//     await click(page, { text: "Single Choice" });
//     await expect(page.getByText("Question 1")).toBeVisible();
//     await expect(page.getByText("Add Option")).toBeVisible();
//   });

//   test("Create a quiz action in space", async ({ page }) => {
//     await gotoActions(page);

//     // open create action modal
//     await click(page, { text: "Select Action Type" });
//     // Quiz is selected by default, no need to click
//     // hide FAB that overlaps modal buttons
//     await page.evaluate(() => {
//       const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
//       if (fab) fab.style.display = "none";
//     });
//     // confirm creation
//     await click(page, { text: "Create" });

//     // wait for quiz creator page
//     await page.waitForURL(/\/actions\/quizzes\//, {
//       waitUntil: "networkidle",
//     });
//     quizUrl = new URL(page.url()).pathname;

//     // fill quiz title on Overview tab (default tab)
//     await fill(
//       page,
//       { placeholder: "Enter quiz title..." },
//       "Playwright Quiz Challenge"
//     );

//     // fill rich text description via TiptapEditor
//     const editor = await getEditor(page);
//     await editor.fill(
//       "This is a test quiz created by Playwright to verify the quiz creation flow."
//     );

//     await click(page, { text: "Save" });

//     // move to Quiz tab and add one quiz question
//     await click(page, { role: "tab", text: "Quiz" });
//     await click(page, { testId: "quiz-add-question" });
//     await click(page, { text: "Single Choice" });
//     await expect(page.getByText("Question 1")).toBeVisible();
//     await expect(page.getByText("Add Option")).toBeVisible();
//   });

//   test("Create a follow action in space", async ({ page }) => {
//     await gotoActions(page);

//     // open create action modal
//     await click(page, { text: "Select Action Type" });
//     // select Follow (Quiz is default, so click Follow)
//     await click(page, { testId: "action-type-follow" });
//     // hide FAB that overlaps modal buttons
//     await page.evaluate(() => {
//       const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
//       if (fab) fab.style.display = "none";
//     });
//     // confirm creation
//     await click(page, { text: "Create" });

//     // wait for follow creator page
//     await page.waitForURL(/\/actions\/follows\//, {
//       waitUntil: "networkidle",
//     });
//     followUrl = new URL(page.url()).pathname;

//     // verify creator sees the General tab with follower settings
//     await getLocator(page, { text: "General" });
//   });

//   test("Publish space public", async ({ page }) => {
//     await goto(page, spaceUrl + "/dashboard");

//     await click(page, { text: "Publish" });
//     await click(page, { testId: "public-option" });
//     await click(page, { label: "Confirm visibility selection" });
//   });

//   // test("viewer can see discussion but cannot comment", async ({ browser }) => {
//   //   const { context, page } = await signInAsViewer(browser);

//   //   await page.goto(`${CONFIGS.BASE_URL}${discussionUrl}`);
//   //   await expect(
//   //     page.getByRole("heading", { name: "Playwright Discussion Topic" })
//   //   ).toBeVisible();
//   //   await expect(
//   //     page.getByPlaceholder("Write a comment...")
//   //   ).toHaveCount(0);
//   //   await expect(page.getByRole("button", { name: "Send" })).toHaveCount(0);

//   //   await context.close();
//   // });

//   test("viewer can see poll but cannot submit", async ({ browser }) => {
//     const { context, page } = await signInAsViewer(browser);

//     await page.goto(`${CONFIGS.BASE_URL}${pollUrl}`);
//     await expect(page.getByText("Option 1")).toBeVisible();
//     await expect(page.getByText("Option 2")).toBeVisible();
//     await expect(page.getByRole("button", { name: /Submit|제출/ })).toHaveCount(
//       0
//     );

//     await context.close();
//   });

//   test("viewer can see quiz but cannot edit or submit", async ({ browser }) => {
//     const { context, page } = await signInAsViewer(browser);

//     await page.goto(`${CONFIGS.BASE_URL}${quizUrl}`);
//     await page.waitForTimeout(2000);
//     const quizTab = page.getByTestId("quiz-viewer-tab").last();
//     await expect(quizTab).toBeVisible();
//     await quizTab.click({ force: true });
//     const activeQuizPanel = page.getByTestId("quiz-viewer-panel").last();
//     await expect(activeQuizPanel.getByText("Option 1")).toBeVisible();
//     await expect(activeQuizPanel.getByText("Option 2")).toBeVisible();
//     await expect(page.getByRole("button", { name: "Save" })).toHaveCount(0);
//     await expect(page.getByRole("button", { name: /Submit|제출/ })).toHaveCount(
//       0
//     );

//     await context.close();
//   });

//   test("viewer can see follow action and can follow users", async ({
//     browser,
//   }) => {
//     const { context, page } = await signInAsViewer(browser);

//     await page.goto(`${CONFIGS.BASE_URL}${followUrl}`);
//     await expect(page.getByText(/Follow Users|팔로우 유저/)).toBeVisible();
//     await expect(page.getByRole("button", { name: /Remove/ })).toHaveCount(0);
//     const followButton = page
//       .getByRole("button", {
//         name: /Follow|팔로우/,
//       })
//       .first();
//     await expect(followButton).toBeVisible();

//     await context.close();
//   });
// });
