// import { test, expect } from "@playwright/test";
// import { CONFIGS } from "../config";
// import { click, fill, getEditor, getLocator, goto, waitPopup } from "../utils";

// test.describe.serial("Space participate setup", () => {
//   let spaceUrl;

//   async function signInAndParticipate(browser, userIndex) {
//     const context = await browser.newContext({
//       storageState: {
//         cookies: [],
//         origins: [],
//       },
//     });
//     const page = await context.newPage();

//     await goto(page, "/");

//     await click(page, { role: "button", text: /sign in/i });
//     await fill(
//       page,
//       { placeholder: "Enter your email address" },
//       `hi+user${userIndex}@biyard.co`
//     );
//     await click(page, { text: "Continue" });
//     await fill(page, { placeholder: "Enter your password" }, "admin!234");
//     await click(page, { text: "Continue" });
//     await waitPopup(page, { visible: false });

//     await page.goto(`${CONFIGS.BASE_URL}${spaceUrl}/dashboard`);
//     await page.waitForLoadState("networkidle");

//     const participateButton = page.getByRole("button", {
//       name: /Participate|참여하기/,
//     });
//     await expect(participateButton).toBeVisible();
//     await participateButton.click();

//     await expect(page.locator("#space-user-profile")).toContainText(
//       /Participant|참가자/
//     );
//     await expect(page.locator("#space-user-profile")).not.toContainText(
//       /Viewer|뷰어/
//     );
//     await expect(participateButton).toBeHidden();

//     await context.close();
//   }

//   test("create a space from a post", async ({ page }) => {
//     await goto(page, "/");

//     await click(page, { label: "Create Post" });
//     await fill(
//       page,
//       { placeholder: "Title" },
//       `Playwright Space ${Date.now()}`
//     );

//     await click(page, { testId: "skip-space-checkbox" });

//     const editor = await getEditor(page);
//     await editor.fill(
//       "This post creates a test space for the participate scenario."
//     );

//     await click(page, { text: "Go to Space" });

//     await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
//       waitUntil: "networkidle",
//     });

//     await getLocator(page, { text: "Dashboard" });
//     await expect(page).toHaveURL(/\/spaces\/[a-z0-9-]+\/dashboard/);

//     const url = new URL(page.url());
//     spaceUrl = url.pathname.replace(/\/dashboard$/, "");
//   });

//   test("install panels app and configure gender quotas to 50/50", async ({
//     page,
//   }) => {
//     await page.goto(`${CONFIGS.BASE_URL}${spaceUrl}/apps`);
//     await page.waitForLoadState("networkidle");
//     await expect(page.getByText(/Installed Apps|설치된 앱/)).toBeVisible();

//     await page.getByTestId("install-panels-app").first().click();
//     await page.waitForLoadState("networkidle");
//     await expect(page.getByTestId("setting-panels-app").first()).toBeVisible();
//     await page.goto(`${CONFIGS.BASE_URL}${spaceUrl}/apps/panels`);
//     await page.waitForLoadState("networkidle");

//     const totalQuotaInput = page.locator("input").first();
//     await totalQuotaInput.fill("100");
//     await totalQuotaInput.press("Enter");
//     await page.waitForLoadState("networkidle");
//     await expect(totalQuotaInput).toHaveValue("100");
//     await page.reload({ waitUntil: "networkidle" });

//     const refreshedTotalQuotaInput = page.locator("input").first();
//     await expect(refreshedTotalQuotaInput).toHaveValue("100");

//     await page.getByText(/Gender|성별/, { exact: true }).click();

//     const maleRow = page
//       .locator("tr")
//       .filter({ hasText: /Male|남성/ })
//       .first();
//     const femaleRow = page
//       .locator("tr")
//       .filter({ hasText: /Female|여성/ })
//       .first();

//     await expect(maleRow).toBeVisible();
//     await expect(femaleRow).toBeVisible();
//     await expect(maleRow.locator("input")).toHaveValue("50");
//     await expect(femaleRow.locator("input")).toHaveValue("50");
//   });

//   test("publish space publicly", async ({ page }) => {
//     await goto(page, `${spaceUrl}/dashboard`);

//     await click(page, { text: "Publish" });
//     await click(page, { testId: "public-option" });
//     await click(page, { label: "Confirm visibility selection" });
//   });

//   for (const userIndex of [2, 3, 4, 5]) {
//     test(`user${userIndex} can participate from sidebar`, async ({ browser }) => {
//       await signInAndParticipate(browser, userIndex);
//     });
//   }
// });
