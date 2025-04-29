import { test, expect } from "@playwright/test";
import { wrap } from "../utils";
import { CONFIGS } from "../config";
import { credentials } from "../test-data/auth";

test("[Home page] Testing the Join movement modal", async ({
    page, browserName, context
  }, testInfo) => {
    try {
        const p = wrap(page, testInfo.project.name, "home/join-movement");
        await p.goto("/", { waitUntil: "domcontentloaded", timeout: CONFIGS.PAGE_WAIT_TIME });

        const joinButton = page.locator('button', { hasText: "JOIN THE MOVEMENT" });
        await expect(joinButton).toBeVisible();
        await joinButton.click({ force: true });
        const modal = page.locator('#signup_popup');
        await page.waitForSelector('#signup_popup', { state: "attached", timeout: CONFIGS.SELECTOR_WAIT_TIME });
        await expect(modal).toBeVisible({ timeout: CONFIGS.MODAL_WAIT_TIME });
        await p.capture("join-movement-modal");

        await expect(modal).toContainText("Join the Movement");

        const googleButton = modal.getByText("Continue with Google", { exact: true }).first();
        await expect(googleButton).toBeVisible({ timeout: CONFIGS.MODAL_WAIT_TIME });
        await expect(googleButton).toBeEnabled();

        const [popup] = await Promise.all([
          page.waitForEvent("popup"),
          googleButton.click(),
        ]);
        if (popup) {
          await popup.waitForLoadState("domcontentloaded");
          await popup.waitForURL(/accounts.google.com/, { timeout: CONFIGS.MODAL_WAIT_TIME });
          expect(popup.url()).toContain("accounts.google.com");
          await p.capture("google-auth-popup-modal");

          if (await popup.isVisible('input[type="email"]')) {
                await popup.fill('input[type="email"]', credentials.email);
                await popup.click('button:has-text("Next")');
                await p.capture("enter-email.png");
            
                if (await popup.isVisible("text=Couldn’t find your Google Account")) {
                  console.error("Invalid email entered!");
                  return;
                }
            
                await popup.waitForSelector('input[type="password"]', {
                  timeout: CONFIGS.MODAL_WAIT_TIME,
                });
                await popup.fill('input[type="password"]', credentials.pass);
                await popup.click('button:has-text("Next")');
                await p.capture("enter-password.png");
                if (await popup.isVisible("text=Wrong password. Try again")) {
                  console.error("Incorrect password!");
                  return;
                }
            
                await popup.waitForLoadState("networkidle");
          }

          if (await popup.isVisible("text=This app isn’t verified")) {
            await popup.click('button:has-text("Advanced")');
            await popup.click('a:has-text("Go to ratel.foundation (unsafe)")');
            await p.capture("unverified-app.png");
          }

          await page.screenshot({
            path: "screenshots/users/google-001/07-login-success.png",
          });
          await p.capture("google-login-success.png");

          await context.storageState({ path: "../storage/auth.json" });
        
          console.log("Google OAuth login session saved!");
          await context.close();
        } else if (browserName === "firefox") {
          console.warn("Popup blocked or not opened on Firefox");
        }
  }
  catch (err) {
    console.error("Google Auth popup did not open properly:", err);
    if (browserName === "firefox") {
      console.warn("Likely caused by Firefox's stricter popup blocking.");
    }
  }

  });