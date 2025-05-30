import { expect, Page, TestInfo } from "@playwright/test";
// Screenshot util class
export type BiyardPage = Page & {
  order: number;
  capture: (name: string) => Promise<void>;
  fullCapture: (name: string) => Promise<void>;
  clickAndCapture: (name: string) => Promise<void>;
  clickXpathAndCapture: (xpath: string, name: string) => Promise<void>;
};

export function wrap(page: Page, project: string, baseDir: string): BiyardPage {
  const pageWithCapture = page as BiyardPage;
  pageWithCapture.order = 1;

  pageWithCapture.fullCapture = async (name: string) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, "0");
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename, fullPage: true });
  };

  pageWithCapture.capture = async (name: string) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, "0");
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename });
  };

  pageWithCapture.clickAndCapture = async (name: string) => {
    await page.locator(`text=${name}`).click();
    await page.waitForTimeout(500);
    await pageWithCapture.capture(name);
  };

  pageWithCapture.clickXpathAndCapture = async (
    xpath: string,
    name: string,
  ) => {
    await page.locator(`xpath=${xpath}`).click();
    await page.waitForTimeout(500);
    await pageWithCapture.capture(name);
  };

  return pageWithCapture;
}
