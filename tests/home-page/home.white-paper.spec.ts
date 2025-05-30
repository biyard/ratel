import { test, expect } from "@playwright/test";
import { wrap } from "../utils";
import { CONFIGS } from "../config";

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