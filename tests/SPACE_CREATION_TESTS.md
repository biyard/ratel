# Space Creation Tests

This document describes the Playwright tests for the space creation functionality in the application.

## Test Cases

### 1. Poll Space Creation
- **Test File**: `tests/space-creation.spec.ts`
- **Test Name**: `should navigate to space page after creating a Poll space`
- **Description**: Verifies that creating a Poll space correctly navigates to the space page.
- **Steps**:
  1. Log in as a test user
  2. Navigate to a test thread
  3. Open the space creation modal
  4. Select "Poll" as the space type
  5. Submit the form
  6. Verify navigation to the new space page
  7. Verify the space type is displayed as "Poll"

### 2. Deliberation Space Creation
- **Test File**: `tests/space-creation.spec.ts`
- **Test Name**: `should navigate to space page after creating a Deliberation space`
- **Description**: Verifies that creating a Deliberation space correctly navigates to the space page.
- **Steps**:
  1. Log in as a test user
  2. Navigate to a test thread
  3. Open the space creation modal
  4. Select "Deliberation" as the space type
  5. Submit the form
  6. Verify navigation to the new space page
  7. Verify the space type is displayed as "Deliberation"

## Test Setup

### Prerequisites
- Node.js (v14 or later)
- npm or yarn
- Playwright Test

### Environment Variables
Create a `.env` file in the project root with the following variables:

```
TEST_EMAIL=your_test_email@example.com
TEST_PASSWORD=your_test_password
```

### Running the Tests

1. Install dependencies:
   ```bash
   npm install
   # or
   yarn install
   ```

2. Install Playwright browsers:
   ```bash
   npx playwright install
   ```

3. Run the space creation tests:
   ```bash
   npx playwright test tests/space-creation.spec.ts --headed
   ```
   
   For headless mode (CI/CD):
   ```bash
   npx playwright test tests/space-creation.spec.ts
   ```

4. View the test report:
   ```bash
   npx playwright show-report
   ```

## Test Helpers

### `auth.ts`
Location: `tests/helpers/auth.ts`

Contains helper functions for authentication and test data setup:
- `login(page: Page)`: Handles user login
- `createTestThread(page: Page)`: Creates a test thread for space creation

## Debugging

- Tests will automatically take screenshots on failure (saved to `test-results/screenshots/`)
- To debug a specific test, add `.only` to the test:
  ```typescript
  test.only('should navigate to space page...', async ({ page }) => {
    // test code
  });
  ```
- Use Playwright's debug mode:
  ```bash
  npx playwright test tests/space-creation.spec.ts --debug
  ```

## Best Practices

1. **Atomic Tests**: Each test should be independent and not rely on other tests
2. **Page Object Model**: Consider creating page objects for complex pages
3. **Selectors**: Use data-testid attributes for reliable element selection
4. **Assertions**: Make assertions specific and meaningful
5. **Cleanup**: Ensure tests clean up after themselves to avoid side effects
