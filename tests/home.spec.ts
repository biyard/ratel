import { test, expect } from "@playwright/test";
import { wrap } from "./utils";

test("[Home-001] Looking over website w/o signing up", async ({
  page,
}, testInfo) => {
  const p = wrap(page, testInfo.project.name, "home/001-look-over");

  await p.goto("/", { waitUntil: "load" });
  await p.fullCapture("full");
  await p.capture("top");

  const viewport = page.viewportSize();

  if (viewport && viewport.width < 768) {
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
      "Mobile menu",
    );
  }
  await p.clickXpathAndCapture("//nav/a[1]", "About");
  if (viewport && viewport.width < 768) {
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
      "Mobile menu",
    );
  }
  await p.clickXpathAndCapture("//nav/a[2]", "Politician stance");
  if (viewport && viewport.width < 768) {
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
      "Mobile menu",
    );
  }
  await p.clickXpathAndCapture("//nav/a[3]", "Community");
  if (viewport && viewport.width < 768) {
    await p.clickXpathAndCapture(
      "/html/body/div/div[3]/div[2]/div[1]/div/div[1]/button[1]",
      "Mobile menu",
    );
  }
  await p.clickXpathAndCapture("//nav/a[4]", "Support");

  const to_politicians = p.getByRole("link", { name: "View all" });
  await expect(to_politicians).toBeVisible();

  await to_politicians.click();
  await p.waitForURL("/en/politicians");
  await p.waitForSelector('div:text("Politician Stance")');
  await p.capture("politician-stance");
});
