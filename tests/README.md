# Playwright Testing Setup

This directory contains Playwright end-to-end tests for the Ratel application.

## Test Structure

### Test Cases
All tests are organized with the following naming convention:
- `[SU-XXX]` - Signup flow tests
- Screenshots are saved to `test-results/SU-XXX/` directories

### Current Test Suites

#### Anonymous User Signup Flow
- **SU-001**: Display signup option in login popup
- **SU-002**: Open user setup popup when clicking create account  
- **SU-003**: Validate email format in user setup
- **SU-004**: Handle complete signup flow (email signup)
- **SU-005**: Handle Google signup button visibility
- **SU-006**: Display profile image uploader
- **SU-007**: Show newsletter subscription option
- **SU-008**: Prevent signup with blocked keywords
- **SU-009**: Check username availability
- **SU-010**: Handle mobile responsive layout
- **SU-011**: Validate password requirements
- **SU-012**: Validate username format
- **SU-013**: Require terms of service agreement

## Running Tests Locally

### Prerequisites
1. Docker and Docker Compose installed
2. Node.js 20+ installed
3. All services running via `docker-compose up -d`

### Commands
```bash
# Install dependencies
npm ci

# Install Playwright browsers
npx playwright install --with-deps chromium

# Run all tests
npx playwright test

# Run specific test suite
npx playwright test --grep "SU-"

# Run specific test case
npx playwright test --grep "SU-004"

# Run tests with UI mode
npx playwright test --ui

# Generate and view test report
npx playwright show-report
```

## CI/CD Integration

### GitHub Actions Workflow
The tests are automatically run on every pull request via the `pr-workflow.yml` workflow.

#### What happens on PR:
1. **Services Setup**: PostgreSQL, Redis, main-api, and web frontend are started
2. **Test Execution**: All Playwright tests run against the live services
3. **Artifact Upload**: Screenshots and test results are uploaded as artifacts
4. **Report Generation**: HTML test report is generated
5. **GitHub Pages**: Test reports are deployed to GitHub Pages
6. **PR Comments**: Bot comments on PR with test results and links

#### Artifacts Generated:
- `playwright-test-results`: Complete test results and HTML reports
- `playwright-screenshots-{sha}`: All screenshots organized by test case

#### GitHub Pages Deployment:
- Test reports are deployed to: `https://username.github.io/repo-name/`
- Each PR gets its own report URL
- Reports include detailed test results, screenshots, and traces

### Test Report Features:
- ✅ **Interactive HTML Reports**: Detailed test results with filtering
- 📸 **Screenshot Documentation**: Visual verification of each test step
- 🔍 **Test Traces**: Step-by-step execution traces for debugging
- 📊 **Test Statistics**: Pass/fail rates and execution times
- 🔗 **PR Integration**: Direct links from PR comments

## Configuration

### Playwright Config (`playwright.config.ts`)
- **Base URL**: `http://localhost:8080` (CI) or configured local URL
- **Timeout**: 10 seconds (configurable via `tests/config.ts`)
- **Browsers**: Chromium (can be extended to Firefox, Safari)
- **Screenshots**: Captured on failure and at key test points

### Test Config (`tests/config.ts`)
```typescript
const timeout = 10000;
export const CONFIGS = {
  PAGE_WAIT_TIME: timeout,
  MODAL_WAIT_TIME: timeout,
  SELECTOR_WAIT_TIME: timeout,
  DEVICE_SCREEN_SIZES: {
    MOBILE: 768,
  },
  PLAYWRIGHT: {
    TIMEOUT: timeout,
    NAVIGATION_TIME_OUT: timeout,
    BASE_URL: "http://localhost:8080",
  },
};
```

## Best Practices

### Writing Tests
1. **Test IDs**: Use descriptive test IDs like `[SU-XXX]`
2. **Screenshots**: Take screenshots at key verification points
3. **Accessibility**: Use proper selectors (role, text, aria-labels)
4. **Waiting**: Use explicit waits instead of arbitrary timeouts
5. **Cleanup**: Tests should be independent and not rely on previous state

### Screenshot Organization
```
test-results/
├── SU-001/
│   ├── 01-login-popup-opened.png
│   └── 02-create-account-visible.png
├── SU-002/
│   ├── 01-login-popup-before-signup.png
│   └── 02-user-setup-popup-opened.png
└── ...
```

### Debugging Failed Tests
1. Check the HTML test report for detailed failure information
2. Review screenshots to see the visual state when tests failed
3. Use test traces to step through the exact actions taken
4. Check GitHub Actions logs for service startup issues
5. Run tests locally with `--debug` flag for interactive debugging

## Troubleshooting

### Common Issues
- **Services not ready**: Increase wait times in CI if services are slow to start
- **Element not found**: Check if UI has changed or accessibility attributes are missing
- **Timeout errors**: Verify network connectivity and service health
- **Screenshot differences**: Mobile/desktop viewport differences

### Local Development
- Ensure all services are running: `docker-compose ps`
- Check service health: `curl http://localhost:8080` and `curl http://localhost:3000/health`
- Clear test results: `rm -rf test-results/` and `rm -rf playwright-report/`