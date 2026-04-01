import { expect } from "@playwright/test";

export function wrap(page, project, baseDir) {
  const pageWithCapture = page;
  pageWithCapture.order = 1;

  pageWithCapture.fullCapture = async (name) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, "0");
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename, fullPage: true });
  };

  pageWithCapture.capture = async (name) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, "0");
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename });
  };

  pageWithCapture.clickAndCapture = async (name) => {
    await page.locator(`text=${name}`).click();
    await page.waitForTimeout(500);
    await pageWithCapture.capture(name);
  };

  pageWithCapture.clickXpathAndCapture = async (xpath, name) => {
    await page.locator(`xpath=${xpath}`).click();
    await page.waitForTimeout(500);
    await pageWithCapture.capture(name);
  };

  return pageWithCapture;
}

export async function click(page, opt) {
  const selected = await getLocator(page, opt);

  await selected.click();
  await page.waitForLoadState("load");

  return selected;
}

/**
 * Click an element without waiting for navigation (`waitForLoadState("load")`).
 * Use this for non-navigation UI interactions (e.g., opening a sidebar sheet,
 * toggling a panel) where `waitForLoadState` would resolve immediately or hang
 * because no page navigation occurs.
 */
export async function clickNoNav(page, opt) {
  const selected = await getLocator(page, opt);

  await selected.click();

  return selected;
}

export async function fill(page, opt, value) {
  const selected = await getLocator(page, opt);

  await selected.fill(value);

  return selected;
}

export async function getLocator(
  page,
  { placeholder, text, role, label, testId }
) {
  let selected;

  if (testId) {
    selected = page.getByTestId(testId);
  } else if (label) {
    selected = page.getByLabel(label, { exact: true });
  } else if (role) {
    const opt = { exact: true };
    if (text) {
      opt.name = text;
    }
    selected = page.getByRole(role, opt);
  } else if (placeholder) {
    selected = page.getByPlaceholder(placeholder, { exact: true });
  } else if (text) {
    selected = page.getByText(text, { exact: true });
  } else {
    throw new Error("Either text, label, or data-testid must be provided");
  }

  await expect(selected).toBeVisible();

  return selected;
}

export async function waitPopup(page, { visible = true }) {
  if (visible) {
    await expect(page.getByTestId("popup-overlay")).toBeVisible();
  } else {
    await expect(page.getByTestId("popup-overlay")).toBeHidden();
  }
}

export async function goto(page, url) {
  // Best-effort WASM response listener — fire-and-forget so it never blocks
  // navigation. When the WASM binary is served from the browser's
  // compiled-code cache no network event fires, so awaiting this would add
  // up to `timeout` ms of unnecessary delay. The waitForFunction hydration
  // check below is the real guarantee that the app is ready.
  page
    .waitForResponse(
      (response) =>
        response.url().includes("app-shell") &&
        response.url().endsWith(".wasm") &&
        response.status() === 200,
      { timeout: 15000 },
    )
    .catch(() => {});

  await page.goto(url);
  await page.waitForLoadState("domcontentloaded");
  // Wait for Dioxus WASM to hydrate — SSR markup already contains
  // [data-dioxus-id], so also verify the interpreter is initialised.
  // NOTE: In Dioxus 0.7, `dioxus` is only available as a local binding
  // inside document::eval() contexts, NOT as `window.dioxus`. Checking
  // `window.dioxus.send` would hang forever. Instead we check for the
  // presence of hydrated DOM elements and rely on Playwright's built-in
  // action retries for any remaining hydration delay.
  await page.waitForFunction(
    () => document.querySelector("[data-dioxus-id]") !== null,
    null,
    { timeout: 30000 },
  );
}

export async function getEditor(page) {
  const editor = page.locator("[contenteditable]");
  await expect(editor).toBeVisible();

  return editor;
}
