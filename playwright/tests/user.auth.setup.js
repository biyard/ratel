import { execFileSync } from "node:child_process";
import { test } from "@playwright/test";
import { waitPopup, click, fill, goto } from "./utils";

function seedCreator1MembershipForTests() {
  execFileSync(
    "aws",
    [
      "--endpoint-url=http://localhost:4566",
      "dynamodb",
      "update-item",
      "--table-name",
      "ratel-local-main",
      "--key",
      JSON.stringify({
        pk: { S: "USER#00000000-0000-0000-0000-000000000001" },
        sk: { S: "USER_MEMBERSHIP" },
      }),
      "--update-expression",
      "SET total_credits = :credits, remaining_credits = :credits",
      "--expression-attribute-values",
      JSON.stringify({
        ":credits": { N: "100" },
      }),
    ],
    { stdio: "pipe" }
  );

  execFileSync(
    "aws",
    [
      "--endpoint-url=http://localhost:4566",
      "dynamodb",
      "update-item",
      "--table-name",
      "ratel-local-main",
      "--key",
      JSON.stringify({
        pk: { S: "MEMBERSHIP#PRO" },
        sk: { S: "MEMBERSHIP" },
      }),
      "--update-expression",
      "SET max_credits_per_space = :max",
      "--expression-attribute-values",
      JSON.stringify({
        ":max": { N: "-1" },
      }),
    ],
    { stdio: "pipe" }
  );
}

test("create storage state", async ({ page }) => {
  const email = `hi+user1@biyard.co`;
  const password = "admin!234";

  await goto(page, "/");

  await click(page, { label: "Sign In" });
  await fill(page, { placeholder: "Enter your email address" }, email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, password);
  await click(page, { text: "Continue" });

  await waitPopup(page, { visible: false });
  seedCreator1MembershipForTests();

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: "user.json" });

  console.log("✅ Global authenticated user setup completed");
  console.log(`📄 Test user saved: ${email}`);
  console.log(`🔐 Storage state saved to: user.json`);
});
