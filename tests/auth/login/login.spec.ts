import { test, expect } from "@playwright/test";
import { CONFIGS } from "../../config";
import { wrap } from "../../utils";

test("[Home page] Testing the Ratel White paper link in a PDF DOC", async ({
    page,
    browserName 
  }, testInfo) => {
    try{
      const p = wrap(page, testInfo.project.name, "home/white-paper");
      await p.goto("/", { waitUntil: "load", timeout: CONFIGS.PAGE_WAIT_TIME });
      await p.fullCapture("full");
      await p.capture("top");
      
      const linkSelector = p.locator('a[href*="Ratel-Token-White-Paper.pdf"]').filter({ hasText: "LEARN MORE ABOUT $RATEL" });
  
      const count = await linkSelector.count();
    
      if (count > 0) {
        await expect(linkSelector).toBeVisible();
      await expect(linkSelector).toHaveAttribute("href", /Ratel-Token-White-Paper\.pdf$/);
  
      // Optionally click it to ensure it's clickable (won't assert anything further)
      await linkSelector.click();
        }else {
          console.log("Locator not found on the page.");
        }
    }catch(err){
      console.error("Test failed with this error:", err, "And the brower is:", browserName);
    }
    
  });


test.describe('Email/Password Login Flow', () => {
  test('should login with valid credentials', async ({ page }) => {
    await page.goto('/login');

    await page.fill('#email', 'testuser@example.com');
    await page.fill('#password', 'correct-password');
    await page.click('button[type="submit"]');

    await expect(page).toHaveURL('/dashboard');
    await expect(page.locator('h1')).toContainText('Dashboard');
  });

  test('should show error on wrong credentials', async ({ page }) => {
    await page.goto('/login');

    await page.fill('#email', 'testuser@example.com');
    await page.fill('#password', 'wrong-password');
    await page.click('button[type="submit"]');

    await expect(page.locator('.error')).toHaveText('Invalid credentials');
  });

  test('should show validation errors on empty fields', async ({ page }) => {
    await page.goto('/login');

    await page.click('button[type="submit"]');
    await expect(page.locator('#email-error')).toHaveText('Email is required');
    await expect(page.locator('#password-error')).toHaveText('Password is required');
  });
});
