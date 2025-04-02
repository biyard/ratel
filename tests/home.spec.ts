import { test, expect } from "@playwright/test";
import { wrap } from "./utils";

test("[Home-001] Looking over website w/o signing up", async ({ page }) => {
  const p = wrap(page, "home/001-look-over");
  await p.goto("/", { waitUntil: "load" });
  await p.fullCapture("full");
  await p.capture("top");

  await p.clickXpathAndCapture("//nav/a[1]", "About");
  await p.clickXpathAndCapture("//nav/a[2]", "Politician stance");
  await p.clickXpathAndCapture("//nav/a[3]", "Community");
  await p.clickXpathAndCapture("//nav/a[4]", "Support");

  const to_politicians = p.getByRole("link", { name: "View all" });
  await expect(to_politicians).toBeVisible();

  await to_politicians.click();
  await p.waitForURL("/en/politicians");
  await p.waitForSelector('div:text("Politician Stance")');
  await p.capture("politician-stance");
});
