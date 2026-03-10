When Performing a code review, respond in Korean.

When performing a code review,
* Find out the anti-pattern in the `/docs/troubleshooting.md` file.
* For files under `app/`, apply the Dioxus convention checks defined in `/docs/dioxus-convention.md`. See `/docs/dioxus-convention-review-checklist.md` for the review checklist.
* Pay special attention to `toast.error()` usage: it must receive a typed `common::Error` variant, never a raw string. See section 12 of `/docs/dioxus-convention.md` for the full toast convention.
* For TailwindCSS styling, apply the conventions defined in `/docs/tailwindcss-convention.md`. Key checks:
  - Use semantic design tokens (`bg-card`, `text-text-primary`), not raw color values (`bg-neutral-800`).
  - Do not use `light:` or `dark:` prefixes in component classes; define both light/dark values as semantic tokens in `app/ratel/tailwind.css`.
  - Prefer `aria-selected:` / `group-aria-selected:` variants over Rust if/else class selection for toggleable states.
  - Flag excessive `!important` overrides (`!bg-*`, `!text-*`) — components should accept a `class` prop instead.
  - Use `max-tablet:` / `max-mobile:` for responsive breakpoints.

## Playwright E2E Tests

When writing Playwright test code under `playwright/`, follow the conventions defined in `/docs/playwright-testing.md`.

Key rules:
* Test files go in `playwright/tests/users/*.spec.js` (plain JavaScript, not TypeScript).
* Always use shared helpers (`goto`, `click`, `fill`, `getLocator`, `getEditor`, `waitPopup`) from `tests/utils.js` instead of raw Playwright APIs.
* Locator options: `testId` > `label` > `role` > `placeholder` > `text` (priority order).
* See `/docs/playwright-testing.md` for utility function signatures, locator option details, app-specific selectors, and complete examples.
