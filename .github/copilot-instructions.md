When Performing a code review, respond in Korean.

When performing a code review,
* Find out the anti-pattern in the `/docs/troubleshooting.md` file.
* For files under `app/`, apply the Dioxus convention checks defined in `/docs/dioxus-convention.md`. See `/docs/dioxus-convention-review-checklist.md` for the review checklist.

## Playwright E2E Tests

When writing Playwright test code under `playwright/`, follow the conventions defined in `/docs/playwright-testing.md`.

Key rules:
* Test files go in `playwright/tests/users/*.spec.js` (plain JavaScript, not TypeScript).
* Always use shared helpers (`goto`, `click`, `fill`, `getLocator`, `getEditor`, `waitPopup`) from `tests/utils.js` instead of raw Playwright APIs.
* Locator options: `testId` > `label` > `role` > `placeholder` > `text` (priority order).
* See `/docs/playwright-testing.md` for utility function signatures, locator option details, app-specific selectors, and complete examples.
