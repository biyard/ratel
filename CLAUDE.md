# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ratel is a decentralized legislative platform built with Rust and TypeScript, designed to bridge the gap between crypto users and policymakers. The project consists of multiple Rust services and a Next.js web frontend.

## Architecture

This is a monorepo with a workspace structure:
- **packages/** - Rust workspace packages (APIs, workers, shared DTOs)
- **ts-packages/** - TypeScript packages (Next.js web frontend)
- **deps/** - Shared Rust SDK dependencies
- **benchmark/** - Performance testing tools
- **kaia/** - Blockchain contracts
- **tests/** - Playwright integration tests

### Key Components

- **main-api** (`packages/main-api/`) - Primary REST API built with Axum
- **fetcher** (`packages/fetcher/`) - Data fetching service for legislative information
- **image-worker** (`packages/image-worker/`) - Image processing service
- **telegram-bot** (`packages/telegram-bot/`) - Telegram integration
- **dto** (`packages/dto/`) - Shared data transfer objects
- **web** (`ts-packages/web/`) - Next.js frontend with React 19

## Build System

### Rust Services
- Rust workspace managed by Cargo
- Each service has its own Makefile
- Uses custom build profiles (wasm-dev, server-dev, android-dev)
- Common dependencies defined in workspace Cargo.toml

### Frontend
- Next.js with TypeScript
- Package manager: pnpm
- Uses Tailwind CSS v4

## Development Commands

### Docker (Recommended for Local Development)
```bash
# Start all services with Docker Compose
docker-compose up -d

# Start specific services
docker-compose up -d postgres redis hasura main-api web

# Include telegram bot (requires TELEGRAM_TOKEN)
docker-compose --profile telegram up -d

# View logs
docker-compose logs -f main-api
docker-compose logs -f web

# Stop all services
docker-compose down

# Rebuild services after code changes
docker-compose build main-api
docker-compose up -d main-api
```

### Manual Service Development
```bash
# Run main API
cd packages/main-api && make run

# Run web frontend  
cd ts-packages/web && make run

# Run any service via root Makefile
make run SERVICE=main-api
make serve SERVICE=main-api
```

### Building
```bash
# Build specific service
make build SERVICE=main-api

# Build for different environments
ENV=dev make build SERVICE=main-api
```

### Testing
```bash
# Run Rust tests for main-api
cd packages/main-api && make test

# Run Playwright tests (from root)
make test
# or
npx playwright test
```

### Linting/Formatting
```bash
# Next.js linting
cd ts-packages/web && npm run lint
```

## Key Technologies

- **Backend**: Rust, Axum, SQLx (PostgreSQL/SQLite), Tokio
- **Frontend**: Next.js 15, React 19, TailwindCSS v4, Apollo GraphQL
- **Testing**: Playwright for E2E tests
- **Infrastructure**: AWS (Lambda, S3, RDS), Docker
- **Blockchain**: Ethereum-compatible contracts

## Database

- Uses SQLx for database operations
- Supports both PostgreSQL (production) and SQLite (development)
- Database migrations handled automatically when MIGRATE=true

## Docker Services

The docker-compose.yaml provides:

- **postgres** - PostgreSQL database (port 5432)
- **redis** - Redis cache (port 6379)  
- **hasura** - GraphQL API (port 28080)
- **main-api** - REST API (port 3000)
- **fetcher** - Legislative data fetching (port 3001)
- **image-worker** - Image processing service
- **telegram-bot** - Telegram bot (optional, requires TELEGRAM_TOKEN)
- **web** - Next.js frontend (port 3002)

Access points:
- Web Application: http://localhost:3002
- Main API: http://localhost:3000
- Hasura Console: http://localhost:28080
- Fetcher API: http://localhost:3001

## Environment Configuration

Copy `.env.example` to `.env` and configure:
- `TELEGRAM_TOKEN` - Required for telegram bot functionality
- AWS credentials - Leave empty for local development
- Other optional integrations (Slack, OpenAPI)

## Development Notes

- The web frontend requires environment setup via `setup-env.sh`
- Services use environment variables for configuration
- AWS integration for cloud deployments (disabled in local Docker setup)
- Telegram SDK integration for bot functionality
- Real-time features using WebSockets and collaborative editing
- Database migrations run automatically on startup when MIGRATE=true