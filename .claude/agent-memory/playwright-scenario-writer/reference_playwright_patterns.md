---
name: Playwright test patterns and selectors
description: Key UI selectors, test patterns, and component mappings discovered in the Ratel Playwright E2E tests
type: reference
---

## Test File Location and Config

- Tests go in `playwright/tests/users/` or `playwright/tests/spaces/`
- Config at `playwright/playwright.config.js`
- Test match pattern: `tests/users/**/*.spec.js` and `tests/spaces/**/*.spec.js`
- Auth dependency: tests depend on `auth-setup` project which saves `user.json` storage state
- Default BASE_URL: `http://localhost:8080` (configured via `PLAYWRIGHT_BASE_URL` env var)
- Default timeout: 5000ms (via `PLAYWRIGHT_TIMEOUT`)

## Utility Functions (tests/utils.js)

- `goto(page, path)` - navigates to BASE_URL + path, waits for networkidle and WASM hydration (`[data-dioxus-id]`)
- `click(page, opt)` - finds locator via getLocator, clicks, waits networkidle
- `fill(page, opt, value)` - finds locator via getLocator, fills value
- `getLocator(page, { placeholder, text, role, label, testId })` - finds element and asserts visible
- `getEditor(page)` - returns `[contenteditable]` locator (TiptapEditor)
- `waitPopup(page, { visible })` - waits for backdrop-blur popup to appear/disappear

## Key UI Selectors

### Team Creation
- Profile dropdown trigger: `page.getByRole("button", { name: "User Profile" })` -- DO NOT use text "User1" as it matches many post card elements
- "Create Team" button in dropdown (text match)
- Team form fields: `[data-pw="team-nickname-input"]`, `[data-pw="team-username-input"]`, `[data-pw="team-description-input"]`
- Team navigates to `/teams/{username}/home` after creation

### Team Sidemenu
- Menu items by text: "Home", "Drafts", "Manage Group", "Members", "DAO", "Rewards", "Settings"
- Drafts page URL: `/teams/{teamname}/drafts`
- IMPORTANT: After team creation, the sidemenu shows "Loading team..." while fetching data. Use `goto(page, url)` for direct navigation to sub-pages instead of clicking sidemenu links to avoid timing issues.

### Post Creation
- "Create Post" button: `aria_label: "Create Post"` (use `{ label: "Create Post" }`)
- Post edit URL: `/posts/{post_id}/edit`
- Skip space checkbox: `data-testid: "skip-space-checkbox"` (default checked = true, uncheck to enable space creation)
- "Go to Space" button text appears when checkbox unchecked
- "Publish" button text appears when checkbox checked

### Space Actions
- Actions page URL: `/spaces/{id}/actions`
- "Select Action Type" button opens layover modal (not popup)
- Action types in create modal: "Quiz" (default selected), "Poll", "Discussion", "Follow"
- "Create" button in modal footer to confirm action creation
- FAB may overlap buttons: hide via `document.querySelector('[class*="fixed right-4 bottom-4"]')`

### Action Creator Pages
- Discussion: URL `/actions/discussions/{id}`, fields: `placeholder="Enter discussion title..."`, `placeholder="Enter category (optional)..."`, TiptapEditor, Save button
- Poll: URL `/actions/polls/{id}`, fields: `placeholder="Enter poll title..."`, Tab to blur-save
- Quiz: URL `/actions/quizzes/{id}`, fields: `placeholder="Enter quiz title..."`, TiptapEditor, Save button
- Follow: URL `/actions/follows/{id}`, verify "General" tab visible

### Quiz Creator Page (Tabs & Question Editor)
- Tabs: "Overview" (default), "Upload", "Quiz", "Setting" -- select via `page.getByRole("tab", { name: "..." })`
- Overview tab: quiz title (`placeholder="Enter quiz title..."`), TiptapEditor for description, Save button
- Quiz tab: Pass Score input (type="number"), Retry Count input (type="number"), then question cards
- Add question button: `data-testid="quiz-add-question"` -- shows type selector
- Question type selector: "Single Choice" and "Multiple Choice" buttons
- Each question card has header "Question {N}" (e.g., "Question 1")
- Question title input: `placeholder="Input"` (shared by all question cards)
- Option inputs: `input[type="text"]` without placeholder, ordered after the question title in DOM
- Default options: "Option 1", "Option 2" (auto-created with each question)
- "Add Option" button adds another option to the question card
- Correct answer checkboxes: `label:has(input[type="checkbox"])` -- hidden checkbox with visible indicator div
- "Delete" button at bottom of each question card to remove a question
- Save is triggered automatically on blur of inputs; also on clicking checkboxes
- Use `input[type="text"]:visible` to match only question/option text inputs, excluding number inputs

### Space Publishing
- Dashboard URL: `/spaces/{id}/dashboard`
- "Publish" button in SpaceTop (only visible to creator, when not yet published)
- SpaceVisibilityModal: `data-testid="public-option"`, `data-testid="private-option"`
- Confirm button: `aria-label="Confirm visibility selection"`
