# Admin Memberships Page - Testing Guide

## Overview

This directory contains the admin membership management page and its comprehensive Playwright test suite.

## Files

- **`memberships-page.tsx`** - Main page component (exported and referenced in `router.tsx`)
- **`use-memberships-page-controller.tsx`** - Page controller logic
- **`use-memberships-data.tsx`** - Data fetching hooks
- **`memberships-page.admin.spec.tsx`** - Playwright E2E tests

**Note**: This project uses Vite/React with React Router. Routes are defined in `/ts-packages/web/src/router.tsx`, not using Next.js page router convention.

## Running Tests

```bash
# From ts-packages/web directory
make test

# Or directly with Playwright
npx playwright test memberships-page.admin.spec.tsx --project="Admin tests"
```

## Test Prerequisites

⚠️ **IMPORTANT**: For tests to run successfully, the admin user must have proper permissions in the database.

### Required Setup

The admin user `admin@ratel.foundation` must have `user_type = 98` in the DynamoDB database.

If tests are being skipped with warnings about redirection, run this SQL to fix:

```sql
-- Note: Adjust syntax for DynamoDB
UPDATE users
SET user_type = 98
WHERE email = 'admin@ratel.foundation';
```

### What Gets Tested

The test suite covers:

✅ **Authentication & Access Control**
- Admin user can access the page
- Non-admin users are properly redirected
- Page loads with proper content

✅ **Create Membership Flow**
- Open create form modal
- Fill in all form fields
- Toggle "Infinite Duration" checkbox
- Toggle "Unlimited Credits Per Space" checkbox
- Cancel without saving
- Submit and create new membership

✅ **Edit Operations** (if memberships exist)
- Open edit form with pre-filled data
- "Is Active" toggle visibility
- Modify and save changes
- Cancel without saving

✅ **Delete Operations** (if memberships exist)
- Open delete confirmation dialog
- Confirm deletion
- Cancel deletion

✅ **Form Validation**
- Numeric fields have correct input types
- Required field validation
- Checkbox functionality

✅ **Page States**
- Loading state
- Empty state (no memberships)
- Error state
- Populated state (with memberships)

### Test Structure

Tests are organized into describe blocks:
1. `Authentication & Access Control` - Verifies access permissions
2. `Create Membership Flow` - Tests creating new memberships
3. `Edit and Delete Operations` - Tests modification workflows
4. `Form Validation` - Validates input constraints
5. `Page State Handling` - Ensures proper UI states

### Test IDs

Each test has a unique ID for tracking:
- `[MP-001]` through `[MP-010]`

### Graceful Degradation

The tests are designed to gracefully handle different environments:
- **No admin permissions**: Tests skip with clear warnings
- **No memberships**: Edit/delete tests skip gracefully
- **API errors**: Tests report the error state

### Debugging Failed Tests

If tests fail, check:

1. **Admin permissions**: Is `admin@ratel.foundation` set to `user_type = 98`?
2. **Backend API**: Is the main-api service running on port 3000?
3. **Database**: Is DynamoDB accessible and populated?
4. **Frontend**: Is the web service running on port 8080?

### Console Output

Tests provide helpful console output:
- ✓ Success markers for passed checks
- ⚠️ Warning markers for skipped tests
- Detailed state information for debugging

### Example Output

```
Current URL: http://localhost:8080/
⚠️  User was redirected from /admin/memberships - admin user may not have user_type=98
   To fix: Ensure admin@ratel.foundation has user_type=98 in the database

10 skipped
1 passed (10.8s)
```

## Development

When modifying the page, ensure:
1. All existing tests still pass
2. New features have corresponding tests
3. Tests follow the established patterns
4. Console output remains helpful

## Related Files

- Backend API: `/packages/main-api/src/controllers/m3/memberships/`
- Frontend features: `/ts-packages/web/src/features/membership/`
- i18n translations: `/ts-packages/web/src/features/membership/i18n.tsx`
