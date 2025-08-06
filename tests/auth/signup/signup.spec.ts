import { test, expect } from "@playwright/test";
import { CONFIGS } from "../../config";
import { wrap } from "../../utils";

test.describe('Email/Password Signup Flow', () => {
  test('should create account with valid credentials', async ({ page }) => {
    await page.goto('/');

    await page.fill('#email', 'testuser@example.com');
    await page.fill('#password', 'correct-password');
    await page.click('button[type="submit"]');

    await expect(page).toHaveURL('/login');
    await expect(page.locator('h1')).toContainText('Login');
  });

  test('should show error on wrong credentials types', async ({ page }) => {
    await page.goto('/');

    await page.fill('#email', 'testuser@example.com');
    await page.fill('#password', 'wrong-password');
    await page.click('button[type="submit"]');

    await expect(page.locator('.error')).toHaveText('Invalid credentials');
  });

  test('should show validation errors on empty fields', async ({ page }) => {
    await page.goto('/');

    await page.click('button[type="submit"]');
    await expect(page.locator('#email-error')).toHaveText('Email is required');
    await expect(page.locator('#password-error')).toHaveText('Password is required');
  });
});
