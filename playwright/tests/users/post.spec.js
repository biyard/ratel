import { test } from "@playwright/test";
import { waitPopup, click, fill, goto, getLocator, getEditor } from "../utils";

test("Create a post", async ({ page }) => {
  await goto(page, "/");

  await click(page, { label: "Create Post" });
  await fill(page, { placeholder: "Title" }, "Playwright Post");

  const contents =
    "This is a test post created using Playwright. The purpose of this post is to verify that the post creation functionality works correctly. We will fill in the title and content fields, publish the post, and check if it appears as expected. This test ensures that users can successfully create and publish posts on the platform.";

  const editor = await getEditor(page);
  await editor.fill(contents);

  await click(page, { text: "Publish" });
  await click(page, { testId: "public-option" });
  await click(page, { label: "Confirm visibility selection" });
  await getLocator(page, { label: "Create a Space" });
});

test.describe.serial("Post with Space", () => {
  test("Create a post", async ({ page }) => {
    await goto(page, "/");

    await click(page, { label: "Create Post" });
    await fill(
      page,
      { placeholder: "Title" },
      "Why Personal Knowledge Should Become AI-Readable Memory",
    );

    const contents =
      "Introduction\n\nFor centuries, knowledge has been stored in books, articles, and databases. However, most of this information is written for humans, not for machines.\n\nAs AI agents become increasingly integrated into everyday work, a new gap becomes clear: human knowledge is abundant, but machine-readable knowledge is scarce.\n\nRatel proposes a new concept called Essence — a unit of knowledge designed to be both human-understandable and AI-retrievable.\n\nThe Problem: Knowledge That AI Cannot Use\n\nMost knowledge platforms today are optimized for publishing content rather than structuring it for reasoning and retrieval.\n\nTypical formats include blogs, research papers, and long articles. While these formats are readable, they are not ideal for AI systems.\n\nSeveral issues commonly appear:\n\nImportant insights are buried deep inside long paragraphs\n\nArticles mix multiple ideas within a single document\n\nRelationships between concepts are not explicitly defined\n\nKnowledge is difficult to reuse in smaller units\n\nBecause of this, AI systems must rely heavily on approximate embeddings and summarization, which can lead to shallow or inaccurate responses.\n\nThe Concept of Knowledge Essence\n\nAn Essence is a minimal unit of knowledge containing one clear idea, explanation, or insight.\n\nInstead of storing knowledge only as long documents, information can be broken down into atomic units that are easier for both humans and machines to understand.\n\nAn Essence typically has three important properties:\n\nAtomic\nEach essence focuses on a single concept or claim.\n\nStructured\nInformation follows a predictable structure so it can be interpreted consistently.\n\nComposable\nMultiple essences can connect together to form larger knowledge networks.\n\nThis model treats knowledge more like a neural system than a static document.\n\nFrom Documents to Neural Knowledge\n\nTraditional knowledge systems follow a document-centric structure.\n\nA document contains sections, and sections contain paragraphs. Meaning is spread across the entire text.\n\nIn contrast, the Ratel model treats knowledge as a network of connected ideas.\n\nEach essence becomes a node in a knowledge graph. AI agents can retrieve these nodes individually and combine them to produce more precise answers.\n\nThis improves:\n\nretrieval accuracy\n\nknowledge reuse\n\nexplainability of AI responses\n\nAI Agents and the Future of Personal Knowledge\n\nIn the near future, individuals will work alongside personal AI agents that help with research, writing, and decision making.\n\nFor these agents to be truly useful, they must have access to high-quality human knowledge.\n\nThis includes:\n\npersonal insights and experiences\n\nprofessional expertise\n\nverified evidence\n\ncommunity opinions\n\nRatel allows users to publish their knowledge in a format that AI agents can directly retrieve and use.\n\nInstead of searching through unstructured internet content, AI agents can query curated networks of human knowledge.\n\nToward a Collective Intelligence Network\n\nWhen many people contribute essences, a new form of knowledge infrastructure emerges.\n\nThis system can include several types of essences:\n\nKnowledge Essence\nExpert knowledge, deep explanations, and professional insights.\n\nResponse Essence\nCommunity opinions, surveys, and collective judgments.\n\nEvidence Essence\nVerified facts, references, and supporting data.\n\nTogether, these layers form a collaborative knowledge network where experts contribute ideas, communities evaluate them, and AI agents synthesize the results.\n\nConclusion\n\nThe future of knowledge is not only about publishing information. It is about structuring knowledge so that both humans and AI can reason with it.\n\nRatel introduces a new paradigm: knowledge as AI-readable memory.\n\nBy turning ideas into essences, we can build a global network of structured knowledge that powers the next generation of intelligent systems.";
    await click(page, { testId: "skip-space-checkbox" });

    const editor = await getEditor(page);
    await editor.fill(contents);

    await click(page, { text: "Go to Space" });

    // wait for url space/{uuid}/dashboard
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
      waitUntil: "networkidle",
    });
    await getLocator(page, { text: "Dashboard" });
  });
});
