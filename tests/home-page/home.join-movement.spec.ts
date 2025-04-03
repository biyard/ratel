import { test, expect } from "@playwright/test";
import { wrap } from "../utils";

test("[Home page] Testing the Join movement modal", async ({
    page
  }, testInfo) => {

    const p = wrap(page, testInfo.project.name, "home/join-movement");
    await p.goto("/", { waitUntil: "load", timeout: 600000 });

    const joinButton = page.locator('button', { hasText: "JOIN THE MOVEMENT" });
    await expect(joinButton).toBeVisible();
    await joinButton.click({ force: true });
    const modal = page.locator('#signup_popup');
    await page.waitForSelector('#signup_popup', { state: "attached", timeout: 7000 });
    await expect(modal).toBeVisible({ timeout: 600000 });
    await p.capture("join-movement-modal");

    await expect(modal).toContainText("Join the Movement");
  });