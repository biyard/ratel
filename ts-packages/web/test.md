# Playwright Test Files

This document lists all Playwright test files in the `web` package, structured by their location and purpose.

## E2E Tests
Tests located in the `e2e` directory, typically covering end-to-end user flows.

- `e2e/deliberation/anonymous.web.spec.ts`: Anonymous user flows for deliberation.

## App Page Tests
Tests located in `src/app`, corresponding to specific pages or routes in the application.

### Social Pages
- `src/app/(social)/home-page.anon.spec.ts`: Home page tests for anonymous users.
- `src/app/(social)/home-page.auth.spec.ts`: Home page tests for authenticated users.

### Team Pages
- `src/app/teams/teams.auth.spec.ts`: General teams page tests for authenticated users.
- `src/app/teams/[username]/groups/groups.auth.spec.ts`: Team groups page tests for authenticated users.
- `src/app/teams/[username]/members/members.auth.spec.ts`: Team members page tests for authenticated users.
- `src/app/teams/[username]/settings/settings.auth.spec.ts`: Team settings page tests for authenticated users.

## Feature Components Tests
Tests located in `src/features`, targeting specific feature components.

### Posts
- `src/features/posts/components/create-post-page/create-post-page.anon.spec.ts`: Create post page tests for anonymous users.
- `src/features/posts/components/create-post-page/create-post-page.auth.spec.ts`: Create post page tests for authenticated users.
