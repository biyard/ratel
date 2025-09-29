import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("Authenticated User", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });
});
