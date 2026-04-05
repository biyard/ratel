---
globs: ["app/ratel/**/*.rs"]
---

# Lint & Format

Apply both commands to every `.rs` file you created or modified before committing.

## Tailwind Class Sorting

```bash
rustywind --custom-regex "class: \"(.*)\"" -write <file>
```

Sorts Tailwind utility classes into a consistent order.

## Rust/RSX Formatting

```bash
dx fmt -f <file>
```

Formats Rust code and RSX markup via Dioxus formatter.

## Checking Dioxus Linting

```bash
dx check --web -f <file>
```
