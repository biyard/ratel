# Repository Guidelines

## Project Structure & Modules
- Rust workspace in `packages/*` (e.g., `packages/main-api`, `packages/fetcher`, `packages/image-worker`, `packages/telegram-bot`). Shared types in `packages/dto`.
- Web app (Next.js) in `ts-packages/web`.
- E2E tests (Playwright) in `tests/`.
- Infra/CDK assets under `deps/rust-sdk/cdk` (deploy helper) and `cdk/` (web artifacts copy on deploy).
- Docker and local tooling: `docker-compose.yaml`, `scripts/`, `.build/` (generated artifacts), `benchmark/` fixtures.

## Build, Test, and Dev
- Web (Next.js):
  - Dev: `pnpm -C ts-packages/web dev`
  - Build/Start: `pnpm -C ts-packages/web build && pnpm -C ts-packages/web start`
  - Lint: `pnpm -C ts-packages/web lint`
- Rust services (per package):
  - Dev (watch): `cd packages/main-api && make run`
  - Test: `cd packages/main-api && make test`
  - Build (lambda target): `cd packages/main-api && make build`
  - Replace `main-api` with the target service as needed.
- Top-level helpers:
  - Local stack: `make run` (generates `.build/evm-keys.json`, starts containers)
  - E2E: `make test` or `pnpm -C ts-packages/web test:e2e`

## Coding Style & Naming
- Rust: edition 2024; deny warnings. Run `cargo fmt --all` and prefer `snake_case` modules, `PascalCase` types, `camelCase` locals.
- TypeScript/React: ESLint + Prettier via `pnpm -C ts-packages/web lint`. Use `PascalCase` for components, `camelCase` for variables, and colocate UI under `ts-packages/web/src`.
- Commits should be imperative and scoped (e.g., "fix: z-index on header").

## Testing Guidelines
- E2E: Playwright specs in `tests/*.spec.ts`. Use clear IDs/roles; add screenshots when debugging. Run with `make test`.
- Rust: add unit tests in-module (`mod tests`) and integration tests under each crate’s `tests/`. Run with `make test` in the target package.

## Pull Requests
- Describe what/why, link issues, and note any env or migration changes.
- Include repro steps and, for UI changes, screenshots or a short clip.
- Add/adjust tests for new behavior.

## Security & Config
- Never commit secrets. Use `.env.local` (web) or env vars for services; see `ts-packages/web/setup-env.sh` for examples.
- AWS credentials are required for deployment-related Make targets; avoid running deploys from forks.
