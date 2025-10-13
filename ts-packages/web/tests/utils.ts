import { expect, Locator, Page, TestInfo } from '@playwright/test';
import { CONFIGS } from './config';
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
    const paddedOrder = String(pageWithCapture.order).padStart(3, '0');
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename, fullPage: true });
  };

  pageWithCapture.capture = async (name: string) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, '0');
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

export async function click(
  page: Page,
  { text, label }: { text?: string; label?: string },
): Promise<Locator> {
  let timeout = { timeout: CONFIGS.PAGE_WAIT_TIME };

  let selected: Locator;

  if (label) {
    selected = page.getByLabel(label, { exact: true });
  } else if (text) {
    selected = page.getByRole('button', { name: text, exact: true });
  } else {
    throw new Error('Either text or id must be provided');
  }

  await expect(selected).toBeVisible(timeout);
  await selected.click();

  return selected;
}

export async function fill(
  page: Page,
  {
    placeholder,
    label,
  }: {
    placeholder?: string;
    label?: string;
  },
  value: string,
): Promise<Locator> {
  let opt = { exact: true };
  let timeout = { timeout: CONFIGS.PAGE_WAIT_TIME };

  let selected: Locator;

  if (placeholder) {
    selected = page.getByPlaceholder(placeholder, opt);
  } else if (label) {
    selected = page.getByLabel(label, opt);
  } else {
    throw new Error('unsupported selector');
  }
  await expect(selected).toBeVisible(timeout);

  await selected.fill(value);

  return selected;
}
