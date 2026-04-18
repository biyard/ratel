# Fix GitHub PR Playwright Testing

Workflow for diagnosing and fixing the `playwright-tests` job in `.github/workflows/pr-workflow.yml`. Use this when CI Playwright fails but local `npx playwright test` passes — the gap is usually that CI tests against the production-built Docker image while local tests hit `dx serve` (dev mode).

## Step 1: Identify the Failing Run and Job

```bash
# List recent CI runs for the PR's branch
gh run list --repo biyard/ratel --branch <branch> --limit 5 \
  --json databaseId,status,conclusion,headSha

# Per-job status of a specific run
gh run view <run-id> --repo biyard/ratel --json jobs \
  | python3 -c "import json,sys; [print(f\"{j['name']}: {j['status']}/{j.get('conclusion','-')}\") for j in json.load(sys.stdin)['jobs']]"
```

If `playwright-tests` is the only failure, proceed. If builds also failed, fix those first via `workflows/fix-pr-testing.md`.

## Step 2: Download the Playwright Result Artifact

```bash
mkdir -p /tmp/pw-result && cd /tmp/pw-result
gh run download <run-id> --repo biyard/ratel --name playwright-test-results -D .
```

The artifact contains:

- `playwright-report/` — HTML report with each failing test's screenshot, video, trace, and `error-context.md`

## Step 3: Analyze the Trace

For each failing test:

```bash
# View the trace (opens browser viewer)
npx playwright show-trace /tmp/pw-result/playwright-report/data/<trace-hash>.zip

# Read the page snapshot at the failure point (often enough to identify the cause)
cat /tmp/pw-result/playwright-report/data/<test-dir>/error-context.md
```

Common failure signatures and their root causes:

| Snapshot text | Root cause |
|---|---|
| `Something went wrong` / `An unexpected error occurred` | Server function panicked or returned an unhandled error |
| `Your app is being rebuilt` / `Hot-patch success!` | Dev-server toast intercepting clicks (local-only — won't appear in CI) |
| element selector not found | UI changed → testid renamed/removed, or hydration race |
| `Test timeout of 60000ms exceeded` with no error context | Real timeout — page never loaded, or selector waits forever |
| `apiRequestContext._wrapApiCall: ENOENT … traces/...trace` | Test was killed mid-write — usually parallel-execution flake |

If the root cause is clear from the snapshot/trace, jump to Step 6.

## Step 4: Replay CI's Image Locally (when trace is inconclusive)

The fastest way to reproduce CI behavior is to run Playwright against the EXACT Docker image CI built.

```bash
# Find the image artifact name (it's keyed by the merge commit SHA)
gh api repos/biyard/ratel/actions/runs/<run-id>/artifacts \
  | python3 -c "import json,sys; [print(a['name']) for a in json.load(sys.stdin)['artifacts']]"

# Download and load
mkdir -p /tmp/pw-image && cd /tmp/pw-image
gh run download <run-id> --repo biyard/ratel --name app-shell-image-<sha> -D .
docker load < app-shell-image.tar.gz   # prints "Loaded image: ratel/app-shell:pr-<sha>"

# Bring up the testing infra against that exact image
cd /home/hackartist/data/devel/github.com/biyard/ratel
COMMIT=pr-<sha> make testing

# Wait for app-shell to come up
timeout 120 bash -c 'until curl -sf http://localhost:8080/ > /dev/null; do sleep 5; done'

# Run the failing tests against the CI image
cd playwright && PLAYWRIGHT_BASE_URL=http://localhost:8080 PLAYWRIGHT_TIMEOUT=60000 CI=true \
  npx playwright test <failing-spec> --workers=1
```

If the failure reproduces here, the bug is in app/ratel code (or the prod build). If it does NOT reproduce, the failure is environmental (DB state, timing, parallel race) — see Step 5.

## Step 5: Build a Local Docker Image (when CI artifact is unavailable or you need to test a fix)

Use this when (a) the CI artifact has expired (>7 days), (b) you've made a code fix and want to verify against a prod build before pushing, or (c) you need `RUST_LOG=debug` output that CI doesn't capture.

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel/app/ratel

# Build with the same env CI uses (see .github/workflows/pr-workflow.yml ratel-app job)
DYNAMO_TABLE_PREFIX=ratel-local \
ENV=local \
RUSTFLAGS='-D warnings' \
RUST_LOG=debug \
DYNAMO_ENDPOINT=http://localstack:4566 \
AWS_ACCESS_KEY_ID=test AWS_SECRET_ACCESS_KEY=test \
AWS_REGION=ap-northeast-2 AWS_ACCOUNT_ID=test \
COMMIT=local-test \
ECR=ratel/app-shell \
  make build-testing && \
  COMMIT=local-test ECR=ratel/app-shell make docker

# Bring up testing infra against the local image
cd /home/hackartist/data/devel/github.com/biyard/ratel
COMMIT=local-test make testing

# Tail server logs while tests run (separate terminal)
docker compose logs -f app-shell-testing

# Replay the failing tests
cd playwright && make test
# or a single spec:
cd playwright && PLAYWRIGHT_BASE_URL=http://localhost:8080 PLAYWRIGHT_TIMEOUT=60000 CI=true \
  npx playwright test tests/web/<file>.spec.js --workers=1
```

Notes:
- `ECR=ratel/app-shell` matches the image name in `docker-compose.yaml` (`app-shell-testing` service uses `ratel/app-shell:${COMMIT}`)
- `DYNAMO_ENDPOINT=http://localstack:4566` is the **container-internal** hostname; only the build env needs it set this way for parity with CI
- `RUST_LOG=debug` surfaces every server function call — invaluable for tracing why a test's API call returns the wrong data
- Use `COMMIT=local-test` (or any tag) to avoid clobbering a previously loaded CI image

## Step 6: Fix the Root Cause

Apply the fix in the relevant layer and re-run Step 4 or Step 5 to verify.

- **Server function returning unexpected error/panic**: `conventions/server-functions.md`, `conventions/error-handling.md`. Check `docker compose logs app-shell-testing` for the actual server-side error.
- **Test selector broken by UI change**: update `data-testid` in the RSX or the spec. See `conventions/playwright-tests.md`.
- **Hydration race (selector visible but click silently dropped)**: use `waitForHydrated(page, "<testid>")` from `playwright/tests/utils.js` before the click.
- **Dev-toast interference (local only)**: `goto()` in `utils.js` already calls `suppressDevToast()` — verify it's running.
- **Parallel-execution flake**: confirm by running `--workers=1` solo. If it passes solo, the test relies on shared state; either serialize via `test.describe.serial` or scope state per test.
- **References**: `conventions/playwright-tests.md`, `conventions/anti-patterns.md`
- **Skills**: `superpowers:systematic-debugging`

## Step 7: Verify

Re-run the **full** Playwright suite against the local prod image (Step 5) before pushing — fixing one spec sometimes breaks another:

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel
COMMIT=local-test make testing
cd playwright && CI=true make test
```

- **Skills**: `superpowers:verification-before-completion`

## Step 8: Push and Watch CI

Push to the `hackartists` fork (per project convention) and monitor:

```bash
git push hackartists <branch>
gh run watch --repo biyard/ratel $(gh run list --repo biyard/ratel --branch <branch> --limit 1 --json databaseId -q '.[0].databaseId')
```

If CI still fails, repeat from Step 2 with the new run ID.

## Cleanup

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel
docker compose --profile testing down --remove-orphans
docker image rm ratel/app-shell:pr-<sha> ratel/app-shell:local-test 2>/dev/null
```

## Checklist

- [ ] Identified the failing run + job
- [ ] Downloaded `playwright-test-results` artifact and read `error-context.md` for each failure
- [ ] Reproduced locally — either against the CI Docker image (Step 4) or a fresh local build (Step 5)
- [ ] Identified the root cause (not a guess)
- [ ] Applied fix in the correct layer (server / RSX / spec / utils)
- [ ] Full Playwright suite passes against the prod-built image with `CI=true`
- [ ] Pushed to `hackartists` fork and CI is green
- [ ] Cleaned up local Docker resources
