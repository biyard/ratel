import { expect, Page } from "@playwright/test";
import { BASE_URL, TIMEOUT } from "../configs";

// Helper functions
export async function login(page: Page, email: string, password: string) {
  await page.goto(BASE_URL);
  await page.getByRole("button", { name: /sign in/i }).click();
  await page.getByTestId("email-input").fill(email);
  await page.getByTestId("continue-button").click();
  await page.getByTestId("password-input").fill(password);
  await page.getByTestId("continue-button").click();
  await page.waitForURL(BASE_URL, { timeout: TIMEOUT });
}

export async function mobileLogin(page: Page, email: string, password: string) {
  await page.goto(BASE_URL);
  const menuButton = page.getByTestId("mobile-menu-toggle");
  await menuButton.waitFor({ state: "visible", timeout: TIMEOUT });
  await menuButton.click();

  const signInButton = page.getByRole("button", { name: /sign in/i });
  await signInButton.waitFor({ state: "visible", timeout: TIMEOUT });
  await signInButton.click();

  await page.getByTestId("email-input").fill(email);
  await page.getByTestId("continue-button").click();
  await page.getByTestId("password-input").fill(password);
  await page.getByTestId("continue-button").click();
  await page.waitForURL(BASE_URL, { timeout: TIMEOUT });
}

export async function clickTeamSidebarMenu(page: Page, menuName: string) {
  const menu = page.getByTestId(`sidemenu-team-${menuName}`);
  await menu.waitFor({ state: "visible", timeout: TIMEOUT });
  await menu.click();
  await page.waitForLoadState("networkidle");
}

export async function setEndTimeOneHourLater(page: Page) {
  // Calculate current time and 1 hour later
  const now = new Date();
  const oneHourLater = new Date(now.getTime() + 60 * 60 * 1000);

  // Format time for selection (e.g., "02:00 PM")
  const endHour = oneHourLater.getHours();
  const endHour12 = endHour % 12 || 12;
  const endPeriod = endHour < 12 ? "AM" : "PM";
  const endTimeText = `${endHour12.toString().padStart(2, "0")}:00 ${endPeriod}`;

  // Click end time dropdown
  const endTimeButton = page.getByTestId("time-end-dropdown");
  await endTimeButton.waitFor({ state: "visible", timeout: TIMEOUT });
  await endTimeButton.click();
  await page.waitForTimeout(500);

  // Select the time option
  const timeOption = page.getByText(endTimeText, { exact: true });
  await timeOption.waitFor({ state: "visible", timeout: TIMEOUT });
  await timeOption.click();
  await page.waitForTimeout(500);

  // Verify the time was set
  await expect(endTimeButton).toContainText(endTimeText);
}
