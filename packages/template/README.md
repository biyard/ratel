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
make generate NAME=space-action-poll
```

You will be prompted for:

```
Project description? [default: A Dioxus app]
Default dev server port? [default: 8000]
```

This creates the `packages/space-action-poll/` directory.

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

