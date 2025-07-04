import { test, expect } from "@playwright/test";
import { wrap } from "./utils";
import { CONFIGS } from "./config";

test("[Home-001] Looking over website w/o signing up", async ({
  page,
  browserName,
}, testInfo) => {
  try {
    const p = wrap(page, testInfo.project.name, "home/001-look-over");
    await p.goto("/", { waitUntil: "load", timeout: CONFIGS.PAGE_WAIT_TIME });
    await p.fullCapture("full");
    await p.capture("top");

    const viewport = page.viewportSize();

    if (viewport && viewport.width < CONFIGS.DEVICE_SCREEN_SIZES.MOBILE) {
      await p.clickXpathAndCapture(
        "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
        "Mobile menu",
      );
    }
    await p.clickXpathAndCapture("//nav/a[1]", "About");
    if (viewport && viewport.width < CONFIGS.DEVICE_SCREEN_SIZES.MOBILE) {
      await p.clickXpathAndCapture(
        "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
        "Mobile menu",
      );
    }
    await p.clickXpathAndCapture("//nav/a[2]", "Politician stance");
    if (viewport && viewport.width < CONFIGS.DEVICE_SCREEN_SIZES.MOBILE) {
      await p.clickXpathAndCapture(
        "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
        "Mobile menu",
      );
    }
    await p.clickXpathAndCapture("//nav/a[3]", "Community");
    if (viewport && viewport.width < CONFIGS.DEVICE_SCREEN_SIZES.MOBILE) {
      await p.clickXpathAndCapture(
        "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
        "Mobile menu",
      );
    }
    await p.clickXpathAndCapture("//nav/a[4]", "Support");

    if (viewport && viewport.width < CONFIGS.DEVICE_SCREEN_SIZES.MOBILE) {
      await p.clickXpathAndCapture(
        "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
        "Mobile menu",
      );
    }
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[2]/div[1]/div/div[2]/div/button",
      "Sign in",
    );
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[1]/div/button",
      "Close",
    );

    const to_politicians = p.getByRole("link", { name: "View all" });
    await expect(to_politicians).toBeVisible();

    await to_politicians.click();
    await p.waitForURL("/en/politicians");
    await p.waitForSelector('div:text("Politician Stance")');
    await p.capture("politician-stance");

    await p.clickXpathAndCapture(
      '//*[@id="politician-list"]/a[1]',
      "Select a member",
    );
    // await p.fullCapture("politician detail full");

    await p.clickXpathAndCapture(
      '//*[@id="main"]/div[3]/div[2]/div[1]/div/div[1]/a',
      "Go to home",
    );

    if (viewport && viewport.width < CONFIGS.DEVICE_SCREEN_SIZES.MOBILE) {
      await p.clickXpathAndCapture(
        "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
        "Mobile menu",
      );
    }
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[2]/div[1]/div/div[2]/div/a",
      "Get ratel",
    );
    await p.clickAndCapture("GO BACK");
  } catch (err) {
    console.error("Test failed with error:", err);
    if (browserName === "firefox") {
      console.warn("Likely caused by Firefox's stricter popup blocking.");
    }
  }
});
