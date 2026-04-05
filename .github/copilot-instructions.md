When Performing a code review, respond in Korean or English. If a PR is written by Korean, respond in Korean, otherwise English.

When performing a code review,
* Find out the anti-pattern in the `/docs/troubleshooting.md` file.
* For files under `app/`, apply the Dioxus convention checks defined in `/docs/dioxus-convention.md`. See `/docs/dioxus-convention-review-checklist.md` for the review checklist.
* Pay special attention to `toast.error()` usage: it must receive a typed `common::Error` variant, never a raw string. See section 12 of `/docs/dioxus-convention.md` for the full toast convention.
* **Do not use `Error::BadRequest(String)` for domain-specific errors.** Instead, define a specific error enum with `Translate` derive (e.g., `SpaceActionQuizError`, `SpaceRewardError`) for user-friendly, i18n-compatible error handling. See the `SpaceRewardError` pattern in `app/ratel/src/common/types/reward/error.rs` and the "Error Handling Convention" section of `CLAUDE.md`. When reviewing code that introduces new `Error::BadRequest(...)` calls, flag it and suggest a typed error variant instead. **Important:** `common::Error` uses `derive(Translate)` with the `#[translate(from)]` attribute on wrapper variants to delegate translation to the inner error's `translate()` method. When adding a new wrapper variant, annotate it with `#[translate(from)]` so the derive macro generates the delegation automatically.
* For TailwindCSS styling, apply the conventions defined in `/docs/tailwindcss-convention.md`. Key checks:
  - Use semantic design tokens (`bg-card`, `text-text-primary`), not raw color values (`bg-neutral-800`).
  - Do not use `light:` or `dark:` prefixes in component classes; define both light/dark values as semantic tokens in `app/ratel/tailwind.css`.
  - Prefer `aria-selected:` / `group-aria-selected:` variants over Rust if/else class selection for toggleable states.
  - Flag excessive `!important` overrides (`!bg-*`, `!text-*`) — components should accept a `class` prop instead.
  - Use `max-tablet:` / `max-mobile:` for responsive breakpoints.
