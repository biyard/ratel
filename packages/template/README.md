# Dioxus App Template

A `cargo-generate` template for scaffolding new Dioxus app packages.

## Prerequisites

```bash
cargo binstall cargo-generate
```

## Usage

```bash
cd packages/template
make generate NAME=<package-name>
```

### Example

```bash
cd packages/template
make generate NAME=space-action-vote
```

You will be prompted for:

```
Project description? [default: A Dioxus app]
Default dev server port? [default: 8000]
```

This creates the `packages/space-action-vote/` directory.

## Generated Structure

```
<package-name>/
├── .cargo/config.toml       # Release build profile
├── Cargo.toml               # Uses workspace dependencies
├── Dioxus.toml              # Dioxus configuration
├── Makefile                 # run / build commands
├── clippy.toml              # Dioxus clippy settings
├── assets/
│   └── favicon.ico
└── src/
    ├── main.rs              # Entry point (web/server feature gating)
    ├── lib.rs               # Module declarations
    ├── config.rs            # Configuration management
    ├── route.rs             # Routing
    ├── layout.rs            # Layout component
    ├── components/mod.rs
    ├── controllers/mod.rs
    ├── hooks/mod.rs
    ├── models/mod.rs
    ├── views/
    │   ├── mod.rs
    │   └── home.rs          # Home page
    ├── web/mod.rs           # Web launcher
    └── server/mod.rs        # Server (session + DynamoDB)
```

## Running After Generation

```bash
cd packages/<package-name>
make run
```

## Placeholders

| Placeholder | Type | Description | Default |
|---|---|---|---|
| `project-name` | built-in | Package name (kebab-case) | from `--name` flag |
| `crate_name` | built-in | Crate name (snake_case) | auto-derived |
| `description` | interactive | Project description | `A Dioxus app` |
| `port` | interactive | Dev server port | `8000` |
