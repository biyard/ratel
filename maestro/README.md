# Maestro Mobile E2E Tests

Automated Android/iOS end-to-end tests using [Maestro](https://maestro.mobile.dev).

## Prerequisites

```bash
# Install maestro CLI
curl -Ls "https://get.maestro.mobile.dev" | bash

# Android emulator or physical device connected
adb devices   # should list at least one device

# Ratel mobile app installed on the device
# (run `cd app/ratel && make android` first)
```

## Run

```bash
cd maestro

# All scenarios
make test

# Single scenario
make test-login
make test-post
make test-settings
make test-error
make test-logout

# Specific device (when multiple connected)
make test DEVICE=emulator-5554
make test-login DEVICE=RFCY70NHC4H
```

## Scenarios

| File | What it tests |
|------|--------------|
| `01-login.yaml` | Email login flow (Sign In → email → password → signed-in home) |
| `02-create-post.yaml` | Post creation with editor (title + body typing + autosave) |
| `03-settings-theme-language.yaml` | Theme Dark/Light toggle + Language EN/KO switch |
| `04-error-page-recovery.yaml` | Error page "Go home" button when accessing protected route |
| `05-logout.yaml` | Logout via Settings → Sign In button reappears |

## Writing new scenarios

Maestro uses YAML. Key commands:

```yaml
- launchApp                          # start the app
- launchApp: { clearState: true }    # fresh install state
- tapOn: "Button text"               # tap by visible text
- tapOn: { id: "testId" }           # tap by accessibility ID
- assertVisible: "Expected text"
- assertNotVisible: "Gone text"
- inputText: "hello"                 # type into focused field
- scrollUntilVisible:
    element: "Target"
    direction: DOWN
- takeScreenshot: filename
- extendedWaitUntil:
    visible: "Loading done"
    timeout: 15000
```

Full reference: https://maestro.mobile.dev/api-reference/commands
