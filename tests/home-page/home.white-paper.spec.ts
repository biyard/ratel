import { test, expect } from "@playwright/test";
import { wrap } from "../utils";

test("[Home page] Testing the Ratel White paper link in a PDF DOC", async ({
    page,
    browserName 
  }, testInfo) => {

    const p = wrap(page, testInfo.project.name, "home/white-paper");
    await p.goto("/", { waitUntil: "load", timeout: 600000 });
    await p.fullCapture("full");
    await p.capture("top");
    
    const linkSelector = p.locator('a[href*="Ratel-Token-White-Paper.pdf"]').filter({ hasText: "LEARN MORE ABOUT $RATEL" });

    const count = await linkSelector.count();
  
    if (count > 0) {
      await expect(linkSelector).toBeVisible();
      if (browserName === "firefox") {
        const [download] = await Promise.all([
          page.waitForEvent("download"), 
          linkSelector.click(),
        ]);
    
        console.log("Downloaded file:", download.suggestedFilename());
        expect(download.suggestedFilename()).toMatch(/\.pdf$/);
      }
      else{
        const [newPage] = await Promise.all([
          page.context().waitForEvent("page"), 
          linkSelector.click(), 
        ]);
        await newPage.waitForLoadState();
        const pdfUrl = newPage.url();
    
        await expect(pdfUrl).toMatch(/\.pdf$/);
    
        const response = await newPage.waitForResponse((resp) => resp.url() === pdfUrl);
        const contentType = response.headers()["content-type"];
        console.log("PDF Content-Type:", contentType);
        expect(contentType).toContain("application/pdf");
      } 
      }else {
        console.log("Locator not found on the page.");
      }
  });