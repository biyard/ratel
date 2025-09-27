# Playwright Testing Guide

This guide provides comprehensive instructions for writing and maintaining Playwright tests in the Ratel project.

## Overview

Playwright tests in this project use a structured approach that mirrors the Next.js App Router directory structure. All tests are located in the `tests/` directory and follow specific naming and organizational conventions.

## Directory Structure

### Mirroring App Router Structure
The test directory structure should exactly match the app router structure:

```
ts-packages/web/src/app/
├── (social)/
│   ├── page.tsx
│   ├── settings/
│   └── experimental/
└── teams/
    ├── [username]/
    │   ├── page.tsx
    │   ├── home/
    │   ├── settings/
    │   ├── members/
    │   ├── drafts/
    │   └── groups/
    └── page.tsx

tests/
├── (social)/
│   ├── homepage.auth.spec.ts
│   ├── homepage.anon.spec.ts
│   ├── settings/
│   └── experimental/
└── teams/
    ├── [username]/
    │   ├── team-page.auth.spec.ts
    │   ├── home/
    │   ├── settings/
    │   ├── members/
    │   ├── drafts/
    │   └── groups/
    └── teams-list.auth.spec.ts
```

## Naming Conventions

### File Naming
Test files must follow this pattern:
- `{descriptive-test-name}.auth.spec.ts` - Tests for authenticated users
- `{descriptive-test-name}.anon.spec.ts` - Tests for anonymous/guest users

### Examples
- Homepage tests: `homepage.auth.spec.ts`, `homepage.anon.spec.ts`
- Settings page tests: `settings.auth.spec.ts`, `settings.anon.spec.ts`
- Team page tests: `team-page.auth.spec.ts`, `team-page.anon.spec.ts`

## Test Structure

### Basic Template
```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature/Page Name', () => {
  test.beforeEach(async ({ page }) => {
    // Common setup for all tests in this describe block
  });

  test('should perform specific action', async ({ page }) => {
    // Test implementation
    await page.goto('/path-to-page');

    // Your test assertions
    await expect(page).toHaveTitle(/Expected Title/);
  });

  test('should handle error cases', async ({ page }) => {
    // Error handling tests
  });
});
```

### Authentication Patterns

#### Authenticated Tests (.auth.spec.ts)
```typescript
import { test, expect } from '@playwright/test';

test.describe('Authenticated User Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Login flow - adjust based on your authentication method
    await page.goto('/login');
    await page.fill('#email', 'test@example.com');
    await page.fill('#password', 'password');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/dashboard'); // or appropriate authenticated page
  });

  test('should create a post', async ({ page }) => {
    await page.goto('/');
    // Test implementation for authenticated user actions
  });
});
```

#### Anonymous Tests (.anon.spec.ts)
```typescript
import { test, expect } from '@playwright/test';

test.describe('Anonymous User Tests', () => {
  test('should redirect to login when accessing protected content', async ({ page }) => {
    await page.goto('/protected-page');
    await expect(page).toHaveURL(/login/);
  });

  test('should view public content', async ({ page }) => {
    await page.goto('/');
    // Test implementation for anonymous user actions
  });
});
```

## Common Test Patterns

### Page Navigation
```typescript
test('should navigate correctly', async ({ page }) => {
  await page.goto('/');
  await page.click('a[href="/teams"]');
  await expect(page).toHaveURL('/teams');
});
```

### Form Interactions
```typescript
test('should submit form successfully', async ({ page }) => {
  await page.goto('/create-post');

  await page.fill('#title', 'Test Post Title');
  await page.fill('#content', 'This is test content');
  await page.click('button[type="submit"]');

  await expect(page.locator('.success-message')).toBeVisible();
});
```

### API Response Mocking
```typescript
test('should handle API errors gracefully', async ({ page }) => {
  await page.route('**/api/posts', route =>
    route.fulfill({ status: 500, body: 'Server Error' })
  );

  await page.goto('/posts');
  await expect(page.locator('.error-message')).toBeVisible();
});
```

### Waiting for Dynamic Content
```typescript
test('should wait for content to load', async ({ page }) => {
  await page.goto('/dynamic-content');

  // Wait for specific element
  await page.waitForSelector('.loaded-content');

  // Wait for network requests
  await page.waitForLoadState('networkidle');

  await expect(page.locator('.loaded-content')).toBeVisible();
});
```

## Best Practices

### 1. Test Organization
- Group related tests using `test.describe()`
- Use descriptive test names that explain what is being tested
- Keep tests focused on a single feature or behavior

### 2. Setup and Cleanup
- Use `test.beforeEach()` for common setup
- Use `test.afterEach()` for cleanup if needed
- Keep setup minimal and focused

### 3. Selectors
- Prefer data-testid attributes: `page.locator('[data-testid="submit-button"]')`
- Use semantic selectors when possible: `page.locator('button[type="submit"]')`
- Avoid brittle selectors based on styling classes

### 4. Assertions
- Use specific assertions: `toHaveText()`, `toBeVisible()`, `toHaveURL()`
- Chain assertions when testing multiple conditions
- Use `soft` assertions for non-critical checks

### 5. Performance
- Use `page.waitForLoadState('networkidle')` sparingly
- Prefer specific waits: `waitForSelector()`, `waitForResponse()`
- Mock external APIs in tests to improve speed and reliability

## Running Tests

### All Tests
```bash
# From project root
npx playwright test

# Using make command
make test
```

### Specific Test Files
```bash
# Run all auth tests
npx playwright test --grep="auth.spec.ts"

# Run all anon tests
npx playwright test --grep="anon.spec.ts"

# Run tests for specific page
npx playwright test tests/(social)/homepage.auth.spec.ts
```

### Debug Mode
```bash
# Run in headed mode
npx playwright test --headed

# Run with debug
npx playwright test --debug

# Run specific test with debug
npx playwright test tests/(social)/homepage.auth.spec.ts --debug
```

## Configuration

The project uses `playwright.config.ts` in the root directory for configuration. Key settings include:

- **Base URL**: Configured for local development
- **Browsers**: Chrome, Firefox, Safari (if configured)
- **Parallel Execution**: Enabled for faster test runs
- **Retry**: Configured for flaky test handling

## Troubleshooting

### Common Issues

1. **Timeout Errors**
   - Increase timeout for slow operations
   - Use appropriate wait strategies
   - Check for network issues or slow API responses

2. **Flaky Tests**
   - Add appropriate waits
   - Use more specific selectors
   - Consider mocking unstable external dependencies

3. **Authentication Issues**
   - Verify login flow in beforeEach hooks
   - Check session persistence
   - Consider using authentication state storage

### Debug Tips

1. Use `page.screenshot()` to capture page state
2. Use `console.log()` to output debugging information
3. Use browser developer tools with `--headed --debug` flags
4. Check network tab for failed requests

## Examples

See the `tests/` directory for working examples of:
- Authentication flows
- Form submissions
- Navigation testing
- Error handling
- API mocking

For more specific examples, refer to the existing test files in your respective page directories.