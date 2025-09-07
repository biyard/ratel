# Ratel Web E2E Testing

This directory contains end-to-end tests for the Ratel web application using Playwright.

## Test Structure

```
tests/
├── README.md                 # This file
├── global-setup.ts          # Global test setup
├── playwright.config.ts     # Playwright configuration (in parent directory)
├── fixtures/                # Test data and fixtures
│   └── test-data.ts         # Mock data and test constants
├── pages/                   # Page Object Models
│   ├── HomePage.ts          # Home page interactions
│   ├── PoliticiansPage.ts   # Politicians page interactions
│   └── SignInModal.ts       # Sign in modal interactions
├── utils/                   # Test utilities
│   └── test-helpers.ts      # Common test helper functions
└── *.spec.ts               # Test files
```

## Test Files

- **auth.spec.ts** - Authentication flows and sign-in/sign-out functionality
- **home-page.spec.ts** - Home page functionality, feed loading, and core features
- **navigation.spec.ts** - Navigation between pages, mobile menu, responsive behavior
- **politicians.spec.ts** - Politicians listing, filtering, search, and detail views
- **social-features.spec.ts** - Post interactions, likes, comments, shares, and user engagement

## Running Tests

### Local Development

```bash
# Install dependencies
npm install

# Run all tests
npm run test:e2e

# Run tests with UI mode (interactive)
npm run test:e2e:ui

# Run tests in debug mode
npm run test:e2e:debug

# Show test report
npm run test:e2e:report
```

### Docker Testing

```bash
# Run tests using Docker Compose (includes full stack)
docker-compose --profile testing up --build

# Run tests against local services
PLAYWRIGHT_BASE_URL=http://localhost:8080 npm run test:e2e
```

### CI/CD

Tests are configured to run in CI environments with:
- Headless mode
- Retry on failure
- Screenshot and video capture on failures
- HTML and JSON reporting

## Configuration

### Environment Variables

- `PLAYWRIGHT_BASE_URL` - Base URL for the application (default: http://localhost:8080)
- `CI` - Set to 'true' to enable CI optimizations

### Browser Configuration

Tests run on:
- **Desktop**: Chrome, Firefox
- **Mobile**: Chrome (Pixel 5), Safari (iPhone 12)

### Viewports

- **Mobile**: 375x667
- **Tablet**: 768x1024
- **Desktop**: 1280x720
- **Large**: 1920x1080

## Test Patterns

### Page Object Model

Tests use the Page Object Model pattern for maintainable and reusable test code:

```typescript
import { HomePage } from './pages/HomePage';

test('should navigate to about page', async ({ page }) => {
  const homePage = new HomePage(page);
  await homePage.goto();
  await homePage.navigateToAbout();
});
```

### Test Helpers

Common functionality is abstracted into helper classes:

```typescript
import { TestHelpers } from './utils/test-helpers';

const helpers = new TestHelpers(page);
await helpers.waitAndScreenshot('selector', 'screenshot-name');
```

### Mock Data

API responses are mocked using test fixtures:

```typescript
import { mockApiResponses } from './fixtures/test-data';

await page.route('**/api/feeds/**', route => {
  route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({ data: mockApiResponses.feedPosts }),
  });
});
```

## Test Data

### Test Users
- Predefined test users with credentials
- Different user types and permissions

### Mock API Responses
- Feed posts with various content types
- User profiles and authentication states
- Politicians data with voting records
- Error states and edge cases

### Test Selectors
- Standardized data-testid attributes
- Accessible selector strategies
- Mobile-responsive element targeting

## Screenshots and Videos

Test artifacts are saved to `test-results/`:
- Screenshots on failure
- Full page screenshots for visual verification
- Videos of failed test runs
- HTML reports with interactive results

## Best Practices

### Writing Tests

1. **Use Page Objects** - Encapsulate page logic in Page Object Models
2. **Mock API Calls** - Use fixtures to ensure consistent test data
3. **Test Multiple Viewports** - Verify responsive behavior
4. **Handle Loading States** - Wait for proper page loading before assertions
5. **Use Descriptive Names** - Test names should clearly describe what they verify

### Assertions

```typescript
// Good - Specific and clear
await expect(homePage.signInButton).toBeVisible();

// Better - With proper waiting
await homePage.expectSignInButtonVisible();
```

### Selectors

```typescript
// Preferred - Use data-testid
page.locator('[data-testid="sign-in-button"]')

// Acceptable - Semantic selectors
page.getByRole('button', { name: 'Sign In' })

// Avoid - Fragile CSS selectors
page.locator('.btn.btn-primary.auth-btn')
```

### Error Handling

```typescript
// Handle optional elements
if (await element.isVisible()) {
  await element.click();
}

// Use try-catch for expected failures
try {
  await page.waitForSelector('.optional-element', { timeout: 3000 });
  // Handle element if present
} catch {
  // Element not present, continue test
}
```

## Debugging Tests

### Local Debugging

```bash
# Run with headed browser
npm run test:e2e -- --headed

# Run specific test
npm run test:e2e -- auth.spec.ts

# Debug mode with browser dev tools
npm run test:e2e:debug
```

### CI Debugging

- Check screenshots and videos in test artifacts
- Review HTML report for detailed failure information
- Use `page.screenshot()` liberally for debugging complex interactions

## Contributing

When adding new tests:

1. **Add to appropriate spec file** or create new one for new features
2. **Update Page Objects** if adding new page interactions
3. **Add test data** to fixtures if needed
4. **Update documentation** for new test patterns or setup requirements
5. **Verify mobile compatibility** for all new tests

## Troubleshooting

### Common Issues

1. **Test timeouts** - Increase timeout values for slower API responses
2. **Element not found** - Verify selectors and wait conditions
3. **Flaky tests** - Add proper waits and handle race conditions
4. **Mobile test failures** - Check viewport-specific element visibility

### Performance

- Tests run in parallel by default
- Use `test.describe.serial()` for tests that must run sequentially
- Mock API calls to reduce external dependencies
- Consider test execution time when adding new test cases